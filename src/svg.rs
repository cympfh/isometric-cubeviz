use crate::model::{BackgroundStyle, BorderStyle, Cube, ViewMode};

const CELL: f64 = 20.0;
const THICKNESS_RATIO: f64 = 0.15;

struct Proj {
    dx_x: f64,
    dx_y: f64,
    dz_x: f64,
    dz_y: f64,
    dy_y: f64,
}

impl Proj {
    fn for_view(view: ViewMode) -> Self {
        match view {
            ViewMode::Balanced => Self {
                dx_x: 0.866,
                dx_y: 0.500,
                dz_x: -0.866,
                dz_y: 0.500,
                dy_y: -1.000,
            },
            ViewMode::Top => Self {
                dx_x: 0.866,
                dx_y: 0.300,
                dz_x: -0.866,
                dz_y: 0.300,
                dy_y: -1.000,
            },
            ViewMode::Side => Self {
                dx_x: 0.866,
                dx_y: 0.700,
                dz_x: -0.866,
                dz_y: 0.700,
                dy_y: -0.700,
            },
        }
    }

    fn project(&self, gx: f64, gy: f64, gz: f64) -> (f64, f64) {
        let sx = gx * self.dx_x * CELL + gz * self.dz_x * CELL;
        let sy = gx * self.dx_y * CELL + gz * self.dz_y * CELL + gy * self.dy_y * CELL;
        (sx, sy)
    }
}

fn border_width(style: BorderStyle) -> f64 {
    match style {
        BorderStyle::Thin => 0.5,
        BorderStyle::Normal => 1.0,
        BorderStyle::Thick => 2.0,
    }
}

