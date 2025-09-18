use bevy::{log, prelude::*};

use crate::{
    components::{Bomb, BombNeighbor, Coordinate, Uncover},
    events::TileTriggerEvent,
    resources::Board,
};

pub fn trigger_event_handler(
    mut commands: Commands,
    board: Res<Board>,
    mut tile_trigger_evr: EventReader<TileTriggerEvent>,
) {
    for trigger_event in tile_trigger_evr.read() {
        if let Some(entity) = board.tile_to_uncover(&trigger_event.0) {
            commands.entity(*entity).insert(Uncover);
        }
    }
}

pub fn uncover_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    children: Query<(Entity, &ChildOf), With<Uncover>>,
    parents: Query<(&Coordinate, Option<&Bomb>, Option<&BombNeighbor>)>,
) {
    for (entity, parent) in children.iter() {
        commands.entity(entity).despawn();
        let (coord, bomb, bomb_counter) = match parents.get(parent.0) {
            Ok(v) => v,
            Err(e) => {
                log::error!("{}", e);
                continue;
            }
        };
        match board.try_uncover_tile(coord) {
            Some(e) => log::debug!("Uncovered tile {} (entity: {:?})", coord, e),
            None => log::debug!("Tried to uncover an already uncovered tile"),
        }
        if bomb.is_some() {
            log::info!("Boom !");
            //TODO: Add explosion event
        } else if bomb_counter.is_none() {
            for entity in board.adjacent_covered_tiles(*coord) {
                commands.entity(entity).insert(Uncover);
            }
        }
    }
}
