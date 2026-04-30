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
        scene.add_object(create_cube_4d(scene.size_of_atom, 6));

        scene
    }

    pub fn is_atom_visible(&self, index: usize) -> bool {
        let position = &self.atoms.positions[index];
        position.w < self.size_of_atom && position.w > -self.size_of_atom
    }

    fn add_object(&mut self, new_atoms: Atoms4D) {
        self.objects.push(Object4D {
            start_index: self.atoms.positions.len(),
            number_of_atoms: new_atoms.positions.len(),
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

/// transforms all atoms in the 4D scene.
/// Currently, this function applies a simple rotation in 3D space, but you can replace it with any transformation you like (e.g., a 4D rotation).
pub fn transform_scene(scene: &Scene4D, time: f32) -> Vec<Vec4> {
    let mut new_positions = scene.atoms.positions.clone();

    //local movements
    let angle = time; // Rotation angle in radians
    let rotation_matrix = Mat3::from_rotation_z(angle); // Rotate around the Z-axis

    let cube_3d = &scene.objects[0];
    for index in cube_3d.range() {  
        let position = &scene.atoms.positions[index];
        let rotated_position = rotation_matrix * vec3(position.x, position.y, position.z);
        new_positions[index] = Vec4::new(
            rotated_position.x,
            rotated_position.y,
            rotated_position.z,
            position.w,
        );
    }

    let rotation_matrix_x = Mat3::from_rotation_x(angle/2.0); // Rotate around the X-axis
    let cube_4d = &scene.objects[1];
    for index in cube_4d.range() {  
        let position = &scene.atoms.positions[index];
        let rotated_position = rotation_matrix_x * vec3(position.x, position.y, position.z);
        new_positions[index] = Vec4::new(
            rotated_position.x + 4.0,
            rotated_position.y + 4.0,
            rotated_position.z + 4.0,
            position.w + 4.0,
        );
    }

    //transformation to 4d
    
    //todo: implement a 4D rotation and apply it here on top of of the 3D rotation above. 
    //This will create a more interesting animation as the atoms will move in and out of the 3D space we can see.
    new_positions
}
