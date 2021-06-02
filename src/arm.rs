use crate::float2::Float2;
use crate::{ENTITY_COUNT, SEGMENT_LENGTH};

pub fn generate_segments(base: &Float2, dir: &Float2, bases: &mut[Float2], dirs: &mut[Float2]) {
    let mut start = *base;

    for i in 0..ENTITY_COUNT {
        bases[i].set_as(&start);

        start.add(dir.x * SEGMENT_LENGTH, dir.y * SEGMENT_LENGTH);

        dirs[i] = *dir;
    }
}
