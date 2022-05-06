use crate::browser::{
    context, create_raf_closure, mouse, now, request_animation_frame, LoopClosure,
};
use crate::{Renderer, Sketch};
use anyhow::{anyhow, Result};
use std::cell::RefCell;
use std::rc::Rc;

const FRAME_SIZE: f32 = 1.0 / 60.0 * 1000.0;
pub struct RenderLoop {
    last_frame: f64,
    accumulated_delta: f32,
}
type SharedLoopClosure = Rc<RefCell<Option<LoopClosure>>>;

impl RenderLoop {
    pub fn start(game: impl Sketch + 'static) -> Result<()> {
        let mut mouse_receiver = mouse()?;
        let mut game = game.initialize()?;
        let mut game_loop = RenderLoop {
            last_frame: now()?,
            accumulated_delta: 0.0,
        };

        let renderer = Renderer {
            context: context()?,
        };

        let f: SharedLoopClosure = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(create_raf_closure(move |perf: f64| {
            let frame_time = perf - game_loop.last_frame;
            game_loop.accumulated_delta += frame_time as f32;
            while game_loop.accumulated_delta > FRAME_SIZE {
                game.update(&mut mouse_receiver);
                game_loop.accumulated_delta -= FRAME_SIZE;
            }
            game_loop.last_frame = perf;
            game.draw(&renderer);

            request_animation_frame(f.borrow().as_ref().unwrap()).unwrap();
        }));

        request_animation_frame(
            g.borrow()
                .as_ref()
                .ok_or_else(|| anyhow!("GameLoop: Loop is None"))?,
        )?;
        Ok(())
    }
}
