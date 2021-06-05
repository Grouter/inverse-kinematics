use glium::{Display, Frame, Program, Surface, VertexBuffer};
use glium::glutin::event::KeyboardInput;
use glium::glutin::dpi::{PhysicalPosition, PhysicalSize};
use vecmath::{Matrix4, Vector2, vec2_add};

use crate::{ENTITY_COUNT, INITIAL_DISPLAY_SIZE, SEGMENT_COLOR, SEGMENT_LENGTH, SEGMENT_WIDTH};
use crate::systems::*;
use crate::graphics::*;

pub struct App {
    pub display: Display,
    pub display_size: PhysicalSize<u32>,
    
    pub perspective: Matrix4<f32>,
    pub segment_mesh: Mesh,
    pub shader: Program,
    pub instance_buffer: VertexBuffer<Transform>,

    pub target: Vector2<f32>,
    pub base: Vector2<f32>,
    pub components: Components,
}

pub struct Components {
    pub bases: [Vector2<f32>; ENTITY_COUNT],
    pub directions: [Vector2<f32>; ENTITY_COUNT],
    pub targets: [Vector2<f32>; ENTITY_COUNT],
    pub transforms: [Transform; ENTITY_COUNT]
}

impl App {
    pub fn new(display: Display) -> App {
        let (vertices, indices) = create_segment_shape(
            SEGMENT_WIDTH,
            SEGMENT_LENGTH,
            SEGMENT_COLOR
        );

        let segment_mesh = create_mesh(&display, &vertices, &indices);

        let shader = load_program(
            &display,
            "shaders/vertex.glsl",
            "shaders/fragment.glsl"
        );

        let components = Components {
            bases: [[0.0, 0.0]; ENTITY_COUNT],
            directions: [[0.0, 0.0]; ENTITY_COUNT],
            targets: [[0.0, 0.0]; ENTITY_COUNT],
            transforms: [default_transform(); ENTITY_COUNT] 
        };

        let instance_buffer = VertexBuffer::dynamic(
            &display, 
            &components.transforms
        ).unwrap();

        App {
            display,
            display_size: PhysicalSize {
                width: INITIAL_DISPLAY_SIZE[0],
                height: INITIAL_DISPLAY_SIZE[1]
            },

            perspective: perspective_left_handed(
                INITIAL_DISPLAY_SIZE[0], 
                INITIAL_DISPLAY_SIZE[1]
            ),
            segment_mesh,
            shader,
            instance_buffer,

            target: [0.0, 0.0],
            base: [0.0, 0.0],
            components
        }
    }

    pub fn generate_segments(&mut self, dir: &Vector2<f32>) {
        let mut start = self.base;

        let offset = [dir[0] * SEGMENT_LENGTH, dir[1] * SEGMENT_LENGTH];
    
        for i in 0..ENTITY_COUNT {
            self.components.bases[i][0] = start[0];
            self.components.bases[i][1] = start[1];
    
            start = vec2_add(start, offset);
    
            self.components.directions[i] = *dir;
        }
    }

    pub fn render(&self, target: &mut Frame) {
        // Update the instance buffer
        self.instance_buffer.write(&self.components.transforms);

        // Instancing
        target.draw(
            (&self.segment_mesh.v_buffer, self.instance_buffer.per_instance().unwrap()),
            &self.segment_mesh.i_buffer,
            &self.shader,
            &uniform! {
                perspective: self.perspective,
            },
            &Default::default()
        ).unwrap();
    }

    pub fn update(&mut self) {
        update_targets(
            &mut self.components.targets,
            &self.components.bases,
            &self.target
        );

        follow(
            &mut self.components.bases,
            &mut self.components.directions,
            &mut self.components.targets
        );

        update_bases(
            &mut self.components.bases,
            &mut self.components.directions,
            &self.base
        );

        update_transforms(
            &mut self.components.transforms, 
            &self.components.bases, 
            &self.components.directions
        );
    }

    pub fn on_keyboard(&mut self, _input: KeyboardInput) {
        //println!("{}", input.scancode);
    }

    pub fn on_mouse_move(&mut self, position: &PhysicalPosition<f64>) {
        self.target[0] = position.x as f32;
        self.target[1] = position.y as f32;
    }

    pub fn on_window_resize(&mut self, size: &PhysicalSize<u32>) {
        self.display_size = *size;

        self.perspective = perspective_left_handed(
            self.display_size.width, 
            self.display_size.height
        );
    }
}