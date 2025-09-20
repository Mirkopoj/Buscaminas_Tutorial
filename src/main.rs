use bevy::{log, prelude::*};

#[cfg(feature = "debug")]
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use board_plugin::{
    BoardPlugin,
    resources::{BoardAssets, BoardOptions, SpriteMaterial},
};

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    InGame,
    #[default]
    Out,
    ReGen,
    Pause,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    #[cfg(feature = "debug")]
    app.add_plugins(EguiPlugin::default());
    #[cfg(feature = "debug")]
    app.add_plugins(WorldInspectorPlugin::new());
    app.add_systems(Startup, setup_board);
    app.init_state::<AppState>();
    app.add_plugins(BoardPlugin {
        running_state: AppState::InGame,
        paused_state: AppState::Pause,
    });
    app.add_systems(Startup, camera_setup);
    app.add_systems(Update, state_handler);
    app.run();
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn(Camera2d);
}

fn state_handler(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    state: Res<State<AppState>>,
) {
    if keys.just_pressed(KeyCode::KeyC) {
        log::info!("clearing game");
        next_state.set(AppState::Out);
    }
    if keys.just_pressed(KeyCode::KeyG) {
        log::info!("loading game");
        next_state.set(AppState::ReGen);
    }
    if keys.just_pressed(KeyCode::Escape) {
        log::info!("Pause");
        if let AppState::InGame = state.get() {
            next_state.set(AppState::Pause);
        } else {
            next_state.set(AppState::InGame);
        }
    }
    if let AppState::ReGen = state.get() {
        next_state.set(AppState::InGame);
    }
}

fn setup_board(
    mut commands: Commands,
    mut state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 1.,
        safe_start: true,
        ..Default::default()
    });
    commands.insert_resource(BoardAssets {
        label: "Default".to_string(),
        board_material: SpriteMaterial {
            color: Color::WHITE,
            ..Default::default()
        },
        tile_material: SpriteMaterial {
            color: Color::BLACK.lighter(0.2),
            ..Default::default()
        },
        covered_tile_material: SpriteMaterial {
            color: Color::BLACK.lighter(0.5),
            ..Default::default()
        },
        bomb_counter_font: asset_server.load("fonts/pixeled.ttf"),
        bomb_counter_colors: BoardAssets::default_colors(),
        flag_material: SpriteMaterial {
            texture: asset_server.load("sprites/flag.png"),
            ..Default::default()
        },
        bomb_material: SpriteMaterial {
            texture: asset_server.load("sprites/bomb.png"),
            ..Default::default()
        },
    });
    state.set(AppState::InGame);
}
