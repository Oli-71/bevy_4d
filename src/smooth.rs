use bevy::prelude::*;

// components for 3D entities providing smooth movement and scaling

// add to entities which could be moved
#[derive(Component)]
pub struct PositionTarget {
    translation_target: Vec3,
    reached: bool,
}

impl PositionTarget {
    pub fn new(translation_target: Vec3) -> Self {
        Self {
            translation_target,
            reached: false,
        }
    }

    pub fn set_target(&mut self, translation_target: Vec3) {
        self.translation_target = translation_target;
        self.reached = false;
    }

    pub fn has_been_reached(&self) -> bool {
        self.reached
    }

    pub fn get_target(&self) -> Vec3 {
        self.translation_target
    }

    pub fn get_next_translation(&mut self, current: Vec3, delta_time: f32) -> Vec3 {
        const SPEED: f32 = 2.0;
        const TRANSLATION_EPSILON: f32 = 0.001;

        let mut motion_vec = self.translation_target - current;
        if motion_vec.length() > TRANSLATION_EPSILON {
            motion_vec *= SPEED * delta_time;
            current + motion_vec
        } else {
            self.reached = true;
            self.translation_target
        }
    }
}

// add to cameras which could change field of view
#[derive(Component)]
pub struct FovTarget {
    fov_target: f32,
    reached: bool,
}

impl FovTarget {
    pub fn new(fov_target: f32) -> Self {
        Self {
            fov_target,
            reached: false,
        }
    }

    pub fn set_target(&mut self, fov_target: f32) {
        self.fov_target = fov_target;
        self.reached = false;
    }

    pub fn has_been_reached(&self) -> bool {
        self.reached
    }

    pub fn get_next_fov(&mut self, current: f32, delta_time: f32) -> f32 {
        const SPEED: f32 = 2.0;
        const FOV_EPSILON: f32 = 0.001;

        let mut fov_delta = self.fov_target - current;
        if fov_delta.abs() > FOV_EPSILON {
            fov_delta *= SPEED * delta_time;
            current + fov_delta
        } else {
            self.reached = true;
            self.fov_target
        }
    }
}

// add to entities which could be scaled
#[derive(Component)]
pub struct ScaleTarget {
    scale_target: f32,
    reached: bool,
}

impl ScaleTarget {
    pub fn new(scale_target: f32) -> Self {
        Self {
            scale_target,
            reached: false,
        }
    }

    pub fn has_been_reached(&self) -> bool {
        self.reached
    }

    pub fn get_next_scale(&mut self, current: Vec3) -> Vec3 {
        const SCALE_EPSILON: f32 = 0.001;
        const SCALE_STEP: f32 = 0.1;

        let mut scale_delta = current - vec3(self.scale_target, self.scale_target, self.scale_target);
        if scale_delta.length() > SCALE_EPSILON {
            scale_delta *= SCALE_STEP;
            current - scale_delta
        } else {
            self.reached = true;
            vec3(self.scale_target, self.scale_target, self.scale_target)
        }
    }
}
