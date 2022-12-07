use crate::pvector::PVector;
use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use piston::Position;
use piston_window::math::Matrix2d;
use std::f64::consts::PI;

const RADIUS: f64 = 16.0;
const STEP_COLOUR: [f32; 4] = [1.0; 4];
const TIME_STEP: f64 = 0.01;
const G: f64 = 6674.3;

#[derive(Debug, Copy, Clone)]
pub struct Mover {
    time: f64,
    pub position: PVector,
    acceleration: PVector,
    velocity: PVector,
    pub mass: f64,
}

impl Mover {
    pub fn new(position: PVector, mass: f64) -> Self {
        Mover {
            time: 0.0,
            position,
            acceleration: PVector::new(0.0, 0.0),
            velocity: PVector::new(0.0, 0.0),
            mass,
        }
    }

    pub fn new_rand() -> Self {
        Mover {
            time: 0.0,
            position: PVector::gen_rand(),
            acceleration: PVector::new(0.0, 0.0),
            velocity: PVector::new(0.0, 0.0),
            mass: rand::random::<f64>() * 10.0,
        }
    }

    pub fn attract(&mut self, m: Mover) -> PVector {
        let mut force: PVector = PVector::sub(self.position, m.position);
        let mut distance: f64 = force.magnitude();
        distance = Self::constrain(distance, 50.0, 200.0);
        force = force.normalize();

        let strength: f64 = (G * self.mass * m.mass) / (distance * distance);
        force = PVector::scale(force, strength);

        force
    }

    fn constrain(mut distance: f64, min: f64, max: f64) -> f64 {
        if distance < min {
            distance = min
        } else if distance > max {
            distance = max
        }

        distance
    }

    pub fn apply_force(&mut self, force: PVector) {
        self.acceleration = PVector::add(self.acceleration, PVector::div(force, self.mass));
    }

    pub fn update(&mut self) {
        // self.apply_force(PVector::new(10.0, 10.0));
        self.update_velocity();

        let delta_pos: PVector =
            PVector::new(self.velocity.x * TIME_STEP, self.velocity.y * TIME_STEP);
        self.position = PVector::add(self.position, delta_pos);

        // self.check_edges();
    }

    fn update_velocity(&mut self) {
        let delta_v: PVector = PVector::new(
            self.acceleration.x * TIME_STEP,
            self.acceleration.y * TIME_STEP,
        );

        self.velocity = PVector::add(self.velocity, delta_v);
    }

    fn check_edges(&mut self) {
        if self.position.x > 640.0 {
            self.position.x = 640.0;
            self.velocity.x *= -1.0;
            // self.acceleration.x *= -1.0;
        } else if self.position.x < 0.0 {
            self.velocity.x *= -1.0;
            // self.acceleration.x *= -1.0;
            self.position.x = 0.0;
        }

        if self.position.y > 480.0 {
            self.velocity.y *= -1.0;
            self.position.y = 480.0;
        }
    }

    pub fn render(
        &self,
        graphics: &mut GfxGraphics<Resources, CommandBuffer>,
        transform: Matrix2d,
    ) {
        let square = piston_window::ellipse::circle(
            self.position.x,
            self.position.y,
            RADIUS * self.mass / 10.0,
        );
        piston_window::ellipse(STEP_COLOUR, square, transform, graphics);
    }
}
