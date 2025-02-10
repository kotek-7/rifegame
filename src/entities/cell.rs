use std::fmt::Display;
use termion::color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Cell {
    #[default]
    Dead,
    Alive,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Cell::Dead => format!("{}　{}", color::Bg(color::Black), color::Bg(color::Reset)),
            Cell::Alive => format!("{}　{}", color::Bg(color::White), color::Bg(color::Reset)),
        })
    }
}