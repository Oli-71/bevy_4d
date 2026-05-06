use bevy::color::palettes::css::GOLD;
use bevy::prelude::ops::abs;
use bevy::prelude::*;

/// Sequence of atoms, represented by equal numbered positions and colors.
pub struct Atoms4D {
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
    pub is_4d_view: bool,
}

impl Scene4D {
    pub fn new() -> Self {
        let size = 3.0;
        let number_per_side = 16;
        let size_of_atom = size / number_per_side as f32;
        //empty scene
        let mut scene = Self {
            atoms: Atoms4D {
                positions: Vec::new(),
                colors: Vec::new(),
            },
            size_of_atom,
            objects: Vec::new(),
            is_4d_view: false,
        };
        //add some objects to the scene.
        scene.add_object(create_cube_3d(size_of_atom, number_per_side));

        //scene.add_object(create_cube_3d(size_of_atom, number_per_side));
        scene.add_object(create_heart_3d(size_of_atom, number_per_side * 3));

        //scene.add_object(create_cube_4d(size_of_atom, number_per_side));
        scene.add_object(create_cube_4d_surface(size_of_atom, number_per_side));

        scene
    }

    pub fn is_atom_visible(&self, position: Vec4) -> bool {
        let threshold = self.size_of_atom; // Atoms with |w| less than this threshold will be visible
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
    let wall_thickness = 1; // Number of atoms to keep on each side to create a hollow cube

    for x in start..=end {
        for y in start..=end {
            for z in start..=end {
                if x > start + wall_thickness
                    && x < end - wall_thickness
                    && y > start + wall_thickness
                    && y < end - wall_thickness
                    && z > start + wall_thickness
                    && z < end - wall_thickness
                {
                    continue; // Skip inner atoms to create a hollow cube
                }
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
    Atoms4D { positions, colors }
}

pub fn create_cube_4d(size_atom: f32, number_per_side: usize) -> Atoms4D {
    let capacity = number_per_side * number_per_side * number_per_side * number_per_side;
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

    let end = (number_per_side / 2) as i32;
    let start = -end;
    let spacing = 1.1 * size_atom;
    let wall_thickness = 1; // Number of atoms to keep on each side to create a hollow cube

    for x in start..=end {
        for y in start..=end {
            for z in start..=end {
                for w in start..=end {
                    if x > start + wall_thickness
                        && x < end - wall_thickness
                        && y > start + wall_thickness
                        && y < end - wall_thickness
                        && z > start + wall_thickness
                        && z < end - wall_thickness
                        && w > start + wall_thickness
                        && w < end - wall_thickness
                    {
                        continue; // Skip inner atoms to create a hollow cube
                    }
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
    Atoms4D { positions, colors }
}

pub fn create_cube_4d_surface(size_atom: f32, number_per_side: usize) -> Atoms4D {
    let capacity = number_per_side * number_per_side * number_per_side * 8;
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

    let end = (number_per_side / 2) as i32;
    let start = end;
    let spacing = 1.1 * size_atom;

    //1st and 2nd cube (w=0 and w=end)
    for x in start..=end {
        for y in start..=end {
            for z in start..=end {
                positions.push(Vec4::new(
                    x as f32 * spacing,
                    y as f32 * spacing,
                    z as f32 * spacing,
                    start as f32 * spacing,
                ));
                colors.push(Color::from(Srgba::rgb_u8(255, 0, 0))); //red for w=0
            }
        }
    }
    for x in start..=end {
        for y in start..=end {
            for z in start..=end {
                positions.push(Vec4::new(
                    x as f32 * spacing,
                    y as f32 * spacing,
                    z as f32 * spacing,
                    end as f32 * spacing,
                ));
                colors.push(Color::from(Srgba::rgb_u8(255, 255, 0))); //yellow for w=end
            }
        }
    }
    // 3rd and 4th cube (z=0 and z=end)
    for x in start..=end {
        for y in start..=end {
            for w in start..=end {
                positions.push(Vec4::new(
                    x as f32 * spacing,
                    y as f32 * spacing,
                    start as f32 * spacing,
                    w as f32 * spacing,
                ));
                colors.push(Color::from(Srgba::rgb_u8(0, 255, 0))); //green for z=0
            }
        }
    }
    for x in start..=end {
        for y in start..=end {
            for w in start..=end {
                positions.push(Vec4::new(
                    x as f32 * spacing,
                    y as f32 * spacing,
                    end as f32 * spacing,
                    w as f32 * spacing,
                ));
                colors.push(Color::from(Srgba::rgb_u8(0, 255, 255))); //cyan for z=end
            }
        }
    }
    // 5th and 6th cube (y=0 and y=end)
    for x in start..=end {
        for z in start..=end {
            for w in start..=end {
                positions.push(Vec4::new(
                    x as f32 * spacing,
                    start as f32 * spacing,
                    z as f32 * spacing,
                    w as f32 * spacing,
                ));
                colors.push(Color::from(Srgba::rgb_u8(0, 0, 255))); //blue for y=0
            }
        }
    }
    for x in start..=end {
        for z in start..=end {
            for w in start..=end {
                positions.push(Vec4::new(
                    x as f32 * spacing,
                    end as f32 * spacing,
                    z as f32 * spacing,
                    w as f32 * spacing,
                ));
                colors.push(Color::from(Srgba::rgb_u8(255, 0, 255))); //purple for y=end
            }
        }
    }
    // 7th and 8th cube (x=0 and x=end)
    for y in start..=end {
        for z in start..=end {
            for w in start..=end {
                positions.push(Vec4::new(
                    start as f32 * spacing,
                    y as f32 * spacing,
                    z as f32 * spacing,
                    w as f32 * spacing,
                ));
                colors.push(Color::from(Srgba::rgb_u8(255, 255, 255))); //white for x=0
            }
        }
    }
    for y in start..=end {
        for z in start..=end {
            for w in start..=end {
                positions.push(Vec4::new(
                    end as f32 * spacing,
                    y as f32 * spacing,
                    z as f32 * spacing,
                    w as f32 * spacing,
                ));
                colors.push(Color::from(Srgba::rgb_u8(0, 0, 0))); //black for x=end
            }
        }
    }

    Atoms4D { positions, colors }
}

fn create_heart_3d(size_atom: f32, number_per_side: usize) -> Atoms4D {
    let capacity = number_per_side * number_per_side * number_per_side;
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

    let end = (number_per_side / 2) as i32;
    let start = -end;
    let spacing = 1.1 * size_atom;
    let scale = 4.0 / (spacing * number_per_side as f32); // Scale the heart to fit within the cube

    for x in start..=end {
        for y in start..=end {
            for z in start..=end {
                let mut xx = x as f32 * spacing;
                let mut yy = y as f32 * spacing;
                let mut zz = z as f32 * spacing;
                let pos = Vec3::new(xx, yy, zz);
                // try later: Heart shape formula: (x^2 + (5/4)y^2 + z^2 - 1)^3 - x^2z^3 - (5/4)y^2z^3 <= 0
                // Heart shape formula: (x^2 + 2y^2 + z^2 - 1)^3 - x^2z^3 - (1/10)y^2z^3 = 0
                xx *= scale;
                let yyy = yy; // switch y and z to make the heart stand upright, instead of lying on its side
                yy = scale * zz;
                zz = scale * yyy;
                let part: f32 = xx.powi(2) + 2.0 * yy.powi(2) + zz.powi(2) - 1.0;
                let value =
                    part.powi(3) - xx.powi(2) * zz.powi(3) - (1.0 / 10.0) * yy.powi(2) * zz.powi(3);
                if value <= 0.0 {
                    positions.push(Vec4::new(pos.x, pos.y, pos.z, 0.0));
                    if x.abs() < 3 && y.abs() < 3 && z.abs() < 3 {
                        colors.push(Color::from(GOLD));
                    } else {
                        colors.push(Color::from(Srgba::rgb(1., 0., 0.)));
                    }
                }
            }
        }
    }
    Atoms4D { positions, colors }
}

pub fn create_sphere_4d(radius: f32, number_per_side: usize) -> Atoms4D {
    let capacity = number_per_side * number_per_side * number_per_side * number_per_side;
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

    let end = (number_per_side / 2) as i32;
    let start = -end;
    let spacing = 1.1 * radius / (number_per_side as f32 / 2.0);

    for x in start..=end {
        for y in start..=end {
            for z in start..=end {
                for w in start..=end {
                    let pos = Vec4::new(
                        x as f32 * spacing,
                        y as f32 * spacing,
                        z as f32 * spacing,
                        w as f32 * spacing,
                    );
                    if pos.length() <= radius {
                        positions.push(pos);
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
    }
    Atoms4D { positions, colors }
}

pub fn rotate_4d_xy(point: Vec4, angle: f32) -> Vec4 {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();

    Vec4::new(
        point.x,
        point.y,
        point.z * cos_angle - point.w * sin_angle,
        point.z * sin_angle + point.w * cos_angle,
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

pub fn rotate_4d_xw(point: Vec4, angle: f32) -> Vec4 {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();

    Vec4::new(
        point.x,
        point.y * cos_angle - point.z * sin_angle,
        point.y * sin_angle + point.z * cos_angle,
        point.w,
    )
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

// Rotates a point in 4D space around the YW plane by a given angle.
pub fn rotate_4d_yw(point: Vec4, angle: f32) -> Vec4 {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();

    Vec4::new(
        point.x * cos_angle - point.z * sin_angle,
        point.y,
        point.x * sin_angle + point.z * cos_angle,
        point.w,
    )
}

// Rotates a point in 4D space around the ZW plane by a given angle.
pub fn rotate_4d_zw(point: Vec4, angle: f32) -> Vec4 {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();

    Vec4::new(
        point.x * cos_angle - point.y * sin_angle,
        point.x * sin_angle + point.y * cos_angle,
        point.z,
        point.w,
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
    let rotation_z_matrix = Mat3::from_rotation_z(angle); // Rotate around the Z-axis
    let drag_matrix = cube_3d.drag_rotation(); // Get the drag rotation matrix for the 3D cube
    for index in cube_3d.range() {
        let position = &scene.atoms.positions[index];
        let mut rotated_position = rotation_z_matrix * vec3(position.x, position.y, position.z);
        rotated_position = drag_matrix * rotated_position; // Apply dragging transformation

        new_positions[index] = Vec4::new(
            rotated_position.x,
            rotated_position.y,
            rotated_position.z,
            position.w,
        );
    }

    // heart/cube3d (not rotated, only dragged)
    let cube_3d = &scene.objects[1];
    let drag_matrix = cube_3d.drag_rotation(); // Get the drag rotation matrix for the 3D cube
    for index in cube_3d.range() {
        let position = &scene.atoms.positions[index];
        let rotated_position = drag_matrix * vec3(position.x, position.y, position.z); // Apply dragging transformation

        new_positions[index] = Vec4::new(
            rotated_position.x - 6.0,
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
        let mut rotated_position = rotation_z_matrix * vec3(position.x, position.y, position.z);
        rotated_position = drag_matrix * rotated_position; // Apply dragging transformation

        new_positions[index] = Vec4::new(
            rotated_position.x + 6.0,
            rotated_position.y,
            rotated_position.z,
            position.w,
        );
    }

    // 4D transformation of the whole scene
    // applied on top of the local transformations above.
    // This will create a more interesting animation as the atoms will move in and out of the 3D space we can see.
    if scene.is_4d_view {
        for position in &mut new_positions {
            // Example of a simple 4D rotation around the XZ plane
            *position = rotate_4d_xz(*position, angle / 4.0);
        }
    }
    new_positions
}
