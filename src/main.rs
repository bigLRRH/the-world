use bevy::prelude::*;

fn main() {
    App::new()
        //  Bevy built-ins.
        .add_plugins(DefaultPlugins)
        // User-defined plugins.
        // .add_plugins(DebugPlugin)
        .run();
}
