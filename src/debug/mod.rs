use avian3d::prelude::*;
use bevy::{
    diagnostic::*,
    feathers::{FeathersPlugins, dark_theme::create_dark_theme, theme::UiTheme},
    prelude::*,
};
use bevy_console::*;

pub(super) struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // Feather
        app.add_plugins(FeathersPlugins)
            .insert_resource(UiTheme(create_dark_theme()));

        // Console
        app.add_plugins(ConsolePlugin)
            .insert_resource(ConsoleConfiguration { ..default() });

        // Physics
        app.add_plugins((PhysicsDebugPlugin, PhysicsDiagnosticsUiPlugin));

        // Bevy Log
        app.add_plugins((
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
            EntityCountDiagnosticsPlugin::default(),
            SystemInformationDiagnosticsPlugin,
        ));
    }
}

// IDEAS!
//      commands should be a tree
//      physics_debug
//          enable / disable
//          gizmos
//              aabb {Color}
//              joint
//                  anchor {Color}
//                  separation {Color}
