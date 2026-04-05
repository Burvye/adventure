mod hero;
mod motion;

use bevy::prelude::*;
use avian3d::prelude::*;

// TODO: Debug, remove later along with "tick" function
use crate::motion::definition::WantMove;

fn main() -> AppExit {
    App::new().add_plugins((DefaultPlugins, MainPlugin, SidePlugin)).run()
}

/// The bare minimum essential functions to run this game.
pub struct MainPlugin;
impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default());
        app.insert_resource(Gravity(Vec3::NEG_Y * 30.0));
        app.add_systems(Startup, build_lobby); // all world load stuff
        app.add_systems(Update, (
            hero::control::hero_input,
            motion::movement::move_all,
        ));
    }
}

/// Non-essential stuff that can be removed and the game would still function well.
pub struct SidePlugin;
impl Plugin for SidePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick); // TODO: Debug, remove later
    }
}
/// Construct the default environment to spawn in
fn build_lobby(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // "hero"
    hero::definition::spawn_hero(&mut cmds);

    // TODO: Add a dedicated "spawn cube" function
    // cube
    cmds.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 102, 0))),
        Transform::from_xyz(0.0, 5.0, 0.0),
        Collider::cuboid(1.0, 1.0, 1.0),
        RigidBody::Dynamic,
    ));

    // ground
    cmds.spawn((
        Mesh3d(meshes.add(Cuboid::new(50.0, 0.5, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(60, 8, 0))),
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

// TODO: Debug, remove later
/// Function to debug and test some random stuff.
fn tick(query: Query<&WantMove>) {
    for mover in query {
        println!("Hero forward {:?} and right {:?}", &mover.forward, &mover.right);
    }
}
