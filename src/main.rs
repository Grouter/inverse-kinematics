#[macro_use]
extern crate glium;

mod float2;
mod systems;
mod graphics;
mod app;

use std::time::{Instant, Duration};
use glium::glutin::event_loop::{EventLoop, ControlFlow};
use glium::glutin::event::{Event, WindowEvent};
use glium::Surface;

use crate::float2::Float2;
use crate::graphics::create_display;
use crate::app::App;

const INITIAL_DISPLAY_SIZE: [u32; 2] = [1280, 720];

const BG: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
const SEGMENT_COLOR: [f32; 3] = [1.0, 1.0, 1.0];

pub const SEGMENT_LENGTH: f32 = 25.0;
pub const SEGMENT_WIDTH: f32 = 10.0;
pub const ENTITY_COUNT: usize = 100;

fn handle_events(event: Event<()>, control_flow: &mut ControlFlow, app: &mut App) {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            WindowEvent::KeyboardInput { input, .. } => {
                app.on_keyboard(input);
            }
            WindowEvent::CursorMoved { position, .. } => {
                app.on_mouse_move(&position);
            }
            WindowEvent::Resized(size) => {
                app.on_window_resize(&size);
            }
            _ => {},
        }
        Event::MainEventsCleared => {
            // Logic
            app.update();

            // Graphics
            let mut target = app.display.draw();
            target.clear_color(BG[0], BG[1], BG[2], BG[3]);
            app.render(&mut target);
            target.finish().unwrap();
        }
        _ => (),
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let display = create_display(
        &event_loop,
        INITIAL_DISPLAY_SIZE[0],
        INITIAL_DISPLAY_SIZE[1]
    );

    let mut app = App::new(display);

    app.generate_segments(&Float2::new(0.0, 1.0));

    // Approx 60 FPS
    let frame_time = Duration::from_nanos(16_666_667);

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = Instant::now() + frame_time;

        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        handle_events(event, control_flow, &mut app);
    });
}