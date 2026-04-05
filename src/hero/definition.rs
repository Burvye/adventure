use bevy::prelude::*;
use avian3d::prelude::*;

use crate::WantMove;
use crate::hero;

/// A tag to identify the singular hero. Every player is a hero in their own instance.
/// There should never be more than one hero. Everyone else is not a hero.
#[derive(Component)]
pub struct Hero;

/// Quickly spawn a hero, Only spawn one please.
pub fn spawn_hero(
    cmds: &mut Commands,
) {
    cmds.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 2.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        Collider::capsule(0.5, 1.8),
        LockedAxes::ROTATION_LOCKED,
        RigidBody::Dynamic,
        WantMove { forward: 0, right: 0 },
        hero::definition::Hero,
    ));
}