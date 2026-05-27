use crate::atoms::*;
use bevy::prelude::ops::abs;
use bevy::prelude::*;

use std::f32::consts::PI;

/// Sequence of atoms, represented by equal numbered positions and colors.
// `Atoms4D` and helper `create_*` functions moved to `src/atoms.rs`.

#[derive(PartialEq, Clone)]
pub enum Rotation {
    Xy,
    Xz,
    Xw,
    Yz,
    Yw,
    Zw,
}
/// An object in the 4D scene, which consists of a sequence of atoms.
/// The `start_index` and `number_of_atoms` fields specify which atoms belong to this object.
struct Object4D {
    start_index: usize, // in Atoms4d
    number_of_atoms: usize,
    drag: Vec2, //for dragging the object with the mouse.
}

impl Object4D {
    fn range(&self) -> std::ops::Range<usize> {
        self.start_index..self.start_index + self.number_of_atoms
    }
    fn drag_rotation_xy(&self) -> Mat3 {
        Mat3::from_rotation_y(self.drag.x) * Mat3::from_rotation_x(self.drag.y)
    }
    fn drag_rotation_x(&self) -> Mat3 {
        Mat3::from_rotation_y(self.drag.x)
    }
}

/// The 4D scene, which contains a list of atoms and a list of objects.
pub struct Scene4D {
    pub atoms: Atoms4D, // all atoms in the scene

    pub size_of_atom: f32,
    pub number_of_atoms_per_side: usize,

    objects: Vec<Object4D>, // all objects in the scene

    pub is_2row_structure: bool,
    // indices of objects in the scene, grouped by type for easier access.
    objects_spaceland: Vec<usize>,
    objects_flatland: Vec<usize>,
    objects_planar: Vec<usize>,
    objects_4d: Vec<usize>,

    pub is_projection_view: bool,
    pub is_high_dimension_view: bool,
    start_time_high_dimension: f32,
    speed_3d_rotation: f32,
    higher_dimension_height: f32,
    angle_high_dimension: f32,

    pub rotation: Rotation,
}

impl Scene4D {
    /// compose a Scene from a Object4Ds
    pub fn new() -> Self {
        let size = 2.6;
        let number_per_side = 8; //16; // Total atoms will be number_per_side^4, 
        // so be careful with this number to avoid performance issues.
        let size_of_atom = size / number_per_side as f32;

        //empty scene
        let mut scene = Self {
            atoms: Atoms4D {
                positions: Vec::new(),
                colors: Vec::new(),
            },
            size_of_atom,
            number_of_atoms_per_side: number_per_side,
            objects: Vec::new(),
            is_2row_structure: true,
            objects_spaceland: Vec::new(),
            objects_flatland: Vec::new(),
            objects_planar: Vec::new(),
            objects_4d: Vec::new(),
            is_high_dimension_view: false,
            is_projection_view: false,
            start_time_high_dimension: 0.0,
            speed_3d_rotation: 0.0, // default: no continuous rotation
            higher_dimension_height: 0.0,
            angle_high_dimension: 0.0,
            rotation: Rotation::Xy,
        };

        //add some objects to the scene.
        let heart_index = scene.add_object(create_heart_3d(size_of_atom, number_per_side));
        //let heart_index = scene.add_object(create_atoms_from_file(size_of_atom,"C:/Dev/bevy/bevy_4d/src/fruits.txt".to_string()));

        let cube3d_index = scene.add_object(create_cube_surface(size_of_atom, number_per_side));
        let cube4d_index = scene.add_object(create_cube_4d_surface(size_of_atom, number_per_side));

        //let cube4d_edges_index = scene.add_object(create_tripod_4d(size_of_atom, number_per_side));
        let cube4d_edges_index =
            scene.add_object(create_cube_4d_edges(size_of_atom, number_per_side));

        let circle_index = scene.add_object(create_circle(size_of_atom, number_per_side));
        let square_index = scene.add_object(create_square_surface(size_of_atom, number_per_side));
        let cube_index = scene.add_object(create_cube_surface(size_of_atom, number_per_side));
        let cube_edges_index = scene.add_object(create_cube_edges(size_of_atom, number_per_side));

        scene.objects_spaceland = vec![heart_index, cube3d_index, cube4d_index, cube4d_edges_index];
        scene.objects_flatland = vec![circle_index, square_index, cube_index, cube_edges_index];
        scene.objects_planar = vec![circle_index, square_index];
        scene.objects_4d = vec![cube4d_index, cube4d_edges_index];

        scene
    }

