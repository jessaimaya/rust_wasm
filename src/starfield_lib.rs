mod browser;
mod palette;
mod renderer;
mod renderloop;
mod sketch;
mod star;
mod starfield;
mod utils;

use crate::palette::Palette;
use crate::renderer::Renderer;
use crate::renderloop::RenderLoop;
use crate::sketch::Sketch;
use crate::starfield::Starfield;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let renderer = Renderer::new();
    renderer.background("#272727".to_owned());

    let starfield = Starfield::new();
    let _ = RenderLoop::start(starfield).expect("RenderLoop");

    Ok(())
}
