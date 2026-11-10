use avian3d::collision::collider::Collider;
use bevy::prelude::*;
use vleue_navigator::prelude::*;

pub(super) struct NavigatorPlugin;

impl Plugin for NavigatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            VleueNavigatorPlugin,
            NavmeshUpdaterPlugin::<Collider, NavigationObstacle>::default(),
        ));
    }
}

#[derive(Component, Reflect, Debug, Clone, Default)]
#[reflect(Component, Clone, Default)]
pub struct NavigationObstacle;
