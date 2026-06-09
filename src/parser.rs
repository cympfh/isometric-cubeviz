use crate::model::{Color, Cube};

pub fn parse_state(input: &str) -> Result<Cube, String> {
    let mut size: Option<usize> = None;
    let face_names = ["U", "F", "R"];
    let mut face_strings: [Option<String>; 3] = [None, None, None];

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
            continue;
        }

        for (i, name) in face_names.iter().enumerate() {
            let prefix = format!("{name}:");
            if let Some(rest) = line.strip_prefix(prefix.as_str()) {
                face_strings[i] = Some(rest.to_string());
                break;
            }
        }
    }

    let n = size.ok_or_else(|| "missing size line".to_string())?;
    if n < 3 {
        return Err(format!("size must be >= 3, got {n}"));
    }

    let expected = n * n;
    let mut faces_vec: Vec<Vec<Vec<Color>>> = Vec::with_capacity(6);
    for (i, name) in face_names.iter().enumerate() {
        match face_strings[i].as_deref() {
            None => {
                faces_vec.push(vec![vec![Color::H; n]; n]);
            }
            Some(raw) => {
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
}
