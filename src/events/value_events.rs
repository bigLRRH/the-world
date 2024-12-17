use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct ValueUpdated {
    target_id: Id,
    pub new_value: f32,
}

#[derive(Event, Debug)]
pub struct ValueIncreased {
    target_id: Id,
    pub increase_value: f32,
}

#[derive(Event, Debug)]
pub struct ValueDecreased {
    target_id: Id,
    pub decrease_value: f32,
}
