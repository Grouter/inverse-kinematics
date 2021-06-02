use crate::float2::Float2;
use crate::{ENTITY_COUNT, SEGMENT_LENGTH};

pub fn generate_segments(base: &Float2, dir: &Float2, bases: &mut[Float2], dirs: &mut[Float2]) {
    let mut start = *base;
    let offset = Float2::new(dir.x * SEGMENT_LENGTH, dir.y * SEGMENT_LENGTH);

    for i in 0..ENTITY_COUNT {
        bases[i].set_as(&start);

        start.add(offset.x, offset.y);

        dirs[i] = *dir;
    }
}
