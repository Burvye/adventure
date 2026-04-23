mod objects;
mod almighty;
mod build;
mod ui;

use crate::objects::ferris;
use crate::objects::hero;
use crate::objects::cash_register;

use std::env;

use bevy::prelude::*;
use bevy::render::{ settings::{ Backends, WgpuSettings, WgpuSettingsPriority }, RenderPlugin };
use avian3d::prelude::*;
use bevy::remote::http::DEFAULT_PORT;
use bevy::remote::{ http::RemoteHttpPlugin, RemotePlugin };

fn main() -> AppExit {
    let port: u16 = env
        ::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(DEFAULT_PORT);

    // AMD's current Windows driver stack is more stable here when Bevy uses
    // Vulkan with compatibility-oriented limits instead of the default auto
    // backend selection.
    let default_plugins = DefaultPlugins.set(RenderPlugin {
        render_creation: (WgpuSettings {
            backends: Some(Backends::VULKAN),
            priority: WgpuSettingsPriority::Compatibility,
            ..default()
        }).into(),
        ..default()
    });

    App::new()
        .add_plugins(default_plugins)
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
        app.add_systems(Startup, (build::build_world::build_lobby, ui::crosshair::spawn_crosshair));
        app.add_observer(ferris::definition::spawn_ferrises);
        app.add_systems(Update, (
            hero::control::hero_input, // paramount importance
            hero::control::hero_left_click,
            hero::control::read_camera,
            hero::control::update_body.after(hero::control::read_camera),
            hero::control::update_camera.after(hero::control::read_camera),
            ferris::logic::update_ferris, // ferris logic
            ui::input::toggle_pause,
            almighty::logic::update_visibilities,
            almighty::logic::move_all,
        ));
    }
}
