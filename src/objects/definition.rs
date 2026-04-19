use bevy::prelude::*;

#[derive(Component)]
/// List of different interaction enumerations.
pub enum Thing {
    CashRegister,
}


#[derive(Component)]
pub struct Visible(pub bool);