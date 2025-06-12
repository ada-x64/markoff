use bevy::prelude::*;
use bevy_hui::prelude::*;

use crate::{
    sim::{SimLayout, SimSettings, SimState},
    ui::{
        Slider,
        screens::{CurrentScreen, ScreenRoot},
        widgets::data::{SelectInput, SelectionChangedEvent, SliderChangedEvent},
    },
};

pub struct SandboxScreenPlugin;
impl Plugin for SandboxScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_slider_input_change)
            .add_observer(on_select_change)
            .add_systems(Startup, register)
            .add_systems(OnEnter(CurrentScreen::Sandbox), render);
    }
}

fn render(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((ScreenRoot, HtmlNode(server.load("hui/screens/sandbox.xml"))));
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
    register("tools");
    html_funcs.register(
        "load_sim",
        |In(entity),
         mut sim_state: ResMut<NextState<SimState>>,
         mut settings: ResMut<SimSettings>| {
            settings.parent_node = Some(entity);
            sim_state.set(SimState::Init);
        },
    );
    html_funcs.register(
        "apply_settings",
        |In(_entity), mut sim_state: ResMut<NextState<SimState>>| {
            sim_state.set(SimState::Init);
        },
    );
    html_funcs.register(
        "on_select_layout",
        |In(entity), selects: Query<&SelectInput>, mut settings: ResMut<SimSettings>| {
            let Ok(select) = selects.get(entity) else {
                warn!("Could not get select {entity}");
                return;
            };
            let Ok(layout) = SimLayout::try_from(&select.value) else {
                warn!("Unknown layout {}", select.value);
                return;
            };
            settings.layout = layout;
        },
    );
    html_funcs.register(
        "goto_main_menu",
        |In(_), mut screen: ResMut<NextState<CurrentScreen>>| {
            screen.set(CurrentScreen::MainMenu);
        },
    )
}

fn on_select_change(
    trigger: Trigger<SelectionChangedEvent>,
    selects: Query<&SelectInput>,
    tags: Query<&Tags>,
    mut settings: ResMut<SimSettings>,
) {
    info!("on-select-change");
    let event = trigger.event();
    let Some(select) = selects.get(event.select).ok() else {
        warn!("couldnt' get select");
        return;
    };
    let Some(name) = tags
        .get(event.select)
        .ok()
        .and_then(|tags| tags.get("name"))
    else {
        warn!("Couldn't get tags");
        return;
    };
    match name.as_str() {
        "layout_select" => {
            let Ok(layout) = SimLayout::try_from(&select.value) else {
                warn!("Unknown layout {}", select.value);
                return;
            };
            settings.layout = layout;
            info!("settings.layout = {}", settings.layout);
        }
        _ => {
            warn!("Unknown select: {name}")
        }
    }
}

fn on_slider_input_change(
    trigger: Trigger<SliderChangedEvent>,
    sliders: Query<(&Slider, &UiTarget, &Tags)>,
    mut settings: ResMut<SimSettings>,
    mut texts: Query<&mut Text>,
) {
    let Ok((slider, target, tags)) = sliders.get(trigger.slider) else {
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
            text.0 = value.to_string();
        }
        "sim_speed_slider" => {
            let value = ((slider.value * 11.) as u32) * 5 + 5;
            settings.timestep = value;
            text.0 = value.to_string();
        }
        "sim_steps_slider" => {
            let value = ((slider.value * 99.) as u32) * 10 + 10;
            settings.steps_per_turn = value;
            text.0 = value.to_string();
        }
        _ => {
            warn!("Unknown name {name}")
        }
    }
}
