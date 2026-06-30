use crate::atoms::*;
use bevy::prelude::ops::abs;
use bevy::prelude::*;

use std::f32::consts::PI;

#[derive(PartialEq, Clone, Copy)]
pub enum Rotation4d {
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

#[derive(PartialEq, Clone, Copy)]
pub enum SceneType
{
    FlatlandSpaceland,
    Aquarium,
    EntangledPhotons,
}

/// The 4D scene, which contains a list of atoms and a list of objects.
pub struct Scene4D {
    pub atoms: Atoms4D, // all atoms in the scene

    pub spacing: f32,// ortho dist between neighbored atom's center points
    pub number_of_atoms_per_side: usize,// axis parallel bounding cube

    objects: Vec<Object4D>, // all objects in the scene

    pub scene_type: SceneType,
    // indices of objects in the scene, grouped by type for easier access.
    objects_spaceland: Vec<usize>,
    objects_flatland: Vec<usize>,
    objects_planar: Vec<usize>,
    objects_4d: Vec<usize>,

    pub is_projection_view: bool,
    pub is_high_dimension_view: bool,
    pub is_synchronized_drag: bool,
    start_time_high_dimension: f32,
    speed_3d_rotation: f32,
    higher_dimension_height: f32,
    angle_high_dimension: f32,

    pub rotation: Rotation4d,
    
    pub photon1: usize, //index in atoms
    pub photon2: usize,
}

impl Scene4D {

    /// Diameter of a sphere like atom
    pub fn size_of_atom(&self) -> f32 {
        1.5 * self.spacing // some overlap, more than sqrt(2) to avoid gaps for sphere like atoms 
    }
    /// compose a Scene from a Object4Ds
    pub fn new() -> Self {
        let size = 2.6;
        let number_per_side = 8;//16; // Total atoms will be number_per_side^4, 
        // so be careful with this number to avoid performance issues.
        let spacing = size / number_per_side as f32;

        //empty scene
        let mut scene = Self {
            atoms: Atoms4D {
                positions: Vec::new(),
                colors: Vec::new(),
            },
            spacing,
            number_of_atoms_per_side: number_per_side,
            objects: Vec::new(),
            scene_type: SceneType::FlatlandSpaceland,
            objects_spaceland: Vec::new(),
            objects_flatland: Vec::new(),
            objects_planar: Vec::new(),
            objects_4d: Vec::new(),
            is_high_dimension_view: false,
            is_projection_view: false,
            is_synchronized_drag: false,
            start_time_high_dimension: 0.0,
            speed_3d_rotation: 0.0, // default: no continuous rotation
            higher_dimension_height: 0.0,
            angle_high_dimension: 0.0,
            rotation: Rotation4d::Xz,
            photon1:0,
            photon2:0,
        };

        //add some objects to the scene.
        let heart_index = scene.add_object(create_heart_3d(spacing, number_per_side));
        let cube3d_index = scene.add_object(create_cube_surface(spacing, number_per_side));
        let cube4d_index = scene.add_object(create_cube_4d_surface(spacing, number_per_side));
        let cube4d_edges_index =
            scene.add_object(create_cube_4d_edges(spacing, number_per_side));
        let circle_index = scene.add_object(create_circle(spacing, number_per_side));
        let square_index = scene.add_object(create_square_surface(spacing, number_per_side));
        let cube_index = scene.add_object(create_cube_surface(spacing, number_per_side));
        let cube_edges_index = scene.add_object(create_cube_edges(spacing, number_per_side));

        scene.objects_spaceland = vec![heart_index, cube3d_index, cube4d_index, cube4d_edges_index];
        scene.objects_flatland = vec![circle_index, square_index, cube_index, cube_edges_index];
        scene.objects_planar = vec![circle_index, square_index];
        scene.objects_4d = vec![cube4d_index, cube4d_edges_index];

        scene
    }

