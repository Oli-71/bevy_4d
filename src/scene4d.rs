use bevy::prelude::*;


struct Object4D {
    start_index: usize,
    number_of_atoms: usize,
}
/// An object in 4D space, composed of atoms (positions and colors) and a size for the meshes representing the atoms
pub struct Scene4D {
    pub positions: Vec<Vec4>,
    pub colors: Vec<Color>,
    pub size_of_atom: f32,

    pub cube_3d: Object4D,
    pub cube_4d: Object4D,
}

impl Scene4D {
    pub fn new() -> Self {
        let s = create_cube_3d(0.4, 10);
        let number = s.positions.len();
        Self {
            positions: s.positions,
            colors: s.colors,
            size_of_atom: s.size_of_atom,
            cube_3d: Object4D {
                start_index: 0,
                number_of_atoms: number,
            },
            cube_4d: Object4D {
                start_index: 0,
                number_of_atoms: 0,
            },
        }
    }
}

/// Creates a cube composed of atoms.
/// `size` is the length of an edge of the cube, and `number_per_side` is how many smaller cubes there are along each edge.
/// objects are centered around the origin, so if number_per_side is 10, atoms will be placed from -5 to +5 on each axis.
pub fn create_cube_3d(size_atom: f32, number_per_side: usize) -> Scene4D {
    let end = (number_per_side / 2) as i32;
    let start = -end;
    let spacing = 1.1 * size_atom;
    let size_single_cube = size_atom;
    // some spacing between the atoms, so they don't visually merge into a big cube. You can adjust this factor as needed.

    let capacity = number_per_side * number_per_side * number_per_side;
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

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
    Scene4D {
        positions,
        colors,
        size_of_atom: size_single_cube,
        cube_3d: Object4D {
            start_index: 0,
            number_of_atoms: 0,
        },
        cube_4d: Object4D {
            start_index: 0,
            number_of_atoms: 0,
        },
    }
}

/// Rotates the positions of the atoms in 3D space around the Z-axis by the given angle (in radians).
pub fn rotate_3d(positions: &[Vec4], rotation_angle_z: f32) -> Vec<Vec4> {
    let angle = rotation_angle_z; // Rotation angle in radians
    let rotation_matrix = Mat3::from_rotation_z(angle); // Rotate around the Z-axis

    positions
        .iter()
        .map(|position| {
            let rotated_position = rotation_matrix * vec3(position.x, position.y, position.z);
            Vec4::new(
                rotated_position.x,
                rotated_position.y,
                rotated_position.z,
                position.w,
            )
        })
        .collect()
}
