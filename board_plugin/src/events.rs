use crate::components::Coordinate;
use bevy::prelude::Event;

#[derive(Debug, Copy, Clone, Event)]
pub struct TileTriggerEvent(pub Coordinate);

#[derive(Debug, Copy, Clone, Event)]
pub struct BoardCompletedEvent;

#[derive(Debug, Copy, Clone, Event)]
pub struct BombExplosionEvent;

#[derive(Debug, Copy, Clone, Event)]
pub struct TileMarkEvent(pub Coordinate);
