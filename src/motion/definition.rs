use bevy::prelude::*;

// TODO: Add a jump impulse
/// A struct to indicate where an entity wants to move.
#[derive(Component)]
pub struct WantMove {
    pub forward: i8,
    pub right: i8,
}