    // complexer Spaceland to have more fun
    pub fn new_complex_scene() -> Self {
        let size = 5.;
        let number_per_side = 50; // be careful with this number to avoid performance issues.
        let size_of_atom = size / number_per_side as f32;

        //empty scene
        let mut scene = Self {
            atoms: Atoms4D {
                positions: Vec::new(),
                colors: Vec::new(),
            },
            size_of_atom,
            number_of_atoms_per_side: number_per_side,
            objects: Vec::new(),
            is_2row_structure: false,
            objects_spaceland: Vec::new(),
            objects_flatland: Vec::new(),
            objects_planar: Vec::new(),
            objects_4d: Vec::new(),
            is_high_dimension_view: false,
            is_projection_view: false,
            start_time_high_dimension: 0.0,
            speed_3d_rotation: 0.0, // default: no continuous rotation
            higher_dimension_height: 0.0,
            angle_high_dimension: 0.0,
            rotation: Rotation::Xy,
        };

        //add some objects to the scene.
        //scene.add_object(create_cube_surface_colorful(size_of_atom, number_per_side));
        //let heart_index = scene.add_object(create_heart_3d(size_of_atom, number_per_side));
        //let tripod_index = scene.add_object(create_tripod_4d(size_of_atom, number_per_side));
        let cube4d_index = scene.add_object(create_cube_4d_edges(size_of_atom, number_per_side));

        scene.objects_spaceland = vec![/*heart_index, tripod_index,*/ cube4d_index];

        scene
    }

    pub fn is_atom_visible(&self, position: Vec4) -> bool {
        let threshold = 0.8 * self.size_of_atom; // Atoms with |w| less than this threshold will be visible
        abs(position.w) < threshold
    }

    pub fn drag_object_from_atom(&mut self, atom_index: usize, delta: Vec2) {
        for object in &mut self.objects {
            if object.range().contains(&atom_index) {
                object.drag += delta * 0.02;
                break;
            }
        }
    }

    pub fn drag_all_objects(&mut self, delta: Vec2) {
        let current_drag = self.objects[0].drag; // take reference from first object. Todo: last rotated object
        for object in &mut self.objects {
            object.drag = current_drag + delta * 0.02;
        }
    }

    fn add_object(&mut self, new_atoms: Atoms4D) -> usize {
        let index = self.objects.len();
        self.objects.push(Object4D {
            start_index: self.atoms.positions.len(),
            number_of_atoms: new_atoms.positions.len(),
            drag: Vec2::ZERO,
        });
        self.atoms.positions.extend(new_atoms.positions);
        self.atoms.colors.extend(new_atoms.colors);
        index
    }

    pub fn toggle_high_dimension_view(&mut self, current_time: f32) {
        self.is_high_dimension_view = !self.is_high_dimension_view;
        if self.is_high_dimension_view {
            self.start_time_high_dimension = current_time;
        } else {
            self.angle_high_dimension = 0.0;
        }
    }

    pub fn toggle_projection_view(&mut self) {
        self.is_projection_view = !self.is_projection_view;
    }

    pub fn reset_view(&mut self) {
        self.is_projection_view = false;
        self.is_high_dimension_view = false;
        self.angle_high_dimension = 0.0;
    }

    fn objects_flatland(&self) -> impl Iterator<Item = &Object4D> {
        self.objects_flatland
            .iter()
            .map(move |&index| &self.objects[index])
    }

    fn objects_spaceland(&self) -> impl Iterator<Item = &Object4D> {
        self.objects_spaceland
            .iter()
            .map(move |&index| &self.objects[index])
    }

