use std::u32;

use glium::{Display, Frame, Program, Surface, VertexBuffer};
use glium::glutin::event::KeyboardInput;
use glium::glutin::dpi::{PhysicalPosition, PhysicalSize};

use crate::{ENTITY_COUNT, INITIAL_DISPLAY_SIZE, SEGMENT_COLOR, SEGMENT_LENGTH, SEGMENT_WIDTH};
use crate::float2::Float2;
use crate::systems::*;
use crate::graphics::*;

pub struct App {
    pub display: Display,
    pub display_size: PhysicalSize<u32>,
    
    pub perspective: [[f32; 4]; 4],
    pub segment_mesh: Mesh,
    pub shader: Program,
    pub instance_buffer: VertexBuffer<Transform>,

    pub target: Float2,
    pub base: Float2,
    pub components: Components,
}

pub struct Components {
    pub bases: [Float2; ENTITY_COUNT],
    pub directions: [Float2; ENTITY_COUNT],
    pub targets: [Float2; ENTITY_COUNT],
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
            bases: [Float2::default(); ENTITY_COUNT],
            directions: [Float2::default(); ENTITY_COUNT],
            targets: [Float2::default(); ENTITY_COUNT],
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

            target: Float2::default(),
            base: Default::default(),
            components
        }
    }

    pub fn generate_segments(&mut self, dir: &Float2) {
        let mut start = self.base;

        let offset = Float2::new(dir.x * SEGMENT_LENGTH, dir.y * SEGMENT_LENGTH);
    
        for i in 0..ENTITY_COUNT {
            self.components.bases[i].set_as(&start);
    
            start.add(offset.x, offset.y);
    
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
        self.target.x = position.x as f32;
        self.target.y = position.y as f32;
    }

    pub fn on_window_resize(&mut self, size: &PhysicalSize<u32>) {
        self.display_size = *size;

        self.perspective = perspective_left_handed(
            self.display_size.width, 
            self.display_size.height
        );
    }
}