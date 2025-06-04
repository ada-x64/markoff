//! Render to the screen.
//! Rendering = running the compute shader!
//! Largely inspired by the Game Of Life example.
//! https://github.com/bevyengine/bevy/blob/main/examples/shader/compute_shader_game_of_life.rs

use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResource,
        render_graph::{self, RenderLabel},
        render_resource::{
            CachedPipelineState, ComputePassDescriptor, PipelineCache, PipelineCacheError,
        },
        renderer::RenderContext,
    },
};

use crate::{SHADER_ASSET_PATH, SimBindGroups, SimPipeline};

pub const DISPLAY_FACTOR: u32 = 1;
pub const SIMULATION_SIZE: (u32, u32) = (512 / DISPLAY_FACTOR, 512 / DISPLAY_FACTOR);
pub const WORKGROUP_SIZE: u32 = 8; // workgroup = num threads

/// Double buffer.
#[derive(Resource, Clone, ExtractResource)]
pub struct SimImages {
    pub texture_a: Handle<Image>,
    pub texture_b: Handle<Image>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct SimLabel;

#[derive(Default, Debug)]
pub enum SimNodeState {
    #[default]
    Loading,
    Init,
    Update(usize),
}
#[derive(Default, Debug)]
pub struct SimulationNode {
    pub state: SimNodeState,
}

impl render_graph::Node for SimulationNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<SimPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        match self.state {
            // wait for shader to load
            SimNodeState::Loading => {
                info_once!("Simulation node state: {:#?}", &self.state);
                match pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline) {
                    CachedPipelineState::Ok(_) => {
                        self.state = SimNodeState::Init;
                        info_once!("OK!");
                    }
                    CachedPipelineState::Err(PipelineCacheError::ShaderNotLoaded(_)) => {
                        info_once!("Shader not loaded.");
                    }
                    CachedPipelineState::Err(err) => {
                        panic!("Initializing assets/{SHADER_ASSET_PATH}:\n{err}");
                    }
                    _ => {}
                }
            }
            // once initialized, start rendering
            SimNodeState::Init => {
                info_once!("Simulation node state: {:#?}", &self.state);
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.update_pipeline)
                {
                    self.state = SimNodeState::Update(1);
                }
            }
            // switch buffer
            SimNodeState::Update(0) => self.state = SimNodeState::Update(1),
            SimNodeState::Update(1) => self.state = SimNodeState::Update(0),
            SimNodeState::Update(_) => unreachable!(),
        }
    }
    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let bind_groups = &world.resource::<SimBindGroups>().0;
        let pipeline_cache = &world.resource::<PipelineCache>();
        let pipeline = &world.resource::<SimPipeline>();
        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        match self.state {
            SimNodeState::Loading => {}
            SimNodeState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.init_pipeline)
                    .unwrap();
                pass.set_bind_group(0, &bind_groups[0], &[]);
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(
                    SIMULATION_SIZE.0 / WORKGROUP_SIZE,
                    SIMULATION_SIZE.1 / WORKGROUP_SIZE,
                    1,
                );
            }
            // switch buffer
            SimNodeState::Update(idx) => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.update_pipeline)
                    .unwrap();
                pass.set_bind_group(0, &bind_groups[idx], &[]);
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(
                    SIMULATION_SIZE.0 / WORKGROUP_SIZE,
                    SIMULATION_SIZE.1 / WORKGROUP_SIZE,
                    1,
                );
            }
        }
        Ok(())
    }
}
