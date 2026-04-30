use bevy::prelude::*;

/// Sequence of atoms, represented by equal numbered positions and colors.
pub struct Atoms4D{    
    pub positions: Vec<Vec4>,
    pub colors: Vec<Color>,
}

/// An object in the 4D scene, which consists of a sequence of atoms. 
/// The `start_index` and `number_of_atoms` fields specify which atoms belong to this object.
struct Object4D {
    start_index: usize,
    number_of_atoms: usize,
    drag: Vec2, //for dragging the object with the mouse.
}

impl Object4D {
    fn start(&self) -> usize {
        self.start_index
    }
    fn end(&self) -> usize {
        self.start_index + self.number_of_atoms
    }
    fn range(&self) -> std::ops::Range<usize> {
        self.start()..self.end()
    }
    fn drag_rotation(&self) -> Mat3 {
        Mat3::from_rotation_y(self.drag.x) * Mat3::from_rotation_x(self.drag.y)
    }
}

/// The 4D scene, which contains a list of atoms and a list of objects.
pub struct Scene4D {
    pub atoms: Atoms4D,
    pub size_of_atom: f32,
    objects: Vec<Object4D>,
}

impl Scene4D {
    pub fn new() -> Self {
        //empty scene 
        let mut scene = Self {
            atoms: Atoms4D {
                positions: Vec::new(),
                colors: Vec::new(),
            },
            size_of_atom: 0.3,
            objects: Vec::new(),
        };
        //add some objects to the scene. 
        scene.add_object(create_cube_3d(scene.size_of_atom, 10));
        scene.add_object(create_cube_3d(scene.size_of_atom, 10));
        scene.add_object(create_cube_4d(scene.size_of_atom, 10));

        scene
    }

    pub fn is_atom_visible(&self, index: usize) -> bool {
        let position = &self.atoms.positions[index];
        position.w < self.size_of_atom && position.w > -self.size_of_atom
    }

    pub fn drag_object_from_atom(&mut self, atom_index: usize, delta: Vec2) {
        for object in &mut self.objects {
            if object.range().contains(&atom_index) {
                object.drag += delta * 0.02;
                break;
            }
        }
    }

    fn add_object(&mut self, new_atoms: Atoms4D) {
        self.objects.push(Object4D {
            start_index: self.atoms.positions.len(),
            number_of_atoms: new_atoms.positions.len(),
            drag: Vec2::ZERO,
        });
        self.atoms.positions.extend(new_atoms.positions);
        self.atoms.colors.extend(new_atoms.colors);
    }   
}

/// Creates a cube composed of atoms.
/// `size` is the length of an edge of the cube, and `number_per_side` is how many smaller cubes there are along each edge.
/// objects are centered around the origin, so if number_per_side is 10, atoms will be placed from -5 to +5 on each axis.
pub fn create_cube_3d(size_atom: f32, number_per_side: usize) -> Atoms4D {
    let capacity = number_per_side * number_per_side * number_per_side;
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

    let end = (number_per_side / 2) as i32;
    let start = -end;
    let spacing = 1.1 * size_atom;

    for x in start..end {
        for y in start..end {
            for z in start..end {
                positions.push(Vec4::new(
                    x as f32 * spacing,
                    y as f32 * spacing,
                    z as f32 * spacing,
                    0.0,
                ));
                colors.push(Color::from(Srgba::rgb(
                    (x + end) as f32 / number_per_side as f32,
                    (y + end) as f32 / number_per_side as f32,
                    (z + end) as f32 / number_per_side as f32,
                )));
            }
        }
    }
    Atoms4D {
        positions,
        colors,
    }
}

