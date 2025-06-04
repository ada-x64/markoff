use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{
        Render, RenderApp, RenderSet,
        extract_resource::ExtractResourcePlugin,
        graph::CameraDriverLabel,
        render_graph::RenderGraph,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
    },
};

pub mod render;
pub use render::*;

pub mod shader;
pub use shader::*;

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum SimulationPluginState {
    #[default]
    Closed,
    Init,
    Loading,
    Paused,
    Running,
}

pub struct SimulationPlugin;
impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ExtractResourcePlugin::<SimulationImages>::default(),))
            .init_state::<SimulationPluginState>()
            .add_systems(Startup, setup)
            .add_systems(OnEnter(SimulationPluginState::Init), spawn_sprite)
            .add_systems(OnEnter(SimulationPluginState::Running), || {
                info!("Running simulation!")
            })
            .add_systems(OnEnter(SimulationPluginState::Closed), cleanup)
            .add_systems(
                Update,
                (
                    (switch_textures).run_if(in_state(SimulationPluginState::Running)),
                    (init).run_if(in_state(SimulationPluginState::Init)),
                ),
            );
        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(
            Render,
            prepare_bind_group.in_set(RenderSet::PrepareBindGroups),
        );

        let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();
        render_graph.add_node(SimulationLabel, SimulationNode::default());
        render_graph.add_node_edge(SimulationLabel, CameraDriverLabel)
    }
    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<SimulationPipeline>();
    }
}
fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut image = Image::new_fill(
        Extent3d {
            width: SIMULATION_SIZE.0,
            height: SIMULATION_SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        // !NB! compute shader should reflect this
        TextureFormat::R32Float,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let image0 = images.add(image.clone());
    let image1 = images.add(image.clone());
    commands.insert_resource(SimulationImages {
        texture_a: image0,
        texture_b: image1,
    });
}

#[derive(Component)]
pub struct SimulationSprite;

pub fn spawn_sprite(mut commands: Commands, images: Res<SimulationImages>) {
    commands.spawn((
        SimulationSprite,
        Sprite {
            image: images.texture_a.clone_weak(),
            custom_size: Some(Vec2::new(
                SIMULATION_SIZE.0 as f32,
                SIMULATION_SIZE.1 as f32,
            )),
            ..Default::default()
        },
        Transform::from_scale(Vec3::splat(DISPLAY_FACTOR as f32)),
    ));
}
pub fn cleanup(mut commands: Commands, query: Single<Entity, With<SimulationSprite>>) {
    commands.get_entity(query.entity()).unwrap().despawn();
}

pub fn switch_textures(
    images: Res<SimulationImages>,
    mut sprite: Single<&mut Sprite, With<SimulationSprite>>,
) {
    if sprite.image == images.texture_a {
        sprite.image = images.texture_b.clone_weak();
    } else {
        sprite.image = images.texture_a.clone_weak();
    }
}

pub fn init(mut next: ResMut<NextState<SimulationPluginState>>) {
    next.set(SimulationPluginState::Running);
}
