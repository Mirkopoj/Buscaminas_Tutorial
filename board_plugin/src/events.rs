use crate::components::Coordinate;
use bevy::prelude::Event;

#[derive(Debug, Copy, Clone, Event)]
pub struct TileTriggerEvent(pub Coordinate);
