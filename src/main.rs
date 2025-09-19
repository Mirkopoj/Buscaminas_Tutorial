use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use board_plugin::{BoardPlugin, resources::BoardOptions};

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
    app.add_plugins(BoardPlugin);
    app.add_systems(Startup, camera_setup);
    app.run();
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn(Camera2d);
}
