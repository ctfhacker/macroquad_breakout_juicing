//! Provide an abstraction over animating objects

use crate::*;

#[derive(Default)]
pub struct AnimationState {
    /// The absolute target coordinates of the animation
    pub absolute_position: Option<Vec2>,

    /// The relative target coordinates of the animation
    pub relative_position: Option<Vec2>,
}

pub struct Animation {
    /// The target state of this object animation
    pub target: AnimationState,

    /// The ease function to use for this animation
    pub ease: EaseFunc,

    /// How long this animation should last (in seconds)
    pub duration: f32,
}
