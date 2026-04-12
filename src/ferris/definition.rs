use bevy::prelude::*;
use avian3d::prelude::*;

/// Spawn Ferris.
pub fn spawn_ferris(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.spawn((
        SceneRoot(asset_server.load("models/crab.glb#Scene0")),
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
        Transform::from_xyz(0.0, 20.0, 0.0),
        RigidBody::Dynamic,
    ));
}
