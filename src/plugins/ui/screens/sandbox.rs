use bevy::prelude::*;
use bevy_hui::prelude::*;
use tiny_bail::prelude::*;

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
            let select = r!(selects.get(entity));
            let layout = r!(SimLayout::try_from(&select.value));
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
    let select = r!(selects.get(event.select));
    let tags = r!(tags.get(event.select));
    let name = r!(tags.get("name").ok_or("tag 'name' not found"));
    match name.as_str() {
        "layout_select" => {
            let layout = r!(SimLayout::try_from(&select.value));
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
    let (slider, target, tags) = r!(sliders.get(trigger.slider));
    let mut text = r!(texts.get_mut(target.0));
    let name = r!(tags.get("name").ok_or("tag 'name' not found"));
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
