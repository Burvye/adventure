use bevy::prelude::*;
use bevy::prelude::KeyCode::{ KeyW, KeyA, KeyS, KeyD, Space };
use bevy::input::mouse::AccumulatedMouseMotion;

use avian3d::prelude::*;

use crate::hero::definition::Hero;
use crate::hero::definition::HeroCamera;
use crate::hero::definition::HeroBody;
use crate::motion::definition::WantMove;

use crate::hero;
use crate::cash_register;
use crate::build;

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

// TODO: More modular detection for cash registers
/// Detects when this instance left clicks.
pub fn hero_left_click(
    mut cmds: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    click: Res<ButtonInput<MouseButton>>,
    cast: Single<(&RayHits, &GlobalTransform), With<hero::definition::DebugTool>>,
    myself: Single<Entity, With<Hero>>,
    cash_register: Single<Entity, With<cash_register::definition::CashRegister>>
) {
    if click.just_pressed(MouseButton::Left) {
        on_click(&mut cmds, &mut mesh, &mut mats, &cast.0, &cast.1, *myself, *cash_register);
    }
}
/// Stuff to run when left clicks are detected.
fn on_click(
    cmds: &mut Commands,
    mesh: &mut ResMut<Assets<Mesh>>,
    mats: &mut ResMut<Assets<StandardMaterial>>,
    hits: &RayHits,
    loc: &GlobalTransform,
    myself: Entity,
    cash_register: Entity
) {
    // iterator that excludes myself
    let hit_non_self = hits.iter().find(|hit| hit.entity != myself);

    let hit_register = hits.iter().find(|hit| hit.entity == cash_register);

    match hit_register {
        Some(hit) => hit_register_function(),
        None => info!("Miss"),
    }
    fn hit_register_function() {
        info!("Hit a cash register!!!");
    }

    match hit_non_self {
        // prints out the data to bevy logger
        Some(hit) => {

            let hit_position = loc.translation() + loc.forward() * hit.distance;

            build::build_cube::physics_cube(
                cmds,
                mesh,
                mats,
                hit_position.x,
                hit_position.y,
                hit_position.z,
            );
            info!("Hit {:?}", hit_position);
        }
        None => info!("Miss"),
    }
}
