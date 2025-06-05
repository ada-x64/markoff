//! The Native simulation uses compute shaders.

use bevy::{
    prelude::*,
    render::{
        Render, RenderApp, RenderSet, extract_resource::ExtractResourcePlugin,
        graph::CameraDriverLabel, render_graph::RenderGraph,
    },
};

pub mod render;
pub use render::*;

pub mod shader;
pub use shader::*;

use crate::sim::{SimImages, SimRenderState, SimSprite, SimState, UseCompute};

pub const DISPLAY_FACTOR: u32 = 1;
pub const IMG_SIZE: u32 = 512;
pub const SIM_SIZE: u32 = IMG_SIZE / DISPLAY_FACTOR;
pub const WORKGROUP_SIZE: u32 = 8; // workgroup = num threads
pub const SHADER_ASSET_PATH: &str = "shader/simulation.wgsl";

#[derive(SystemSet, Hash, Copy, Clone, PartialEq, Eq, Debug)]
pub struct ShaderSimSet;

pub struct ShaderSimPlugin;
impl Plugin for ShaderSimPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractResourcePlugin::<SimImages>::default(),
            ExtractResourcePlugin::<SimRenderState>::default(),
            ExtractResourcePlugin::<UseCompute>::default(),
        ))
        .add_systems(FixedUpdate, swap_buffer.in_set(ShaderSimSet));
        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_systems(
                Render,
                prepare_bind_group
                    .in_set(RenderSet::PrepareBindGroups)
                    .in_set(ShaderSimSet),
            )
            .add_systems(PreUpdate, route_state)
            .add_systems(OnEnter(SimState::Init), init.in_set(ShaderSimSet))
            .add_systems(OnEnter(SimState::Closed), cleanup.in_set(ShaderSimSet))
            .configure_sets(
                Render,
                ShaderSimSet.run_if(resource_exists_and_equals(UseCompute(true))),
            )
            .configure_sets(
                OnEnter(SimState::Init),
                ShaderSimSet.run_if(resource_exists_and_equals(UseCompute(true))),
            )
            .configure_sets(
                OnEnter(SimState::Closed),
                ShaderSimSet.run_if(resource_exists_and_equals(UseCompute(true))),
            );
    }
    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<SimPipeline>();
    }
}

fn route_state(state: Changed<) {

}

fn init(mut render_graph: ResMut<RenderGraph>) {
    render_graph.add_node(SimLabel, SimulationNode::default());
    render_graph.add_node_edge(SimLabel, CameraDriverLabel);
}
fn cleanup(mut render_graph: ResMut<RenderGraph>) {
    render_graph.remove_node(SimLabel);
    render_graph.remove_node_edge(SimLabel);
}

fn swap_buffer(mut sprite: Single<&mut ImageNode, With<SimSprite>>, imgs: Res<SimImages>) {
    if sprite.image == imgs.texture_a {
        sprite.image = imgs.texture_b.clone();
    } else {
        sprite.image = imgs.texture_a.clone();
    }
}
