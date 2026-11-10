use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(super) struct BeiPlugin;

impl Plugin for BeiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnhancedInputPlugin);
    }
}
