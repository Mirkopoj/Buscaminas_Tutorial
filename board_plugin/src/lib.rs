use bevy::log;
use bevy::{ecs::relationship::RelatedSpawnerCommands, platform::collections::HashMap, prelude::*};

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

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::create_board)
            .add_systems(Update, systems::input::input_handling)
            .add_systems(Update, systems::uncover::trigger_event_handler)
            .add_systems(Update, systems::uncover::uncover_tiles)
            .add_event::<TileTriggerEvent>();
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

impl BoardPlugin {
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        window: Query<&Window>,
        asset_server: Res<AssetServer>,
    ) {
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
        let font = asset_server.load("fonts/pixeled.ttf");
        let bomb_image = asset_server.load("sprites/bomb.png");
        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            }
            BoardPosition::Custom(p) => p,
        };
        let mut covered_tiles =
            HashMap::with_capacity((tile_map.width() * tile_map.height()).into());
        commands
            .spawn((
                Name::new("Board"),
                Transform::from_translation(board_position),
                GlobalTransform::default(),
                Visibility::default(),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Sprite {
                        color: Color::WHITE,
                        custom_size: Some(board_size),
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
                    Color::BLACK.lighter(0.5),
                    bomb_image,
                    font,
                    Color::BLACK.lighter(0.25),
                    &mut covered_tiles,
                );
            });
        commands.insert_resource(Board {
            tile_map,
            bounds: Bounds2 {
                position: board_position.xy(),
                size: board_size,
            },
            tile_size,
            covered_tiles,
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
        font: Handle<Font>,
        font_size: f32,
    ) -> (Text2d, TextFont, TextColor, Transform) {
        let (text, color) = (
            count.to_string(),
            match count {
                1 => Color::WHITE,
                2 => Color::linear_rgb(0., 1., 0.),
                3 => Color::linear_rgb(1., 1., 0.),
                4 => Color::linear_rgb(1., 0.65, 0.),
                _ => Color::linear_rgb(0.5, 0., 0.5),
            },
        );
        (
            Text2d::new(text),
            TextFont {
                font,
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
        color: Color,
        bomb_image: Handle<Image>,
        font: Handle<Font>,
        covered_tile_color: Color,
        covered_tiles: &mut HashMap<Coordinate, Entity>,
    ) {
        for (y, line) in tile_map.iter().enumerate() {
            for (x, tile_type) in line.iter().enumerate() {
                let coordinate = Coordinate {
                    x: x as u16,
                    y: y as u16,
                };
                let mut tile = parent.spawn((
                    Sprite {
                        color,
                        custom_size: Some(Vec2::splat(size - padding)),
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
                                color: covered_tile_color,
                                ..Default::default()
                            },
                            Transform::from_xyz(0., 0., 2.),
                            Name::new("Tile Cover"),
                        ))
                        .id();
                    covered_tiles.insert(coordinate, entity);
                });
                match tile_type {
                    Tile::Bomb => {
                        tile.insert(Bomb);
                        tile.with_children(|parent| {
                            parent.spawn((
                                Sprite {
                                    custom_size: Some(Vec2::splat(size - padding)),
                                    image: bomb_image.clone(),
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
                                font.clone(),
                                (size - padding) / 2.,
                            ));
                        });
                    }
                    Tile::Empty => (),
                }
            }
        }
    }
}
