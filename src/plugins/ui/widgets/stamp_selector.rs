use bevy::{prelude::*, ui::RelativeCursorPosition};
use bevy_hui::prelude::*;

use crate::{sim::SimSettings, stamps::Stamps, ui::Scrollable};

pub struct StampSelectorWidgetPlugin;
impl Plugin for StampSelectorWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        // .add_observer(change_stamp_img);
    }
}
fn init(mut funcs: HtmlFunctions) {
    funcs.register("init_stamp_selector", init_stamp_selector);
}

#[derive(Default, Debug, Component)]
struct StampSelectorWidget;

fn init_stamp_selector(
    In(entity): In<Entity>,
    mut commands: Commands,
    stamps: Res<Stamps>,
    settings: Res<SimSettings>,
    mut nodes: Query<&mut Node>,
) {
    // #333
    let border_color = Color::linear_rgb(3. / 16., 3. / 16., 3. / 16.);
    let mut wrapper = commands.spawn((
        Name::new("stamps"),
        Pickable::default(),
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            border: UiRect::all(Val::Px(2.)),
            padding: UiRect::all(Val::Px(5.)),
            overflow: Overflow::scroll_y(),
            ..Default::default()
        },
        Scrollable,
        BorderColor(border_color),
        BorderRadius::all(Val::Px(5.)),
    ));
    let stamps = stamps.get_from_size(settings.size);
    stamps.iter().for_each(|(name, stamp)| {
        let image_node = (
            Node {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            children![ImageNode {
                image: stamp.texture.clone(),
                texture_atlas: Some(stamp.atlas.clone()),
                ..Default::default()
            }],
        );
        let text_node = |text| (Text::new(text), TextFont::from_font_size(16.));
        wrapper.with_child((
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
            BorderColor(border_color),
            BorderRadius::all(Val::Px(5.)),
            children![image_node, text_node(name.clone(),)],
        ));
    });
    let wrapper = wrapper.id();
    let mut node = nodes.get_mut(entity).expect("node");
    node.overflow = Overflow::scroll_y();
    let mut parent = commands.entity(entity);
    parent.add_child(wrapper);
}

// TODO: Should react to sim settings change.
fn change_stamp_img(
    In(entity): In<Entity>,
    tags: Query<&Tags>,
    stamps: Res<Stamps>,
    settings: Res<SimSettings>,
    mut images: Query<&mut ImageNode>,
) {
    let tags = tags.get(entity).expect("img tags");
    let name = tags.get("name").expect("name");
    let stamp = stamps
        .get_from_size(settings.size)
        .get(name)
        .expect("stamp");
    let mut img = images.get_mut(entity).expect("entity");
    img.image = stamp.texture.clone();
    img.texture_atlas = Some(stamp.atlas.clone());
}
