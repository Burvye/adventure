mod hero;
mod motion;
mod setup;
mod ui;

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
        app.add_systems(Startup, setup::build_world::build_lobby); // all world load stuff
        app.add_systems(Update, (
            hero::control::hero_input,
            hero::control::read_camera,
            hero::control::update_body.after(hero::control::read_camera),
            hero::control::update_camera.after(hero::control::read_camera),
            ui::input::toggle_pause,
            motion::movement::move_all
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

// TODO: Debug, remove later
/// Function to debug and test some random stuff.
fn tick(query: Query<&WantMove>) {
    for mover in query {
        println!("Hero forward {:?} and right {:?}", &mover.zinput, &mover.xinput);
    }
}
