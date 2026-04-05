use bevy::prelude::*;
use avian3d::prelude::*;

use crate::motion::definition::WantMove;

/// Horizontal movement speed.
const MOVE_SPEED: f32 = 4.0;

/// Fulfill the movement wants of all entities and move them by applying velocity.
pub fn move_all(query: Query<(&WantMove, &Transform, &mut LinearVelocity)>) {
    for (dir, tf, mut velocity) in query {
        *velocity = LinearVelocity(move_local(dir, tf) * MOVE_SPEED);
    }
}
// TODO: Allow move_local to accept vertical vectors like jumping
/// Gives you a local horizontal movement vector.
fn move_local(direction: &WantMove, transform: &Transform) -> Vec3 {
    *transform.forward() * direction.forward as f32 + *transform.right() * direction.right as f32
}
