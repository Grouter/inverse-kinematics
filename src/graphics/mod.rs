use cgmath::conv::array4x4;
use glium::glutin::dpi::PhysicalSize;
use glium::{Display, VertexBuffer, Program, IndexBuffer};
use glium::glutin::event_loop::EventLoop;
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::index::PrimitiveType;
use std::fs;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 3]
}
implement_vertex!(Vertex, position, color);

#[derive(Copy, Clone)]
pub struct Transform {
    pub transform: [[f32; 4]; 4]
}
implement_vertex!(Transform, transform);

pub struct Mesh {
    pub v_buffer: VertexBuffer<Vertex>,
    pub i_buffer: IndexBuffer<u16>,
}

pub fn create_display(event_loop: &EventLoop<()>, w: u32, h: u32) -> Display {
    let display = Display::new(
        WindowBuilder::new(),
        ContextBuilder::new(),
        &event_loop
    ).expect("Could not create display");

    display.gl_window().resize(PhysicalSize {
        width: w,
        height: h
    });

    display
}

pub fn create_segment_shape(w: f32, h: f32, color: [f32; 3]) -> ([Vertex; 4], [u16; 6]) {
    let vertices = [
        Vertex { position: [ 0.0,  w / 2.0], color },
        Vertex { position: [   h,  w / 2.0], color },
        Vertex { position: [   h, -w / 2.0], color },
        Vertex { position: [ 0.0, -w / 2.0], color },
    ];

    let indices = [
        1, 0, 3,
        3, 2, 1,
    ];

    (vertices, indices)
}

pub fn create_mesh(display: &Display, vertices: &[Vertex], indices: &[u16]) -> Mesh {
    let v_buffer = VertexBuffer::new(
        display,
        vertices
    ).expect("Error creating vertex buffer");

    let i_buffer = IndexBuffer::new(
        display,
        PrimitiveType::TrianglesList,
        indices
    ).expect("Error creating index buffer");

    Mesh { v_buffer, i_buffer }
}

/// Loads vertex and fragment shader program
pub fn load_program(display: &Display, vertex_shader: &str, fragment_shader: &str) -> Program {
    let vertex_source = fs::read_to_string(vertex_shader)
        .expect("Error while loading vertex shader");

    let fragment_source = fs::read_to_string(fragment_shader)
        .expect("Error while loading fragment shader");

    Program::from_source(
        display,
        vertex_source.as_str(),
        fragment_source.as_str(),
        None
    ).expect("Error in shader program compilation")
}

pub fn perspective_left_handed(display_w: u32, display_h: u32) -> [[f32; 4]; 4] {
    const Z_NEAR: f32 = -1.0;
    const Z_FAR: f32 = 1.0;

    let ortho = cgmath::ortho::<f32>(
        0.0, 
        display_w as f32, 
        display_h as f32, 
        0.0, 
        Z_NEAR, 
        Z_FAR
    );

    array4x4(ortho)
}

pub fn default_transform() -> Transform {
    Transform { 
        transform: [
            [1.0, 0.0 ,0.0, 0.0],
            [0.0, 1.0 ,0.0, 0.0],
            [0.0, 0.0 ,1.0, 0.0],
            [0.0, 0.0 ,0.0, 1.0],
        ]
    }
}