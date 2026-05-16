use bevy::color::palettes::css::GOLD;
use bevy::prelude::ops::abs;
use bevy::prelude::*;

use std::f32::consts::PI;

/// Sequence of atoms, represented by equal numbered positions and colors.
pub struct Atoms4D {
    pub positions: Vec<Vec4>,
    pub colors: Vec<Color>,
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
    pub atoms: Atoms4D,
    pub size_of_atom: f32,
    pub number_of_atoms_per_side: usize,
    objects: Vec<Object4D>,
    objects_3d: Vec<usize>,
    objects_2d: Vec<usize>,
    pub is_projection_view: bool,
    pub is_high_dimension_view: bool,
    start_time_high_dimension: f32,
    speed_3d_rotation: f32,
    higher_dimension_height: f32,
    angle_high_dimension: f32,
}

impl Scene4D {
    /// compose a Scene from a Object4Ds
    pub fn new() -> Self {
        let size = 2.6;
        let number_per_side = 8;//16; // Total atoms will be number_per_side^4, 
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
            objects_3d: Vec::new(),
            objects_2d: Vec::new(),
            is_high_dimension_view: false,
            is_projection_view: false,
            start_time_high_dimension: 0.0,
            speed_3d_rotation: 0.0,// default: no continuous rotation
            higher_dimension_height: 0.0,
            angle_high_dimension: 0.0,
        };

        //add some objects to the scene.
        let heart_index = scene.add_object(create_heart_3d(size_of_atom, number_per_side * 2));
        let cube3d_index = scene.add_object(create_cube_3d(size_of_atom, number_per_side));
        let cube4d_index = scene.add_object(create_cube_4d_surface(size_of_atom, number_per_side));
        let cube4d_edges_index = scene.add_object(create_cube_4d_edges(size_of_atom, number_per_side));
        let circle_index = scene.add_object(create_circle(size_of_atom, number_per_side));
        let square_index = scene.add_object(create_square_surface(size_of_atom, number_per_side));
        let cube_index = scene.add_object(create_cube_surface(size_of_atom, number_per_side));
        let cube_edges_index = scene.add_object(create_cube_edges(size_of_atom, number_per_side));
        
