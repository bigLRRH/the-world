mod camera;
mod debug;
mod movement;
mod spaceship;

use bevy::prelude::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

pub struct SpaceshipGamePlugins;

impl Plugin for SpaceshipGamePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(SpaceshipPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(MovementPlugin)
            .add_plugins(DebugPlugin);
    }
}
