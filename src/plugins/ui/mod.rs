use bevy::prelude::*;
pub mod screens;
pub mod widgets;

use bevy_hui::{
    HuiPlugin,
    prelude::{HtmlComponents, HtmlFunctions, Tags, TemplateProperties, TemplateScope},
};
use screens::*;

use crate::ui::widgets::{WidgetsPlugin, slider::Slider};
// use theme::*;
// use widgets::*;

// #[derive(Resource, Debug, Clone)]
// pub struct UiAssets {
//     pub bg1: Handle<Image>,
// }
// impl FromWorld for UiAssets {
//     fn from_world(world: &mut World) -> Self {
//         let bg1 = world.load_asset("textures/bg1.png");
//         Self { bg1 }
//     }
// }

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        let _ = {
            app.add_plugins((ScreensPlugin, HuiPlugin, WidgetsPlugin))
                // .init_resource::<UiAssets>()
                .add_systems(
                    PreStartup,
                    |mut pick_settings: ResMut<UiPickingSettings>| {
                        *pick_settings = UiPickingSettings {
                            require_markers: true,
                        }
                    },
                )
                .add_systems(Startup, (spawn_camera, register_widgets))
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
    register("grid_layout");
    register("row");
    register("column");
    register("slider");
    register("text_input");
    register("slider_input");

    html_funcs.register(
        "on_spawn_slider_input",
        |In(entity), mut commands: Commands, tags: Query<&Tags>| {
            let Some(name) = tags.get(entity).ok().and_then(|tags| tags.get("name")) else {
                warn!("Could not get entity name! {entity}");
                return;
            };
            let Ok(mut cmds) = commands.get_entity(entity) else {
                warn!("Could not get entity for slider_input with name {name}!");
                return;
            };
            cmds.insert(SliderInput {
                value: 0.,
                name: name.clone(),
            });
        },
    );
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

#[derive(Component, Debug, Clone)]
pub struct SliderInput {
    pub value: f32,
    pub name: String,
}
