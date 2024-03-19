pub use macroquad::math::Vec2;
use macroquad::prelude::*;

pub use macroquad::color::*;
pub use macroquad::prelude::{KeyCode, TextParams};

mod memory;
pub use memory::{Allocation, Memory, MEMORY_BASE_ADDR, MEMORY_LENGTH};

mod world;
pub use world::*;

mod macroquad_funcs;
pub use macroquad_funcs::*;

mod easefuncs;
pub use easefuncs::*;

mod animation;
pub use animation::*;

/// The context of a game
pub struct GameContext<'a> {
    /// Potential error when executing the game logic
    pub error: Result<(), ()>,

    /// Reference to the memory backing the game
    pub memory: &'a mut Memory,

    ///
    pub frame_time: f32,
}

pub const BLOCKS_WIDTH: usize = 10;
pub const BLOCKS_HEIGHT: usize = 10;
pub const SCREEN_WIDTH: f32 = 20.0;
pub const SCREEN_HEIGHT: f32 = 20.0;

// The game state data. This data is initialized in the game code itself.
pub struct State {
    pub reset_initialized: bool,
    pub blocks: [[bool; BLOCKS_WIDTH]; BLOCKS_HEIGHT],
    pub ball: Vec2,
    pub ball_velocity: Vec2,
    pub platform: ObjectIndex,
    pub platform_width: f32,
    pub platform_height: f32,
    pub stick: bool,
    pub world: World,
}
