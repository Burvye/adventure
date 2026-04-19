use bevy::prelude::*;
use crate::objects;

pub fn toggle_light(
    register: Entity,
    children: &Query<&Children>,
    visibles: &mut Query<&mut objects::definition::Visible>
) {
    let Ok(childs) = children.get(register) else {
        return;
    };
    for child in childs.iter() {
        if let Ok(mut visible) = visibles.get_mut(child) {
            visible.0 = !visible.0;
            break;
        }
    }
}