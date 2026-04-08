use bevy::prelude::*;
use avian3d::prelude::*;
use avian3d::math::*;

use crate::motion::definition::WantMove;
use crate::hero;

/// A tag to identify the singular hero. Every player is a hero in their own instance.
/// There should never be more than one hero. Everyone else is not a hero.
#[derive(Component)]
pub struct Hero {
    /// Stores if the hero is in a paused state.
    pub paused: bool,
    /// Mouse sensitivity of the hero.
    pub sens: Vec2,
    /// Pitch rotation value of the hero to be applied to HeroCamera.
    pub pitch: f32,
    /// Yaw rotation value of the hero to be applied to HeroCamera.
    pub yaw: f32,
}
/// Tag the camera
#[derive(Component)]
pub struct HeroCamera;

/// Tag the body
#[derive(Component)]
pub struct HeroBody;

/// Quickly spawn a hero, Only spawn one please.
pub fn spawn_hero(
    cmds: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>
) {
    cmds.spawn((
        children![
            (
                HeroCamera,
                Camera3d::default(),
                Transform::from_xyz(0.0, 1.6, 0.0),
                children![
                    (
                        // arm
                        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
                        MeshMaterial3d(materials.add(Color::srgb_u8(255, 55, 0))),
                        Transform::from_xyz(1.0, 0.0, -2.0).looking_at(Vec3::Z, Vec3::Y),
                    ),
                    (
                        // flashlight
                        SpotLight {
                            inner_angle: 0.25,
                            outer_angle: 0.5,
                            range: 200.0,
                            intensity: 5_000_000.0, // readible 5 million
                            shadows_enabled: true,
                            ..default()
                        },
                        Transform::default(),
                    )
                ],
            ),
            (
                HeroBody,
                Mesh3d(meshes.add(Capsule3d::new(0.5, 1.8))),
                MeshMaterial3d(materials.add(Color::srgb_u8(255, 55, 0))),
                Transform::default(),
            )
        ],
        Transform::from_xyz(10.0, 2.0, 10.0),
        Collider::capsule(0.5, 1.8),
        LockedAxes::ROTATION_LOCKED,
        RigidBody::Dynamic,
        WantMove { zinput: 0, xinput: 0, jump: false, forward: Vec3::ZERO, move_speed: 10.0 },
        hero::definition::Hero {
            paused: true,
            sens: Vec2 { x: 0.01, y: 0.01 },
            pitch: 0.0,
            yaw: 0.0,
        },
        ShapeCaster::new(
            Collider::capsule(0.49, 1.79),
            Vector::ZERO,
            Quaternion::default(),
            Dir3::NEG_Y
        ).with_max_distance(0.15),
    ));
}
