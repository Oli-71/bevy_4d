use bevy::color::palettes::css::GOLD;
use bevy::prelude::ops::abs;
use bevy::prelude::*;

use std::f32::consts::PI;

/// Sequence of atoms, represented by equal numbered positions and colors.
pub struct Atoms4D {
    pub positions: Vec<Vec4>,
    pub colors: Vec<Color>,
}

/// Object4D's names for save access
#[derive(PartialEq)]
enum ObjectName {
    Heart,
    Cube3d,
    Cube4d,
    Cube4dEdges,
    Circle,
    Square,
    Cube,
}

/// An object in the 4D scene, which consists of a sequence of atoms.
/// The `start_index` and `number_of_atoms` fields specify which atoms belong to this object.
struct Object4D {
    name: ObjectName,
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
    pub atoms: Atoms4D,
    pub size_of_atom: f32,
    pub number_of_atoms_per_side: usize,
    objects: Vec<Object4D>,
    objects_3d: Vec<usize>,
    objects_2d: Vec<usize>,
    is_projection_view: bool,
    is_4d_view: bool,
    start_time_4d: f32,
    speed_3d_rotation: f32,
    w_height: f32,
    angle_4d: f32,
}

impl Scene4D {
    /// compose a Scene from a Object4Ds
    pub fn new() -> Self {
        let size = 2.6;
        let number_per_side = 8;//16; // Total atoms will be number_per_side^4, so be careful with this number to avoid performance issues.
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
            objects_3d: Vec::new(),
            objects_2d: Vec::new(),
            is_4d_view: false,
            is_projection_view: false,
            start_time_4d: 0.0,
            speed_3d_rotation: 0.4,
            w_height: 0.0,
            angle_4d: 0.0,
        };

        //add some objects to the scene.
        let heart_index = scene.add_object(ObjectName::Heart, create_heart_3d(size_of_atom, number_per_side * 2));
        let cube3d_index = scene.add_object(ObjectName::Cube3d, create_cube_3d(size_of_atom, number_per_side));
        let cube4d_index = scene.add_object(ObjectName::Cube4d, create_cube_4d_surface(size_of_atom, number_per_side));
        let cube4d_edges_index = scene.add_object(ObjectName::Cube4dEdges, create_cube_4d_edges2(size_of_atom, number_per_side));
        let circle_index = scene.add_object(ObjectName::Circle, create_circle(size_of_atom, number_per_side));
        let square_index = scene.add_object(ObjectName::Square, create_square_surface(size_of_atom, number_per_side));
        let cube_index = scene.add_object(ObjectName::Cube, create_cube_surface(size_of_atom, number_per_side));
        
        scene.objects_3d = vec![heart_index, cube3d_index, cube4d_index, cube4d_edges_index];
        scene.objects_2d = vec![circle_index, square_index, cube_index];

