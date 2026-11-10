use bevy::prelude::*;

mod asset_loader;
mod avian3d;
mod common_assets;
mod enhanced_input;
mod navigator;
mod progress;
mod tnua;

pub use navigator::NavigationObstacle;
pub use tnua::ControllerSchemePlugin;

pub(super) struct ThirdPartyPlugin;

impl Plugin for ThirdPartyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            avian3d::PhysicsPlugin,
            enhanced_input::BeiPlugin,
            asset_loader::AssetLoaderPlugin,
            common_assets::CommonAssetsPlugin,
            tnua::ControllerPlugin,
            progress::ProgressPlugin,
            navigator::NavigatorPlugin,
        ));
    }
}
