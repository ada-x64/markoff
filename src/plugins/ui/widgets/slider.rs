use crate::ui::data::TemplateHandles;

// From https://github.com/Lommix/bevy_hui/blob/0f7c1aff0563b823e62c6e3514a43cfb24d134e7/crates/bevy_hui_widgets/src/slider.rs
use super::data::*;
use bevy::prelude::*;
use bevy_hui::prelude::*;

pub struct SliderWidgetPlugin;
impl Plugin for SliderWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SliderAxis>();
        app.register_type::<Slider>();
        app.register_type::<SliderChangedEvent>();
        app.add_event::<SliderChangedEvent>();
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (
                update_drag,
                update_slider_value.run_if(on_event::<SliderChangedEvent>),
            ),
        );
    }
}

fn setup(
    mut html_funcs: HtmlFunctions,
    mut comps: HtmlComponents,
    mut templates: ResMut<TemplateHandles>,
    server: Res<AssetServer>,
) {
    let handle = server.load("hui/widgets/slider.xml");
    comps.register("slider", handle.clone());
    templates.insert("slider", handle.clone());
    html_funcs.register("init_slider", init_slider);
}

fn init_slider(
    In(entity): In<Entity>,
    children: Query<&Children>,
    tags: Query<&Tags>,
    buttons: Query<(), With<Button>>,
    mut cmd: Commands,
) {
    let Some(nob_entity) = children.get(entity).ok().and_then(|children| {
        children
            .iter()
            .find_map(|child| buttons.get(child).ok().map(|_| child))
    }) else {
        error!(
            "Your slider needs to have an absolute button child, which will be the draggable nob"
        );
        return;
    };

    let value = tags
        .get(entity)
        .ok()
        .and_then(|tags| {
            tags.get("initial_position")
                .map(|str| str.parse::<f32>().unwrap_or_default())
        })
        .unwrap_or_default();

    let axis = tags
        .get(entity)
        .ok()
        .and_then(|tags| {
            tags.get(TAG_AXIS)
                .map(|str_val| SliderAxis::from(str_val.as_str()))
        })
        .unwrap_or_default();

    cmd.entity(entity).insert(Slider { value, axis });
    cmd.entity(nob_entity).insert(SliderNob { slider: entity });
}

fn update_drag(
    mut slider_events: EventWriter<SliderChangedEvent>,
    mut events: EventReader<bevy::input::mouse::MouseMotion>,
    mut nobs: Query<(Entity, &SliderNob, &mut HtmlStyle, &Interaction)>,
    sliders: Query<&Slider>,
    computed_nodes: Query<&ComputedNode>,
    mut commands: Commands,
) {
    for event in events.read() {
        nobs.iter_mut()
            .filter(|(_, _, _, interaction)| matches!(interaction, Interaction::Pressed))
            .for_each(|(nob_entity, nob, mut style, _)| {
                let Ok(slider_computed) = computed_nodes.get(nob.slider) else {
                    return;
                };

                let Ok(nob_computed) = computed_nodes.get(nob_entity) else {
                    return;
                };

                let Ok(slider) = sliders.get(nob.slider) else {
                    return;
                };

                match slider.axis {
                    SliderAxis::Horizontal => {
                        let current_pos = match style.computed.node.left {
                            Val::Px(pos) => pos,
                            _ => 0.,
                        };

                        let max_pos = (slider_computed.unrounded_size().x)
                            * slider_computed.inverse_scale_factor()
                            - nob_computed.unrounded_size().x * nob_computed.inverse_scale_factor();

                        let next_pos = (current_pos
                            + event.delta.x / slider_computed.inverse_scale_factor())
                        .min(max_pos)
                        .max(0.);

                        let slider_value = next_pos / max_pos;
                        style.computed.node.left = Val::Px(next_pos);
                        let event = SliderChangedEvent {
                            slider: nob.slider,
                            value: slider_value,
                        };
                        slider_events.write(event);
                        commands.trigger(event);
                    }
                    SliderAxis::Vertical => {
                        let current_pos = match style.computed.node.bottom {
                            Val::Px(pos) => pos,
                            _ => 0.,
                        };

                        let max_pos = slider_computed.unrounded_size().y
                            * slider_computed.inverse_scale_factor()
                            - nob_computed.unrounded_size().y * nob_computed.inverse_scale_factor();

                        let next_pos = (current_pos
                            - event.delta.y / slider_computed.inverse_scale_factor())
                        .min(max_pos)
                        .max(0.);

                        let slider_value = next_pos / max_pos;
                        style.computed.node.bottom = Val::Px(next_pos);
                        slider_events.write(SliderChangedEvent {
                            slider: nob.slider,
                            value: slider_value,
                        });
                    }
                };
            });
    }
}

fn update_slider_value(
    mut cmd: Commands,
    mut events: EventReader<SliderChangedEvent>,
    mut sliders: Query<(Entity, &mut Slider)>,
) {
    for event in events.read() {
        _ = sliders.get_mut(event.slider).map(|(entity, mut slider)| {
            slider.value = event.value;
            cmd.trigger_targets(UiChangedEvent, entity);
        });
    }
}
