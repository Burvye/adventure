use bevy::prelude::*;
use avian3d::prelude::*;
use crate::objects;

use crate::cash_register;
use crate::objects::ferris;
use crate::objects::definition::Thing;

/// Stuff to run when left clicks are detected.
pub fn on_click(
    hits: &RayHits,
    cmds: &mut Commands,
    interacts: &Query<&objects::definition::Thing>,
    children: &Query<&Children>,
    visibles: &mut Query<&mut objects::definition::Visible>
) {
    // more idiomatic way to check for hits
    for hit in hits.iter() {
        if let Ok(thing) = interacts.get(hit.entity) {
            match thing {
                Thing::CashRegister => {
                    cash_register::logic::toggle_light(hit.entity, children, visibles);
                    cmds.trigger(ferris::definition::SpawnFerrisesEvent);
                },
                Thing::Ferris => {
                    // TODO: Add Ferris logic here
                    ferris::logic::click_ferris();
                    info!("Hit Ferris");
                }
                _ => {
                    info!("Hit something else");
                }
            }
            // break on the first valid collision
            break;
        }
    }
}
