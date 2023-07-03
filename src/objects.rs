use bevy::prelude::*;

pub fn spawn_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube {size: 1.0})),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
}

pub fn spawn_craft_speeder_a(
    mut commands: Commands,
    ass: Res<AssetServer>,
) {
    // note that we have to include the `Scene0` label
    let my_gltf = ass.load("models/craft_speederA.glb#Scene0");

    // to position our 3d model, simply use the Transform
    // in the SceneBundle
    commands.spawn(SceneBundle {
        scene: my_gltf,
        transform: Transform::from_xyz(0.0, 0.2, 0.0),
        ..Default::default()
    });
}

pub fn spawn_craft_speeder_b(
    mut commands: Commands,
    ass: Res<AssetServer>,
) {
    let my_gltf = ass.load("models/craft_speederB.glb#Scene0");

    // to position our 3d model, simply use the Transform
    // in the SceneBundle
    commands.spawn(SceneBundle {
        scene: my_gltf,
        transform: Transform::from_xyz(4.0, 0.2, 0.0),
        ..Default::default()
    });
}

pub fn spawn_craft_speeder_c(
    mut commands: Commands,
    ass: Res<AssetServer>,
) {
    let my_gltf = ass.load("models/craft_speederC.glb#Scene0");

    // to position our 3d model, simply use the Transform
    // in the SceneBundle
    commands.spawn(SceneBundle {
        scene: my_gltf,
        transform: Transform::from_xyz(8.0, 0.2, 0.0),
        ..Default::default()
    });
}
