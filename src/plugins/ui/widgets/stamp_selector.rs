use bevy::{prelude::*, ui::RelativeCursorPosition};
use bevy_hui::prelude::*;

use crate::{
    sim::{SimGameplayState, SimSettings},
    stamps::Stamps,
    ui::Scrollable,
};

pub struct StampSelectorWidgetPlugin;
impl Plugin for StampSelectorWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
            .add_systems(Update, update_wrapper);
        // .add_observer(change_stamp_img);
    }
}
fn init(mut funcs: HtmlFunctions) {
    funcs.register("init_stamp_selector", init_stamp_selector);
}

#[derive(Default, Debug, Component)]
struct StampWidget {
    name: String,
    selected: bool,
}

fn init_stamp_selector(
    In(entity): In<Entity>,
    mut commands: Commands,
    stamps: Res<Stamps>,
    mut nodes: Query<&mut Node>,
) {
    // #333
    let border_color = Color::linear_rgb(3. / 16., 3. / 16., 3. / 16.);
    let wrapper = commands
        .spawn((
            Name::new("stamps"),
            Pickable::IGNORE,
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(2.)),
                padding: UiRect::all(Val::Px(5.)),
                overflow: Overflow::scroll_y(),
                width: Val::Percent(100.),
                ..Default::default()
            },
            Scrollable,
            BorderColor(border_color),
            BorderRadius::all(Val::Px(5.)),
        ))
        .id();
    let stamps = &stamps.px32;
    stamps.iter().for_each(|(name, stamp)| {
        let image_node = (
            Node {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            Pickable::IGNORE,
            children![(
                ImageNode {
                    image: stamp.texture.clone(),
                    texture_atlas: Some(stamp.atlas.clone()),
                    ..Default::default()
                },
                Pickable::IGNORE,
            )],
        );
        let text_node = |text| {
            (
                Pickable::IGNORE,
                Text::new(text),
                TextFont::from_font_size(16.),
                Label,
            )
        };
        let child_id = commands
            .spawn((
                Node {
                    display: Display::Flex,
                    border: UiRect::all(Val::Px(2.)),
                    padding: UiRect::all(Val::Px(5.)),
                    margin: UiRect::bottom(Val::Px(5.)),
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::Stretch,
                    column_gap: Val::Px(5.),
                    ..Default::default()
                },
                BackgroundColor(Color::linear_rgb(0.3, 0.3, 0.3)),
                BorderColor(border_color),
                BorderRadius::all(Val::Px(5.)),
                StampWidget {
                    name: name.to_owned(),
                    selected: false,
                },
                Pickable {
                    should_block_lower: true,
                    is_hoverable: true,
                },
                RelativeCursorPosition::default(),
                children![image_node, text_node(name.clone(),)],
            ))
            .observe(wrapper_click)
            .id();
        commands.entity(wrapper).add_child(child_id);
    });
    let mut node = nodes.get_mut(entity).expect("node");
    node.overflow = Overflow::scroll_y();
    let mut parent = commands.entity(entity);
    parent.add_child(wrapper);
}

fn update_wrapper(mut query: Query<(&StampWidget, &mut BackgroundColor, &RelativeCursorPosition)>) {
    query
        .iter_mut()
        .for_each(|(widget, mut bg_color, cursor_pos)| {
            if widget.selected {
                *bg_color = BackgroundColor(Color::linear_rgb(0.3, 0.3, 0.3));
            } else if cursor_pos.mouse_over() {
                *bg_color = BackgroundColor(Color::linear_rgb(0.2, 0.2, 0.2));
            } else {
                *bg_color = BackgroundColor(Color::linear_rgb(0.1, 0.1, 0.1));
            }
        })
}
fn wrapper_click(
    event: Trigger<Pointer<Click>>,
    mut wrappers: Query<&mut StampWidget>,
    mut sim_state: ResMut<SimGameplayState>,
) {
    wrappers
        .iter_mut()
        .for_each(|mut wrapper| wrapper.selected = false);
    let Ok(mut widget) = wrappers.get_mut(event.target) else {
        return;
    };
    sim_state.current_stamp = Some(widget.name.clone());
    widget.selected = true;
    info!("clicked ok");
}
