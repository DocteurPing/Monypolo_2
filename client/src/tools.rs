use bevy::prelude::*;
use shared::action::Action;

#[allow(dead_code)]
pub trait ToAction {
    fn to_action(self) -> Action;
}

impl ToAction for &str {
    fn to_action(self) -> Action {
        match self {
            "identify" => Action::Identify,
            "roll" => Action::Roll,
            "buy_all" => Action::BuyAll,
            "buy" => Action::BuyProperty,
            "skip" => Action::SkipBuyProperty,
            _ => Action::Invalid,
        }
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub(crate) fn despawn_screen<T: Component>(
    to_despawn: Query<Entity, With<T>>,
    mut commands: Commands,
) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
