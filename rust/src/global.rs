use rand::Rng;
use std::io::{self, Write};

pub const M_PI: f32 = std::f32::consts::PI;
pub const EPSILON: f32 = f32::EPSILON;
pub const K_INFINITY: f32 = f32::MAX;

pub fn clamp(lo: f32, hi: f32, v: f32) -> f32 {
    lo.max(v.min(hi))
}

pub fn get_random_float() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

pub fn update_progress(progress: f32) {
    let bar_width = 70usize;
    let mut stdout = io::stdout();
    let pos = (bar_width as f32 * progress) as usize;

    let _ = write!(stdout, "[");
    for i in 0..bar_width {
        if i < pos {
            let _ = write!(stdout, "=");
        } else if i == pos {
            let _ = write!(stdout, ">");
        } else {
            let _ = write!(stdout, " ");
        }
    }

    let _ = write!(stdout, "] {} %\r", (progress * 100.0) as i32);
    let _ = stdout.flush();
}
