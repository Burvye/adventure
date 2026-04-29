use avian3d::prelude::*;
use bevy::prelude::*;

/// Event to spawn a cube easily
/// Just do
#[derive(Event)]
pub struct SpawnCubeEvent {
    position: Vec3,
}

impl SpawnCubeEvent {
    pub fn new(position: Vec3) -> Self {
        Self { position }
    }
}

/// Spawns a rigidbody cube at these coordinates.
pub fn spawn_physics_cube_at(
    cmds: &mut Commands,
    mesh: &mut Assets<Mesh>,
    mats: &mut Assets<StandardMaterial>,
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

/// Spawns a cube through SpawnCubeEvent.
pub fn spawn_physics_cube(
    event: On<SpawnCubeEvent>,
    mut cmds: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    spawn_physics_cube_at(
        &mut cmds,
        &mut mesh,
        &mut mats,
        event.position.x,
        event.position.y,
        event.position.z,
    );
}
