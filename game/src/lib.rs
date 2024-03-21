use game_context::*;
use std::time::Duration;

// Reset the game state
fn reset_state(state: &mut Option<State>, macroquad: &Macroquad) {
    let mut world = World::default();

    let platform_width = 9.0;
    let platform_height = 1.0;
    let platform_x = 4.0;

    // Initialize the  above the map
    let platform = world.push(Object {
        position: Vec2 {
            x: platform_x,
            y: -1.0,
        },
        color: YELLOW,
        shape: Shape::Rectangle {
            width: platform_width,
            height: platform_height,
        },
    });

    // Begin by animating a single platform
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
            ease: EaseFunc::SmoothStop2,
            duration: Duration::from_millis(800).as_secs_f32(),
        },
    );

    // Check for ball collision with blocks
    for j in 0..BLOCKS_HEIGHT {
        for i in 0..BLOCKS_WIDTH {
            let block_padding = 0.2;

            let block_w = SCREEN_WIDTH / BLOCKS_WIDTH as f32;
            let block_h = 7.0 / BLOCKS_HEIGHT as f32;

            let block_x = i as f32 * block_w + block_padding / 2.0;
            let block_y = j as f32 * block_h + block_padding / 2.0;

            // Initialize each block just above the screen to fade in
            let block_obj = world.push(Object {
                position: Vec2 {
                    x: block_x,
                    y: block_y - SCREEN_HEIGHT,
                },
                color: RED,
                shape: Shape::Rectangle {
                    width: block_w - block_padding,
                    height: block_h - block_padding,
                },
            });

            /*
            let ease = match rng.next() % 10 {
                0 => EaseFunc::SmoothStop2,
                1 => EaseFunc::SmoothStop2,
                2 => EaseFunc::SmoothStop3,
                3 => EaseFunc::SmoothStop4,
                4 => EaseFunc::SmoothStart2,
                5 => EaseFunc::SmoothStart2,
                6 => EaseFunc::SmoothStart3,
                7 => EaseFunc::SmoothStart4,
                8 => EaseFunc::SmoothStep2,
                9 => EaseFunc::ElasticStop {
                    elastic: rng.gen_range(1.0..5.0),
                },
                _ => unreachable!(),
            };
            */

            let millis = 800;

            let ease = EaseFunc::ElasticStop {
                elastic: (macroquad.gen_range)(2.0, 4.0),
            };

            // Initially animate the block to fall from the top of the screen
            world.animate(
                &block_obj,
                Animation {
                    target: AnimationState {
                        absolute_position: Some(Vec2 {
                            x: block_x,
                            y: block_y,
                        }),
                        ..Default::default()
                    },
                    ease,
                    duration: Duration::from_millis(millis).as_secs_f32(),
                },
            );
        }
    }

    // Set the initial state
    *state = Some(State {
        reset_initialized: false,
        blocks: [[true; BLOCKS_WIDTH]; BLOCKS_HEIGHT],
        ball: Vec2::new(12., 7.),
        ball_velocity: Vec2::new(6.0, -6.5),
        platform,
        stick: false,
        platform_width: 5.,
        platform_height: 0.2,
        world,
    });
}

#[no_mangle]
pub extern "C" fn game_update_and_render(
    game: &GameContext,
    state: &mut Option<State>,
    macroquad: &Macroquad,
) {
    let Macroquad {
        clear_background,
        draw_circle,
        is_key_down,
        camera_font_scale,
        draw_text_ex,
        ..
    } = macroquad;

    // Initialize an empty game state
    if state.is_none() {
        reset_state(state, &macroquad);
    }

    // De-structure the game state itself
    let Some(State {
        reset_initialized,
        blocks,
        ball,
        ball_velocity,
        platform,
        platform_width,
        platform_height,
        stick,
        world,
    }) = state
    else {
        unreachable!()
    };

    // Q - reset the world
    if game.buttons.contains(&KeyCode::Q) {
        *state = None;
        return;
    }

    //
    if !*reset_initialized {
        clear_background(BLACK);

        world.update(game.frame_time);

        // The world is finished initializing
        *reset_initialized = !world.animating.iter().any(|x| *x);

        world.draw(macroquad);

        return;
    }

    clear_background(SKYBLUE);

    let delta = game.frame_time;

    let platform_pos = world.position(&platform);

    // Right - Move paddle right
    if game.buttons.contains(&KeyCode::Right)
        && platform_pos.x < SCREEN_WIDTH - *platform_width / 2.
    {
        world.position_mut(&platform).x += 6.0 * delta;
    }

    // Left - Move paddle right
    if game.buttons.contains(&KeyCode::Left) && platform_pos.x > *platform_width / 2. {
        let new_x = (platform_pos.x - 6.0 * delta).max(0.0);
        world.position_mut(&platform).x = new_x;
    }

    let platform_x = world.position(&platform).x;

    // Update the ball or wait for user input to start
    if *stick == false {
        ball.x += ball_velocity.x * delta;
        ball.y += ball_velocity.y * delta;
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

        ball.x = platform_x;
        ball.y = SCREEN_HEIGHT - 0.5;

        *stick = !game.buttons.contains(&KeyCode::Space);
    }

    // Bounce the ball off the left/right walls
    if ball.x <= 0. || ball.x > SCREEN_WIDTH {
        // dbg!(&ball_velocity);

        // Increase/decrease speed of ball on collision
        ball_velocity.x *= 1.0;

        if ball_velocity.x >= 20.0 {
            ball_velocity.x = 5.0;
        }

        if ball_velocity.x <= -20.0 {
            ball_velocity.x = -5.0;
        }

        ball_velocity.x *= -1.;
    }

    // Platform collision
    if ball.y <= 0.
        || (ball.y > SCREEN_HEIGHT - *platform_height - 0.15 / 2.
            && ball.x >= platform_x - *platform_width / 2.
            && ball.x <= platform_x + *platform_width / 2.)
    {
        ball_velocity.y *= -1.;
    }

    // Bottom screen kill condition
    if ball.y >= SCREEN_HEIGHT {
        // ball.y = 10.;
        // *stick = true;
        ball_velocity.y = -ball_velocity.y.abs();
    }

    /*
    // Check for ball collision with blocks
    for j in 0..BLOCKS_HEIGHT {
        for i in 0..BLOCKS_WIDTH {
            if blocks[j][i] {
                let block_w = SCREEN_WIDTH / BLOCKS_WIDTH as f32;
                let block_h = 7.0 / BLOCKS_HEIGHT as f32;

                let block_padding = 0.1;

                let block_x = i as f32 * block_w + block_padding / 2.0;
                let block_y = j as f32 * block_h + block_padding / 2.0;

                draw_rectangle(
                    block_x,
                    block_y,
                    block_w - block_padding,
                    block_h - block_padding,
                    DARKBLUE,
                );
                if ball.x >= block_x
                    && ball.x < block_x + block_w
                    && ball.y >= block_y
                    && ball.y < block_y + block_h
                {
                    ball_velocity.y *= -1.;
                    blocks[j][i] = false;
                }
            }
        }
    }
    */

    // Draw the ball
    draw_circle(ball.x, ball.y, 0.2, RED);

    // Draw the world
    world.draw(macroquad);
}
