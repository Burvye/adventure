use bevy::prelude::*;
use avian3d::prelude::*;

use crate::objects;

/// Ferris logic, whatever Ferris does
pub fn update_ferris(
    targets: Query<&Transform, With<objects::definition::Target>>
) {
    // TODO: Make ferris go towards the closest target
}

/// Logic for when Ferris is clicked
pub fn click_ferris() {
    info!("Ferris Clicked");
}