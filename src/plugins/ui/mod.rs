use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    ui::RelativeCursorPosition,
};
pub mod screens;
pub mod widgets;
use bevy_hui_widgets::prelude::*;

use bevy_hui::{
    HuiPlugin,
    prelude::{HtmlComponents, HtmlFunctions},
};
use screens::*;

use crate::ui::widgets::{WidgetsPlugin, slider::Slider};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        let _ = {
            app.add_plugins((
                ScreensPlugin,
                HuiPlugin,
                WidgetsPlugin,
                HuiSelectWidgetPlugin,
            ))
            .add_systems(Startup, (spawn_camera, register_widgets))
            .add_systems(Update, update_scrollable)
        };
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, UiPickingCamera));
}

fn register_widgets(
    mut html_comps: HtmlComponents,
    mut html_funcs: HtmlFunctions,
    server: Res<AssetServer>,
) {
    let mut register = |name: &str| {
        html_comps.register(name, server.load(format!("hui/components/{name}.xml")));
    };
    register("menu_button");
    register("slider");
    register("text_input");
    register("slider_input");
    register("settings_button");
    register("select");
    register("option");

    html_funcs.register(
        "notify_slider_update",
        |In(entity), sliders: Query<&Slider>, mut commands: Commands| {
            let Ok(slider) = sliders.get(entity) else {
                warn!("Could not get slider!");
                return;
            };
            // Should this be on the entity? Would require getting it at spawn time.
            commands.trigger(SliderChangedEvent {
                slider_entity: entity,
                value: slider.value,
            });
        },
    );
}

#[derive(Event, Reflect, Debug)]
#[reflect]
pub struct SliderChangedEvent {
    pub slider_entity: Entity,
    pub value: f32,
}

#[derive(Component, Default, Debug)]
#[require(RelativeCursorPosition)]
struct Scrollable;

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
