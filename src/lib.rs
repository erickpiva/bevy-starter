#![allow(unused_imports)]

use bevy::prelude::*;

mod components;
mod plugins;
mod resources;
mod styles;
mod third_party;
mod utils;

#[cfg(feature = "dev")]
mod debug;

/// Use this module instead of importing the `components`, `plugins`, `resources`, and `utils`
/// modules directly.
mod prelude {
    pub use super::*;
    pub use {components::*, plugins::*, resources::*, styles::*, third_party::*, utils::*};
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            plugins::defaults::DefaultPlugin,
            third_party::ThirdPartyPlugin,
            plugins::camera::CameraPlugin,
            plugins::game::GamePlugin,
            plugins::input::InputPlugin,
            plugins::physics::PhysicsPlugin,
        ));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(debug::DebugPlugin);
    }
}
