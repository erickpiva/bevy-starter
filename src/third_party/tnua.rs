use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::prelude::*;

pub(super) struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TnuaAvian3dPlugin::new(FixedUpdate));
    }
}

use bevy::prelude::*;

pub struct ControllerSchemePlugin<S: TnuaScheme>(PhantomData<S>);

impl<S: TnuaScheme> Plugin for ControllerSchemePlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_plugins(TnuaControllerPlugin::<S>::new(FixedUpdate));
    }
}
