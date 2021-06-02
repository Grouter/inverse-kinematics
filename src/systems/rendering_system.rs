use graphics::types::Rectangle;
use graphics::Context;
use opengl_graphics::GlGraphics;

use crate::{ENTITY_COUNT, SEGMENT_WIDTH};
use crate::float2::Float2;

static SEGMENT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub fn render_segments(c: &Context, gl: &mut GlGraphics, bases: &[Float2], dirs: &[Float2], shape: &Rectangle) {
    use graphics::*;

    for i in 0..ENTITY_COUNT {
        let t = c
            .transform
            .trans(bases[i].x as f64, bases[i].y as f64)
            .rot_rad(dirs[i].rotation() as f64)
            .trans(0.0, -SEGMENT_WIDTH as f64 / 2.0);

        rectangle(SEGMENT_COLOR, *shape, t, gl);
    }
}
