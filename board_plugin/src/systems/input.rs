use crate::Board;
use crate::events::TileTriggerEvent;
use bevy::input::{ButtonState, mouse::MouseButtonInput};
use bevy::log;
use bevy::prelude::*;

pub fn input_handling(
    window: Query<&Window>,
    board: Res<Board>,
    mut button_evr: EventReader<MouseButtonInput>,
    mut tile_trigger_ewr: EventWriter<TileTriggerEvent>,
) {
    let Ok(window) = window.single() else {
        return;
    };

    for event in button_evr.read() {
        if let ButtonState::Pressed = event.state {
            let position = window.cursor_position();
            if let Some(pos) = position {
                let pos = Vec2::new(pos.x, window.height() - pos.y);
                log::trace!("Mouse button pressed: {:?} at {}", event.button, pos);
                let tile_coordinates = board.mouse_position(window, pos);
                if let Some(coordinates) = tile_coordinates {
                    match event.button {
                        MouseButton::Left => {
                            log::info!("Trying to uncover tile on {}", coordinates);
                            tile_trigger_ewr.write(TileTriggerEvent(coordinates));
                        }
                        MouseButton::Right => {
                            log::info!("Trying to mark tile on {}", coordinates);
                            //TODO: gen event
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}
