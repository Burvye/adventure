use bevy::prelude::*;

/// Tag all crosshair entities
#[derive(Component)]
pub struct Crosshair;

/// Spawns a cross crosshair at the middle of the screen
pub fn spawn_crosshair(mut cmds: Commands) {
    cmds.spawn((
        // root node
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            ..default()
        },
        children![
            (
                // horizontal wide bar
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(10.0),
                    height: Val::Px(1.0),
                    left: Val::Px(-5.0),
                    top: Val::Px(-0.5),
                    ..default()
                },
                BackgroundColor(Color::WHITE),
                Crosshair,
            ),
            (
                /// vertical tall bar
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(1.0),
                    height: Val::Px(10.0),
                    left: Val::Px(-0.5),
                    top: Val::Px(-5.0),
                    ..default()
                },
                BackgroundColor(Color::WHITE),
                Crosshair,
            )
        ],
    ));
}
