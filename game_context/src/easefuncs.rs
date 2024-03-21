//! Various easing functions

use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub enum EaseFunc {
    Linear,
    SmoothStop2,
    SmoothStop3,
    SmoothStop4,
    SmoothStop5,
    SmoothStart2,
    SmoothStart3,
    SmoothStart4,
    SmoothStart5,
    SmoothStep2,
    SmoothStep3,
    SmoothStep4,
    ElasticStop {
        // Amount of elastic to add
        elastic: f32,
    },
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
            EaseFunc::SmoothStart2 => t * t,
            EaseFunc::SmoothStart3 => t * t * t,
            EaseFunc::SmoothStart4 => t * t * t * t,
            EaseFunc::SmoothStart5 => t * t * t * t * t,
            EaseFunc::SmoothStep2 => {
                // return x < 0.5 ? 2 * x * x : 1 - Math.pow(-2 * x + 2, 2) / 2;
                let t1 = (t < 0.5) as u32 as f32 * 2. * t * t + (t >= 0.5) as u32 as f32;
                let t2 = ((-2. * t + 2.) * (-2. * t + 2.)) / 2.;
                t1 - t2
            }
            EaseFunc::SmoothStep3 => {
                // return x < 0.5 ? 4 * x * x * x : 1 - Math.pow(-2 * x + 2, 3) / 2;
                let t1 = (t < 0.5) as u32 as f32 * 4. * t * t + (t >= 0.5) as u32 as f32;
                let t2 = ((-2. * t + 2.) * (-2. * t + 2.) * (-2. * t + 2.)) / 2.;
                t1 - t2
            }
            EaseFunc::SmoothStep4 => {
                // return x < 0.5 ? 8 * x * x * x : 1 - Math.pow(-2 * x + 2, 4) / 2;
                let t1 = (t < 0.5) as u32 as f32 * 8. * t * t + (t >= 0.5) as u32 as f32;
                let t2 = ((-2. * t + 2.) * (-2. * t + 2.) * (-2. * t + 2.) * (-2. * t + 2.)) / 2.;
                t1 - t2
            }
            EaseFunc::ElasticStop { elastic } => {
                // const c4 = (2 * Math.PI) / 3;
                // return x === 0
                //   ? 0
                //   : x === 1
                //   ? 1
                //   : Math.pow(2, -10 * x) * Math.sin((x * 10 - 0.75) * c4) + 1;
                const C4: f32 = (2. * PI) / 3.;

                let is_zero = (t == 0.) as u32 as f32;
                let is_one = (t == 1.) as u32 as f32;
                let is_middle = (t != 0. && t != 1.) as u32 as f32;

                is_zero * 0.
                    + is_one * 1.
                    + is_middle * (2.0_f32.powf(-10. * t) * ((t * elastic - 0.75) * C4).sin() + 1.0)
            }
        }
    }
}
