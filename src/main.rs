mod arm;
mod float2;
mod systems;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::MouseCursorEvent;
use graphics::types::Rectangle;
use graphics::{rectangle, clear};

use crate::float2::Float2;
use crate::arm::generate_segments;
use crate::systems::*;

const BG: [f32; 4] = [0.3, 0.3, 0.3, 1.0];

pub const SEGMENT_LENGTH: f32 = 1.0;
pub const SEGMENT_WIDTH: f32 = 4.0;
pub const ENTITY_COUNT: usize = 800;

pub struct App {
    target: Float2,
    base: Float2,
    segment_shape: Rectangle,
    bases: [Float2; ENTITY_COUNT],
    directions: [Float2; ENTITY_COUNT],
    targets: [Float2; ENTITY_COUNT],
}

impl App {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        clear(BG, gl);

        gl.draw(
            args.viewport(),
            |c, gl| {
                render_segments(&c, gl, &self.bases, &self.directions, &self.segment_shape);
            }
        );

        self.base.x = args.window_size[0] as f32 / 2.0;
    }

    fn update(&mut self, _args: &UpdateArgs) {
        
        update_targets(&mut self.targets, &self.bases, &self.target);
        
        follow(&mut self.bases, &mut self.directions, &mut self.targets);

        update_bases(&mut self.bases, &mut self.directions, &self.base);
    }

    fn on_mouse_move(&mut self, position: &[f64; 2]) {
        self.target.x = position[0] as f32;
        self.target.y = position[1] as f32;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("inverse_kinematics", [1280, 720])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl: GlGraphics = GlGraphics::new(opengl);

    let base = Float2::new(340.0, 0.0);

    // Run the app
    let mut app = App {
        target: Float2::default(),
        base: base,
        segment_shape: rectangle::rectangle_by_corners(
            0.0,
            0.0,
            SEGMENT_LENGTH as f64,
            SEGMENT_WIDTH as f64
        ),
        bases: [Float2::default(); ENTITY_COUNT],
        directions: [Float2::default(); ENTITY_COUNT],
        targets: [Float2::default(); ENTITY_COUNT],
    };

    generate_segments(
        &base, 
        &Float2::new(0.0, 1.0), 
        &mut app.bases, 
        &mut app.directions
    );

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.mouse_cursor_args() {
            app.on_mouse_move(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.render_args() {
            app.render(&mut gl, &args);
        }
    }
}