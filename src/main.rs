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

#[macroquad::main("Juicing example")]
async fn main() {
    let mut game_code = dl::get_game_funcs();
    let mut game_update_and_render;

    // Get the macroquad functions
    let macroquad = game_context::get_macroquad();

    let mut memory = Memory::new();
    let mut game = None;
    let frame_time = get_frame_time();

    // One-time setup of the camera
    setup_camera();

    // Main game loop
    loop {
        // Check if the game library has been updated
        // If so, reload the main game function
        game_code = game_code.reload();
        game_update_and_render = &game_code.game_update_and_render;

        // Create the context for this frame
        let context = GameContext {
            error: Ok(()),
            memory: &mut memory,
            frame_time,
        };

        // Call the game function
        game_update_and_render(&context, &mut game, &macroquad);

        // Goto next frame
        next_frame().await
    }
}
