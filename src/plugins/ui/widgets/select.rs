use bevy::prelude::*;
use bevy_hui::prelude::*;

use crate::ui::{
    data::TemplateHandles,
    widgets::data::{SelectInput, SelectOption, SelectionChangedEvent},
};

/// # Select Widget
///
/// A select is a button with 2 children. The current
/// selected node and a hidden node, holding the options.
///
pub struct SelectWidgetPlugin;
impl Plugin for SelectWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SelectInput>();
        app.register_type::<SelectOption>();
        app.register_type::<SelectionChangedEvent>();
        app.add_event::<SelectionChangedEvent>();
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (
                open_list,
                selection,
                update_selection.run_if(on_event::<SelectionChangedEvent>),
            ),
        );
    }
}

fn setup(
    mut html_funcs: HtmlFunctions,
    mut html_comps: HtmlComponents,
    mut handles: ResMut<TemplateHandles>,
    server: Res<AssetServer>,
) {
    let handle = server.load("hui/widgets/select.xml");
    html_comps.register("select", handle.clone());
    handles.insert("select", handle);
    html_funcs.register("init_select", init_select);
}

fn init_select(
    In(entity): In<Entity>,
    mut cmd: Commands,
    children: Query<&Children>,
    targets: Query<&UiTarget>,
) {
    cmd.entity(entity).insert(SelectInput::default());

    let Ok(option_holder) = targets.get(entity) else {
        warn!("your select does not have a target option list");
        return;
    };

    info!("option_holder={option_holder:?}");

    _ = children.get(**option_holder).map(|children| {
        children.iter().for_each(|option| {
            info!("adding option component to {entity:?}");
            cmd.entity(option).insert(SelectOption { select: entity });
        });
    });
}

fn open_list(
    selects: Query<(&Interaction, &UiTarget), (With<SelectInput>, Changed<Interaction>)>,
    mut styles: Query<&mut HtmlStyle>,
) {
    for (interaction, target) in selects.iter() {
        let Ok(mut list_style) = styles.get_mut(**target) else {
            continue;
        };

        if interaction == &Interaction::Pressed {
            list_style.computed.node.display = Display::Grid;
        }
    }
}

fn selection(
    mut commands: Commands,
    mut events: EventWriter<SelectionChangedEvent>,
    options: Query<(Entity, &ChildOf, &Interaction, &SelectOption), Changed<Interaction>>,
    mut selects: Query<&mut SelectInput>,
    tags: Query<&Tags>,
    mut styles: Query<&mut HtmlStyle>,
) {
    for (entity, parent, interaction, option) in options.iter() {
        info!(
            "selection entity={entity:?} parent={parent:?} interaction={interaction:?} option={option:?}"
        );
        if !matches!(interaction, Interaction::Pressed) {
            continue;
        }
        let Some(value) = tags.get(entity).ok().and_then(|tags| tags.get("value")) else {
            warn!("Couldn't get value for option {entity}");
            continue;
        };
        let Some(mut select) = selects.get_mut(option.select).ok() else {
            warn!("Couldn't get select for option {entity}");
            continue;
        };
        select.value = value.to_owned();

        let event = SelectionChangedEvent {
            select: option.select,
            option: entity,
            value: value.to_owned(),
        };

        events.write(event.clone());
        commands.trigger(event);
        info!("Triggered selection event");

        // close the list
        _ = styles.get_mut(parent.parent()).map(|mut style| {
            style.computed.node.display = Display::None;
        });
    }
}

fn update_selection(
    mut cmd: Commands,
    mut events: EventReader<SelectionChangedEvent>,
    mut texts: Query<&mut Text>,
    children: Query<&Children>,
    tags: Query<&Tags>,
) {
    for event in events.read() {
        let Some(mut text) = children
            .get(event.select)
            .ok()
            .and_then(|children| children.iter().find(|child| texts.get(*child).is_ok()))
            .and_then(|c| texts.get_mut(c).ok())
        else {
            continue;
        };

        _ = tags
            .get(event.option)
            .map(|tags| tags.get("value").map(|s| s.as_str()).unwrap_or_default())
            .map(|t| text.0 = t.into());

        cmd.trigger_targets(UiChangedEvent, event.select);
    }
}
