use bevy::prelude::*;
use bevy_simple_subsecond_system::hot;

use crate::ui::screens::ScreenMarker;
use crate::ui::screens::sandbox::left_panel_items::left_panel_items;
use crate::ui::screens::sandbox::top_bar::top_bar;
use crate::ui::widgets::*;

mod left_panel_items;
mod top_bar;

#[hot]
pub fn init(mut commands: Commands) {
    let layout = grid_layout(
        vec![GridTrack::px(32.), GridTrack::auto(), GridTrack::px(32.)],
        vec![
            GridTrack::percent(15.),
            GridTrack::percent(70.),
            GridTrack::percent(15.),
        ],
    );

    let top_bar = top_bar(&mut commands);
    let left_panel = commands
        .spawn(grid_panel(
            GridPlacement::start(2),
            GridPlacement::start(1),
            UiRect::all(Val::Px(0.)),
        ))
        .with_children(left_panel_items)
        .id();
    let center_panel = commands
        .spawn(grid_panel(
            GridPlacement::start(2),
            GridPlacement::start(2),
            UiRect::axes(Val::Px(2.), Val::Px(0.)),
        ))
        .with_child(TextBundle::new("center"))
        .id();
    let right_panel = commands
        .spawn(grid_panel(
            GridPlacement::start(2),
            GridPlacement::start(3),
            UiRect::all(Val::Px(0.)),
        ))
        .with_child(TextBundle::new("right"))
        .id();
    let bottom_bar = commands
        .spawn((
            grid_bar(
                GridPlacement::start(3),
                GridPlacement::start_span(1, 3),
                UiRect::top(Val::Px(2.)),
            ),
            children![TextBundle::new("bottom")],
        ))
        .id();

    // layout
    commands.spawn((ScreenMarker, layout)).add_children(&[
        top_bar,
        left_panel,
        center_panel,
        right_panel,
        bottom_bar,
    ]);
}
