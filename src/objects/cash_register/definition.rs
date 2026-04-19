use bevy::prelude::*;
use avian3d::prelude::*;

use crate::objects;

pub fn build_cash_register(cmds: &mut Commands, asset_server: &Res<AssetServer>) {
    cmds.spawn((
        SceneRoot(asset_server.load("models/register.glb#Scene0")),
        Collider::cuboid(1.0, 1.0, 1.0),
        Transform::from_rotation(
            Quat::from_euler(EulerRot::XYZ, 0.0, (90_f32).to_radians(), 0.0)
        ).with_translation(Vec3 { x: 4.5, y: 1.25, z: 23.0 }),
        objects::definition::Thing::CashRegister,
        RigidBody::Static,
        children![(
            PointLight {
                intensity: 240000.0,
                ..Default::default()
            },
            objects::definition::Visible(false),
            Transform::from_xyz(0.0, 5.0, 0.0),
        )],
    ));
}
