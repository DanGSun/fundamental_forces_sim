extern crate nalgebra as na;
use std::time::Duration;
use std::thread;
use std::f64::consts::PI;
use na::{Vector2, Point2};
use std::thread::sleep;

const G: f64 = 6.67430e-11; // should be e-11
const E0: f64 = 8.85418e-12; // should be e-12
const B_CHARGE: f64 = 1.60217e-19; // should be e-19

#[derive(Clone)]
pub struct Body {
    pub loc: Point2<f64>,
    pub force: Vector2<f64>,
    pub mass: f64,
    pub charge: f64
    // z: f64,
    // color: (u8, u8, u8)
}

impl Body {
    pub fn origin() -> Body {
        Body { loc: Point2::origin(), mass: 0.0, force: Vector2::new(0., 0.), charge: 0. }
    }

    pub fn new(x: f64, y: f64, mass: f64, charge: f64) -> Body {
        Body { loc: Point2::new(x, y), mass, force: Vector2::new(0., 0.), charge: charge*B_CHARGE }
    }

    pub fn update_loc(&mut self) {
        self.loc = &self.loc + &self.force
    }
}

pub struct Universe {
    pub bodies: Vec<Body>,
    time_since_beginning: u64,
}

pub fn gravity(a_body: &Body, b_body: &Body) -> Vector2<f64> {
    let mass_a = a_body.mass.clone();
    let mass_b = b_body.mass.clone();
    let loc_a = a_body.loc.clone();
    let loc_b = b_body.loc.clone();

    let gt_force = (G * mass_a * mass_b) / (loc_b - loc_a).norm_squared();
    return gt_force*(loc_b - loc_a);
}

pub fn electrostatic(a_body: &Body, b_body: &Body) -> Vector2<f64> {
    let q_a = a_body.charge.clone();
    let q_b = b_body.charge.clone();
    let loc_a = a_body.loc.clone();
    let loc_b = b_body.loc.clone();
    let result = ((1./(4.*PI*E0))*((q_a*q_b)/(loc_a - loc_b).norm_squared()))*(loc_a-loc_b);
    // println!("{}", result);
    result
}

impl Universe {
    pub fn new() -> Universe {
        Universe { bodies: vec![], time_since_beginning: 0 }
    }

    pub fn tick(&mut self) {
        self.time_since_beginning += 1;
        // Common forces
        for i in 0..self.bodies.len() {
            for j in 0..self.bodies.len() {
                if i == j { continue };
                let mut fin_force = self.bodies[i].force.clone();
                fin_force += gravity(&self.bodies[i], &self.bodies[j]);
                fin_force += electrostatic(&self.bodies[i], &self.bodies[j]);
                self.bodies[i].force = fin_force;
            }
        }
        for i in self.bodies.iter_mut() {
            i.update_loc();
        };
        // sleep(Duration::from_secs_f64(0.1));
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body)
    }
}