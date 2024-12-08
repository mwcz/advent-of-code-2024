use std::fmt::Display;

/// The cardinal directions.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum CardDir {
    Up,
    Down,
    Left,
    Right,
}

impl CardDir {
    pub fn cw(&self) -> CardDir {
        use CardDir::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

impl From<char> for CardDir {
    fn from(value: char) -> Self {
        match value {
            'v' => CardDir::Down,
            '>' => CardDir::Right,
            '<' => CardDir::Left,
            '^' => CardDir::Up,
            _ => panic!("char {value} is not a valid direction"),
        }
    }
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

    pub fn cw(&self) -> CardOrdDir {
        use CardOrdDir::*;
        match self {
            UpLeft => Up,
            Up => UpRight,
            UpRight => Right,
            Right => DownRight,
            DownRight => Down,
            Down => DownLeft,
            DownLeft => Left,
            Left => UpLeft,
        }
    }
}
