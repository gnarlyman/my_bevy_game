use bevy::prelude::*;
use my_bevy_game::setup::SceneSetupPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SceneSetupPlugin)
        .run();
}
