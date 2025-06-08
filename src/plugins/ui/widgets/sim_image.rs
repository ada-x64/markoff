use bevy::{prelude::*, ui::RelativeCursorPosition};
use bevy_hui::prelude::{HtmlComponents, HtmlFunctions};

use crate::sim::{SimSettings, StampEvent};

pub struct SimImageWidgetPlugin;
impl Plugin for SimImageWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
    }
}

fn init(mut components: HtmlComponents, mut funcs: HtmlFunctions, server: Res<AssetServer>) {
    components.register("sim_image", server.load("hui/components/sim_image.xml"));
    funcs.register("init_sim_image", init_sim_image);
}

fn init_sim_image(
    In(entity): In<Entity>,
    mut settings: ResMut<SimSettings>,
    mut commands: Commands,
) {
    settings.parent_node = Some(entity);
    commands
        .entity(entity)
        .observe(
            |trigger: Trigger<Pointer<Click>>,
             mut commands: Commands,
             cursor_pos: Query<&RelativeCursorPosition>| {
                let position = cursor_pos
                    .get(trigger.target)
                    .expect("cursor_pos")
                    .normalized
                    .expect("outside????");
                commands.trigger(StampEvent { position });
            },
        )
        .insert(RelativeCursorPosition::default());
}
