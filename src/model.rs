use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    W,
    Y,
    G,
    B,
    R,
    O,
    K,
    H,
}

impl Color {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'W' => Some(Self::W),
            'Y' => Some(Self::Y),
            'G' => Some(Self::G),
            'B' => Some(Self::B),
            'R' => Some(Self::R),
            'O' => Some(Self::O),
            'K' => Some(Self::K),
            'H' => Some(Self::H),
            _ => None,
        }
    }

    pub fn hex(self) -> &'static str {
        match self {
            Self::W => "#FFFFFF",
            Self::Y => "#FFD500",
            Self::G => "#009B48",
            Self::B => "#0046AD",
            Self::R => "#B71234",
            Self::O => "#FF5800",
            Self::K => "#1A1A1A",
            Self::H => "#808080",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cube {
    pub size: usize,
    pub faces: [Vec<Vec<Color>>; 3],
}

#[derive(Debug, Clone, Copy)]
pub enum ViewMode {
    Balanced,
    Top,
    Side,
}

impl FromStr for ViewMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "balanced" => Ok(Self::Balanced),
            "top" => Ok(Self::Top),
            "side" => Ok(Self::Side),
            _ => Err(format!("unknown view mode: '{s}'")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BorderStyle {
    Thin,
    Normal,
    Thick,
}

impl FromStr for BorderStyle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "thin" => Ok(Self::Thin),
            "normal" => Ok(Self::Normal),
            "thick" => Ok(Self::Thick),
            _ => Err(format!("unknown border style: '{s}'")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BackgroundStyle {
    Transparent,
    Light,
    White,
}

impl FromStr for BackgroundStyle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "transparent" => Ok(Self::Transparent),
            "light" => Ok(Self::Light),
            "white" => Ok(Self::White),
            _ => Err(format!("unknown background style: '{s}'")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_from_char_all() {
        assert_eq!(Color::from_char('W'), Some(Color::W));
        assert_eq!(Color::from_char('Y'), Some(Color::Y));
        assert_eq!(Color::from_char('G'), Some(Color::G));
        assert_eq!(Color::from_char('B'), Some(Color::B));
        assert_eq!(Color::from_char('R'), Some(Color::R));
        assert_eq!(Color::from_char('O'), Some(Color::O));
        assert_eq!(Color::from_char('K'), Some(Color::K));
        assert_eq!(Color::from_char('H'), Some(Color::H));
        assert_eq!(Color::from_char('X'), None);
        assert_eq!(Color::from_char('w'), None);
    }

    #[test]
    fn color_hex_format() {
        assert_eq!(Color::W.hex(), "#FFFFFF");
        assert_eq!(Color::Y.hex(), "#FFD500");
        assert_eq!(Color::G.hex(), "#009B48");
        assert_eq!(Color::B.hex(), "#0046AD");
        assert_eq!(Color::R.hex(), "#B71234");
        assert_eq!(Color::O.hex(), "#FF5800");
        assert_eq!(Color::K.hex(), "#1A1A1A");
        assert_eq!(Color::H.hex(), "#808080");
    }

    #[test]
    fn view_mode_from_str() {
        assert!(matches!(
            "balanced".parse::<ViewMode>(),
            Ok(ViewMode::Balanced)
        ));
        assert!(matches!("top".parse::<ViewMode>(), Ok(ViewMode::Top)));
        assert!(matches!("side".parse::<ViewMode>(), Ok(ViewMode::Side)));
        assert!("unknown".parse::<ViewMode>().is_err());
    }

    #[test]
    fn border_style_from_str() {
        assert!(matches!(
            "thin".parse::<BorderStyle>(),
            Ok(BorderStyle::Thin)
        ));
        assert!(matches!(
            "normal".parse::<BorderStyle>(),
            Ok(BorderStyle::Normal)
        ));
        assert!(matches!(
            "thick".parse::<BorderStyle>(),
            Ok(BorderStyle::Thick)
        ));
        assert!("unknown".parse::<BorderStyle>().is_err());
    }

    #[test]
    fn background_style_from_str() {
        assert!(matches!(
            "transparent".parse::<BackgroundStyle>(),
            Ok(BackgroundStyle::Transparent)
        ));
        assert!(matches!(
            "light".parse::<BackgroundStyle>(),
            Ok(BackgroundStyle::Light)
        ));
        assert!(matches!(
            "white".parse::<BackgroundStyle>(),
            Ok(BackgroundStyle::White)
        ));
        assert!("unknown".parse::<BackgroundStyle>().is_err());
    }
}
