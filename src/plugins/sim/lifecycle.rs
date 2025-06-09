use crate::{sim::data::*, ui::widgets::sim_image::SimImageNode};
use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
};

pub struct SimLifecyclePlugin;
impl Plugin for SimLifecyclePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(10.))
            .init_state::<SimState>()
            .add_systems(
                OnEnter(SimState::Init),
                (init_images, spawn_sprite, populate, init_timestep).chain(),
            )
            .add_systems(OnEnter(SimState::Running), unpause)
            .add_systems(OnEnter(SimState::Paused), pause)
            .add_systems(OnEnter(SimState::Closed), cleanup)
            .add_systems(FixedUpdate, update)
            .add_observer(on_stamp);
    }
}

fn update(mut gameplay: ResMut<SimGameplayState>) {
    gameplay.num_steps += 1;
}

fn init_images(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    settings: Res<SimSettings>,
) {
    let (asset_usage, format) = if settings.use_compute {
        // !NB! compute shader should reflect this
        (RenderAssetUsages::RENDER_WORLD, TextureFormat::Rgba8Unorm)
    } else {
        (
            RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
            TextureFormat::Rgba8UnormSrgb,
        )
    };
    let mut image = Image::new_fill(
        Extent3d {
            width: settings.size,
            height: settings.size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        format,
        asset_usage,
    );
    image.texture_descriptor.usage = TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING;
    if settings.use_compute {
        image.texture_descriptor.usage |= TextureUsages::STORAGE_BINDING;
    }

    commands.insert_resource(SimImages {
        preview_texture: images.add(image.clone()),
        texture_a: images.add(image.clone()),
        texture_b: images.add(image),
    });
}

fn populate(
    sprite: Single<&ImageNode, With<SimSprite>>,
    mut images: ResMut<Assets<Image>>,
    mut next: ResMut<NextState<SimState>>,
    settings: Res<SimSettings>,
) {
    let img = images.get_mut(sprite.image.id()).unwrap();
    let size = img.size();
    for x in 0..size.x {
        for y in 0..size.y {
            let color = match settings.layout {
                SimLayout::Random => {
                    let len = settings.teams.len() + 2;
                    let res = rand::random_range(0..len);
                    match res {
                        0 => WHITE,
                        1 => BLACK,
                        _ => &settings.teams[res - 2].color,
                    }
                }
                // assumes 2 teams...
                // 4 teams would have quadrants, etc
                SimLayout::Horiz5050 => {
                    if y < size.y / 2 {
                        &settings.teams[0].color
                    } else {
                        &settings.teams[1].color
                    }
                }
                SimLayout::Vert5050 => {
                    if x < size.x / 2 {
                        &settings.teams[0].color
                    } else {
                        &settings.teams[1].color
                    }
                }
                SimLayout::Rand5050 => {
                    if rand::random_bool(0.5) {
                        &settings.teams[0].color
                    } else {
                        &settings.teams[1].color
                    }
                }
                SimLayout::Empty => BLACK,
            };
            img.set_color_at(x, y, Color::srgb_u8(color[0], color[1], color[2]))
                .unwrap();
        }
    }
    next.set(SimState::Paused);
}

fn init_timestep(mut time: ResMut<Time<Fixed>>, settings: Res<SimSettings>) {
    time.set_timestep_hz(settings.timestep as f64);
}

fn unpause(
    mut time: ResMut<Time<Virtual>>,
    mut image_node: Single<&mut ImageNode, With<SimImageNode>>,
    sim_images: Res<SimImages>,
    mut images: ResMut<Assets<Image>>,
) {
    time.unpause();
    image_node.image = sim_images.texture_a.clone();
    let preview_image = images
        .get_mut(&sim_images.preview_texture)
        .expect("preview texture")
        .clone();
    images
        .get_mut(&sim_images.texture_a)
        .expect("tex_a")
        .clone_from(&preview_image);
    images
        .get_mut(&sim_images.texture_b)
        .expect("tex_b")
        .clone_from(&preview_image);
}

fn pause(
    mut time: ResMut<Time<Virtual>>,
    sim_imgs: ResMut<SimImages>,
    settings: Res<SimSettings>,
    mut image_nodes: Query<&mut ImageNode>,
    mut images: ResMut<Assets<Image>>,
) {
    time.pause();
    let mut parent_node = settings
        .parent_node
        .and_then(|p| image_nodes.get_mut(p).ok())
        .expect("parent handle");
    let current_handle = parent_node.image.clone();
    let current_img = images.get(&current_handle).expect("current_img").clone();
    images
        .get_mut(&sim_imgs.texture_a)
        .expect("tex_a")
        .clone_from(&current_img);
    images
        .get_mut(&sim_imgs.texture_b)
        .expect("tex_b")
        .clone_from(&current_img);
    images
        .get_mut(&sim_imgs.preview_texture)
        .expect("tex_p")
        .clone_from(&current_img);

    parent_node.image = sim_imgs.preview_texture.clone();
}

pub fn spawn_sprite(
    mut commands: Commands,
    images: Res<SimImages>,
    settings: Res<SimSettings>,
    mut image_nodes: Query<&mut ImageNode>,
) {
    if let Some(parent_node) = settings.parent_node {
        let mut parent = image_nodes
            .get_mut(parent_node)
            .expect("Could not get parent node!");
        parent.image = images.texture_a.clone();
        commands
            .get_entity(parent_node)
            .expect("get_entity")
            .insert(SimSprite);
    }
}
pub fn cleanup(mut commands: Commands, query: Single<Entity, With<SimSprite>>) {
    commands.get_entity(query.entity()).unwrap().despawn();
}

fn on_stamp(_trigger: Trigger<StampEvent>, mut state: ResMut<NextState<SimState>>) {
    state.set(SimState::Running);
}