    pub fn is_2d(&self, atom_index: usize) -> bool {
        self.objects_flatland
            .iter()
            .any(|&index| self.objects[index].range().contains(&atom_index))
    }

    pub fn is_planar(&self, atom_index: usize) -> bool {
        self.objects_planar
            .iter()
            .any(|&index| self.objects[index].range().contains(&atom_index))
    }

    pub fn is_4d(&self, atom_index: usize) -> bool {
        self.objects_4d
            .iter()
            .any(|&index| self.objects[index].range().contains(&atom_index))
    }

    fn time_in_high_dimension_view(&self, current_time: f32) -> f32 {
        if self.is_high_dimension_view {
            current_time - self.start_time_high_dimension
        } else {
            0.0
        }
    }

    pub fn adjust_3d_rotation_speed(&mut self, speed: f32) {
        self.speed_3d_rotation = (speed).clamp(0.0, 1.0);
    }

    pub fn adjust_higher_dimension_height(&mut self, higher_dimension_height: f32) {
        self.higher_dimension_height = higher_dimension_height.clamp(-1.0, 0.0);
    }

    pub fn get_angle_high_dimension(&self) -> f32 {
        self.angle_high_dimension
    }

    /// Transforms all atoms in the 4D scene. Returns the new positions.
    /// 1. local transformations
    ///    - continuous rotation
    ///    - dragging
    /// 2. generate 2D/3D Scenes: spreading center points on X axis
    /// 3. Hyper jump: global rotation of both scenes
    /// 4. optionally handle higher dimension:
    ///    - apply High-dimension offset xor
    ///    - Projection
    /// 5. separate 3D from 2D scene: move up on Y axis
    ///
    pub fn transform_scene(&mut self, time: f32) -> Vec<Vec4> {
        let mut new_positions = self.atoms.positions.clone();

        let angle = time; // Rotation angle for the continuous rotation and the higher-dimension rotation
        let continuous_rotation_matrix = Mat3::from_rotation_y(self.speed_3d_rotation * angle); // Rotate around the Z-axis

        // Closure to apply local transformations for each object based on its drag state.
        //  -For 3D objects, we apply a 3D rotation based on the drag.
        //  -For 2D objects, we apply a 2D rotation based on the drag.
        let mut apply_local_transform = |objects: &Object4D, drag_matrix: Mat3| {
            for index in objects.range() {
                let position = &self.atoms.positions[index];
                let rotated_position = drag_matrix
                    * (continuous_rotation_matrix * vec3(position.x, position.y, position.z));

                new_positions[index] = Vec4::new(
                    rotated_position.x,
                    rotated_position.y,
                    rotated_position.z,
                    position.w,
                );
            }
        };

        // local 3D-transformations for 3d objects
        for object_3d in self.objects_spaceland() {
            apply_local_transform(object_3d, object_3d.drag_rotation_xy());
        }

        // local 2D-transformations for 2d objects
        for object_2d in self.objects_flatland() {
            apply_local_transform(object_2d, object_2d.drag_rotation_x());
        }

        let x_offset = 2.5; // Distance to move the objects apart
        if self.is_2row_structure {
            // Placement of objects in the scenes (2D/3D)-> constructing two rows (both still on x-axis)
            let mut spread_on_x_axis = |objects: &[usize]| {
                let mut delta_x = -3. * x_offset;
                for &obj_index in objects {
                    let object = &self.objects[obj_index];
                    for atom_index in object.range() {
                        new_positions[atom_index].x += delta_x;
                    }
                    delta_x += 2. * x_offset; // next column
                }
            };
            spread_on_x_axis(&self.objects_flatland);
            spread_on_x_axis(&self.objects_spaceland);
        }

        // Global Higher Dimension Transformation
        // applied on top of the local transformations above.
        // Atoms will move in and out of the visible Flatland and Spaceland.
        if self.is_high_dimension_view {
            self.angle_high_dimension = self.time_in_high_dimension_view(time) / 4.0;
            self.angle_high_dimension %= 2.0 * PI; // clap to [0..2pi]

            // a 3D rotation (y is changing)
            for object_2d in self.objects_flatland() {
                for atom_index in object_2d.range() {
                    rotate_4d_xw(&mut new_positions[atom_index], self.angle_high_dimension);
                }
            }

            // a 4D rotation (w is changing)
            for object_3d in self.objects_spaceland() {
                for atom_index in object_3d.range() {
                    match self.rotation {
                        Rotation::Xy => rotate_4d_xy(&mut new_positions[atom_index], self.angle_high_dimension),
                        Rotation::Xz => rotate_4d_xz(&mut new_positions[atom_index], self.angle_high_dimension),
                        Rotation::Xw => rotate_4d_xw(&mut new_positions[atom_index], self.angle_high_dimension),
                        Rotation::Yz => rotate_4d_yz(&mut new_positions[atom_index], self.angle_high_dimension),
                        Rotation::Yw => rotate_4d_yw(&mut new_positions[atom_index], self.angle_high_dimension),
                        Rotation::Zw => rotate_4d_zw(&mut new_positions[atom_index], self.angle_high_dimension),
                    }
                }
            }
        }

        // Placement of objects in the complete scene.
        // optionally Projection to lower dimension.
        let hd_offset = 2.1 * self.higher_dimension_height; // Distance to move the objects in the higher dimension.
        // Spaceland row:
        // - project to w=0 space in projection view
        let y_offset = 2. * x_offset;
        for object_3d in self.objects_spaceland() {
            for atom_index in object_3d.range() {
                new_positions[atom_index].y += y_offset; // separate rows
                if self.is_projection_view {
                    new_positions[atom_index].w = 0.; // move all atoms to the same w level in projection view 
                } else {
                    new_positions[atom_index].w += hd_offset; // move out of view
                }
            }
        }

        // Flatland row:
        // - project to y=0 plane in projection view
        for object_2d in self.objects_flatland() {
            for atom_index in object_2d.range() {
                if self.is_projection_view {
                    new_positions[atom_index].y = 0.; // move all atoms to the same y level in projection view 
                } else {
                    new_positions[atom_index].y += hd_offset; // move out of view
                }
            }
        }

        new_positions
    }
}

