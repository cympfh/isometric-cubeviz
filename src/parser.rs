use crate::model::{Color, Cube};

/// state file に記述できるオプション。未指定は `None`。
#[derive(Debug, Clone, Default)]
pub struct StateOptions {
    pub view: Option<String>,
    pub border: Option<String>,
    pub background: Option<String>,
}

const OPTION_KEYS: [&str; 3] = ["view", "border", "background"];

/// state file から `view:` / `border:` / `background:` 行を抽出する。
pub fn parse_options(input: &str) -> StateOptions {
    let mut opts = StateOptions::default();
    for line in input.lines() {
        let line = line.trim();
        for key in OPTION_KEYS {
            let prefix = format!("{key}:");
            if let Some(rest) = line.strip_prefix(prefix.as_str()) {
                let value = rest.trim().to_string();
                match key {
                    "view" => opts.view = Some(value),
                    "border" => opts.border = Some(value),
                    "background" => opts.background = Some(value),
                    _ => unreachable!(),
                }
                break;
            }
        }
    }
    opts
}

fn is_option_line(line: &str) -> bool {
    OPTION_KEYS
        .iter()
        .any(|key| line.strip_prefix(&format!("{key}:")).is_some())
}

pub fn parse_state(input: &str) -> Result<Cube, String> {
    let mut size: Option<usize> = None;
    let face_names = ["U", "F", "R"];
    let mut face_strings: [String; 3] = [String::new(), String::new(), String::new()];
    let mut current_face: Option<usize> = None;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(rest) = line.strip_prefix("size:") {
            let n: usize = rest
                .trim()
                .parse()
                .map_err(|_| format!("invalid size: '{}'", rest.trim()))?;
            size = Some(n);
            current_face = None;
            continue;
        }

        if is_option_line(line) {
            current_face = None;
            continue;
        }

        let mut matched = false;
        for (i, name) in face_names.iter().enumerate() {
            let prefix = format!("{name}:");
            if let Some(rest) = line.strip_prefix(prefix.as_str()) {
                current_face = Some(i);
                face_strings[i].push_str(rest);
                matched = true;
                break;
            }
        }
        if !matched && let Some(i) = current_face {
            face_strings[i].push_str(line);
        }
    }

    let n = size.ok_or_else(|| "missing size line".to_string())?;
    if n < 3 {
        return Err(format!("size must be >= 3, got {n}"));
    }

    let expected = n * n;
    let mut faces_vec: Vec<Vec<Vec<Color>>> = Vec::with_capacity(6);
    for (i, name) in face_names.iter().enumerate() {
        let raw = &face_strings[i];
        if raw.chars().all(|c| c.is_whitespace()) {
            faces_vec.push(vec![vec![Color::H; n]; n]);
        } else {
            let raw = raw.as_str();
            {
                let chars: Vec<char> = raw.chars().filter(|c| !c.is_whitespace()).collect();
                if chars.len() != expected {
                    return Err(format!(
                        "face {name}: expected {expected} colors, got {}",
                        chars.len()
                    ));
                }
                let mut rows = Vec::with_capacity(n);
                for row_chars in chars.chunks(n) {
                    let row: Result<Vec<Color>, String> = row_chars
                        .iter()
                        .map(|&c| {
                            Color::from_char(c)
                                .ok_or_else(|| format!("unknown color char '{c}' in face {name}"))
                        })
                        .collect();
                    rows.push(row?);
                }
                faces_vec.push(rows);
            }
        }
    }

    let faces = faces_vec
        .try_into()
        .map_err(|_| "internal error: face count mismatch".to_string())?;

    Ok(Cube { size: n, faces })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Color;

    const SOLVED_3X3: &str = "\
size: 3
U: WWWWWWWWW
F: GGGGGGGGG
R: RRRRRRRRR";

    #[test]
    fn parse_solved_3x3() {
        let cube = parse_state(SOLVED_3X3).unwrap();
        assert_eq!(cube.size, 3);
        assert!(cube.faces[0].iter().flatten().all(|&c| c == Color::W));
        assert!(cube.faces[1].iter().flatten().all(|&c| c == Color::G));
        assert!(cube.faces[2].iter().flatten().all(|&c| c == Color::R));
    }

    #[test]
    fn parse_ignores_whitespace() {
        let input = "\
size: 3
U: W W W W W W W W W
F: G G G G G G G G G
R: R R R R R R R R R";
        let cube = parse_state(input).unwrap();
        assert_eq!(cube.size, 3);
        assert!(cube.faces[0].iter().flatten().all(|&c| c == Color::W));
    }

    #[test]
    fn parse_4x4() {
        let input = "\
size: 4
U: WWWWWWWWWWWWWWWW
F: GGGGGGGGGGGGGGGG
R: RRRRRRRRRRRRRRRR";
        let cube = parse_state(input).unwrap();
        assert_eq!(cube.size, 4);
        for face in &cube.faces {
            assert_eq!(face.len(), 4);
            assert_eq!(face[0].len(), 4);
        }
    }

    #[test]
    fn parse_error_wrong_length() {
        let input = "\
size: 3
U: WWWWWWWW
F: GGGGGGGGG
R: RRRRRRRRR";
        assert!(parse_state(input).is_err());
    }

    #[test]
    fn parse_error_unknown_color() {
        let input = "\
size: 3
U: WWWWWWWWX
F: GGGGGGGGG
R: RRRRRRRRR";
        assert!(parse_state(input).is_err());
    }

    #[test]
    fn parse_error_invalid_size() {
        let input = "\
size: 2
U: WWWW
F: GGGG
R: RRRR";
        assert!(parse_state(input).is_err());
    }

    #[test]
    fn parse_options_extracts_values() {
        let input = "\
size: 3
view: balanced
border: thick
background: white
U: WWWWWWWWW
F: GGGGGGGGG
R: RRRRRRRRR";
        let opts = parse_options(input);
        assert_eq!(opts.view.as_deref(), Some("balanced"));
        assert_eq!(opts.border.as_deref(), Some("thick"));
        assert_eq!(opts.background.as_deref(), Some("white"));
    }

    #[test]
    fn parse_options_absent_is_none() {
        let opts = parse_options(SOLVED_3X3);
        assert!(opts.view.is_none());
        assert!(opts.border.is_none());
        assert!(opts.background.is_none());
    }

    #[test]
    fn parse_state_ignores_option_lines() {
        let input = "\
size: 3
view: balanced
U: WWWWWWWWW
border: thin
F: GGGGGGGGG
R: RRRRRRRRR";
        let cube = parse_state(input).unwrap();
        assert_eq!(cube.size, 3);
        assert!(cube.faces[0].iter().flatten().all(|&c| c == Color::W));
        assert!(cube.faces[1].iter().flatten().all(|&c| c == Color::G));
        assert!(cube.faces[2].iter().flatten().all(|&c| c == Color::R));
    }

    #[test]
    fn parse_multiline_face() {
        let input = "\
size: 3
U: RRR
RRR
RRR
F: BBBBBBBBB
R:
GGG
GGG
GGG";
        let cube = parse_state(input).unwrap();
        assert_eq!(cube.size, 3);
        assert!(cube.faces[0].iter().flatten().all(|&c| c == Color::R));
        assert!(cube.faces[1].iter().flatten().all(|&c| c == Color::B));
        assert!(cube.faces[2].iter().flatten().all(|&c| c == Color::G));
    }
}
