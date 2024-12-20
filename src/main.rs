use bevy::prelude::*;

mod example;

use example::SpaceshipGamePlugins;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        //  Bevy built-ins.
        .add_plugins(DefaultPlugins)
        .add_plugins(SpaceshipGamePlugins)
        .run();
}
