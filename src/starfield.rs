use crate::browser::size;
use crate::sketch::Sketch;
use crate::star::Star;
use crate::utils::{clamp, rand_pos_range};
use crate::Renderer;
use futures::channel::mpsc::UnboundedReceiver;
use wasm_bindgen::JsValue;

pub struct Starfield {
    stars: Vec<Star>,
    threshold: f64,
    mouse: (f64, f64),
}

impl Starfield {
    pub fn new() -> Self {
        Starfield {
            stars: vec![],
            threshold: 1.0,
            mouse: (0.0, 0.0),
        }
    }
}

impl Sketch for Starfield {
    fn initialize(&self) -> anyhow::Result<Box<dyn Sketch>> {
        let mut stars = vec![];
        for _ in 0..300 {
            stars.push(Star::new());
        }
        Ok(Box::new(Starfield {
            stars,
            threshold: 1.0,
            mouse: (0.0, 0.0),
        }))
    }

    fn update(&mut self, mouse_receiver: &mut UnboundedReceiver<(f64, f64)>) {
        let (width, height) = size();
        match mouse_receiver.try_next() {
            Ok(Some(mouse)) => {
                self.mouse = mouse;
            }
            _ => (),
        };
        let threshold = clamp(self.mouse.0, 0.0, width, 0.0, 10.0);
        self.threshold = threshold;

        self.stars.iter_mut().for_each(move |star| {
            if (star.z - threshold) < 1.0 {
                (star.x, star.y, star.z) = rand_pos_range(width, height);
                star.pz = star.z;
            } else {
                star.z -= threshold;
            }
        });
    }

    fn draw(&self, renderer: &Renderer) {
        let threshold = self.threshold;
        let (width, height) = size();
        let ctx = renderer.context.to_owned();
        renderer.background("#272727".to_owned());
        self.stars.iter().for_each(|star| {
            let sx = clamp(star.x / star.z, 0.0, 1.0, 0.0, width);
            let sy = clamp(star.y / star.z, 0.0, 1.0, 0.0, height);
            let psx = clamp(star.x / star.pz, 0.0, 1.0, 0.0, width);
            let psy = clamp(star.y / star.pz, 0.0, 1.0, 0.0, height);

            let r = clamp(star.z, 0.0, width, width * 0.01, 0.0);
            if star.z - threshold > 1.0 {
                ctx.set_line_width(1.0);
                ctx.set_stroke_style(&JsValue::from(star.color.as_str()));
                ctx.begin_path();
                ctx.move_to(sx, sy);
                ctx.line_to(psx, psy);
                ctx.close_path();
                ctx.stroke();
            }
            ctx.set_fill_style(&JsValue::from(star.color.as_str()));
            ctx.begin_path();
            ctx.arc(sx, sy, r, 0.0, 2.0 * std::f64::consts::PI)
                .expect("ellipse created");
            ctx.close_path();
            ctx.fill();
        });
    }
}
