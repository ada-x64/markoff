//! Render to the screen.
//! Rendering = running the compute shader!
//! Largely inspired by the Game Of Life example.
//! https://github.com/bevyengine/bevy/blob/main/examples/shader/compute_shader_game_of_life.rs

use bevy::{
    prelude::*,
    render::{
        render_graph::{self, RenderLabel},
        render_resource::{
            CachedPipelineState, ComputePassDescriptor, PipelineCache, PipelineCacheError,
        },
        renderer::RenderContext,
    },
};

use crate::sim::native::{SHADER_ASSET_PATH, SIM_SIZE, SimBindGroups, SimPipeline, WORKGROUP_SIZE};

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
                match pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline) {
                    CachedPipelineState::Ok(_) => {
                        self.state = SimNodeState::Init;
                    }
                    CachedPipelineState::Err(PipelineCacheError::ShaderNotLoaded(_)) => {}
                    CachedPipelineState::Err(err) => {
                        panic!("Initializing assets/{SHADER_ASSET_PATH}:\n{err}");
                    }
                    _ => {}
                }
            }
            // once initialized, start rendering
            SimNodeState::Init => {
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
                pass.dispatch_workgroups(SIM_SIZE / WORKGROUP_SIZE, SIM_SIZE / WORKGROUP_SIZE, 1);
            }
            // switch buffer
            SimNodeState::Update(idx) => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.update_pipeline)
                    .unwrap();
                pass.set_bind_group(0, &bind_groups[idx], &[]);
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(SIM_SIZE / WORKGROUP_SIZE, SIM_SIZE / WORKGROUP_SIZE, 1);
            }
        }
        Ok(())
    }
}
