use crate::hero::logic;

use bevy::prelude::*;
use bevy::prelude::KeyCode::{ KeyW, KeyA, KeyS, KeyD, Space };
use bevy::input::mouse::AccumulatedMouseMotion;

use avian3d::prelude::*;

use crate::objects::hero::definition::Hero;
use crate::objects::hero::definition::HeroCamera;
use crate::objects::hero::definition::HeroBody;
use crate::almighty::definition::WantMove;

use crate::hero;

use crate::objects;

use trig_const::cos;

/// The angle between the player and the ground that jumping should be possible at
const VALID_JUMP_ANGLE: f64 = std::f64::consts::FRAC_PI_3;

/// Valid jump angle but cosined
const VALID_JUMP_ANGLE_COS: f32 = cos(VALID_JUMP_ANGLE) as f32;

/// Reads the hero's input and sets where they want to move.
pub fn hero_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut hero: Single<(&mut Hero, &mut WantMove, &ShapeHits), With<Hero>>
) {
    // tuple destructuring, this does not create side-effects
    let (hero, want_move, collisions) = &mut *hero;

    want_move.zinput = (keys.pressed(KeyW) as i8) - (keys.pressed(KeyS) as i8);
    want_move.xinput = (keys.pressed(KeyD) as i8) - (keys.pressed(KeyA) as i8);

    if keys.pressed(Space) {
        want_move.jump = validate_jump(collisions);
    }

    // store the forward direction to be used later on
    // -Z is forward in bevy
    want_move.forward = Vec3::new(-hero.yaw.sin(), 0.0, -hero.yaw.cos());
}

/// Returns true if the collision list you passed in implies that you can jump.
fn validate_jump(collisions: &ShapeHits) -> bool {
    // iterate through the collisions list and find any valid hit, returns boolean
    collisions.iter().any(|hit| {
        // hit.normal2 is negative to flip the normal around towards the player
        // normal2 is the ground, check if the normal is 45 degrees to player, then it is walkable
        -hit.normal2.y >= VALID_JUMP_ANGLE_COS
    })
}

/// Detects when this instance left clicks.
pub fn hero_left_click(
    click: Res<ButtonInput<MouseButton>>,
    cast: Single<&RayHits, With<hero::definition::DebugTool>>,
    interacts: Query<&objects::definition::Thing>,
    children_q: Query<&Children>,
    mut visibles: Query<&mut objects::definition::Visible>
) {
    if click.just_pressed(MouseButton::Left) {
        logic::on_click(&cast, &interacts, &children_q, &mut visibles);
    }
}

/// Updates the player's stored rotation from mouse movement.
pub fn read_camera(mot: Res<AccumulatedMouseMotion>, mut hero: Single<&mut Hero>) {
    if hero.paused {
        return;
    }
    hero.pitch = (hero.pitch - mot.delta.y * hero.sens.y).clamp(
        -(89.9_f32).to_radians(),
        (89.9_f32).to_radians()
    );
    hero.yaw = (hero.yaw - mot.delta.x * hero.sens.x).rem_euclid(std::f32::consts::TAU);
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
