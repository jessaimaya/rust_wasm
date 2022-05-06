use anyhow::{anyhow, Result};
use futures::channel::mpsc::{unbounded, UnboundedReceiver};
use log::error;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::{Closure, WasmClosure};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, Window};

pub fn window() -> Result<Window> {
    web_sys::window().ok_or_else(|| anyhow!("No Window Found"))
}

pub fn document() -> Result<Document> {
    window()?
        .document()
        .ok_or_else(|| anyhow!("No Document Found"))
}

pub fn canvas() -> Result<HtmlCanvasElement> {
    document()?
        .get_element_by_id("canvas")
        .ok_or_else(|| anyhow!("No Canvas Element found with ID 'canvas'"))?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlCanvasElement", element))
}

pub fn context() -> Result<CanvasRenderingContext2d> {
    canvas()?
        .get_context("2d")
        .map_err(|js_value| anyhow!("Error getting 2d context {:#?}", js_value))?
        .ok_or_else(|| anyhow!("No 2d context found"))?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|element| {
            anyhow!(
                "Error converting {:#?} to CanvasRenderingContext2d",
                element
            )
        })
}
pub fn size() -> (f64, f64) {
    (
        context()
            .expect("CanvasRenderingContext2d")
            .canvas()
            .expect("canvas from context")
            .width() as f64,
        context()
            .expect("CanvasRenderingContext2d")
            .canvas()
            .expect("canvas from context")
            .height() as f64,
    )
}

pub type LoopClosure = Closure<dyn FnMut(f64)>;
pub fn create_raf_closure(f: impl FnMut(f64) + 'static) -> LoopClosure {
    closure_wrap(Box::new(f))
}

pub fn request_animation_frame(callback: &LoopClosure) -> Result<i32> {
    window()?
        .request_animation_frame(callback.as_ref().unchecked_ref())
        .map_err(|err| anyhow!("Cannot request animation frame {:#?}", err))
}

pub fn closure_wrap<T: WasmClosure + ?Sized>(data: Box<T>) -> Closure<T> {
    Closure::wrap(data)
}

pub fn now() -> Result<f64> {
    Ok(window()?
        .performance()
        .ok_or_else(|| anyhow!("Performance object not found"))?
        .now())
}

pub fn mouse() -> Result<UnboundedReceiver<(f64, f64)>> {
    let (mouse_sender, mouse_receiver) = unbounded();
    let mouse_sender = Rc::new(RefCell::new(mouse_sender));
    let on_mouse = Closure::wrap(Box::new(move |ev: web_sys::MouseEvent| {
        let rect = canvas().expect("Canvas").get_bounding_client_rect();
        let x: f64 = ev.client_x() as f64 - rect.left();
        let y: f64 = ev.client_y() as f64 - rect.top();
        if let Err(err) = mouse_sender.as_ref().borrow_mut().start_send((x, y)) {
            error!("Could not send keyDown message {:#?}", err);
        }
    }) as Box<dyn FnMut(web_sys::MouseEvent)>);

    canvas()?.set_onmousemove(Some(on_mouse.as_ref().unchecked_ref()));
    on_mouse.forget();

    Ok(mouse_receiver)
}
