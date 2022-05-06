mod palette;

use crate::palette::Palette;
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

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
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    let ctx = get_ctx();
    sierpinski(
        &ctx,
        [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)],
        String::from("#eee"),
        1,
    );

    Ok(())
}

#[wasm_bindgen]
pub fn update(number: JsValue) {
    let depth: u8 = u8::from_str(&number.as_string().expect("string number")).expect("number");
    let ctx = get_ctx();
    let (width, height) = (
        &ctx.canvas().expect("canvas").client_width(),
        &ctx.canvas().expect("canvas").client_height(),
    );

    let _ = &ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("#EEE"));
    let _ = &ctx.fill_rect(0.0, 0.0, width.to_owned() as f64, height.to_owned() as f64);

    let palette = Palette::new();
    let next_color = palette.rand();

    sierpinski(
        &ctx,
        [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)],
        next_color,
        depth,
    );
}

fn get_ctx() -> CanvasRenderingContext2d {
    let window = web_sys::window().expect("window object");
    let document = window.document().expect("document object");
    let canvas = document
        .get_element_by_id("canvas")
        .expect("some element")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("cast into canvas");
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .expect("context")
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("context");
    ctx
}

fn sierpinski(
    ctx: &web_sys::CanvasRenderingContext2d,
    points: [(f64, f64); 3],
    color: String,
    depth: u8,
) {
    draw_triangle(&ctx, points, color);
    let depth = depth - 1;
    if depth > 0 {
        let [top, left, right] = points;
        let left_middle = midpoint(top, left);
        let right_middle = midpoint(top, right);
        let bottom_middle = midpoint(left, right);

        let palette = Palette::new();
        let next_color = palette.rand();

        sierpinski(
            &ctx,
            [top, left_middle, right_middle],
            next_color.clone(),
            depth,
        );
        sierpinski(
            &ctx,
            [left_middle, left, bottom_middle],
            next_color.clone(),
            depth,
        );
        sierpinski(
            &ctx,
            [right_middle, bottom_middle, right],
            next_color,
            depth,
        );
    }
}

fn midpoint(p1: (f64, f64), p2: (f64, f64)) -> (f64, f64) {
    ((p1.0 + p2.0) / 2.0, (p1.1 + p2.1) / 2.0)
}

fn draw_triangle(ctx: &web_sys::CanvasRenderingContext2d, points: [(f64, f64); 3], color: String) {
    let [top, left, right] = points;
    // let color_str = format!("rgb({},{},{})", color.0, color.1, color.2);

    ctx.set_fill_style(&wasm_bindgen::JsValue::from_str(&color));
    ctx.move_to(top.0, top.1);
    ctx.begin_path();
    ctx.line_to(left.0, left.1);
    ctx.line_to(right.0, right.1);
    ctx.line_to(top.0, top.1);
    ctx.close_path();
    ctx.stroke();
    ctx.fill();
}
