use crate::pvector::PVector;
use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use piston::Position;
use piston_window::math::Matrix2d;
use std::f64::consts::PI;

const RADIUS: f64 = 16.0;
const STEP_COLOUR: [f32; 4] = [1.0; 4];
const STEP_FACTOR: f64 = 100.0;
const TIME_STEP: f64 = 0.001;

pub struct Mover {
    time: f64,
    position: PVector,
    acceleration: PVector,
    velocity: PVector,
    mass: f64
}

impl Mover {
    pub fn new(position: PVector, mass: f64) -> Self {
        Mover {
            time: 0.0,
            position,
            acceleration: PVector::new(0.0, 0.0),
            velocity: PVector::new(0.0, 0.0),
            mass
        }
    }

    pub fn apply_force(&mut self, force: PVector) {
        self.acceleration = PVector::add(self.acceleration, PVector::div(force, self.mass));
    }

    pub fn update(&mut self) {
        // self.apply_force(PVector::new(10.0, 10.0));
        self.update_velocity();

        let delta_pos: PVector = PVector::new(self.velocity.x * TIME_STEP, self.velocity.y * TIME_STEP);
        self.position = PVector::add(self.position, delta_pos);
    }

    fn update_velocity(&mut self) {
        let delta_v: PVector = PVector::new(self.acceleration.x * TIME_STEP, self.acceleration.y * TIME_STEP);
        
        self.velocity = PVector::add(self.velocity, delta_v);
    }

    pub fn render(&self, graphics: &mut GfxGraphics<Resources, CommandBuffer>, transform: Matrix2d) {
        let square = piston_window::ellipse::circle(self.position.x, self.position.y, RADIUS);
        piston_window::ellipse(STEP_COLOUR, square, transform, graphics);
    }
}