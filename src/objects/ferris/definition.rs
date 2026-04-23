use bevy::prelude::*;
use avian3d::prelude::*;

use crate::almighty::definition::WantMove;

pub fn spawn_ferris(cmds: &mut Commands, asset_server: &AssetServer) {
    cmds.spawn((
        SceneRoot(asset_server.load("models/crab.glb#Scene0")),
        Collider::sphere(2.0),
        Transform::from_xyz(0.0, 20.0, 0.0).with_scale(Vec3::new(0.1, 0.1, 0.1)),
        Visibility::default(),
        RigidBody::Dynamic,
        WantMove {
            zinput: 0,
            xinput: 0,
            jump: false,
            forward: Vec3::Z,
            move_speed: 3.0,
        }
    ));
}

pub fn spawn_ferrises(mut cmds: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..=25 {
        spawn_ferris(&mut cmds, &asset_server);
    }
}

