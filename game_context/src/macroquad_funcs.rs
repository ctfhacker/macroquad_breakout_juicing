use macroquad::prelude::*;

pub struct Macroquad {
    pub clear_background: fn(Color),
    pub draw_rectangle: fn(f32, f32, f32, f32, Color),
    pub draw_circle: fn(f32, f32, f32, Color),
    pub is_key_down: fn(KeyCode) -> bool,
    pub camera_font_scale: fn(f32) -> (u16, f32, f32),
    pub draw_text_ex: fn(&str, f32, f32, TextParams),
    pub gen_range: fn(f32, f32) -> f32,
}

pub const fn get_macroquad() -> Macroquad {
    Macroquad {
        clear_background,
        draw_rectangle,
        draw_circle,
        is_key_down,
        camera_font_scale,
        draw_text_ex,
        gen_range: macroquad::rand::gen_range,
    }
}
