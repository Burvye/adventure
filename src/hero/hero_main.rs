use bevy::prelude::*;
use bevy::prelude::KeyCode::{KeyW, KeyA, KeyS, KeyD};

#[derive(Component)]
pub struct Hero {
    pub forward: f64,
    pub right: f64,
}

pub fn update_input(
    query: Query<&mut Hero, With<Hero>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for mut hero in query {
        hero.forward = 0.0;
        hero.right = 0.0;
        if keys.pressed(KeyW) {
            hero.forward += 1.0;
        }
        if keys.pressed(KeyS) {
            hero.forward -= 1.0;
        }
        if keys.pressed(KeyD) {
            hero.right += 1.0;
        }
        if keys.pressed(KeyA) {
            hero.right -= 1.0;
        }
    }
}