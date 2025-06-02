use bevy::prelude::*;
use bevy_simple_subsecond_system::prelude::*;
pub(crate) mod plugins;
pub(crate) use plugins::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((SimpleSubsecondPlugin::default(), UiPlugin))
        .run()
}
