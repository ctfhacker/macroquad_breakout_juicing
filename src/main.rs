use std::io::Write;
use std::mem::size_of;

use game_context::*;
use macroquad::prelude::*;

#[cfg(target_os = "linux")]
mod dl;

fn setup_camera() {
    // build camera with following coordinate system:
    // (0., 0)     .... (SCR_W, 0.)
    // (0., SCR_H) .... (SCR_W, SCR_H)
    const SCREEN_WIDTH: f32 = 20.0;
    const SCREEN_HEIGHT: f32 = 20.0;

    set_camera(&Camera2D {
        zoom: vec2(1. / SCREEN_WIDTH * 2., 1. / SCREEN_HEIGHT * 2.),
        target: vec2(SCREEN_WIDTH / 2., SCREEN_HEIGHT / 2.),
        ..Default::default()
    });
}

enum LoopState {
    Normal,
    Record,
    Play,
}

#[derive(Default)]
struct LoopData {
    game_state: Option<State>,
    memory: Vec<u8>,
    buttons: Vec<Vec<KeyCode>>,
    next_index: usize,
}

impl LoopData {
    pub fn next_input(&mut self) -> Vec<KeyCode> {
        let index = self.next_index;
        self.next_index = (self.next_index + 1) % self.buttons.len();
        self.buttons[index].clone()
    }

    pub fn write_to_disk(&self, filename: &str) {
        if self.game_state.is_none() {
            eprintln!("ERROR: No state found in LoopData.. ");
            return;
        }

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(filename)
            .expect("Failed to open loop state file");

        let bytes = unsafe {
            std::slice::from_raw_parts(
                (self.game_state.as_ref().unwrap() as *const State) as *const u8,
                size_of::<State>(),
            )
        };
        file.write(&bytes).unwrap();

        // Write the memory to the save file
        // [len u64][bytes [u8; len]]
        assert!(self.memory.len() == MEMORY_LENGTH);
        let memory_len = self.memory.len() as u64;
        file.write(&memory_len.to_le_bytes()).unwrap();
        file.write(self.memory.as_slice()).unwrap();
    }
}

#[macroquad::main("Juicing example")]
async fn main() {
    let mut loop_state = LoopState::Normal;
    let mut loop_data = LoopData::default();

    let mut game_code = dl::get_game_funcs();
    let mut game_update_and_render;

    // Get the macroquad functions
    let macroquad = game_context::get_macroquad();

    let mut memory = Memory::new();
    let mut game = None;
    let frame_time = get_frame_time();

    let mut buttons = Vec::new();

    // One-time setup of the camera
    setup_camera();

    // Main game loop
    loop {
        // Check if the game library has been updated
        // If so, reload the main game function
        game_code = game_code.reload();
        game_update_and_render = &game_code.game_update_and_render;

        if is_key_released(KeyCode::L) {
            match loop_state {
                LoopState::Normal => {
                    println!("Loop recording..");

                    loop_data = LoopData {
                        game_state: game.clone(),
                        memory: memory.data_as_vec(),
                        buttons: Vec::new(),
                        next_index: 0,
                    };

                    loop_state = LoopState::Record;
                }
                LoopState::Record => {
                    println!("Loop playing..");
                    loop_state = LoopState::Play;
                }
                LoopState::Play => {
                    println!("Normal game play..");
                    loop_state = LoopState::Normal;
                }
            }
        }

        buttons = get_keys_down().iter().cloned().collect();

        match loop_state {
            LoopState::Play => {
                if loop_data.next_index % 30 == 0 {
                    println!(
                        "Loop play {}/{}",
                        loop_data.next_index,
                        loop_data.buttons.len()
                    );
                }

                if loop_data.next_index == 0 {
                    println!("Loop reset.. ");

                    // Restore the recording memory
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            loop_data.memory.as_ptr(),
                            MEMORY_BASE_ADDR as *mut u8,
                            MEMORY_LENGTH,
                        );
                    }

                    // Reset the game state
                    game = loop_data.game_state.clone();
                }

                // Use the recorded buttons
                buttons = loop_data.next_input();
            }
            LoopState::Record => {
                // Add the currently pressed buttons to the recording
                loop_data.buttons.push(buttons.clone());
            }
            LoopState::Normal => {
                // Nothing to do during normal..
            }
        }

        // Create the context for this frame
        let context = GameContext {
            error: Ok(()),
            memory: &mut memory,
            buttons: &buttons,
            frame_time,
        };

        // Call the game function
        game_update_and_render(&context, &mut game, &macroquad);

        // Goto next frame
        next_frame().await
    }
}
