use bevy::prelude::*;
use avian3d::prelude::*;

use crate::hero;
use crate::build;

/// Construct the default environment to spawn in
pub fn build_lobby(
    mut cmds: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    // "hero"
    hero::definition::spawn_hero(&mut cmds, &mut mesh, &mut mats);

    build::build_cube::physics_cube(&mut cmds, &mut mesh, &mut mats, 2.0, -3.0, 22.0);

    cmds.spawn((
        Mesh3d(mesh.add(Cuboid::new(1.25, 0.5, 1.0))),
        MeshMaterial3d(mats.add(Color::srgb_u8(255, 60, 26))),
        Transform::from_xyz(4.5, -3.0, 23.0),
        Collider::cuboid(1.25, 0.5, 1.0),
        RigidBody::Static,
    ));
    // TODO: Build a cash register at 4.5, -3, 23

    // fps map
    cmds.spawn((
        SceneRoot(asset_server.load("models/map/worlde.glb#Scene0")),
        // 2. Automatically generate Convex Hull colliders for all mesh children
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
        Transform::from_xyz(0.0, -5.0, 0.0),
        RigidBody::Static,
    ));
}
