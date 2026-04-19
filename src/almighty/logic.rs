use bevy::prelude::*;
use avian3d::prelude::*;

use crate::{almighty::definition::WantMove};
use crate::objects;


/// Jumping velocity impulse.
const JUMP_STRENGTH: f32 = 10.0;

/// Fulfill the movement wants of all entities and move them by applying velocity.
pub fn move_all(query: Query<(&mut WantMove, &mut LinearVelocity)>) {
    for (mut des, mut velocity) in query {
        *velocity = LinearVelocity((local_dir(&des) * get_speed(&des)).with_y(velocity.y));
        if des.jump {
            des.jump = false;
            velocity.y = JUMP_STRENGTH;
        }
    }
}

/// Set things visible or invisible based on the custom visibility tag
pub fn update_visibilities(visibilizables: Query<(&mut Visibility, &objects::definition::Visible)>) {
    for (mut visibility, visibilizable) in visibilizables {
        *visibility = if visibilizable.0 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

/// Gives you a local horizontal movement vector.
fn local_dir(want_move: &WantMove) -> Vec3 {
    want_move.forward * (want_move.zinput as f32) +
        Vec3::new(-want_move.forward.z, 0.0, want_move.forward.x) * (want_move.xinput as f32)
}
/// Returns the movement speed specified by WantMove for horizontal movement.
fn get_speed(want_move: &WantMove) -> f32 {
    want_move.move_speed
}