    // complexer Spaceland to have more fun
    pub fn new_aquarium_scene() -> Self {
        let size = 10.;
        let number_per_side = 50; // be careful with this number to avoid performance issues.
        let spacing = size / number_per_side as f32;

        //empty scene
        let mut scene = Self {
            atoms: Atoms4D {
                positions: Vec::new(),
                colors: Vec::new(),
            },
            spacing,
            number_of_atoms_per_side: number_per_side,
            objects: Vec::new(),
            scene_type: SceneType::Aquarium,
            objects_spaceland: Vec::new(),
            objects_flatland: Vec::new(),
            objects_planar: Vec::new(),
            objects_4d: Vec::new(),
            is_high_dimension_view: false,
            is_projection_view: false,
            is_synchronized_drag: true,
            start_time_high_dimension: 0.0,
            speed_3d_rotation: 0.0, // default: no continuous rotation
            higher_dimension_height: 0.0,
            angle_high_dimension: 0.0,
            rotation: Rotation4d::Xz,
            photon1:0,
            photon2:0,
        };

        //add some objects to the scene (an aquarium, a hyper cube with hidden objects, a gold fish)
        let index_aquarium = scene.add_object(create_aquarium(spacing, number_per_side));

        let size_fish = number_per_side / 4;
        let number_per_side_hyper_cube = number_per_side / 3;
        let w_offset = spacing * (number_per_side_hyper_cube as f32) * 0.5;
        let index_cube_4d = scene.add_composed_object(vec![
                    create_cube_4d_edges(spacing, number_per_side_hyper_cube)
                        .translate(vec4(0.,0.,0.,w_offset)),
                    create_fish_3d(spacing, size_fish,Srgba::rgb_u8(50, 150, 255))// blueish
                        .translate(vec4(0.,0.,0.,w_offset)),
                    create_heart_3d(spacing, number_per_side / 4)
                        .translate(vec4(0.,0.,0.,0.5 * w_offset)),
                ]);
        
        let index_fish = scene.add_object(
            create_fish_3d(spacing, size_fish,Srgba::rgb_u8(255, 150, 50)));//orange

        scene.objects_spaceland = vec![index_aquarium, index_cube_4d, index_fish];
        scene
    }

     // fishes with entangled photons
    pub fn new_photon_scene() -> Self {
        let size = 10.;
        let number_per_side = 50; // be careful with this number to avoid performance issues.
        let spacing = size / number_per_side as f32;

        //empty scene
        let mut scene = Self {
            atoms: Atoms4D {
                positions: Vec::new(),
                colors: Vec::new(),
            },
            spacing,
            number_of_atoms_per_side: number_per_side,
            objects: Vec::new(),
            scene_type: SceneType::EntangledPhotons,
            objects_spaceland: Vec::new(),
            objects_flatland: Vec::new(),
            objects_planar: Vec::new(),
            objects_4d: Vec::new(),
            is_high_dimension_view: false,
            is_projection_view: false,
            is_synchronized_drag: true,
            start_time_high_dimension: 0.0,
            speed_3d_rotation: 0.0, // default: no continuous rotation
            higher_dimension_height: 0.0,
            angle_high_dimension: 0.0,
            rotation: Rotation4d::Xz,
            photon1:0,
            photon2:0,
        };

        //add some objects to the scene (an aquarium, two fishes)
        let index_aquarium = scene.add_object(create_aquarium(spacing, number_per_side));

        let size_fish = number_per_side / 4;
        let index_fish_blue = scene.add_object(
            create_fish_3d(spacing, size_fish,Srgba::rgb_u8(50, 50, 255))
        );//blue
        
        let index_fish_orange = scene.add_object(
            create_fish_3d(spacing, size_fish,Srgba::rgb_u8(255, 150, 50)));//orange

        //objects should be in the initial orientation 
        scene.photon1 = scene.find_atom_index_with_max_x(index_fish_orange);
        scene.photon2 = scene.find_atom_index_with_max_x(index_fish_blue);

        // so we can see the blue fish's photon from the beginning
        scene.transform_object(index_fish_blue, Mat4::from_rotation_y(-0.5));

        let index_wormhole = scene.add_object(
            create_wormhole(16));

        scene.objects_spaceland = vec![index_aquarium, index_fish_blue, index_fish_orange, index_wormhole];
        scene
    }

    pub fn is_atom_visible(&self, position: Vec4) -> bool {
        let threshold = 0.8 * self.size_of_atom(); // Atoms with |w| less than this threshold will be visible
        abs(position.w) < threshold
    }

