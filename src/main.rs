use bevy::prelude::*;

mod days;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(days::DaysPlugin)
        .run();
}
