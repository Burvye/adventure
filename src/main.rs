mod hero;

use bevy::prelude::*;
use avian3d::prelude::*;

use crate::hero::hero_main::Hero;

fn main() -> AppExit {
    App::new().add_plugins((DefaultPlugins, MainPlugin)).run()
}

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default());
        app.add_systems(Startup, build_world);
        app.add_systems(Update, hero::hero_main::update_input);
        app.add_systems(Update, tick);
    }
}

fn build_world(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // player
    cmds.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 2.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        Collider::capsule(0.5, 1.8),
        LockedAxes::ROTATION_LOCKED,
        RigidBody::Dynamic,
        Hero { forward: 0.0, right: 0.0 },
    ));

    // cube
    cmds.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 5.0, 0.0),
        Collider::cuboid(1.0, 1.0, 1.0),
        RigidBody::Dynamic,
    ));

    // ground
    cmds.spawn((
        Mesh3d(meshes.add(Cuboid::new(50.0, 0.5, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(50, 200, 50))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Collider::cuboid(50.0, 0.5, 50.0),
        RigidBody::Static,
    ));

    // light
    cmds.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn tick(query: Query<&Hero>) {
    for hero in query {
        println!("Hero forward {:?} and right {:?}", &hero.forward, &hero.right);
    }
}
