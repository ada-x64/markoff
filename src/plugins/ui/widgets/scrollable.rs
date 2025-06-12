use super::data::*;
use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    ui::RelativeCursorPosition,
};

pub struct ScrollableWidgetPlugin;
impl Plugin for ScrollableWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_scrollable);
    }
}

// TODO: Automatically add "scrollable" to any node with overflow,
// or with tag:scrollable
fn update_scrollable(
    query: Query<(&mut ScrollPosition, &RelativeCursorPosition), With<Scrollable>>,
    mut mw_event: EventReader<MouseWheel>,
) {
    for (mut scroll_pos, cursor_pos) in query {
        if cursor_pos.mouse_over() {
            for event in mw_event.read() {
                let (dx, dy) = match event.unit {
                    MouseScrollUnit::Line => (event.x * 12., event.y * 12.),
                    MouseScrollUnit::Pixel => (event.x, event.y),
                };
                scroll_pos.offset_x -= dx;
                scroll_pos.offset_y -= dy;
            }
        }
    }
}
