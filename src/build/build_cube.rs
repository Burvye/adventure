use bevy::prelude::*;
use avian3d::prelude::*;

/// Spawns a simple cube at the given coordinates.
pub fn physics_cube(
    cmds: &mut Commands,
    mesh: &mut ResMut<Assets<Mesh>>,
    mats: &mut ResMut<Assets<StandardMaterial>>,
    x: f32,
    y: f32,
    z: f32,
) {
    cmds.spawn((
        Mesh3d(mesh.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(mats.add(Color::srgb_u8(255, 102, 0))),
        Transform::from_xyz(x, y, z),
        Collider::cuboid(1.0, 1.0, 1.0),
        RigidBody::Dynamic,
    ));
}
