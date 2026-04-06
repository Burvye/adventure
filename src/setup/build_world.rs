use bevy::prelude::*;
use avian3d::prelude::*;

use crate::hero;

/// Construct the default environment to spawn in
pub fn build_lobby(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // "hero"
    hero::definition::spawn_hero(&mut cmds, &mut meshes, &mut materials);

    // TODO: Add a dedicated "spawn cube" function
    // cube
    cmds.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 102, 0))),
        Transform::from_xyz(0.0, 10.0, 0.0),
        Collider::cuboid(1.0, 1.0, 1.0),
        RigidBody::Dynamic,
    ));

    // fps map
    cmds.spawn((
        SceneRoot(asset_server.load("models/map/scene.gltf#Scene0")),
        // 2. Automatically generate Convex Hull colliders for all mesh children
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
        Transform::from_xyz(0.0, -5.0, 0.0),
        RigidBody::Static,
    ));
}