// Helper functions producing `Atoms4D` were moved to `src/atoms.rs` and are imported at the top of this file.

// 4D rotation functions for different planes.
fn rotate_4d_xy(point: &mut Vec4, angle: f32) {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();

    let new_z = point.z * cos_angle - point.w * sin_angle;
    let new_w = point.z * sin_angle + point.w * cos_angle;

    point.z = new_z;
    point.w = new_w;
}

fn rotate_4d_xz(point: &mut Vec4, angle: f32) {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();

    let new_y = point.y * cos_angle - point.w * sin_angle;
    let new_w = point.y * sin_angle + point.w * cos_angle;

    point.y = new_y;
    point.w = new_w;
}

fn rotate_4d_xw(point: &mut Vec4, angle: f32) {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();

    let new_y = point.y * cos_angle - point.z * sin_angle;
    let new_z = point.y * sin_angle + point.z * cos_angle;

    point.y = new_y;
    point.z = new_z;
}

fn rotate_4d_yz(point: &mut Vec4, angle: f32) {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();

    let new_x = point.x * cos_angle - point.w * sin_angle;
    let new_w = point.x * sin_angle + point.w * cos_angle;

    point.x = new_x;
    point.w = new_w;
}

fn rotate_4d_yw(point: &mut Vec4, angle: f32) {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();

    let new_x = point.x * cos_angle - point.z * sin_angle;
    let new_z = point.x * sin_angle + point.z * cos_angle;

    point.x = new_x;
    point.z = new_z;
}

fn rotate_4d_zw(point: &mut Vec4, angle: f32) {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();

    let new_x = point.x * cos_angle - point.y * sin_angle;
    let new_y = point.x * sin_angle + point.y * cos_angle;

    point.x = new_x;
    point.y = new_y;
}