    pub fn drag_object_from_atom(&mut self, atom_index: usize, delta: Vec2) {
        for object in &mut self.objects {
            if object.range().contains(&atom_index) {
                if self.is_synchronized_drag {
                    let current_drag = object.drag; // take reference from the first dragged object. Todo: last rotated object
                    for object in &mut self.objects {
                        object.drag = current_drag + delta * 0.02;
                    }
                    break;
                }
                object.drag += delta * 0.02;
                break;
            }
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

    fn add_composed_object(&mut self, new_atoms_vec: Vec<Atoms4D>) -> usize {
        let index = self.objects.len();
        let mut number_of_atoms = 0;
        let start_index = self.atoms.positions.len();
        for atoms in new_atoms_vec {
            number_of_atoms += atoms.positions.len();
            self.atoms.positions.extend(atoms.positions);
            self.atoms.colors.extend(atoms.colors);
        }
        self.objects.push(Object4D {
            start_index,
            number_of_atoms,
            drag: Vec2::ZERO,
        });
        index
    }

    pub fn force_high_dimension_view(&mut self, current_time: f32) {
        self.is_high_dimension_view = true;
        self.start_time_high_dimension = current_time;
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
        self.is_synchronized_drag = false;
        self.angle_high_dimension = 0.0;
        self.higher_dimension_height = 0.0;
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

    pub fn find_atom_index_with_max_x(&self, object_index: usize) -> usize {
        self.objects[object_index]
            .range()
            .max_by(|&a, &b| {
                self.atoms.positions[a]
                    .x
                    .partial_cmp(&self.atoms.positions[b].x)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(0)
    }

    pub fn transform_object(&mut self, object_index: usize, transform: Mat4) {
        for atom_index in self.objects[object_index].range() {
            let pos = &mut self.atoms.positions[atom_index];
            *pos = transform.mul_vec4(*pos);
        }
    }

    /// Transforms all atoms in the 4D scene. Returns the new positions.
    /// 0. in complex scene only (last state): Positioning of objects in the complex scene
    /// 1. local transformations
    ///    - continuous rotation (around y axis)
    ///    - dragging
    /// 2. generate 2D/3D Scenes: spreading center points on X axis
    /// 3. Hyper jump: global rotation of both scenes
    /// 4. optionally handle higher dimension:
    ///    - apply High-dimension offset (move atoms along w-axis for 3D Scene; move atoms along y-axis for 2D Scene) 
    ///     XOR
    ///    - Projection (move atoms to w=0 space for 3D Scene; move atoms to y=0 plane for 2D Scene)
    /// 5. separate 3D from 2D scene: move up on Y axis
    ///
    pub fn transform_scene(&mut self, time: f32, new_positions: &mut Vec<Vec4>) {
        new_positions.clear(); // (no malloc!)
    
        // cloning default positions
        // new_positions.reserve(self.atoms.positions.len());// only if size is changing very often
        new_positions.extend_from_slice(&self.atoms.positions);

        // Positioning of objects in the complex scenes
        if self.scene_type != SceneType::FlatlandSpaceland {
            let offset_cube_or_blue_fish = if self.scene_type == SceneType::Aquarium {
                vec4( 2.,0.,0.,0.)
            } else {// move the blue fish a bit
                vec4( 0.8,0.4 + 0.5 * time.sin(),0. + 0.1 * (0.8*(time-0.2)).sin(),0.)
            };
            // move the second object in spaceland (cube_4d/fish_blue) to the right in complex scene
            for index_atom in self.objects[self.objects_spaceland[1]].range() { 
                new_positions[index_atom] += offset_cube_or_blue_fish;
            }

            // move the third object in spaceland (fish) to the left in complex scene
            let offset = if self.scene_type == SceneType::Aquarium {2.5} else {2.7};
            let rotation_matrix = Mat3::from_rotation_y(1.0 * time);
            let local_rotation_matrix = Mat3::from_rotation_y(- PI/2.0);
            for index_atom in self.objects[self.objects_spaceland[2]].range() { 
                // align orange fish parallel to the glass of the aquarium
                new_positions[index_atom] = (local_rotation_matrix * vec3(new_positions[index_atom].x, new_positions[index_atom].y, new_positions[index_atom].z))
                    .extend(new_positions[index_atom].w);
                // a bit to the left, so it doesn't overlap with the cube_4d/blue fish
                new_positions[index_atom].x -= offset;
                // apply a slow continuous rotation to rotate through the aquarium.
                new_positions[index_atom] = (rotation_matrix * vec3(new_positions[index_atom].x, new_positions[index_atom].y, new_positions[index_atom].z))
                    .extend(new_positions[index_atom].w);
            }

            if self.scene_type == SceneType::EntangledPhotons {
                // place wormhole
                let from = new_positions[self.photon1];
                let to = new_positions[self.photon2];

                let dir = to-from;
                let step_number = self.objects[self.objects_spaceland[3]].number_of_atoms as f32;
                let step_length = dir.length()/step_number;
                let dir_normalized = dir.normalize();
                let mut i=0.;
                for index_atom in self.objects[self.objects_spaceland[3]].range() {
                    let mut pos = from + i * step_length * dir_normalized;
                    pos.w = new_positions[index_atom].w + self.size_of_atom();
                    new_positions[index_atom] = pos;
                    i += 1.;
                }
            }

        }

        let angle = time; // Rotation angle for the continuous rotation and the higher-dimension rotation
        let continuous_rotation_matrix = Mat3::from_rotation_y(self.speed_3d_rotation * angle); // Rotate around the Z-axis

        // Closure to apply local rotations (drag and continuous rotation) for each object based on its drag state.
        //  -For 3D objects, we apply a 3D rotation based on the drag.
        //  -For 2D objects, we apply a 2D rotation based on the drag.
        let mut apply_local_rotation = |objects: &Object4D, drag_matrix: Mat3| {
            for index in objects.range() {
                let position = new_positions[index];
                let rotated_position = 
                    drag_matrix
                    * (continuous_rotation_matrix 
                    * vec3(position.x, position.y, position.z));

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
            apply_local_rotation(object_3d, object_3d.drag_rotation_xy());
        }

        // local 2D-transformations for 2d objects
        for object_2d in self.objects_flatland() {
            apply_local_rotation(object_2d, object_2d.drag_rotation_x());
        }

        let dist_between_columns = 2.5; // Distance to move the objects apart
        if self.scene_type == SceneType::FlatlandSpaceland {
            // Placement of objects in the scenes (2D/3D)-> constructing two rows (both still on x-axis)
            let mut spread_on_x_axis = |objects: &[usize]| {
                let mut delta_x = -3. * dist_between_columns;
                for &obj_index in objects {
                    let object = &self.objects[obj_index];
                    for atom_index in object.range() {
                        new_positions[atom_index].x += delta_x;
                    }
                    delta_x += 2. * dist_between_columns; // next column
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
                        Rotation4d::Xy => rotate_4d_xy(&mut new_positions[atom_index], self.angle_high_dimension),
                        Rotation4d::Xz => rotate_4d_xz(&mut new_positions[atom_index], self.angle_high_dimension),
                        Rotation4d::Xw => rotate_4d_xw(&mut new_positions[atom_index], self.angle_high_dimension),
                        Rotation4d::Yz => rotate_4d_yz(&mut new_positions[atom_index], self.angle_high_dimension),
                        Rotation4d::Yw => rotate_4d_yw(&mut new_positions[atom_index], self.angle_high_dimension),
                        Rotation4d::Zw => rotate_4d_zw(&mut new_positions[atom_index], self.angle_high_dimension),
                    }
                }
            }
        }

        // Placement of objects in the complete scene.
        // optionally Projection to lower dimension.
        let hd_offset = 2.1 * self.higher_dimension_height; // Distance to move the objects in the higher dimension.
        // Spaceland row:
        // - project to w=0 space in projection view
        let dist_between_rows = if self.scene_type == SceneType::FlatlandSpaceland { 2. * dist_between_columns } else { 1. * dist_between_columns }; // Distance to move the 3D objects up on the Y-axis to separate from 2D objects in 2-row structure
        for object_3d in self.objects_spaceland() {
            for atom_index in object_3d.range() {
                new_positions[atom_index].y += dist_between_rows; // separate rows
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
