use macroquad::prelude::*;

mod animation;
mod easefuncs;
mod prelude;
mod vec;
mod world;

use std::time::Duration;

use crate::prelude::*;

#[derive(Debug)]
pub struct Config {
    /// Has the game board been initialized
    initialized: bool,
}

#[macroquad::main("Juicing Demo")]
async fn main() {
    const BLOCKS_WIDTH: usize = 10;
    const BLOCKS_HEIGHT: usize = 10;
    const SCREEN_WIDTH: f32 = 20.0;
    const SCREEN_HEIGHT: f32 = 20.0;

    // Aliveness of each block
    let mut blocks = [[true; BLOCKS_WIDTH]; BLOCKS_HEIGHT];

    let mut ball_x = 12.;
    let mut ball_y = 7.;
    let mut dx = 3.5;
    let mut dy = -3.5;
    let platform_x = 10.;
    let mut stick = true;
    let platform_width = 5.;
    let platform_height = 0.2;

    // build camera with following coordinate system:
    // (0., 0)     .... (SCR_W, 0.)
    // (0., SCR_H) .... (SCR_W, SCR_H)
    set_camera(&Camera2D {
        zoom: vec2(1. / SCREEN_WIDTH * 2., 1. / SCREEN_HEIGHT * 2.),
        target: vec2(SCREEN_WIDTH / 2., SCREEN_HEIGHT / 2.),
        ..Default::default()
    });

    let mut config = Config { initialized: false };

    let mut world = World::default();

    let platform = world.push(Object {
        position: Vec2 {
            x: platform_x,
            y: -1.0,
        },
        color: Color::from_rgba(0, 0, 255, 128),
        shape: Shape::Rectangle {
            width: platform_width,
            height: platform_height,
        },
    });

    let platform_target = Vec2 {
        x: platform_x,
        y: SCREEN_HEIGHT - platform_height,
    };

    let animation_time = Duration::from_millis(750);
    let mut animation_elapsed = 0.0;

    fn ease(x: f32) -> f32 {
        1. - ((1. - x) * (1. - x) * (1. - x))
    }

    world.animate(
        &platform,
        Animation {
            target: AnimationState {
                absolute_position: Some(Vec2 {
                    x: platform_x,
                    y: SCREEN_HEIGHT - platform_height,
                }),
                ..Default::default()
            },
            ease: EaseFunc::SmoothStop3,
            duration: Duration::from_millis(1200).as_secs_f32(),
        },
    );

    loop {
        clear_background(SKYBLUE);

        if !config.initialized {
            world.update();

            // The world is finished initializing
            config.initialized = !world.is_animating(&platform);

            world.draw();

            next_frame().await;
            continue;
        }

        let delta = get_frame_time();

        if is_key_down(KeyCode::Right) && platform_x < SCREEN_WIDTH - platform_width / 2. {
            world.position_mut(&platform).x += 3.0 * delta;
        }
        if is_key_down(KeyCode::Left) && platform_x > platform_width / 2. {
            world.position_mut(&platform).x -= 3.0 * delta;
        }

        let platform_x = world.position(&platform).x;
        dbg!(platform_x);

        if stick == false {
            ball_x += dx * delta;
            ball_y += dy * delta;
        } else {
            let (font_size, font_scale, font_aspect) = camera_font_scale(1.);
            let text_params = TextParams {
                font_size,
                font_scale,
                font_scale_aspect: font_aspect,
                ..Default::default()
            };
            draw_text_ex(
                "Press space to start",
                SCREEN_WIDTH / 2. - 5.,
                SCREEN_HEIGHT / 2.,
                text_params,
            );

            ball_x = platform_x;
            ball_y = SCREEN_HEIGHT - 0.5;

            stick = !is_key_down(KeyCode::Space);
        }

        if ball_x <= 0. || ball_x > SCREEN_WIDTH {
            dx *= -1.;
        }

        if ball_y <= 0.
            || (ball_y > SCREEN_HEIGHT - platform_height - 0.15 / 2.
                && ball_x >= platform_x - platform_width / 2.
                && ball_x <= platform_x + platform_width / 2.)
        {
            dy *= -1.;
        }

        if ball_y >= SCREEN_HEIGHT {
            ball_y = 10.;
            dy = -dy.abs();
            stick = true;
        }

        for j in 0..BLOCKS_HEIGHT {
            for i in 0..BLOCKS_WIDTH {
                if blocks[j][i] {
                    let block_w = SCREEN_WIDTH / BLOCKS_WIDTH as f32;
                    let block_h = 7.0 / BLOCKS_HEIGHT as f32;
                    let block_x = i as f32 * block_w + 0.05;
                    let block_y = j as f32 * block_h + 0.05;

                    draw_rectangle(block_x, block_y, block_w - 0.1, block_h - 0.1, DARKBLUE);
                    if ball_x >= block_x
                        && ball_x < block_x + block_w
                        && ball_y >= block_y
                        && ball_y < block_y + block_h
                    {
                        dy *= -1.;
                        blocks[j][i] = false;
                    }
                }
            }
        }

        draw_circle(ball_x, ball_y, 0.2, RED);

        world.draw();

        next_frame().await
    }
}
