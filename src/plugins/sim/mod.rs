use bevy::prelude::*;

pub use data::*;

use crate::sim::{
    lifecycle::SimLifecyclePlugin,
    render::{cpu::CpuSimPlugin, gpu::GpuSimPlugin},
};

mod data;
mod lifecycle;
mod render;

pub struct SimPlugin;
impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        let _ = {
            app.add_plugins((CpuSimPlugin, GpuSimPlugin, SimLifecyclePlugin))
                .insert_resource(SimSettings {
                    teams: vec![
                        Team {
                            id: 0,
                            name: "A".into(),
                            players: Vec::new(),
                            color: [255, 0, 0, 255],
                        },
                        Team {
                            id: 1,
                            name: "B".into(),
                            players: Vec::new(),
                            color: [0, 0, 255, 255],
                        },
                    ],
                    ..Default::default()
                })
                .init_resource::<SimImages>()
                .init_resource::<SimGameplayState>()
                // todo: crashing
                // this is super annoying!
                .configure_sets(
                    Update,
                    (
                        GpuSimSystems.run_if(run_gpu_systems),
                        CpuSimSystems.run_if(run_cpu_systems),
                    ),
                )
                .configure_sets(
                    FixedUpdate,
                    (
                        GpuSimSystems.run_if(run_gpu_systems),
                        CpuSimSystems.run_if(run_cpu_systems),
                    ),
                )
                .configure_sets(
                    OnEnter(SimState::Init),
                    (
                        GpuSimSystems.run_if(run_gpu_systems),
                        CpuSimSystems.run_if(run_cpu_systems),
                    ),
                )
                .configure_sets(
                    OnEnter(SimState::Running),
                    (
                        GpuSimSystems.run_if(run_gpu_systems),
                        CpuSimSystems.run_if(run_cpu_systems),
                    ),
                )
                .configure_sets(
                    OnEnter(SimState::Paused),
                    (
                        GpuSimSystems.run_if(run_gpu_systems),
                        CpuSimSystems.run_if(run_cpu_systems),
                    ),
                )
                .configure_sets(
                    OnEnter(SimState::Closed),
                    (
                        GpuSimSystems.run_if(run_gpu_systems),
                        CpuSimSystems.run_if(run_cpu_systems),
                    ),
                )
        };
    }
}

fn run_gpu_systems(settings: Res<SimSettings>) -> bool {
    settings.use_compute
}
fn run_cpu_systems(settings: Res<SimSettings>) -> bool {
    !settings.use_compute
}
