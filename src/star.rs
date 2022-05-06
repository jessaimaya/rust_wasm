use crate::browser::size;
use crate::utils::{get_rand_color, rand_pos_range};

pub struct Star {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pz: f64,
    pub color: String,
}

impl Star {
    pub fn new() -> Self {
        let (width, height) = size();
        let (x, y, z) = rand_pos_range(width, height);
        Star {
            x,
            y,
            z,
            pz: z,
            color: get_rand_color(),
        }
    }
}
