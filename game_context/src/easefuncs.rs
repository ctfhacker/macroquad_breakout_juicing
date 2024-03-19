//! Various easing functions

#[derive(Default, Debug, Clone)]
pub enum EaseFunc {
    #[default]
    Linear,
    SmoothStop2,
    SmoothStop3,
    SmoothStop4,
    SmoothStop5,
}

impl EaseFunc {
    /// Calcuate the result based on the given `t`
    pub fn calculate(&self, t: f32) -> f32 {
        match self {
            EaseFunc::Linear => t,
            EaseFunc::SmoothStop2 => 1. - ((1. - t) * (1. - t)),
            EaseFunc::SmoothStop3 => 1. - ((1. - t) * (1. - t) * (1. - t)),
            EaseFunc::SmoothStop4 => 1. - ((1. - t) * (1. - t) * (1. - t) * (1. - t)),
            EaseFunc::SmoothStop5 => 1. - ((1. - t) * (1. - t) * (1. - t) * (1. - t) * (1. - t)),
        }
    }
}
