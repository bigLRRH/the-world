use bevy::prelude::*;

#[derive(Resource,Debug)]
pub struct GameState{
    is_playing: bool,
}