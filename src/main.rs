use bevy::{log, prelude::*};

#[cfg(feature = "debug")]
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use board_plugin::{BoardPlugin, resources::BoardOptions};

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    InGame,
    Out,
    ReGen,
    Pause
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    #[cfg(feature = "debug")]
    app.add_plugins(EguiPlugin::default());
    #[cfg(feature = "debug")]
    app.add_plugins(WorldInspectorPlugin::new());
    app.insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 3.,
        safe_start: true,
        ..Default::default()
    });
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
