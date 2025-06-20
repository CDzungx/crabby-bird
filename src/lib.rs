use bevy::prelude::*;

mod components;
mod debug;
mod default;
mod game;
mod game_state;
mod input;
mod physics;
mod pipes;
mod resources;
mod scene;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            default::plugin,
            scene::plugin,
            physics::plugin,
            input::plugin,
            game::plugin,
            resources::plugin,
            pipes::plugin,
            game_state::plugin,
        ));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins((dev_tools::plugin, debug::plugin));
    }
}
