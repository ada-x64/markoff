use bevy::prelude::*;
use bevy_simple_subsecond_system::prelude::*;
pub(crate) mod plugins;
pub(crate) use plugins::*;

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins).add_plugins((
        SimpleSubsecondPlugin::default(),
        UiPlugin,
        SimulationPlugin,
    ));

    #[cfg(feature = "dev")]
    app.add_plugins(DevPlugin);

    app.run()
}