fn polygon_points(proj: &Proj, corners: &[(f64, f64, f64)]) -> String {
    corners
        .iter()
        .map(|&(gx, gy, gz)| {
            let (sx, sy) = proj.project(gx, gy, gz);
            format!("{:.3},{:.3}", sx, sy)
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn sticker(proj: &Proj, corners: &[(f64, f64, f64)], fill: &str, stroke_width: f64) -> String {
    let pts = polygon_points(proj, corners);
    format!(
        "<polygon points=\"{pts}\" fill=\"{fill}\" stroke=\"#222222\" stroke-width=\"{stroke_width}\" stroke-linejoin=\"round\"/>"
    )
}

fn compute_viewbox(proj: &Proj, n: usize, thickness: bool) -> (f64, f64, f64, f64) {
    let nf = n as f64;
    let mut xs: Vec<f64> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();

    for gx in [0.0, nf] {
        for gy in [0.0, nf] {
            for gz in [0.0, nf] {
                let (sx, sy) = proj.project(gx, gy, gz);
                xs.push(sx);
                ys.push(sy);
            }
        }
    }

    if thickness {
        let th = CELL * THICKNESS_RATIO;
        for gx in [0.0, nf] {
            for gz in [0.0, nf] {
                let (sx, sy) = proj.project(gx, 0.0, gz);
                xs.push(sx);
                ys.push(sy + th);
            }
        }
    }

    let min_x = xs.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_x = xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_y = ys.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_y = ys.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let pad = CELL * 0.5;
    (
        min_x - pad,
        min_y - pad,
        max_x - min_x + 2.0 * pad,
        max_y - min_y + 2.0 * pad,
    )
}

pub fn render(
    cube: &Cube,
    view: ViewMode,
    border: BorderStyle,
    thickness: bool,
    background: BackgroundStyle,
) -> String {
    let n = cube.size;
    let nf = n as f64;
    let proj = Proj::for_view(view);
    let sw = border_width(border);
    let (vx, vy, vw, vh) = compute_viewbox(&proj, n, thickness);

    let mut out = String::new();

    out.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{vx:.3} {vy:.3} {vw:.3} {vh:.3}">"#
    ));
    out.push_str("<title>iso-cubeviz</title>");
    out.push_str(&format!(
        "<desc>Isometric view of a {n}x{n}x{n} Rubik's cube</desc>"
    ));

    match background {
        BackgroundStyle::Transparent => {}
        BackgroundStyle::Light => {
            out.push_str(&format!(
                "<rect x=\"{vx:.3}\" y=\"{vy:.3}\" width=\"{vw:.3}\" height=\"{vh:.3}\" fill=\"#E8E8E8\"/>"
            ));
        }
        BackgroundStyle::White => {
            out.push_str(&format!(
                "<rect x=\"{vx:.3}\" y=\"{vy:.3}\" width=\"{vw:.3}\" height=\"{vh:.3}\" fill=\"#FFFFFF\"/>"
            ));
        }
    }

    if thickness {
        let th = CELL * THICKNESS_RATIO;
        let (bfl_x, bfl_y) = proj.project(0.0, 0.0, 0.0);
        let (bfr_x, bfr_y) = proj.project(nf, 0.0, 0.0);
        let (brr_x, brr_y) = proj.project(nf, 0.0, nf);
        let (blr_x, blr_y) = proj.project(0.0, 0.0, nf);

        let bottom_f = format!(
            "<polygon points=\"{:.3},{:.3} {:.3},{:.3} {:.3},{:.3} {:.3},{:.3}\" fill=\"#333333\" stroke=\"none\"/>",
            bfl_x,
            bfl_y,
            bfr_x,
            bfr_y,
            bfr_x,
            bfr_y + th,
            bfl_x,
            bfl_y + th,
        );
        out.push_str(&bottom_f);

        let bottom_r = format!(
            "<polygon points=\"{:.3},{:.3} {:.3},{:.3} {:.3},{:.3} {:.3},{:.3}\" fill=\"#2A2A2A\" stroke=\"none\"/>",
            bfr_x,
            bfr_y,
            brr_x,
            brr_y,
            brr_x,
            brr_y + th,
            bfr_x,
            bfr_y + th,
        );
        out.push_str(&bottom_r);

        let bottom_l = format!(
            "<polygon points=\"{:.3},{:.3} {:.3},{:.3} {:.3},{:.3} {:.3},{:.3}\" fill=\"#2A2A2A\" stroke=\"none\"/>",
            bfl_x,
            bfl_y,
            blr_x,
            blr_y,
            blr_x,
            blr_y + th,
            bfl_x,
            bfl_y + th,
        );
        out.push_str(&bottom_l);
    }

    for row in 0..n {
        let r = row as f64;
        for col in 0..n {
            let c = col as f64;
            let fill = cube.faces[0][row][col].hex();
            let corners = [
                (c, nf, r),
                (c + 1.0, nf, r),
                (c + 1.0, nf, r + 1.0),
                (c, nf, r + 1.0),
            ];
            out.push_str(&sticker(&proj, &corners, fill, sw));
        }
    }

    for row in 0..n {
        let r = row as f64;
        for col in 0..n {
            let c = col as f64;
            let fill = cube.faces[1][row][col].hex();
            let corners = [
                (c, nf - r, nf),
                (c + 1.0, nf - r, nf),
                (c + 1.0, nf - r - 1.0, nf),
                (c, nf - r - 1.0, nf),
            ];
            out.push_str(&sticker(&proj, &corners, fill, sw));
        }
    }

    for row in 0..n {
        let r = row as f64;
        for col in 0..n {
            let c = col as f64;
            let fill = cube.faces[2][row][col].hex();
            let corners = [
                (nf, nf - r, c),
                (nf, nf - r, c + 1.0),
                (nf, nf - r - 1.0, c + 1.0),
                (nf, nf - r - 1.0, c),
            ];
            out.push_str(&sticker(&proj, &corners, fill, sw));
        }
    }

    out.push_str("</svg>");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{BackgroundStyle, BorderStyle, Color, Cube, ViewMode};

    fn solved_cube(n: usize) -> Cube {
        let faces = std::array::from_fn(|i| {
            let color = [Color::W, Color::G, Color::R][i];
            vec![vec![color; n]; n]
        });
        Cube { size: n, faces }
    }

    #[test]
    fn render_has_svg_tags() {
        let cube = solved_cube(3);
        let svg = render(
            &cube,
            ViewMode::Balanced,
            BorderStyle::Normal,
            false,
            BackgroundStyle::Transparent,
        );
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn render_has_viewbox() {
        let cube = solved_cube(3);
        let svg = render(
            &cube,
            ViewMode::Balanced,
            BorderStyle::Normal,
            false,
            BackgroundStyle::Transparent,
        );
        assert!(svg.contains("viewBox="));
    }

    #[test]
    fn render_has_title_and_desc() {
        let cube = solved_cube(3);
        let svg = render(
            &cube,
            ViewMode::Balanced,
            BorderStyle::Normal,
            false,
            BackgroundStyle::Transparent,
        );
        assert!(svg.contains("<title>"));
        assert!(svg.contains("<desc>"));
    }

    #[test]
    fn render_polygon_count_3x3() {
        let cube = solved_cube(3);
        let svg = render(
            &cube,
            ViewMode::Balanced,
            BorderStyle::Normal,
            false,
            BackgroundStyle::Transparent,
        );
        let count = svg.matches("<polygon").count();
        assert_eq!(count, 27);
    }

    #[test]
    fn render_polygon_count_4x4() {
        let cube = solved_cube(4);
        let svg = render(
            &cube,
            ViewMode::Balanced,
            BorderStyle::Normal,
            false,
            BackgroundStyle::Transparent,
        );
        let count = svg.matches("<polygon").count();
        assert_eq!(count, 48);
    }

    #[test]
    fn render_views_differ() {
        let cube = solved_cube(3);
        let balanced = render(
            &cube,
            ViewMode::Balanced,
            BorderStyle::Normal,
            false,
            BackgroundStyle::Transparent,
        );
        let top = render(
            &cube,
            ViewMode::Top,
            BorderStyle::Normal,
            false,
            BackgroundStyle::Transparent,
        );
        assert_ne!(balanced, top);
    }

    #[test]
    fn render_background_white() {
        let cube = solved_cube(3);
        let svg = render(
            &cube,
            ViewMode::Balanced,
            BorderStyle::Normal,
            false,
            BackgroundStyle::White,
        );
        assert!(svg.contains("<rect"));
        assert!(svg.contains("#FFFFFF"));
    }

    #[test]
    fn render_thickness_adds_elements() {
        let cube = solved_cube(3);
        let without = render(
            &cube,
            ViewMode::Balanced,
            BorderStyle::Normal,
            false,
            BackgroundStyle::Transparent,
        );
        let with_thickness = render(
            &cube,
            ViewMode::Balanced,
            BorderStyle::Normal,
            true,
            BackgroundStyle::Transparent,
        );
        assert!(with_thickness.len() > without.len());
    }
}
