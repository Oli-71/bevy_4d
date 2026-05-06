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
    
    is_4d_view: bool,
    start_time_4d: f32,
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
            start_time_4d: 0.0,
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

    fn add_object(&mut self, new_atoms: Atoms4D) {
        self.objects.push(Object4D {
            start_index: self.atoms.positions.len(),
            number_of_atoms: new_atoms.positions.len(),
            drag: Vec2::ZERO,
        });
        self.atoms.positions.extend(new_atoms.positions);
        self.atoms.colors.extend(new_atoms.colors);
    }

    pub fn toggle_4d_view(&mut self, current_time: f32) {
        self.is_4d_view = !self.is_4d_view;
        if self.is_4d_view {
            self.start_time_4d = current_time;
        }
    }

    pub fn is_4d_view(&self) -> bool {
        self.is_4d_view
    }

    pub fn time_in_4d_view(&self, current_time: f32) -> f32 {
        if self.is_4d_view {
            current_time - self.start_time_4d
        } else {
            0.0
        }
    }

    /// transforms all atoms in the 4D scene.
    pub fn transform_scene(&self, time: f32) -> Vec<Vec4> {
        let mut new_positions = self.atoms.positions.clone();

        //local movements
        let angle = time; // Rotation angle in radians
        let offset = 6.0; // Distance to offset the left and right objects from the center

        // left: heart/cube3d (not rotated, only dragged)
        let left_object = &self.objects[1];
        let drag_matrix = left_object.drag_rotation(); // Get the drag rotation matrix for the left object
        for index in left_object.range() {
            let position = &self.atoms.positions[index];
            let rotated_position = drag_matrix * vec3(position.x, position.y, position.z); // Apply dragging transformation

            new_positions[index] = Vec4::new(
                rotated_position.x - offset,
                rotated_position.y,
                rotated_position.z,
                position.w,
            );
        }

        // middle: cube3d
        let middle_object = &self.objects[0];
        let rotation_z_matrix = Mat3::from_rotation_z(angle); // Rotate around the Z-axis
        let drag_matrix = middle_object.drag_rotation(); // Get the drag rotation matrix for the 3D cube
        for index in middle_object.range() {
            let position = &self.atoms.positions[index];
            let mut rotated_position = rotation_z_matrix * vec3(position.x, position.y, position.z);
            rotated_position = drag_matrix * rotated_position; // Apply dragging transformation

            new_positions[index] = Vec4::new(
                rotated_position.x,
                rotated_position.y,
                rotated_position.z,
                position.w,
            );
        }

        // right: cube4d
        let right_object = &self.objects[2];
        let drag_matrix = right_object.drag_rotation(); // Get the drag rotation matrix for the 4D cube
        for index in right_object.range() {
            let position = &self.atoms.positions[index];
            let mut rotated_position = rotation_z_matrix * vec3(position.x, position.y, position.z);
            rotated_position = drag_matrix * rotated_position; // Apply dragging transformation

            new_positions[index] = Vec4::new(
                rotated_position.x + offset,
                rotated_position.y,
                rotated_position.z,
                position.w,
            );
        }

        // 4D transformation of the whole scene
        // applied on top of the local transformations above.
        // Atoms will move in and out of the 3D space we can see.
        if self.is_4d_view() {
            for position in &mut new_positions {
                *position = rotate_4d_xz(*position, self.time_in_4d_view(time) / 4.0);
            }
        }
        new_positions
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

    let end = (number_per_side / 2) as i32 - 1;
    let start = -end;

    let spacing = 1.1 * size_atom;

    let low = (start - 1) as f32 * spacing;// Position for the "low" side of the cube (e.g., w = low)
    let high = (end + 1) as f32 * spacing;// Position for the "high" side of the cube (e.g., w = high)

    // Create atoms for the 8 faces of the 4D cube (each face is a 3D cube in the 4D space).
    for a in start..=end {
        for b in start..=end {
            for c in start..=end {
                let aa = a as f32 * spacing;
                let bb = b as f32 * spacing;
                let cc = c as f32 * spacing;

                positions.push(Vec4::new(aa, bb, cc, low));
                colors.push(Color::from(Srgba::rgb_u8(255, 0, 0))); //red for w=low

                positions.push(Vec4::new(aa, bb, cc, high));
                colors.push(Color::from(Srgba::rgb_u8(255, 255, 0))); //yellow for w=high

                positions.push(Vec4::new(aa, bb, low, cc));
                colors.push(Color::from(Srgba::rgb_u8(0, 255, 0))); //green for z=low

                positions.push(Vec4::new(aa, bb, high, cc));
                colors.push(Color::from(Srgba::rgb_u8(0, 255, 255))); //cyan for z=high

                positions.push(Vec4::new(aa, low, bb, cc));
                colors.push(Color::from(Srgba::rgb_u8(0, 0, 255))); //blue for y=low

                positions.push(Vec4::new(aa, high, bb, cc));
                colors.push(Color::from(Srgba::rgb_u8(255, 0, 255))); //purple for y=high

                positions.push(Vec4::new(low, aa, bb, cc));
                colors.push(Color::from(Srgba::rgb_u8(255, 255, 255))); //white for x=low

                positions.push(Vec4::new(high, aa, bb, cc));
                colors.push(Color::from(Srgba::rgb_u8(0, 0, 0))); //black for x=high
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

fn rotate_4d_xy(point: Vec4, angle: f32) -> Vec4 {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();

    Vec4::new(
        point.x,
        point.y,
        point.z * cos_angle - point.w * sin_angle,
        point.z * sin_angle + point.w * cos_angle,
    )
}

fn rotate_4d_xz(point: Vec4, angle: f32) -> Vec4 {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();

    Vec4::new(
        point.x,
        point.y * cos_angle - point.w * sin_angle,
        point.z,
        point.y * sin_angle + point.w * cos_angle,
    )
}

fn rotate_4d_xw(point: Vec4, angle: f32) -> Vec4 {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();

    Vec4::new(
        point.x,
        point.y * cos_angle - point.z * sin_angle,
        point.y * sin_angle + point.z * cos_angle,
        point.w,
    )
}

fn rotate_4d_yz(point: Vec4, angle: f32) -> Vec4 {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();

    Vec4::new(
        point.x * cos_angle - point.w * sin_angle,
        point.y,
        point.z,
        point.x * sin_angle + point.w * cos_angle,
    )
}

fn rotate_4d_yw(point: Vec4, angle: f32) -> Vec4 {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();

    Vec4::new(
        point.x * cos_angle - point.z * sin_angle,
        point.y,
        point.x * sin_angle + point.z * cos_angle,
        point.w,
    )
}

fn rotate_4d_zw(point: Vec4, angle: f32) -> Vec4 {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();

    Vec4::new(
        point.x * cos_angle - point.y * sin_angle,
        point.x * sin_angle + point.y * cos_angle,
        point.z,
        point.w,
    )
}

