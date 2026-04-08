use bevy::prelude::*;
use avian3d::prelude::*;

pub fn physics_cube(
    cmds: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>
) {
    cmds.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 102, 0))),
        Transform::from_xyz(0.0, 10.0, 0.0),
        Collider::cuboid(1.0, 1.0, 1.0),
        RigidBody::Dynamic,
    ));
}
