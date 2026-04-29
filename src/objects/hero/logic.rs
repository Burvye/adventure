use crate::objects;
use avian3d::prelude::*;
use bevy::prelude::*;

use crate::cash_register;
use crate::objects::definition::Thing;
use crate::objects::ferris;

use crate::build::build_cube;

/// Stuff to run when left clicks are detected.
pub fn on_click(
    ray: &RayCaster,
    hits: &RayHits,
    cmds: &mut Commands,
    interacts: &Query<&objects::definition::Thing>,
    children: &Query<&Children>,
    visibles: &mut Query<&mut objects::definition::Visible>,
) {
    // more idiomatic way to check for hits
    if let Some(hit) = hits.iter_sorted().next() {
        if let Ok(thing) = interacts.get(hit.entity) {
            match thing {
                Thing::CashRegister => {
                    cash_register::logic::toggle_light(hit.entity, children, visibles);
                    cmds.trigger(ferris::definition::SpawnFerrisesEvent);
                }
                Thing::Ferris => {
                    // pass in the hit entity into click ferris
                    ferris::logic::click_ferris(hit.entity, cmds);
                    info!("Hit Ferris");
                }
            }
        } else {
            let impact = ray.global_origin() + *ray.global_direction() * hit.distance;
            let spawn = impact + hit.normal + 1.0;
            cmds.trigger(build_cube::SpawnCubeEvent::new(spawn.x, spawn.y, spawn.z))
        }
    }
}
