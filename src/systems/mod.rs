use vecmath::{Vector2, vec2_add, vec2_normalized, vec2_scale, vec2_sub};

use crate::graphics::Transform;
use crate::{ENTITY_COUNT, SEGMENT_LENGTH};

/*
* SEGMENT:
*            ______________________________
*           |                              |
*           | DIRECTION                    |
* BASE ---> O ========>                    |
*           |                              |
*           |______________________________|
*
*/

pub fn update_targets(targets: &mut [Vector2<f32>], bases: &[Vector2<f32>], main_target: &Vector2<f32>) {
    targets[ENTITY_COUNT - 1] = *main_target;

    for i in 0..(ENTITY_COUNT - 1) {
        targets[i] = bases[i + 1];
    }
}

// TODO merge follow and base update? two cycles are redundant really...
pub fn follow(bases: &mut [Vector2<f32>], dirs: &mut [Vector2<f32>], targets: &mut [Vector2<f32>]) {

    // Consider loop unrolling without targets...
    // Targets are basically redundant copies of bases
    for i in (1..ENTITY_COUNT).rev() {

        dirs[i] = vec2_sub(targets[i], bases[i]);
        dirs[i] = vec2_normalized(dirs[i]);
        dirs[i] = vec2_scale(dirs[i], SEGMENT_LENGTH);

        bases[i] = vec2_sub(targets[i], dirs[i]);

        targets[i - 1][0] = bases[i][0];
        targets[i - 1][1] = bases[i][1];
    }

    dirs[0] = vec2_sub(targets[0], bases[0]);
    dirs[0] = vec2_normalized(dirs[0]);
    dirs[0] = vec2_scale(dirs[0], SEGMENT_LENGTH);

    bases[0] = vec2_sub(targets[0], dirs[0]);
}

pub fn update_bases(bases: &mut [Vector2<f32>], dirs: &mut [Vector2<f32>], main_base: &Vector2<f32>) {
    bases[0] = *main_base;

    for i in 1..ENTITY_COUNT {

        dirs[i - 1] = vec2_sub(bases[i], bases[i - 1]);

        dirs[i - 1] = vec2_normalized(dirs[i - 1]);
        dirs[i - 1] = vec2_scale(dirs[i - 1], SEGMENT_LENGTH);

        bases[i] = vec2_add(bases[i - 1], dirs[i - 1]);
    }
}

pub fn update_transforms(transforms: &mut[Transform], bases: &[Vector2<f32>], directions: &[Vector2<f32>]) {
    for i in 0.. ENTITY_COUNT {
        let d = vec2_normalized(directions[i]);

        let cos = d[0];
        let sin = -d[1];

        let t = [
            [cos, -sin, 0.0, 0.0],
            [sin, cos, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [bases[i][0], bases[i][1], 0.0, 1.0],
        ];

        transforms[i].transform = t;
    }
}