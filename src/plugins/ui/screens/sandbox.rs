use bevy::prelude::*;
use bevy_hui::prelude::*;
use bevy_simple_subsecond_system::hot;

use crate::{
    sim::{SimSettings, SimState},
    ui::{
        Slider, SliderChangedEvent,
        screens::{CurrentScreen, ScreenMarker},
    },
};

pub struct SandboxScreenPlugin;
impl Plugin for SandboxScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_slider_input_change)
            .add_systems(Startup, register)
            .add_systems(OnEnter(CurrentScreen::Sandbox), render);
    }
}

fn render(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((
        ScreenMarker,
        HtmlNode(server.load("hui/screens/sandbox.xml")),
    ));
}

fn register(
    mut html_comps: HtmlComponents,
    mut html_funcs: HtmlFunctions,
    server: Res<AssetServer>,
) {
    let mut register = |name: &str| {
        html_comps.register(
            format!("sandbox_{name}"),
            server.load(format!("hui/screens/sandbox/{name}.xml")),
        );
    };
    register("settings");
    html_funcs.register(
        "load_sim",
        |In(entity),
         mut sim_state: ResMut<NextState<SimState>>,
         mut settings: ResMut<SimSettings>| {
            settings.parent_node = Some(entity);
            sim_state.set(SimState::Init);
        },
    );
}

#[hot]
fn on_slider_input_change(
    trigger: Trigger<SliderChangedEvent>,
    sliders: Query<(&Slider, &UiTarget, &Tags)>,
    mut settings: ResMut<SimSettings>,
    mut texts: Query<&mut Text>,
) {
    let Ok((slider, target, tags)) = sliders.get(trigger.slider_entity) else {
        warn!("Could not get slider! Trigger: {trigger:?}");
        return;
    };
    let Ok(mut text) = texts.get_mut(target.0) else {
        warn!("Could not get text from entity {target:?}\nTrigger: {trigger:?}");
        return;
    };
    let Some(name) = tags.get("name") else {
        warn!("Missing name tag");
        return;
    };
    match name.as_str() {
        "sim_size_slider" => {
            let value = u32::pow(2, (5. + slider.value * 4.).round() as u32);
            settings.size = value;
            text.0 = value.to_string() + "px";
            // would like it to snap but this is good enough
            info!("sim_size: {value}");
        }
        _ => {
            warn!("Unknown name {name}")
        }
    }
}