        scene
    }

    fn object(&self, name: ObjectName) -> Option<&Object4D> {
        self.objects.iter().find(|&obj| obj.name == name).map(|v| v as _)
    }

    fn object_mut(&mut self, name: ObjectName) -> Option<&mut Object4D> {
        self.objects.iter_mut().find(|obj| obj.name == name).map(|v| v as _)
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

    fn add_object(&mut self, name: ObjectName, new_atoms: Atoms4D) -> usize {
        let index = self.objects.len();
        self.objects.push(Object4D {
            name,
            start_index: self.atoms.positions.len(),
            number_of_atoms: new_atoms.positions.len(),
            drag: Vec2::ZERO,
        });
        self.atoms.positions.extend(new_atoms.positions);
        self.atoms.colors.extend(new_atoms.colors);
        index
    }

    pub fn toggle_4d_view(&mut self, current_time: f32) {
        self.is_4d_view = !self.is_4d_view;
        if self.is_4d_view {
            self.start_time_4d = current_time;
        }
        else {
            self.angle_4d = 0.0;
        }
    }

    pub fn toggle_projection_view(&mut self) {
        self.is_projection_view = !self.is_projection_view;
    }

    pub fn is_4d_view(&self) -> bool {
        self.is_4d_view
    }

    fn objects_2d(&self) -> impl Iterator<Item = &Object4D> {
        self.objects_2d.iter().map(move |&index| &self.objects[index])
    }

    fn objects_3d(&self) -> impl Iterator<Item = &Object4D> {
        self.objects_3d.iter().map(move |&index| &self.objects[index])
    }

    fn time_in_4d_view(&self, current_time: f32) -> f32 {
        if self.is_4d_view {
            current_time - self.start_time_4d
        } else {
            0.0
        }
    }

    pub fn adjust_3d_rotation_speed(&mut self, speed: f32) {
        self.speed_3d_rotation = (speed).clamp(0.0, 1.0);
    }

    pub fn adjust_w_height(&mut self, w_height: f32) {
        self.w_height = w_height.clamp(-3.0, 3.0);
    }

    pub fn get_angle_4d(& self) -> f32 {
        self.angle_4d
    }

    /// transforms all atoms in the 4D scene. Returns the new positions.
    pub fn transform_scene(&mut self, time: f32) -> Vec<Vec4> {
        let mut new_positions = self.atoms.positions.clone();

        //local movements
        let angle = time; // Rotation angle in radians
        let x_offset = 2.5; // Distance to move the objects apart

        // standard 3D rotation (continuously)
        let continuous_rotation_matrix = Mat3::from_rotation_y(self.speed_3d_rotation * angle); // Rotate around the Z-axis

        // local 3D-transformations for 3d objects
        for object_3d in self.objects_3d() {
            let drag_matrix_xy = object_3d.drag_rotation_xy(); 
            for index in object_3d.range() {
                let position = &self.atoms.positions[index];
                let mut rotated_position = continuous_rotation_matrix * vec3(position.x, position.y, position.z);
                rotated_position = drag_matrix_xy * rotated_position; // Apply dragging transformation

                new_positions[index] = Vec4::new(
                    rotated_position.x, 
                    rotated_position.y,
                    rotated_position.z,
                    position.w + self.w_height, // Adjust the w coordinate based on the w_height control
                );
            }
        }

        // local 2D-transformations for 2d objects
        for object_2d in self.objects_2d() {
            let drag_matrix_x = object_2d.drag_rotation_x(); // Get the drag rotation matrix for the 2D object
            for index in object_2d.range() {
                let position = &self.atoms.positions[index];
                let mut rotated_position = continuous_rotation_matrix * vec3(position.x, position.y, position.z);
                rotated_position = drag_matrix_x * rotated_position; // Apply dragging transformation

                new_positions[index] = Vec4::new(
                    rotated_position.x,
                    rotated_position.y,
                    rotated_position.z,
                    position.w,
                );
            }
        }

        // higher dimension transformation
        // applied on top of the local transformations above.
        // Atoms will move in and out of the visible flat-land and space-land.
        if self.is_4d_view() {
            self.angle_4d = self.time_in_4d_view(time) / 4.0;
            self.angle_4d %= 2.0 * PI;// clap to [0..pi/2]

            // a 3D rotation (y is changing)
            for object_2d in self.objects_2d() { 
                for atom_index in object_2d.range() {
                    new_positions[atom_index] = rotate_4d_xw(new_positions[atom_index], self.angle_4d);
                }
            }

            // a 4D rotation (w is changing)
            for object_3d in self.objects_3d() { 
                for atom_index in object_3d.range() {
                    new_positions[atom_index] = rotate_4d_xy(new_positions[atom_index], self.angle_4d);
                }
            }    
        }

        // 3d row: move upwards and spread out within the row 
        // project to w=0 space in projection view
        let y_offset = 2. * x_offset;
        let mut delta_x = -3. * x_offset;
        for object_3d in self.objects_3d(){
            for atom_index in object_3d.range() {
                new_positions[atom_index].x += delta_x;
                new_positions[atom_index].y += y_offset;
                if self.is_projection_view{
                    new_positions[atom_index].w = 0.; // align all atoms to the same w level in projection view 
                }
            }
            delta_x += 2. * x_offset;// next column
        }

        // 2d row: spread out within the row
        // project to y=0 plane in projection view
        let mut delta_x = -3. * x_offset;
        for object_2d in self.objects_2d(){
            for atom_index in object_2d.range() {
                new_positions[atom_index].x += delta_x;
                if self.is_projection_view{
                    new_positions[atom_index].y = 0.; // align all atoms to the same y level in projection view 
                }
            }
            delta_x += 2. * x_offset;// next column
        }

        new_positions
    }
}

