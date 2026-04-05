use bevy::prelude::*;
use avian3d::prelude::*;

#[derive(Component)]
pub struct Hero;

pub fn update_input(
    mut query: Query<(&mut LinearVelocity, &Transform), With<Hero>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let speed = 10.0;
}