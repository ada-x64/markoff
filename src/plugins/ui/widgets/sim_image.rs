use anyhow::anyhow;
use bevy::{prelude::*, ui::RelativeCursorPosition};
use bevy_hui::prelude::{HtmlComponents, HtmlFunctions};

use crate::{
    sim::{SimGameplayState, SimImages, SimSettings, SimState, StampEvent},
    stamps::{Stamp, Stamps},
};

#[derive(Component, Debug, Copy, Clone)]
pub struct SimImageNode;

pub struct SimImageWidgetPlugin;
impl Plugin for SimImageWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
            .add_systems(Update, hover_preview);
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
        .observe(|_: Trigger<Pointer<Click>>, mut commands: Commands| {
            commands.trigger(StampEvent);
        })
        .insert((RelativeCursorPosition::default(), SimImageNode));
}

fn hover_preview(
    pos: Single<&RelativeCursorPosition, With<SimImageNode>>,
    sim_state: Res<State<SimState>>,
    gameplay_state: Res<SimGameplayState>,
    settings: Res<SimSettings>,
    sim_images: Res<SimImages>,
    mut images: ResMut<Assets<Image>>,
    stamps: Res<Stamps>,
    stamp_assets: Res<Assets<Stamp>>,
    atlases: Res<Assets<TextureAtlasLayout>>,
) {
    if !matches!(**sim_state, SimState::Paused) {
        return;
    }
    let Some(current_stamp) = gameplay_state.current_stamp.as_ref() else {
        return;
    };
    let Some(pos) = pos.normalized else {
        return;
    };
    if let Err(e) = (|| {
        let stamps = stamps.get_from_sim_size(settings.size);
        let stamp = stamps.get(current_stamp).ok_or(anyhow!("stamp"))?;
        let stamp = stamp_assets.get(stamp).ok_or(anyhow!("stamp asset"))?;

        let original = images
            .get(&sim_images.texture_a)
            .ok_or(anyhow!("texture_a"))?;
        let mut new_preview = original.clone();

        let pos = pos * Vec2::splat(settings.size as f32);
        stamp.add_to_texture(&mut new_preview, pos, &images, &atlases)?;

        images
            .get_mut(&sim_images.preview_texture)
            .ok_or(anyhow!("preview_texture"))?
            .clone_from(&new_preview);
        anyhow::Ok(())
    })() {
        error!("Could not hover with error: {e}");
    }
}
