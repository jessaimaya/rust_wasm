use crate::browser::{context, size};
use web_sys::CanvasRenderingContext2d;

pub struct Renderer {
    pub context: CanvasRenderingContext2d,
}
impl Renderer {
    pub fn new() -> Self {
        let ctx = context().expect("CanvasRenderingContext2d");
        let (width, height) = size();

        ctx.translate(width / 2.0, height / 2.0)
            .expect("context center translated");
        Renderer { context: ctx }
    }

    pub fn background(&self, color: String) {
        let (width, height) = size();

        self.context
            .set_fill_style(&wasm_bindgen::JsValue::from_str(&color));
        self.context
            .fill_rect(-width, -height, width * 2.0, height * 2.0);
        self.context.fill();
    }
}
