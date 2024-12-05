use std::fmt::Display;

/// The cardinal directions.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum CardDir {
    Up,
    Down,
    Left,
    Right,
}

impl Display for CardDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CardDir::Up => "⬆️".to_string(),
                CardDir::Down => "⬇️".to_string(),
                CardDir::Left => "⬅️".to_string(),
                CardDir::Right => "➡️".to_string(),
            }
        )
    }
}

/// Cardinal and ordinal (intercardinal) directions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CardOrdDir {
    UpLeft,
    Up,
    UpRight,
    Left,
    Right,
    DownLeft,
    Down,
    DownRight,
}

impl CardOrdDir {
    pub fn all() -> [CardOrdDir; 8] {
        use CardOrdDir::*;
        [UpLeft, Up, UpRight, Left, Right, DownLeft, Down, DownRight]
    }
}
