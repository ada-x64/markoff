use bevy::{prelude::*, render::extract_resource::ExtractResource};
use derivative::Derivative;

pub type TeamID = u8;
pub type PlayerID = u8;

#[derive(Clone, Debug, PartialEq)]
pub struct Team {
    pub id: TeamID,
    pub name: String,
    pub players: Vec<PlayerID>,
    pub color: [u8; 4],
}
#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    pub id: PlayerID,
    pub name: String,
}

/// Used to check cell state
#[derive(Debug, PartialEq, Eq, Copy, Clone, Default)]
pub enum CellCondition {
    #[default]
    Empty,
    Active,
    Owned,
    Enemy,
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CellResult {
    Empty,
    Active,
    Untouched,
}

/// Double buffer.
#[derive(Debug, Resource, Clone, ExtractResource, Default)]
pub struct SimImages {
    pub texture_a: Handle<Image>,
    pub texture_b: Handle<Image>,
    pub preview_texture: Handle<Image>,
}

#[derive(Component)]
pub struct SimSprite;

/// srgba_u8
pub type PixelColor<'a> = &'a [u8; 4];
pub const BLACK: PixelColor = &[0, 0, 0, 255];
pub const WHITE: PixelColor = &[255, 255, 255, 255];

#[derive(Default, Debug, strum::Display, Copy, Clone, PartialEq)]
pub enum SimLayout {
    #[default]
    Random,
    #[strum(to_string = "50/50 Horizontal")]
    Horiz5050,
    #[strum(to_string = "50/50 Vertical")]
    Vert5050,
    #[strum(to_string = "50/50 Random")]
    Rand5050,
    Empty,
}
impl TryFrom<&String> for SimLayout {
    type Error = anyhow::Error;
    fn try_from(value: &String) -> anyhow::Result<Self> {
        match value.to_string().as_str() {
            "Random" => Ok(Self::Random),
            "50/50 Horizontal" => Ok(Self::Horiz5050),
            "50/50 Vertical" => Ok(Self::Vert5050),
            "50/50 Random" => Ok(Self::Rand5050),
            "Empty" => Ok(Self::Empty),
            _ => Err(anyhow::anyhow!("No such layout")),
        }
    }
}

#[derive(Event)]
pub struct StampEvent;

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum SimState {
    #[default]
    Closed,
    Init,
    Paused,
    Running,
}

#[derive(Resource, Debug, Default)]
pub struct SimGameplayState {
    pub current_stamp: Option<String>,
    pub num_steps: u32,
}

// Intialized through the UI.
#[derive(Resource, Clone, Debug, PartialEq, Derivative, ExtractResource)]
#[derivative(Default)]
pub struct SimSettings {
    pub teams: Vec<Team>,
    pub players: Vec<Player>,
    pub parent_node: Option<Entity>,
    #[derivative(Default(value = "32"))]
    pub size: u32, // Must be a power of 2.
    #[derivative(Default(value = "10"))]
    pub timestep: u32, // fps
    #[derivative(Default(value = "100"))]
    pub steps_per_turn: u32,
    pub layout: SimLayout,
    pub use_compute: bool,
}

pub const DISPLAY_FACTOR: u32 = 1;
pub const IMG_SIZE: u32 = 512;
// TODO: Replace with SimSize setting
pub const SIM_SIZE: u32 = IMG_SIZE / DISPLAY_FACTOR;
pub const WORKGROUP_SIZE: u32 = 8; // workgroup = num threads
pub const SHADER_ASSET_PATH: &str = "shader/simulation.wgsl";

#[derive(SystemSet, Hash, Copy, Clone, PartialEq, Eq, Debug)]
pub struct GpuSimSystems;

#[derive(SystemSet, Hash, Copy, Clone, PartialEq, Eq, Debug)]
pub struct CpuSimSystems;
