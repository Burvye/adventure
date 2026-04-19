use bevy::prelude::*;
use avian3d::prelude::*;
use crate::objects;

use crate::cash_register::logic::toggle_light;

/// Stuff to run when left clicks are detected.
pub fn on_click(
    hits: &RayHits,
    interacts: &Query<&objects::definition::Thing>,
    children: &Query<&Children>,
    visibles: &mut Query<&mut objects::definition::Visible>
) {
    if let Some(hit) = hits.iter().next() {
        if let Ok(thing) = interacts.get(hit.entity) {
            match thing {
                objects::definition::Thing::CashRegister => {
                    toggle_light(hit.entity, children, visibles);
                }
            }
        }
    }
}

