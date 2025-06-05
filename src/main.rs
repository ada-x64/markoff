#![feature(iter_advance_by)]
#![feature(iter_next_chunk)]
#![feature(slice_as_array)]

use bevy::prelude::*;
use bevy_simple_subsecond_system::prelude::*;
pub(crate) mod plugins;
pub(crate) use plugins::*;

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest()),
        SimpleSubsecondPlugin::default(),
        ui::UiPlugin,
        sim::SimPlugin,
    ));

    #[cfg(feature = "dev")]
    app.add_plugins(dev::DevPlugin);

    app.run()
}
