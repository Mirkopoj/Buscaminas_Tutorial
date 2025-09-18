#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Bomb,
    BombNeighbor(u8),
    Empty,
}

impl Tile {
    pub const fn is_bomb(&self) -> bool {
        matches!(self, Self::Bomb)
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        match self {
            Tile::Bomb => "*".to_string(),
            Tile::BombNeighbor(v) => v.to_string(),
            Tile::Empty => " ".to_string(),
        }
    }
}
