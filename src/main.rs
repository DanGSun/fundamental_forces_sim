mod universe;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events, EventLoop};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, ButtonEvent, PressEvent, ReleaseEvent};
use piston::window::WindowSettings;
use graphics::*;
use universe::Universe;
use crate::universe::Body;
use nalgebra::{Point2, Vector2, Matrix2, Matrix2x1};
use graphics::ellipse::circle;

pub struct App {
    gl: GlGraphics,
    universe: Universe,
    old_pos: Vec<(f64, f64)>,
    pub cam_x: f64,
    pub cam_y: f64
}

impl App {
    fn render(&mut self, args: &RenderArgs) {

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];



        let im = self.universe.bodies.iter();
        let cam = (self.cam_x.clone(), self.cam_y.clone());
        self.gl.clear_color(WHITE);
        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            for i in im {
                if i.mass < 10000. {
                    if i.charge < 0. {
                        ellipse(GREEN, circle(i.loc.x+cam.0, i.loc.y+cam.1,4.), transform, gl);
                    } else {
                        ellipse(RED, circle(i.loc.x+cam.0, i.loc.y+cam.1,4.), transform, gl);
                    }
                } else {
                    ellipse(BLACK, circle(i.loc.x+cam.0, i.loc.y+cam.1,4.), transform, gl);
                }
            }
        });

    }

    fn update(&mut self, args: &UpdateArgs) {
        self.universe.tick();
    }
}


fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Orbit-Sim", [1920, 1080])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new sim and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        universe: Universe::new(),
        old_pos: vec![],
        cam_x: 0.,
        cam_y: 0.
    };

    // Setting up some testing particles

    // At.1
    let pos = Body::new(500., 500., 20000., 2e12);
    let mut neg = Body::new(540., 500., 8000., -2e12);
    let mut neg2 = Body::new(460., 500., 8000., -2e12);

    neg.force += Vector2::new(0., 0.05);
    neg2.force += Vector2::new(0., -0.05);

    app.universe.add_body(pos);
    app.universe.add_body(neg);
    app.universe.add_body(neg2);

//    // At.2
//    let pos2 = Body::new(1600., 500., 20000., 2e12);
//    let mut neg3 = Body::new(1640., 500., 8000., -2e12);
//    let mut neg4 = Body::new(1560., 500., 8000., -2e12);
//
//    neg3.force += Vector2::new(0., 0.05);
//    neg4.force += Vector2::new(0., -0.05);
//
//    app.universe.add_body(pos2);
//    app.universe.add_body(neg3);
//    app.universe.add_body(neg4);


    let mut event_sets = EventSettings::new();
    event_sets.set_ups(2000);

    let mut events = Events::new(event_sets);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
            // break
        }
    }
}
