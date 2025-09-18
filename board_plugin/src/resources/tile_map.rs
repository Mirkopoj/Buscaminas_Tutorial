use std::ops::{Deref, DerefMut};

use rand::{Rng, rng};

use crate::components::Coordinate;
use crate::resources::tile::Tile;

#[derive(Debug, Clone)]
pub struct TileMap {
    bomb_count: u16,
    height: u16,
    width: u16,
    map: Vec<Vec<Tile>>,
}

impl TileMap {
    pub fn empty(width: u16, height: u16) -> Self {
        let map = (0..height)
            .into_iter()
            .map(|_| (0..width).into_iter().map(|_| Tile::Empty).collect())
            .collect();
        Self {
            bomb_count: 0,
            height,
            width,
            map,
        }
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!(
            "Map ({}, {}) with {} bombs:\n",
            self.width, self.height, self.bomb_count
        );
        let line: String = "-".repeat((self.width + 2).into());
        buffer.push_str(&line);
        buffer.push('\n');
        for line in self.iter().rev() {
            buffer.push('|');
            for tile in line.iter() {
                buffer.push_str(&tile.console_output());
            }
            buffer.push_str("|\n");
        }
        buffer.push_str(&line);
        buffer
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn bomb_count(&self) -> u16 {
        self.bomb_count
    }

    const SQUARE_COORDINATES: [(i8, i8); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    pub fn safe_square_at(&self, coordinate: Coordinate) -> impl Iterator<Item = Coordinate> {
        Self::SQUARE_COORDINATES
            .iter()
            .copied()
            .map(move |tuple| coordinate + tuple)
    }

    pub fn is_bomb_at(&self, coordinate: Coordinate) -> bool {
        if coordinate.x >= self.width || coordinate.y >= self.height {
            return false;
        }
        self.map[coordinate.y as usize][coordinate.x as usize].is_bomb()
    }

    pub fn bomb_count_at(&self, coordinate: Coordinate) -> u8 {
        if self.is_bomb_at(coordinate) {
            return 0;
        }
        self.safe_square_at(coordinate)
            .filter(|&coord| self.is_bomb_at(coord))
            .count() as u8
    }

    pub fn set_bombs(&mut self, bomb_count: u16) {
        self.bomb_count = bomb_count;
        let mut remaining_bombs = bomb_count;
        let mut rng = rng();
        while remaining_bombs > 0 {
            let (x, y) = (
                rng.random_range(0..self.width) as usize,
                rng.random_range(0..self.height) as usize,
            );
            if let Tile::Empty = self[y][x] {
                self[y][x] = Tile::Bomb;
                remaining_bombs -= 1;
            }
        }
        for y in 0..self.height {
            for x in 0..self.width {
                let coords = Coordinate { x, y };
                if self.is_bomb_at(coords) {
                    continue;
                }
                let num = self.bomb_count_at(coords);
                if num == 0 {
                    continue;
                }
                self[y as usize][x as usize] = Tile::BombNeighbor(num);
            }
        }
    }
}

impl Deref for TileMap {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
