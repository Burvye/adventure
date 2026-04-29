mod almighty;
mod build;
mod objects;
mod ui;

use crate::build::build_cube;
use crate::objects::cash_register;
use crate::objects::ferris;
use crate::objects::hero;

use std::env;

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::remote::http::DEFAULT_PORT;
use bevy::remote::{RemotePlugin, http::RemoteHttpPlugin};
use bevy::render::{
    RenderPlugin,
    settings::{Backends, WgpuSettings, WgpuSettingsPriority},
};
use bevy::window::WindowPlugin;
use bevy::window::WindowResolution;

use bevy_embedded_assets::EmbeddedAssetPlugin;

fn main() -> AppExit {
    let port: u16 = env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(DEFAULT_PORT);

    let default_plugins = DefaultPlugins
        .set(RenderPlugin {
            render_creation: (WgpuSettings {
                backends: Some(Backends::VULKAN),
                priority: WgpuSettingsPriority::Compatibility,
                ..default()
            })
            .into(),
            ..default()
        })
        .set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(640, 360),
                title: "My Bevy App".to_string(),
                ..default()
            }),
            ..default()
        });

    App::new()
        .add_plugins((
            EmbeddedAssetPlugin {
                mode: bevy_embedded_assets::PluginMode::ReplaceDefault,
            },
            default_plugins,
        ))
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
        app.add_systems(
            Startup,
            (
                build::build_world::build_lobby,
                ui::crosshair::spawn_crosshair,
            ),
        );
        app.add_observer(ferris::definition::spawn_ferrises);
        app.add_observer(build_cube::spawn_physics_cube);
        app.add_systems(
            Update,
            (
                hero::control::hero_input, // paramount importance
                hero::control::hero_left_click,
                hero::control::read_camera,
                hero::control::update_body.after(hero::control::read_camera),
                hero::control::update_camera.after(hero::control::read_camera),
                ferris::logic::update_ferris, // ferris logic
                ui::input::toggle_pause,
                almighty::logic::update_visibilities,
                almighty::logic::move_all,
            ),
        );
    }
}
