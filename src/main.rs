use bevy::prelude::*;
use bevy_flycam::NoCameraPlayerPlugin;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_editor_pls::prelude::*;
// use bevy_rapier3d::prelude::*;

use bevy_game::planes::spawn_plane;
// use bevy_game::planes::setup_physics;
use bevy_game::lights::spawn_light;
use bevy_game::cameras::spawn_camera;
use bevy_game::objects::spawn_craft_speeder_a;
use bevy_game::objects::spawn_craft_speeder_b;
use bevy_game::objects::spawn_craft_speeder_c;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_startup_system(spawn_camera)
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy game".to_string(), // ToDo
                    resolution: (1280., 720.).into(),
                    canvas: Some("#bevy".to_owned()),
                    ..default()
                }),
                ..default()
            }))
            .add_plugin(EditorPlugin::default())
            .add_plugin(NoCameraPlayerPlugin)
            // .add_plugin(WorldInspectorPlugin::new())
            // .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            // .add_plugin(RapierDebugRenderPlugin::default())
            // .add_startup_system(setup_graphics)
            // .add_startup_system(setup_physics)
            .add_startup_system(spawn_plane)
            .add_startup_system(spawn_light)
            .add_startup_system(spawn_camera)
            .add_startup_system(spawn_craft_speeder_a)
            .add_startup_system(spawn_craft_speeder_b)
            .add_startup_system(spawn_craft_speeder_c);
    }
}

fn main() {
    App::new()
        // .insert_resource(Msaa::Sample4)
        .add_plugin(HelloPlugin)
        .run();
}
