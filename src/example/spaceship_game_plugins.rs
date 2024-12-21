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
        app.insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
            .insert_resource(AmbientLight {
                color: Color::default(),
                brightness: 1000.0,
            })
            .add_plugins(MovementPlugin)
            .add_plugins(DebugPlugin)
            .add_plugins(SpaceshipPlugin)
            .add_plugins(CameraPlugin);
    }
}
