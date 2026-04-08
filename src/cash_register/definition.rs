use bevy::prelude::*;
use avian3d::prelude::*;

#[derive(Component)]
pub struct CashRegister;

pub fn build_cash_register(cmds: &mut Commands, asset_server: &Res<AssetServer>) {
    cmds.spawn((
        SceneRoot(asset_server.load("models/map/register.glb#Scene0")),
        Collider::cuboid(1.0, 1.0, 1.0),
        Transform::from_rotation(
            Quat::from_euler(EulerRot::XYZ, 0.0, (90_f32).to_radians(), 0.0)
        ).with_translation(Vec3 { x: 4.5, y: 1.25, z: 23.0 }),
        CashRegister,
        RigidBody::Static,
    ));
}
