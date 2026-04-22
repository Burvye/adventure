use bevy::prelude::*;
use avian3d::prelude::*;

use crate::almighty::definition::WantMove;

/// Spawn Ferris.
pub fn spawn_ferris(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.spawn((
        SceneRoot(asset_server.load("models/crab.glb#Scene0")),
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
        Transform::from_xyz(0.0, 20.0, 0.0).with_scale(Vec3::new(0.1, 0.1, 0.1)),
        Visibility::default(),
        RigidBody::Dynamic,
        WantMove {
            zinput: 0,
            xinput: 0,
            jump: false,
            forward: Vec3::Z,
            move_speed: 2.0,
        }
    ));
}
