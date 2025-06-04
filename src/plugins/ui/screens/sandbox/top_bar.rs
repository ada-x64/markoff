use bevy::prelude::*;
use bevy_simple_subsecond_system::hot;

use crate::{screens::Screen, widgets::*};

#[hot]
pub fn top_bar(commands: &mut Commands) -> Entity {
    let back_btn = commands
        .spawn(button("back", "Back"))
        .observe(on_click)
        .id();
    commands
        .spawn((grid_bar(
            GridPlacement::start(1),
            GridPlacement::start_span(1, 3),
            UiRect::bottom(Val::Px(1.)),
        ),))
        .add_child(back_btn)
        .id()
}

fn on_click(_trigger: Trigger<Pointer<Click>>, mut screen: ResMut<NextState<Screen>>) {
    info!("Got click!");
    screen.set(Screen::MainMenu);
}
