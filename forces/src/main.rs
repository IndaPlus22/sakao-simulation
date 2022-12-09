// mod walker;
mod mover;
mod pvector;

extern crate gfx_graphics;
extern crate opengl_graphics;
extern crate piston_window;

use mover::Mover;
use opengl_graphics::OpenGL;
use piston::{
    input::{RenderEvent, UpdateEvent},
    window::WindowSettings,
};
use piston_window::{Event, PistonWindow};
use pvector::PVector;
// use walker::Walker;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

const BLACK_COLOUR: [f32; 4] = [0.0; 4];


pub struct App {
    movers: Vec<Mover>,
}

impl App {
    fn new() -> Self {
        App {
            movers: Vec::new(),
        }
    }

    fn init(&mut self) {
        // self.movers.push(Mover::new(PVector::new(SCREEN_WIDTH as f64 / 2.0, SCREEN_HEIGHT as f64 / 2.0), 100.0));
        for i in 0..20 {
            self.movers.push(Mover::new_rand());
        }

        // println!("start");

        // for i in 0..self.movers.len() {
        //     self.movers[i].apply_force(PVector::new(30.0, 0.0));
        // }
    }

    // render the app
    fn render(&mut self, event: &Event, window: &mut PistonWindow) {

        // Update application window.
        window.draw_2d(event, |context, graphics, _| {
            // Fill the window with white colour.
            piston_window::clear(BLACK_COLOUR, graphics);

            for i in 0..self.movers.len() {
                self.movers[i].render(graphics, context.transform);
            }
        });
    }

    // update fn
    fn update(&mut self) {
        for i in 0..self.movers.len() {
            for j in 0..self.movers.len() {
                // println!("s acc: {}, {}", self.movers[i].acceleration.x, self.movers[i].acceleration.y);
                if i != j {
                    let tmp = self.movers[i];
                    let force: PVector = self.movers[j].attract(tmp);
                    self.movers[i].apply_force(force);
                }
                // println!("e acc: {}, {}", self.movers[i].acceleration.x, self.movers[i].acceleration.y);
            }

            // self.movers[i].apply_force(force);


            // let wind = PVector::new(10.0, 0.0);
            // let force_g: PVector = PVector::new(0.0, STEP_FACTOR * 0.982 * self.movers[i].mass);

            // self.movers[i].apply_force(wind);
            // self.movers[i].apply_force(force_g);
            self.movers[i].update();
        }
    }
}

fn main() {
    use std::{thread, time};

    let mut window: PistonWindow =
        WindowSettings::new("Chapter 2 Forces", (SCREEN_WIDTH, SCREEN_HEIGHT))
            .exit_on_esc(true)
            .graphics_api(OpenGL::V3_2)
            .build()
            .unwrap();

    let mut app = App::new();

    app.init();

    // game loop
    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            app.render(&event, &mut window);
        }
        if let Some(_) = event.update_args() {
            app.update();
        }

        let duration = time::Duration::from_millis(10);
        thread::sleep(duration);
    }
}
