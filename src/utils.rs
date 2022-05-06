use crate::Palette;
use rand::Rng;

pub fn clamp(value: f64, start_1: f64, stop_1: f64, start_2: f64, stop_2: f64) -> f64 {
    (value - start_2) / (stop_1 - start_1) * (stop_2 - start_2) + start_2
}

pub fn rand_pos_range(width: f64, height: f64) -> (f64, f64, f64) {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-width..width);
    let y = rng.gen_range(-height..height);
    let z = rng.gen_range(0.0..width);
    (x, y, z)
}

pub fn get_rand_color() -> String {
    let palette = Palette::new();
    palette.rand()
}
