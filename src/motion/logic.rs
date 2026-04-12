use bevy::prelude::*;
use avian3d::prelude::*;

use crate::motion::definition::WantMove;

/// Jumping velocity impulse.
const JUMP_STRENGTH: f32 = 10.0;

/// Fulfill the movement wants of all entities and move them by applying velocity.
pub fn move_all(query: Query<(&mut WantMove, &mut LinearVelocity)>) {
    for (mut des, mut velocity) in query {
        *velocity = LinearVelocity((move_local(&des) * get_speed(&des)).with_y(velocity.y));
        if des.jump {
            des.jump = false;
            velocity.y = JUMP_STRENGTH;
        }
    }
}
/// Gives you a local horizontal movement vector.
fn move_local(want_move: &WantMove) -> Vec3 {
    want_move.forward * (want_move.zinput as f32) +
        Vec3::new(-want_move.forward.z, 0.0, want_move.forward.x) * (want_move.xinput as f32)
}
/// Returns the movement speed specified by WantMove for horizontal movement.
fn get_speed(want_move: &WantMove) -> f32 {
    want_move.move_speed
}