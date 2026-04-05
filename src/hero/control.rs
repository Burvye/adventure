use bevy::prelude::*;
use bevy::prelude::KeyCode::{ KeyW, KeyA, KeyS, KeyD, Space };
use bevy::input::mouse::AccumulatedMouseMotion;

use crate::hero::definition::Hero;
use crate::hero::definition::HeroCamera;
use crate::hero::definition::HeroBody;
use crate::motion::definition::WantMove;

/// Reads the hero's input and sets where they want to move.
pub fn hero_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut hero: Single<(&mut Hero, &mut WantMove), With<Hero>>
) {
    hero.1.zinput = (keys.pressed(KeyW) as i8) - (keys.pressed(KeyS) as i8);
    hero.1.xinput = (keys.pressed(KeyD) as i8) - (keys.pressed(KeyA) as i8);
    hero.1.jump = keys.just_pressed(Space);

    hero.1.forward = Vec3::new(-hero.0.yaw.sin(), 0.0, -hero.0.yaw.cos());
}

/// Updates the player's stored rotation from mouse movement.
pub fn read_camera(mot: Res<AccumulatedMouseMotion>, mut hero: Single<&mut Hero>) {
    // clamp pitch to avoid gimbal lock
    hero.pitch = (hero.pitch + pitch_diff(&mot, &hero)).clamp(
        -(89.9_f32).to_radians(),
        (89.9_f32).to_radians()
    );
    // wrap yaw around 2 pi to save space
    hero.yaw = (hero.yaw + yaw_diff(&mot, &hero)).rem_euclid(std::f32::consts::TAU);
}

/// Applies stored rotation to the body yaw of the hero.
pub fn update_body(mut hbod: Single<&mut Transform, With<HeroBody>>, hero: Single<&Hero>) {
    hbod.rotation = Quat::from_rotation_y(hero.yaw);
}
/// Applies stored rotation to the camera of the hero
pub fn update_camera(mut hcam: Single<&mut Transform, With<HeroCamera>>, hero: Single<&Hero>) {
    // actually apply transformation.
    hcam.rotation = Quat::from_euler(EulerRot::YXZ, hero.yaw, hero.pitch, 0.0);
}

/// Return the change in yaw this frame
fn yaw_diff(mot: &Res<AccumulatedMouseMotion>, hero: &Hero) -> f32 {
    -mot.delta.x * hero.sens.x
}
/// Return the change in pitch this frame
fn pitch_diff(mot: &Res<AccumulatedMouseMotion>, hero: &Hero) -> f32 {
    -mot.delta.y * hero.sens.y
}
