use bevy::prelude::*;
use bevy::prelude::KeyCode::{ KeyW, KeyA, KeyS, KeyD };

use crate::hero::definition::Hero;
use crate::motion::definition::WantMove;

/// Reads the hero's input and sets where they want to move.
pub fn hero_input(mut hero_want: Single<&mut WantMove, With<Hero>>, keys: Res<ButtonInput<KeyCode>>) {
    hero_want.forward = (keys.pressed(KeyW) as i8) - (keys.pressed(KeyS) as i8);
    hero_want.right = (keys.pressed(KeyD) as i8) - (keys.pressed(KeyA) as i8);
}
