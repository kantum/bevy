use bevy::prelude::*;
// use bevy_rapier3d::prelude::*;

pub fn spawn_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(100.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
}

// pub fn setup_physics(mut commands: Commands) {
//     /* Create the ground. */
//     commands
//         .spawn(Collider::cuboid(100.0, 0.1, 100.0))
//         .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));
// }
