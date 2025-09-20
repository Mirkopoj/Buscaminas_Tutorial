use bevy::log;
use bevy::{ecs::relationship::RelatedSpawnerCommands, platform::collections::HashMap, prelude::*};
use events::{BoardCompletedEvent, BombExplosionEvent, TileMarkEvent};
use resources::BoardAssets;

use crate::components::Uncover;
use crate::{
    bounds::Bounds2,
    components::{Bomb, BombNeighbor, Coordinate},
    events::TileTriggerEvent,
    resources::{Board, BoardOptions, BoardPosition, TileSize, tile::Tile, tile_map::TileMap},
};

mod bounds;
pub mod components;
mod events;
pub mod resources;
mod systems;

pub struct BoardPlugin<T> {
    pub running_state: T,
    pub paused_state: T,
}

impl<T: States> Plugin for BoardPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.running_state.clone()), Self::create_board)
            .add_systems(
                Update,
                (
                    systems::input::input_handling,
                    systems::uncover::trigger_event_handler,
                    systems::uncover::uncover_tiles,
                    systems::mark::mark_tiles,
                )
                    .run_if(in_state(self.running_state.clone())),
            )
            .add_systems(
                OnExit(self.running_state.clone()),
                Self::cleanup_board.run_if(not(in_state(self.paused_state.clone()))),
            )
            .add_event::<TileTriggerEvent>()
            .add_event::<TileMarkEvent>()
            .add_event::<BombExplosionEvent>()
            .add_event::<BoardCompletedEvent>();
        #[cfg(feature = "debug")]
        {
            app.register_type::<crate::components::Coordinate>();
            app.register_type::<crate::components::BombNeighbor>();
            app.register_type::<crate::components::Bomb>();
            app.register_type::<crate::components::Uncover>();
        }
        log::info!("Loaded Board Plugin");
    }
}

impl<T> BoardPlugin<T> {
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        board_assets: Res<BoardAssets>,
        window: Query<&Window>,
        board: Option<Res<Board>>,
    ) {
        if board.is_some() {
            return;
        }
        let options = match board_options {
            None => BoardOptions::default(),
            Some(o) => o.clone(),
        };
        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.set_bombs(options.bomb_count);
        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());
        let tile_size = match options.tile_size {
            TileSize::Fixed(v) => v,
            TileSize::Adaptive { min, max } => Self::adaptative_tile_size(
                window,
                (min, max),
                (tile_map.width(), tile_map.height()),
            ),
        };
        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );
        log::info!("board size: {}", board_size);
        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            }
            BoardPosition::Custom(p) => p,
        };
        let mut covered_tiles =
            HashMap::with_capacity((tile_map.width() * tile_map.height()).into());
        let mut safe_start = None;
        let board_entity = commands
            .spawn((
                Name::new("Board"),
                Transform::from_translation(board_position),
                GlobalTransform::default(),
                Visibility::default(),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Sprite {
                        color: board_assets.board_material.color,
                        custom_size: Some(board_size),
                        image: board_assets.board_material.texture.clone(),
                        ..Default::default()
                    },
                    Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                    Name::new("Background"),
                ));
                Self::spawn_tiles(
                    parent,
                    &tile_map,
                    tile_size,
                    options.tile_padding,
                    &board_assets,
                    &mut covered_tiles,
                    &mut safe_start,
                );
            })
            .id();
        if options.safe_start {
            if let Some(entity) = safe_start {
                commands.entity(entity).insert(Uncover);
            }
        }
        commands.insert_resource(Board {
            tile_map,
            bounds: Bounds2 {
                position: board_position.xy(),
                size: board_size,
            },
            tile_size,
            covered_tiles,
            entity: board_entity,
            marked_tiles: Vec::new(),
        });
    }

    fn adaptative_tile_size(
        window: Query<&Window>,
        (min, max): (f32, f32),      // Tile size constraints
        (width, height): (u16, u16), // Tile map dimensions
    ) -> f32 {
        let window = &window.single().unwrap().resolution;
        let max_width = window.width() / width as f32;
        let max_heigth = window.height() / height as f32;
        max_width.min(max_heigth).clamp(min, max)
    }

    fn bomb_count_text_bundle(
        count: u8,
        board_assets: &BoardAssets,
        font_size: f32,
    ) -> (Text2d, TextFont, TextColor, Transform) {
        let color = board_assets.bomb_counter_color(count);
        (
            Text2d::new(count.to_string()),
            TextFont {
                font: board_assets.bomb_counter_font.clone(),
                font_size,
                ..Default::default()
            },
            TextColor(color),
            Transform::from_xyz(0., 0., 1.),
        )
    }

    fn spawn_tiles(
        parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
        tile_map: &TileMap,
        size: f32,
        padding: f32,
        board_assets: &BoardAssets,
        covered_tiles: &mut HashMap<Coordinate, Entity>,
        safe_start_entity: &mut Option<Entity>,
    ) {
        for (y, line) in tile_map.iter().enumerate() {
            for (x, tile_type) in line.iter().enumerate() {
                let coordinate = Coordinate {
                    x: x as u16,
                    y: y as u16,
                };
                let mut tile = parent.spawn((
                    Sprite {
                        color: board_assets.tile_material.color,
                        custom_size: Some(Vec2::splat(size - padding)),
                        image: board_assets.tile_material.texture.clone(),
                        ..Default::default()
                    },
                    Transform::from_xyz(
                        (x as f32 * size) + (size / 2.),
                        (y as f32 * size) + (size / 2.),
                        1.,
                    ),
                    Name::new(format!("Tile ({}, {})", x, y)),
                    coordinate,
                    Visibility::default(),
                ));
                tile.with_children(|p| {
                    let entity = p
                        .spawn((
                            Sprite {
                                custom_size: Some(Vec2::splat(size - padding)),
                                color: board_assets.covered_tile_material.color,
                                image: board_assets.covered_tile_material.texture.clone(),
                                ..Default::default()
                            },
                            Transform::from_xyz(0., 0., 2.),
                            Name::new("Tile Cover"),
                        ))
                        .id();
                    covered_tiles.insert(coordinate, entity);
                    if safe_start_entity.is_none() && *tile_type == Tile::Empty {
                        *safe_start_entity = Some(entity)
                    }
                });
                match tile_type {
                    Tile::Bomb => {
                        tile.insert(Bomb);
                        tile.with_children(|parent| {
                            parent.spawn((
                                Sprite {
                                    custom_size: Some(Vec2::splat(size - padding)),
                                    color: board_assets.bomb_material.color,
                                    image: board_assets.bomb_material.texture.clone(),
                                    ..Default::default()
                                },
                                Transform::from_xyz(0., 0., 1.),
                            ));
                        });
                    }
                    Tile::BombNeighbor(v) => {
                        tile.insert(BombNeighbor { count: *v });
                        tile.with_children(|parent| {
                            parent.spawn(Self::bomb_count_text_bundle(
                                *v,
                                board_assets,
                                (size - padding) / 2.,
                            ));
                        });
                    }
                    Tile::Empty => (),
                }
            }
        }
    }

    fn cleanup_board(board: Res<Board>, mut commands: Commands) {
        commands.entity(board.entity).despawn();
        commands.remove_resource::<Board>();
    }
}
