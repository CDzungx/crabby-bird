use bevy::prelude::*;

mod components;
mod debug;
mod default;
mod dev_tools;
mod game;
mod input;
mod physics;
mod resources;
mod scene;
mod utils;

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
        ));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins((dev_tools::plugin, debug::plugin));
    }
}
