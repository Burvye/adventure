use bevy::prelude::*;
use avian3d::prelude::*;
use crate::objects;

/// Stuff to run when left clicks are detected.
pub fn on_click(
    hits: &RayHits,
    interacts: &Query<&objects::definition::Interaction>,
) {

    for hit in hits.iter() {
        match interacts.get(hit.entity) {
            Ok(interact) => handle_interact(interact),
            Err(_) => {},
        }
    }
}
/// Matches interactions to desired behaviors.
fn handle_interact(interact: &objects::definition::Interaction) {
    match interact {
        objects::definition::Interaction::CashRegister => {
            info!("Cash Register!");
        }
    }
}