        scene.objects_3d = vec![heart_index, cube3d_index, cube4d_index, cube4d_edges_index];
        scene.objects_2d = vec![circle_index, square_index, cube_index, cube_edges_index];

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
        for object in &mut self.objects {
            object.drag += delta * 0.02;
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
        }
        else {
            self.angle_high_dimension = 0.0;
        }
    }

    pub fn toggle_projection_view(&mut self) {
        self.is_projection_view = !self.is_projection_view;
    }

    fn objects_2d(&self) -> impl Iterator<Item = &Object4D> {
        self.objects_2d.iter().map(move |&index| &self.objects[index])
    }

    fn objects_3d(&self) -> impl Iterator<Item = &Object4D> {
        self.objects_3d.iter().map(move |&index| &self.objects[index])
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

    pub fn get_angle_high_dimension(& self) -> f32 {
        self.angle_high_dimension
    }

    /// transforms all atoms in the 4D scene. Returns the new positions.
    pub fn transform_scene(&mut self, time: f32) -> Vec<Vec4> {
        let mut new_positions = self.atoms.positions.clone();

        let angle = time; // Rotation angle for the continuous rotation and the higher-dimension rotation
        let continuous_rotation_matrix = Mat3::from_rotation_y(self.speed_3d_rotation * angle); // Rotate around the Z-axis

        // Apply local transformations for each object based on its drag state.
        //  -For 3D objects, we apply a 3D rotation based on the drag.
        //  -For 2D objects, we apply a 2D rotation based on the drag.
        let mut apply_local_transform = |object: &Object4D, drag_matrix: Mat3| {
            for index in object.range() {
                let position = &self.atoms.positions[index];
                let rotated_position =
                    drag_matrix * (continuous_rotation_matrix * vec3(position.x, position.y, position.z));

                new_positions[index] = Vec4::new(
                    rotated_position.x,
                    rotated_position.y,
                    rotated_position.z,
                    position.w,
                );
            }
        };

        // local 3D-transformations for 3d objects
        for object_3d in self.objects_3d() {
            apply_local_transform(object_3d, object_3d.drag_rotation_xy());
        }

        // local 2D-transformations for 2d objects
        for object_2d in self.objects_2d() {
            apply_local_transform(object_2d, object_2d.drag_rotation_x());
        }

        // Higher dimension transformation
        // applied on top of the local transformations above.
        // Atoms will move in and out of the visible flat-land and space-land.
        if self.is_high_dimension_view {
            self.angle_high_dimension = self.time_in_high_dimension_view(time) / 4.0;
            self.angle_high_dimension %= 2.0 * PI;// clap to [0..2pi]

            // a 3D rotation (y is changing)
            for object_2d in self.objects_2d() { 
                for atom_index in object_2d.range() {
                    rotate_4d_xw(&mut new_positions[atom_index], self.angle_high_dimension);
                }
            }

            // a 4D rotation (w is changing)
            for object_3d in self.objects_3d() { 
                for atom_index in object_3d.range() {
                    rotate_4d_xy(&mut new_positions[atom_index], self.angle_high_dimension);
                }
            }    
        }

        // Placement of objects in the scene.
        // Projection to lower dimension. 
        let x_offset = 2.5; // Distance to move the objects apart
        let hd_offset = 2.1 * self.higher_dimension_height;// Distance to move the objects in the higher dimension.

        // 3d row:
        // - move upwards
        // - spread out within the row 
        // - project to w=0 space in projection view
        let y_offset = 2. * x_offset;
        let mut delta_x = -3. * x_offset;
        for object_3d in self.objects_3d(){
            for atom_index in object_3d.range() {
                new_positions[atom_index].x += delta_x;
                new_positions[atom_index].y += y_offset;
                if self.is_projection_view{
                    new_positions[atom_index].w = 0.; // move all atoms to the same w level in projection view 
                } else {
                    new_positions[atom_index].w += hd_offset;
                }
            }
            delta_x += 2. * x_offset;// next column
        }

        // 2d row: 
        // - spread out within the row
        // - project to y=0 plane in projection view
        let mut delta_x = -3. * x_offset;
        for object_2d in self.objects_2d(){
            for atom_index in object_2d.range() {
                new_positions[atom_index].x += delta_x;
                if self.is_projection_view{
                    new_positions[atom_index].y = 0.; // move all atoms to the same y level in projection view 
                } else {
                    new_positions[atom_index].y += hd_offset;
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
                colors.push(Color::from(Srgba::rgb_u8(255, 0, 255))); //purple

                positions.push(Vec4::new(aa, bb, cc, high));
                colors.push(Color::from(Srgba::rgb_u8(0, 255, 255))); //cyan

                positions.push(Vec4::new(aa, bb, low, cc));
                colors.push(Color::from(Srgba::rgb_u8(0, 255, 0))); //green

                positions.push(Vec4::new(aa, bb, high, cc));
                colors.push(Color::from(Srgba::rgb_u8(0, 0, 255))); //blue

                positions.push(Vec4::new(aa, low, bb, cc));
                colors.push(Color::from(Srgba::rgb_u8(255, 255, 0))); //yellow

                positions.push(Vec4::new(aa, high, bb, cc));
                colors.push(Color::from(Srgba::rgb_u8(255, 255, 255))); //white

                positions.push(Vec4::new(low, aa, bb, cc));
                colors.push(Color::from(Srgba::rgb_u8(255, 0, 0))); //red

                positions.push(Vec4::new(high, aa, bb, cc));
                colors.push(Color::from(Srgba::rgb_u8(255, 165, 0))); //orange
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
fn create_cube_4d_edges(size_atom: f32, number_per_side: usize) -> Atoms4D {
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
                    colors.push(Color::from(Srgba::rgb_u8(255, 0, 255))); //purple

                    positions.push(Vec4::new(aa, bb, cc, high));
                    colors.push(Color::from(Srgba::rgb_u8(0, 255, 255))); //cyan

                    positions.push(Vec4::new(aa, bb, low, cc));
                    colors.push(Color::from(Srgba::rgb_u8(0, 255, 0))); //green

                    positions.push(Vec4::new(aa, bb, high, cc));
                    colors.push(Color::from(Srgba::rgb_u8(0, 0, 255))); //blue

                    positions.push(Vec4::new(aa, low, bb, cc));
                    colors.push(Color::from(Srgba::rgb_u8(255, 255, 0))); //yellow

                    positions.push(Vec4::new(aa, high, bb, cc));
                    colors.push(Color::from(Srgba::rgb_u8(255, 255, 255))); //white

                    positions.push(Vec4::new(low, aa, bb, cc));
                    colors.push(Color::from(Srgba::rgb_u8(255, 0, 0))); //red

                    positions.push(Vec4::new(high, aa, bb, cc));
                    colors.push(Color::from(Srgba::rgb_u8(255, 165, 0))); //orange
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
                colors.push(Color::from(Srgba::rgb_u8(0, 255, 0))); //green

                positions.push(Vec4::new(aa, bb, high, 0.0));
                colors.push(Color::from(Srgba::rgb_u8(0, 0, 255))); //blue

                positions.push(Vec4::new(aa, low, bb, 0.0));
                colors.push(Color::from(Srgba::rgb_u8(255, 255, 0))); //yellow

                positions.push(Vec4::new(aa, high, bb, 0.0));
                colors.push(Color::from(Srgba::rgb_u8(255, 255, 255))); //white

                positions.push(Vec4::new(low, aa, bb, 0.0));
                colors.push(Color::from(Srgba::rgb_u8(255, 0, 0))); //red

                positions.push(Vec4::new(high, aa, bb, 0.0));
                colors.push(Color::from(Srgba::rgb_u8(255, 165, 0))); //orange
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
        colors.push(Color::from(Srgba::rgb_u8(255, 165, 0))); //orange for w=high

        positions.push(Vec4::new(aa,0.0, low, 0.0));
        colors.push(Color::from(Srgba::rgb_u8(0, 255, 0))); //green for z=low

        positions.push(Vec4::new(aa, 0.0, high, 0.0));
        colors.push(Color::from(Srgba::rgb_u8(0, 0, 255))); //blue for z=high
    }
    Atoms4D { positions, colors }
}

fn create_cube_edges(size_atom: f32, number_per_side: usize) -> Atoms4D {
    let mut positions = Vec::new();
    let mut colors = Vec::new();

    let end = (number_per_side / 2) as i32 - 1;
    let start = -end;

    let spacing = 1.1 * size_atom;

    let low = (start - 1) as f32 * spacing; // Position for the "low" side of the cube (e.g., w = low)
    let high = (end + 1) as f32 * spacing; // Position for the "high" side of the cube (e.g., w = high)

    // Create atoms for the 6 faces of the 3D cube.
    for a in start..=end {
        for b in start..=end {
            
                if a==start || a==end || b==start || b==end { // Only create atoms for the edges of the 4D cube
                    let aa = a as f32 * spacing;
                    let bb = b as f32 * spacing;
                    let cc = 0.0;

                    positions.push(Vec4::new(aa, bb, low, cc));
                    colors.push(Color::from(Srgba::rgb_u8(0, 255, 0))); //green

                    positions.push(Vec4::new(aa, bb, high, cc));
                    colors.push(Color::from(Srgba::rgb_u8(0, 0, 255))); //blue

                    positions.push(Vec4::new(aa, low, bb, cc));
                    colors.push(Color::from(Srgba::rgb_u8(255, 255, 0))); //yellow

                    positions.push(Vec4::new(aa, high, bb, cc));
                    colors.push(Color::from(Srgba::rgb_u8(255, 255, 255))); //white

                    positions.push(Vec4::new(low, aa, bb, cc));
                    colors.push(Color::from(Srgba::rgb_u8(255, 0, 0))); //red

                    positions.push(Vec4::new(high, aa, bb, cc));
                    colors.push(Color::from(Srgba::rgb_u8(255, 165, 0))); //orange
            }
        }
    }
    Atoms4D { positions, colors }
    /* let capacity = number_per_side * number_per_side * 12;
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

    let end = (number_per_side / 2) as i32 - 1;
    let start = -end;

    let spacing = 1.1 * size_atom;

    let low = (start - 1) as f32 * spacing; // Position for the "low" side of the cube (e.g., w = low)
    let high = (end + 1) as f32 * spacing; // Position for the "high" side of the cube (e.g., w = high)

    // Create atoms for the 12 edges of the cube.
    for a in start..=end {
            let aa = a as f32 * spacing;

            positions.push(Vec4::new(aa, low, low, 0.0));
            colors.push(Color::from(Srgba::rgb_u8(255, 0, 0))); //red

            positions.push(Vec4::new(aa, high, low, 0.0));
            colors.push(Color::from(Srgba::rgb_u8(255, 0, 255))); //purple

            positions.push(Vec4::new(aa, low, high, 0.0));
            colors.push(Color::from(Srgba::rgb_u8(255, 0, 255))); //purple

            positions.push(Vec4::new(aa, high, high, 0.0));
            colors.push(Color::from(Srgba::rgb_u8(255, 0, 255))); //purple


            positions.push(Vec4::new(low, aa, low, 0.0));
            colors.push(Color::from(Srgba::rgb_u8(0, 0, 255))); //blue

            positions.push(Vec4::new(high, aa, low, 0.0));
            colors.push(Color::from(Srgba::rgb_u8(255, 0, 255))); //purple

            positions.push(Vec4::new(low, aa, high, 0.0));
            colors.push(Color::from(Srgba::rgb_u8(0, 255, 0))); //green

            positions.push(Vec4::new(high, aa, high, 0.0));
            colors.push(Color::from(Srgba::rgb_u8(0, 255, 255))); //cyan


            positions.push(Vec4::new(low, low, aa, 0.0));
            colors.push(Color::from(Srgba::rgb_u8(0, 0, 255))); //blue

            positions.push(Vec4::new(high, low, aa, 0.0));
            colors.push(Color::from(Srgba::rgb_u8(255, 0, 255))); //purple

            positions.push(Vec4::new(low, high, aa, 0.0));
            colors.push(Color::from(Srgba::rgb_u8(0, 255, 0))); //green

            positions.push(Vec4::new(high, high, aa, 0.0));
            colors.push(Color::from(Srgba::rgb_u8(0, 255, 255))); //cyan
    }
    Atoms4D { positions, colors } */
}

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
