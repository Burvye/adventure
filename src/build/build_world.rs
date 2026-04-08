use bevy::prelude::*;
use avian3d::prelude::*;

use crate::hero;
use crate::build;

/// Construct the default environment to spawn in
pub fn build_lobby(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // "hero"
    hero::definition::spawn_hero(&mut cmds, &mut meshes, &mut materials);
    
    build::build_cube::physics_cube(&mut cmds, &mut meshes, &mut materials);

    // fps map
    cmds.spawn((
        SceneRoot(asset_server.load("models/map/worlde.glb#Scene0")),
        // 2. Automatically generate Convex Hull colliders for all mesh children
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
        Transform::from_xyz(0.0, -5.0, 0.0),
        RigidBody::Static,
    ));
}