use crate::bounds::Bounds2;
use crate::{Coordinate, TileMap};
use bevy::platform::collections::HashMap;
use bevy::{log, prelude::*};

#[derive(Debug, Resource)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Bounds2,
    pub tile_size: f32,
    pub covered_tiles: HashMap<Coordinate, Entity>,
    pub entity: Entity,
    pub marked_tiles: Vec<Coordinate>,
}

impl Board {
    pub fn mouse_position(&self, window: &Window, position: Vec2) -> Option<Coordinate> {
        let window_size = Vec2::new(window.width(), window.height());
        let position = position - window_size / 2.;
        if !self.bounds.in_bounds(position) {
            return None;
        }
        let coordinate = position - self.bounds.position;
        Some(Coordinate {
            x: (coordinate.x / self.tile_size) as u16,
            y: (coordinate.y / self.tile_size) as u16,
        })
    }

    pub fn tile_to_uncover(&self, coord: &Coordinate) -> Option<&Entity> {
        if self.marked_tiles.contains(coord) {
            None
        } else {
            self.covered_tiles.get(coord)
        }
    }

    pub fn try_uncover_tile(&mut self, coord: &Coordinate) -> Option<Entity> {
        if self.marked_tiles.contains(coord) {
            self.unmark_tile(coord)?;
        }
        self.covered_tiles.remove(coord)
    }

    pub fn adjacent_covered_tiles(&self, coord: Coordinate) -> Vec<Entity> {
        self.tile_map
            .safe_square_at(coord)
            .filter_map(|c| self.covered_tiles.get(&c))
            .copied()
            .collect()
    }

    fn unmark_tile(&mut self, coord: &Coordinate) -> Option<Coordinate> {
        let pos = match self.marked_tiles.iter().position(|a| a == coord) {
            None => {
                log::error!("Failed to unmark tile at {}", coord);
                return None;
            }
            Some(p) => p,
        };
        Some(self.marked_tiles.remove(pos))
    }

    pub fn is_completed(&self) -> bool {
        self.tile_map.bomb_count() as usize == self.covered_tiles.len()
    }

    pub fn try_toggle_mark(&mut self, coord: &Coordinate) -> Option<(Entity, bool)> {
        let entity = *self.covered_tiles.get(coord)?;
        let mark = if self.marked_tiles.contains(coord) {
            self.unmark_tile(coord)?;
            false
        }else{
            self.marked_tiles.push(*coord);
            true
        };
        Some((entity, mark))
    }
}
