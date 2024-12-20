use bevy::prelude::*;

mod example;

use example::SpaceshipGamePlugins;

fn main() {
    App::new()
        //  Bevy built-ins.
        .add_plugins(DefaultPlugins)
        .add_plugins(SpaceshipGamePlugins)
        .run();
}
