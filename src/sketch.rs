use crate::renderer::Renderer;
use anyhow::Result;
use futures::channel::mpsc::UnboundedReceiver;

pub trait Sketch {
    fn initialize(&self) -> Result<Box<dyn Sketch>>;
    fn update(&mut self, mouse: &mut UnboundedReceiver<(f64, f64)>);
    fn draw(&self, renderer: &Renderer);
}
