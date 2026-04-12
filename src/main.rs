mod hero;
mod objects;
mod motion;
mod build;
mod cash_register;
mod ui;
mod ferris;

use std::env;

use bevy::prelude::*;
use avian3d::prelude::*;
use bevy::remote::http::DEFAULT_PORT;
use bevy::remote::{ http::RemoteHttpPlugin, RemotePlugin };

fn main() -> AppExit {
    let port: u16 = env
        ::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(DEFAULT_PORT);
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MainPlugin)
        .add_plugins(RemotePlugin::default())
        .add_plugins(RemoteHttpPlugin::default().with_port(port))
        .run()
}

/// The bare minimum essential functions to run this game.
pub struct MainPlugin;
impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default());
        app.insert_resource(Gravity(Vec3::NEG_Y * 30.0));
        app.add_systems(Startup, (
            build::build_world::build_lobby,
            ui::crosshair::spawn_crosshair,
            ferris::definition::spawn_ferris,
        )); // all world load stuff
        app.add_systems(Update, (
            hero::control::hero_input, // paramount importance
            hero::control::hero_left_click,
            hero::control::read_camera,
            hero::control::update_body.after(hero::control::read_camera),
            hero::control::update_camera.after(hero::control::read_camera),
            ui::input::toggle_pause,
            motion::movement::move_all,
        ));
    }
}
