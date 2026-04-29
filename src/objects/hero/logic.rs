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
            cmds.trigger(build_cube::SpawnCubeEvent::new(spawn_loc(
                get_impact(ray.global_origin(), ray.global_direction(), hit.distance),
                hit.normal,
                1.0,
            )))
        }
    }
}
fn get_impact(origin: Vec3, direction: Dir3, distance: f32) -> Vec3 {
    origin + direction * distance
}
fn spawn_loc(impact_pos: Vec3, normal: Vec3, added: f32) -> Vec3 {
    impact_pos + normal * added
}