pub fn create_cube_4d(size_atom: f32, number_per_side: usize) -> Atoms4D {
    let capacity = number_per_side * number_per_side * number_per_side * number_per_side;
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

    let end = (number_per_side / 2) as i32;
    let start = -end;
    let spacing = 1.1 * size_atom;

    for x in start..end {
        for y in start..end {
            for z in start..end {
                for w in start..end {
                    positions.push(Vec4::new(
                        x as f32 * spacing,
                        y as f32 * spacing,
                        z as f32 * spacing,
                        w as f32 * spacing,
                    ));
                    colors.push(Color::from(Srgba::rgba_u8(
                        ((x + end) * 255 / number_per_side as i32) as u8,
                        ((y + end) * 255 / number_per_side as i32) as u8,
                        ((z + end) * 255 / number_per_side as i32) as u8,
                        ((w + end) * 255 / number_per_side as i32) as u8,
                    )));
                }
            }
        }
    }
    Atoms4D {
        positions,
        colors,
    }
}

// Rotates a point in 4D space around the YZ plane by a given angle.
pub fn rotate_4d_yz(point: Vec4, angle: f32) -> Vec4 {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();

    Vec4::new(
        point.x * cos_angle - point.w * sin_angle,
        point.y,
        point.z,
        point.x * sin_angle + point.w * cos_angle,
    )
}

// Rotates a point in 4D space around the XZ plane by a given angle.
pub fn rotate_4d_xz(point: Vec4, angle: f32) -> Vec4 {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();

    Vec4::new(
        point.x,
        point.y * cos_angle - point.w * sin_angle,
        point.z,
        point.y * sin_angle + point.w * cos_angle,
    )
}

/// transforms all atoms in the 4D scene.
/// Currently, this function applies a simple rotation in 3D space, but you can replace it with any transformation you like (e.g., a 4D rotation).
pub fn transform_scene(scene: &Scene4D, time: f32) -> Vec<Vec4> {
    let mut new_positions = scene.atoms.positions.clone();

    //local movements
    let angle = time; // Rotation angle in radians
    
    // cube3d
    let cube_3d = &scene.objects[0];
    let rotation_matrix = Mat3::from_rotation_z(angle); // Rotate around the Z-axis
    let drag_matrix = cube_3d.drag_rotation(); // Get the drag rotation matrix for the 3D cube
    for index in cube_3d.range() {  
        let position = &scene.atoms.positions[index];
        let mut rotated_position = rotation_matrix * vec3(position.x, position.y, position.z);
        rotated_position = drag_matrix * rotated_position; // Apply dragging transformation

        new_positions[index] = Vec4::new(
            rotated_position.x,
            rotated_position.y,
            rotated_position.z,
            position.w,
        );
    }

    // cube3d (not rotated, only dragged)
    let cube_3d = &scene.objects[1];
    let drag_matrix = cube_3d.drag_rotation(); // Get the drag rotation matrix for the 3D cube
    for index in cube_3d.range() {  
        let position = &scene.atoms.positions[index];
        let rotated_position = drag_matrix * vec3(position.x, position.y, position.z); // Apply dragging transformation

        new_positions[index] = Vec4::new(
            rotated_position.x-6.0,
            rotated_position.y,
            rotated_position.z,
            position.w,
        );
    }

    // cube4d
    let cube_4d = &scene.objects[2];
    let drag_matrix = cube_4d.drag_rotation(); // Get the drag rotation matrix for the 4D cube
    for index in cube_4d.range() {  
        let position = &scene.atoms.positions[index];

        let mut rotated_position4d = rotate_4d_yz(*position, angle/2.0);
        rotated_position4d = rotate_4d_xz(rotated_position4d, angle/2.0);

        let mut position3d = vec3(rotated_position4d.x, rotated_position4d.y, rotated_position4d.z);
        position3d = drag_matrix * position3d; // Apply dragging transformation
        
        new_positions[index] = Vec4::new(
            position3d.x + 6.0,
            position3d.y + 0.0,
            position3d.z + 0.0,
            rotated_position4d.w,
        );
    }

    //transformation to 4d
    
    //todo: implement a 4D rotation and apply it here on top of of the 3D rotation above. 
    //This will create a more interesting animation as the atoms will move in and out of the 3D space we can see.
    new_positions
}