/// Helper functions to create different 3D and 4D objects as sequences of atoms.
/// Objects are centered around the origin.
///
/// Creates a 3D cube composed of atoms.
/// `size` is the length of an edge of the cube, and `number_per_side` is how many smaller cubes there are along each edge..
fn create_cube_3d(size_atom: f32, number_per_side: usize) -> Atoms4D {
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

fn create_cube_4d(size_atom: f32, number_per_side: usize) -> Atoms4D {
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

fn create_cube_4d_surface(size_atom: f32, number_per_side: usize) -> Atoms4D {
    let capacity = number_per_side * number_per_side * number_per_side * 8;
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

    let end = (number_per_side / 2) as i32 - 1;
    let start = -end;

    let spacing = 1.1 * size_atom;

    let low = (start - 1) as f32 * spacing; // Position for the "low" side of the cube (e.g., w = low)
    let high = (end + 1) as f32 * spacing; // Position for the "high" side of the cube (e.g., w = high)

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

fn create_cube_4d_corners(size_atom: f32, number_per_side: usize) -> Atoms4D {
    let capacity = 16; // A 4D cube has 16 corners
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

    let spacing = 1.1 * size_atom;
    let end = (number_per_side / 2) as f32 * spacing;
    let start = -end;

    positions.push(Vec4::new(start, start, start, start));
    colors.push(Color::from(Srgba::rgb_u8(255, 0, 0))); //red
    positions.push(Vec4::new(start, start, start, end));
    colors.push(Color::from(Srgba::rgb_u8(255, 255, 0))); //yellow
    positions.push(Vec4::new(start, start, end, start));
    colors.push(Color::from(Srgba::rgb_u8(0, 255, 0))); //green
    positions.push(Vec4::new(start, start, end, end));
    colors.push(Color::from(Srgba::rgb_u8(0, 255, 255))); //cyan
    positions.push(Vec4::new(start, end, start, start));
    colors.push(Color::from(Srgba::rgb_u8(255, 0, 255))); //purple
    positions.push(Vec4::new(start, end, start, end));
    colors.push(Color::from(Srgba::rgb_u8(255, 255, 255))); //white
    positions.push(Vec4::new(start, end, end, start));
    colors.push(Color::from(Srgba::rgb_u8(0, 0, 255))); //blue
    positions.push(Vec4::new(start, end, end, end));
    colors.push(Color::from(Srgba::rgb_u8(0, 0, 0))); //black
    positions.push(Vec4::new(end, start, start, start));
    colors.push(Color::from(Srgba::rgb_u8(255, 0, 0))); //red
    positions.push(Vec4::new(end, start, start, end));
    colors.push(Color::from(Srgba::rgb_u8(255, 255, 0))); //yellow
    positions.push(Vec4::new(end, start, end, start));
    colors.push(Color::from(Srgba::rgb_u8(0, 255, 0))); //green
    positions.push(Vec4::new(end, start, end, end));
    colors.push(Color::from(Srgba::rgb_u8(0, 255, 255))); //cyan
    positions.push(Vec4::new(end, end, start, start));
    colors.push(Color::from(Srgba::rgb_u8(255, 0, 255))); //purple
    positions.push(Vec4::new(end, end, start, end));
    colors.push(Color::from(Srgba::rgb_u8(255, 255, 255))); //white
    positions.push(Vec4::new(end, end, end, start));
    colors.push(Color::from(Srgba::rgb_u8(0, 0, 255))); //blue
    positions.push(Vec4::new(end, end, end, end));
    colors.push(Color::from(Srgba::rgb_u8(0, 0, 0))); //black
    Atoms4D { positions, colors }
}

// thick edges with multiple colors to visualize the 3d cubes ("faces") too. 
fn create_cube_4d_edges2(size_atom: f32, number_per_side: usize) -> Atoms4D {
    let mut positions = Vec::new();
    let mut colors = Vec::new();

    let end = (number_per_side / 2) as i32 - 1;
    let start = -end;

    let spacing = 1.1 * size_atom;

    let low = (start - 1) as f32 * spacing; // Position for the "low" side of the cube (e.g., w = low)
    let high = (end + 1) as f32 * spacing; // Position for the "high" side of the cube (e.g., w = high)

    // Create atoms for the 8 "faces" of the 4D cube (each face is a 3D cube in the 4D space).
    for a in start..=end {
        for b in start..=end {
            for c in start..=end {
                let mut count = 0;
                if a==start || a==end {
                    count += 1;
                }
                if b==start || b==end {
                    count += 1;
                }
                if c==start || c==end {
                    count += 1;
                }
                if count == 2 { // Only create atoms for the edges of the 4D cube
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
                    if vec3(xx,yy,zz).length() < (scale / 2.0) {
                        colors.push(Color::from(GOLD));// golden for inner atoms
                    } else {
                        colors.push(Color::from(Srgba::rgb(1., 0., 0.)));
                    }
                }
            }
        }
    }
    Atoms4D { positions, colors }
}

fn create_sphere_4d(radius: f32, number_per_side: usize) -> Atoms4D {
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

fn create_square(size_atom: f32, number_per_side: usize) -> Atoms4D {
    let capacity = number_per_side * number_per_side;
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

    let end = (number_per_side / 2) as i32;
    let start = -end;
    let spacing = 1.1 * size_atom;

    for x in start..=end {
        for z in start..=end {
            positions.push(vec4(
                x as f32 * spacing, 
                0.0, 
                z as f32 * spacing, 
                0.0
            ));
            colors.push(Color::from(Srgba::rgb_u8(
                ((x + end) * 255 / number_per_side as i32) as u8,
                ((z + end) * 255 / number_per_side as i32) as u8,
                128,
            )));
        }
    }
    Atoms4D { positions, colors }
}

fn create_circle(size_atom: f32, number_per_side: usize) -> Atoms4D {
    let capacity = number_per_side * number_per_side;
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

    let end = (number_per_side / 2) as i32;
    let start = -end;
    let spacing = 1.1 * size_atom;

    for x in start..=end {
        for z in start..=end {
            let pos = vec2(x as f32 * spacing, z as f32 * spacing);
            if pos.length() <= (number_per_side as f32 / 2.0) * spacing {
                positions.push(vec4(
                    pos.x,
                    0.0, 
                    pos.y, 
                    0.0
                ));
                if pos.length() < (number_per_side as f32 / 4.0) * spacing {
                    colors.push(Color::from(Srgba::rgb_u8(
                        255,
                        255,
                        0,
                    ))); // yellow for inner atoms
                } else {
                colors.push(Color::from(Srgba::rgb_u8(
                    255,
                    0,
                    0,
                )))}; // red for outer atoms
            }
        }
    }
    Atoms4D { positions, colors }
}

fn create_cube_surface(size_atom: f32, number_per_side: usize) -> Atoms4D {
    let capacity = number_per_side * number_per_side * 6;
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

    let end = (number_per_side / 2) as i32 - 1;
    let start = -end;

    let spacing = 1.1 * size_atom;

    let low = (start - 1) as f32 * spacing; // Position for the "low" side of the cube (e.g., w = low)
    let high = (end + 1) as f32 * spacing; // Position for the "high" side of the cube (e.g., w = high)

    // Create atoms for the 8 faces of the 4D cube (each face is a 3D cube in the 4D space).
    for a in start..=end {
        for b in start..=end {
                let aa = a as f32 * spacing;
                let bb = b as f32 * spacing;

                positions.push(Vec4::new(aa, bb, low, 0.0));
                colors.push(Color::from(Srgba::rgb_u8(255, 0, 0))); //red for w=low

                positions.push(Vec4::new(aa, bb, high, 0.0));
                colors.push(Color::from(Srgba::rgb_u8(255, 255, 0))); //yellow for w=high

                positions.push(Vec4::new(aa, low, bb, 0.0));
                colors.push(Color::from(Srgba::rgb_u8(0, 255, 0))); //green for z=low

                positions.push(Vec4::new(aa, high, bb, 0.0));
                colors.push(Color::from(Srgba::rgb_u8(0, 255, 255))); //cyan for z=high

                positions.push(Vec4::new(low, aa, bb, 0.0));
                colors.push(Color::from(Srgba::rgb_u8(0, 0, 255))); //blue for y=low

                positions.push(Vec4::new(high, aa, bb, 0.0));
                colors.push(Color::from(Srgba::rgb_u8(255, 0, 255))); //purple for y=high
        }
    }

    Atoms4D { positions, colors }
}

fn create_square_surface(size_atom: f32, number_per_side: usize) -> Atoms4D {
    let capacity = number_per_side * 4;
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

    let end = (number_per_side / 2) as i32 - 1;
    let start = -end;

    let spacing = 1.1 * size_atom;

    let low = (start - 1) as f32 * spacing; // Position for the "low" side of the cube (e.g., w = low)
    let high = (end + 1) as f32 * spacing; // Position for the "high" side of the cube (e.g., w = high)

    // Create atoms for the 8 faces of the 4D cube (each face is a 3D cube in the 4D space).
    for a in start..=end {
                let aa = a as f32 * spacing;

                positions.push(Vec4::new(low,0.0, aa, 0.0));
                colors.push(Color::from(Srgba::rgb_u8(255, 0, 0))); //red for w=low

                positions.push(Vec4::new(high, 0.0, aa, 0.0));
                colors.push(Color::from(Srgba::rgb_u8(255, 255, 0))); //yellow for w=high

                positions.push(Vec4::new(aa,0.0, low, 0.0));
                colors.push(Color::from(Srgba::rgb_u8(0, 255, 0))); //green for z=low

                positions.push(Vec4::new(aa, 0.0, high, 0.0));
                colors.push(Color::from(Srgba::rgb_u8(0, 255, 255))); //cyan for z=high
    }

    Atoms4D { positions, colors }
}


// 4D rotation functions for different planes. Each function takes a point in 4D space and an angle, and returns the rotated point.
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
