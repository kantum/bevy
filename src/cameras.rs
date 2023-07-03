use bevy::prelude::*;
use bevy_flycam::FlyCam;

pub fn spawn_camera(
    mut commands: Commands,
){
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(5.0, 2.0, 10.0),
        ..default()
    }, FlyCam));
}
