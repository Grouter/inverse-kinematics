use cgmath::conv::array4x4;
use cgmath::{Matrix4, Vector3};

use crate::graphics::Transform;
use crate::{ENTITY_COUNT, SEGMENT_LENGTH};
use crate::float2::Float2;

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

pub fn update_targets(targets: &mut [Float2], bases: &[Float2], main_target: &Float2) {
    targets[ENTITY_COUNT - 1] = *main_target;

    for i in 0..(ENTITY_COUNT - 1) {
        targets[i] = bases[i + 1];
    }
}

// TODO merge follow and base update? two cycles are redundant really...
pub fn follow(bases: &mut [Float2], dirs: &mut [Float2], targets: &mut [Float2]) {

    // Consider loop unrolling without targets...
    // Targets are basically redundant copies of bases
    for i in (1..ENTITY_COUNT).rev() {

        dirs[i] = targets[i] - bases[i];
        dirs[i].set_length(SEGMENT_LENGTH);

        bases[i] = targets[i] - dirs[i];

        targets[i - 1].set_as(&bases[i]);
    }

    dirs[0] = targets[0] - bases[0];
    dirs[0].set_length(SEGMENT_LENGTH);

    bases[0] = targets[0] - dirs[0];
}

pub fn update_bases(bases: &mut [Float2], dirs: &mut [Float2], main_base: &Float2) {
    bases[0] = *main_base;

    for i in 1..ENTITY_COUNT {
        dirs[i - 1] = bases[i] - bases[i - 1];
        dirs[i - 1].set_length(SEGMENT_LENGTH);

        bases[i] = bases[i - 1] + dirs[i - 1];
    }
}

pub fn update_transforms(transforms: &mut[Transform], bases: &[Float2], directions: &[Float2]) {

    for i in 0.. ENTITY_COUNT {
        // This is veeeery expensive... refactor this pls
        let rot = Matrix4::from_angle_z::<cgmath::Rad<f32>>(cgmath::Rad(directions[i].rotation()));
        let pos = Matrix4::from_translation(Vector3::new(bases[i].x, bases[i].y, 0.0));

        transforms[i].transform = array4x4(pos * rot);
    }
}