//! Collection of all objects in the world

use crate::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Shape {
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
}

#[derive(Default)]
pub struct World {
    /// Positions of all objects in the world
    positions: Vec<Vec2>,

    /// Shapes of an object
    shapes: Vec<Shape>,

    /// Colors of an object
    colors: Vec<Color>,

    /// Is the current object being animated
    animating: Vec<bool>,

    /// The total time of this animation
    animation_duration: Vec<f32>,

    /// The elapsed time of this animation
    animation_elapsed: Vec<f32>,

    /// The starting animation position for this object
    animation_position_start: Vec<Vec2>,

    /// The target position for this object to finish by the end of the animation
    animation_position_target: Vec<Vec2>,

    /// The target color for this object to finish by the end of the animation
    animation_color_target: Vec<Vec2>,

    /// The target position for this object to finish by the end of the animation
    animation_position_ease: Vec<EaseFunc>,
}

pub struct Object {
    pub position: Vec2,
    pub color: Color,
    pub shape: Shape,
}

pub struct ObjectIndex(usize);

impl World {
    pub fn push(
        &mut self,
        Object {
            position,
            shape,
            color,
        }: Object,
    ) -> ObjectIndex {
        // Get the index for this object
        let index = ObjectIndex(self.positions.len());

        let Self {
            positions,
            shapes,
            colors,
            animating,
            animation_duration,
            animation_elapsed,
            animation_position_start,
            animation_position_target,
            animation_color_target,
            animation_position_ease,
        } = self;

        // Add this object to the world
        positions.push(position);
        shapes.push(shape);
        colors.push(color);
        animating.push(false);
        animation_duration.push(Default::default());
        animation_elapsed.push(Default::default());
        animation_position_start.push(Default::default());
        animation_position_target.push(Default::default());
        animation_color_target.push(Default::default());
        animation_position_ease.push(Default::default());

        assert!(positions.len() == shapes.len() && positions.len() == colors.len());

        index
    }

    pub fn update(&mut self, frame_time: f32) {
        for index in 0..self.positions.len() {
            // Only updating the animating objects
            if !self.animating[index] {
                continue;
            }

            let progress = self.animation_elapsed[index] / self.animation_duration[index];

            let progress = self.animation_position_ease[index].calculate(progress);

            let target = self.animation_position_target[index];
            let start = self.animation_position_start[index];
            let new_pos = start + (target - start) * progress;

            // Update the position
            self.positions[index] = new_pos;

            // Stop this animation if the duration has finished
            self.animation_elapsed[index] += frame_time;

            if self.animation_elapsed[index] >= self.animation_duration[index] {
                self.animating[index] = false;
            }
        }
    }

    // Draw the current world state!
    pub fn draw(&self, macroquad: &Macroquad) {
        for index in 0..self.positions.len() {
            let shape = &self.shapes[index];
            let position = &self.positions[index];
            let color = self.colors[index];

            match shape {
                Shape::Rectangle { width, height } => {
                    (macroquad.draw_rectangle)(position.x, position.y, *width, *height, color);
                }
                Shape::Circle { radius } => {
                    (macroquad.draw_circle)(position.x, position.y, *radius, RED);
                }
            }
        }
    }

    pub fn animate(&mut self, object: &ObjectIndex, animation: Animation) {
        let Animation {
            target,
            ease,
            duration,
        } = animation;

        match target {
            AnimationState {
                absolute_position: Some(target),
                ..
            } => {
                self.animation_position_target[object.0] = target;
                self.animation_position_start[object.0] = self.position(object);
            }
            _ => panic!("Unknown target position for animation"),
        }

        self.animation_position_ease[object.0] = ease;
        self.animation_duration[object.0] = duration;
        self.animating[object.0] = true;
    }

    /// Returns `true` if this object is animating and `false` otherwise
    pub fn is_animating(&self, object: &ObjectIndex) -> bool {
        self.animating[object.0]
    }

    /// Get the position of the given object
    pub fn position(&self, object: &ObjectIndex) -> Vec2 {
        self.positions[object.0]
    }

    /// Get a mut ref to the position of the given object
    pub fn position_mut(&mut self, object: &ObjectIndex) -> &mut Vec2 {
        &mut self.positions[object.0]
    }
}
