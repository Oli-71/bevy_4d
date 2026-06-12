use bevy::color::palettes::css::GOLD;
use bevy::prelude::*;

use bevy::prelude::ops::sin;
use bevy::prelude::ops::cos;

/// Sequence of atoms, represented by equal numbered positions and colors.
pub struct Atoms4D {
    pub positions: Vec<Vec4>,
    pub colors: Vec<Color>,
}

impl Atoms4D {
    pub fn transform(mut self, transform: Mat4) -> Self {
        for pos in &mut self.positions {
            *pos = transform.mul_vec4(*pos);
        }
        self
    }
    pub fn translate(mut self, translation: Vec4) -> Self {
        for pos in &mut self.positions {
            *pos += translation;
        }
        self
    }
}

fn orange() -> Srgba {
    Srgba::rgb_u8(255, 140, 26)
}
// Helper functions to create different 3D and 4D objects as sequences of atoms.
// Objects are centered around the origin.

/// Creates a 3D cube composed of atoms.
/// `size` is the length of an edge of the cube, and `number_per_side` is how many smaller cubes there are along each edge.
pub(crate) fn create_cube_surface_colorful(spacing: f32, number_per_side: usize) -> Atoms4D {
    let capacity = number_per_side * number_per_side * 6 * 2;
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

    let end = (number_per_side / 2) as i32;
    let start = -end;
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

pub(crate) fn create_cube_4d(spacing: f32, number_per_side: usize) -> Atoms4D {
    let capacity = number_per_side * number_per_side * number_per_side * number_per_side;
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

    let end = (number_per_side / 2) as i32;
    let start = -end;
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

pub(crate) fn create_cube_4d_surface(spacing: f32, number_per_side: usize) -> Atoms4D {
    let capacity = number_per_side * number_per_side * number_per_side * 8;
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

    let end = (number_per_side / 2) as i32 - 1;
    let start = -end;

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
                colors.push(Color::from(orange())); //orange
            }
        }
    }
    Atoms4D { positions, colors }
}

pub(crate) fn create_cube_4d_corners(spacing: f32, number_per_side: usize) -> Atoms4D {
    let capacity = 16; // A 4D cube has 16 corners
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

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
pub(crate) fn create_cube_4d_edges(spacing: f32, number_per_side: usize) -> Atoms4D {
    let mut positions = Vec::new();
    let mut colors = Vec::new();

    let end = (number_per_side / 2) as i32 - 1;
    let start = -end;

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
                    colors.push(Color::from(orange())); //orange
                }
            }
        }
    }
    Atoms4D { positions, colors }
}

pub(crate) fn create_heart_3d(spacing: f32, number_per_side: usize) -> Atoms4D {
    let capacity = number_per_side * number_per_side * number_per_side;
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

    let actual_number_per_side = number_per_side * 2;

    let end = (actual_number_per_side / 2) as i32;
    let start = -end;
    let scale = 4.0 / (spacing * actual_number_per_side as f32); // Scale the heart to fit within the cube

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

pub(crate) fn create_sphere_4d(spacing: f32, number_per_side: usize) -> Atoms4D {
    let capacity = number_per_side * number_per_side * number_per_side * number_per_side;
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

    let end = (number_per_side / 2) as i32;
    let start = -end;
    let radius  = number_per_side as f32 / 2.0;

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

pub(crate) fn create_square(spacing: f32, number_per_side: usize) -> Atoms4D {
    let capacity = number_per_side * number_per_side;
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

    let end = (number_per_side / 2) as i32;
    let start = -end;

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

pub(crate) fn create_circle(spacing: f32, number_per_side: usize) -> Atoms4D {
    let capacity = number_per_side * number_per_side;
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

    let end = (number_per_side / 2) as i32;
    let start = -end;

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

pub(crate) fn create_cube_surface(spacing: f32, number_per_side: usize) -> Atoms4D {
    let capacity = number_per_side * number_per_side * 6;
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

    let end = (number_per_side / 2) as i32 - 1;
    let start = -end;

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
                colors.push(Color::from(orange())); //orange
        }
    }
    Atoms4D { positions, colors }
}

pub(crate) fn create_square_surface(spacing: f32, number_per_side: usize) -> Atoms4D {
    let capacity = number_per_side * 4;
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

    let end = (number_per_side / 2) as i32 - 1;
    let start = -end;

    let low = (start - 1) as f32 * spacing; // Position for the "low" side of the cube (e.g., w = low)
    let high = (end + 1) as f32 * spacing; // Position for the "high" side of the cube (e.g., w = high)

    // Create atoms for the 8 faces of the 4D cube (each face is a 3D cube in the 4D space).
    for a in start..=end {
        let aa = a as f32 * spacing;

        positions.push(Vec4::new(low,0.0, aa, 0.0));
        colors.push(Color::from(Srgba::rgb_u8(255, 0, 0))); //red for w=low

        positions.push(Vec4::new(high, 0.0, aa, 0.0));
        colors.push(Color::from(orange())); //orange for w=high

        positions.push(Vec4::new(aa,0.0, low, 0.0));
        colors.push(Color::from(Srgba::rgb_u8(0, 255, 0))); //green for z=low

        positions.push(Vec4::new(aa, 0.0, high, 0.0));
        colors.push(Color::from(Srgba::rgb_u8(0, 0, 255))); //blue for z=high
    }
    Atoms4D { positions, colors }
}

pub(crate) fn create_cube_edges(spacing: f32, number_per_side: usize) -> Atoms4D {
    let mut positions = Vec::new();
    let mut colors = Vec::new();

    let end = (number_per_side / 2) as i32 - 1;
    let start = -end;

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
                    colors.push(Color::from(orange())); //orange
            }
        }
    }
    Atoms4D { positions, colors }
}

pub(crate) fn create_tripod_4d(spacing: f32, number_per_side: usize) -> Atoms4D {
    let mut positions = Vec::new();
    let mut colors = Vec::new();

    // origin
    positions.push(Vec4::new(0., 0., 0., 0.));
    colors.push(Color::from(Srgba::rgb_u8(255, 255, 255))); //gray

    for a in 1..=number_per_side {
        let aa = a as f32 * spacing;
        positions.push(Vec4::new(aa, 0., 0., 0.));
        colors.push(Color::from(Srgba::rgb_u8(255, 0, 0))); //red
        positions.push(Vec4::new(0., aa, 0., 0.));
        colors.push(Color::from(Srgba::rgb_u8(0, 255, 0))); //green
        positions.push(Vec4::new(0., 0., aa, 0.));
        colors.push(Color::from(Srgba::rgb_u8(0, 0, 255))); //blue
        positions.push(Vec4::new(0., 0., 0., aa));
        colors.push(Color::from(Srgba::rgb_u8(255, 255, 0))); //yellow
    }

    Atoms4D { positions, colors }
}

pub(crate) fn create_head(spacing: f32) -> Atoms4D {
    let number_per_side = 16;
    let capacity = number_per_side * number_per_side * number_per_side;
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

    let start = -(number_per_side as i32 / 2);
    let end = start + number_per_side as i32 - 1;

    for z in start..=end {
        for y in start..=end {
            for x in start..=end {
                let px = x as f32 * spacing;
                let py = y as f32 * spacing;
                let pz = z as f32 * spacing;

                let head = (px / (7.2 * spacing)).powi(2)
                    + (py / (8.0 * spacing)).powi(2)
                    + (pz / (5.8 * spacing)).powi(2) <= 1.0;

                if !head {
                    continue;
                }

                let brain = (px / (4.0 * spacing)).powi(2)
                    + ((py - 2.0 * spacing) / (4.5 * spacing)).powi(2)
                    + (pz / (3.5 * spacing)).powi(2) <= 1.0;

                let nose = pz > 2.0 * spacing
                    && py > 0.5 * spacing
                    && px.abs() < 3.5 * spacing
                    && (px / (3.0 * spacing)).powi(2)
                        + ((py - 1.5 * spacing) / (2.5 * spacing)).powi(2)
                        + ((pz - 4.0 * spacing) / (2.0 * spacing)).powi(2)
                        <= 1.0;

                let left_eye = (px + 3.0 * spacing).abs() < 1.2 * spacing
                    && (py - 2.0 * spacing).abs() < 1.0 * spacing
                    && (pz - 4.0 * spacing).abs() < 1.0 * spacing;
                let right_eye = (px - 3.0 * spacing).abs() < 1.2 * spacing
                    && (py - 2.0 * spacing).abs() < 1.0 * spacing
                    && (pz - 4.0 * spacing).abs() < 1.0 * spacing;

                let mouth = py < -1.5 * spacing
                    && py > -3.5 * spacing
                    && pz > 3.0 * spacing
                    && px.abs() < 3.5 * spacing
                    && (pz - 3.5 * spacing).abs() < 1.5 * spacing;

                let surface = (px / (7.2 * spacing)).powi(2)
                    + (py / (8.0 * spacing)).powi(2)
                    + (pz / (5.8 * spacing)).powi(2) > 0.72;

                let color = if left_eye || right_eye {
                    Color::from(Srgba::rgb_u8(20, 20, 20))
                } else if mouth {
                    Color::from(Srgba::rgb_u8(170, 40, 40))
                } else if nose {
                    Color::from(Srgba::rgb_u8(220, 160, 120))
                } else if brain {
                    Color::from(Srgba::rgb_u8(200, 200, 210))
                } else if surface {
                    Color::from(Srgba::rgb_u8(240, 190, 160))
                } else {
                    Color::from(Srgba::rgb_u8(190, 80, 80))
                };

                positions.push(Vec4::new(px, py, pz, 0.0));
                colors.push(color);
            }
        }
    }

    Atoms4D { positions, colors }
}

pub(crate) fn create_atoms_from_file (spacing: f32, name: String) -> Atoms4D {
    let file_contents = std::fs::read_to_string(&name)
        .expect("Could not read file");
    let lines = file_contents
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect::<Vec<_>>();

    let mut positions = Vec::with_capacity(lines.len());
    let mut colors = Vec::with_capacity(lines.len());

    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if let [x, y, z, r, g, b, a] = parts.as_slice() {
            let x: i32 = x.parse().unwrap();
            let y: i32 = y.parse().unwrap();
            let z: i32 = z.parse().unwrap();
            let r: u8 = r.parse().unwrap();
            let g: u8 = g.parse().unwrap();
            let b: u8 = b.parse().unwrap();
            let a: u8 = a.parse().unwrap();
            

            positions.push(Vec4::new(
                (x-70) as f32 * spacing,
                (y-25) as f32 * spacing,
                (z-10) as f32 * spacing,
                0.0,
            ));
            colors.push(Color::from(Srgba::rgba_u8(r, g, b, a)));
        }
    }

    Atoms4D { positions, colors }
}

pub(crate) fn create_aquarium(spacing: f32, number_per_side: usize) -> Atoms4D {
    let capacity = number_per_side * number_per_side * 6;
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

    let end = (number_per_side / 2) as i32;
    let start = -end;
    let radius = (number_per_side as f32 / 2.0) * spacing; 
    let y_start = start + number_per_side as i32 / 5; 
    let y_end = end - number_per_side as i32 / 5;
    for x in start..=end {
        for y in y_start..=y_end {
            for z in start..=end {
                let pos = Vec4::new(
                    x as f32 * spacing,
                    y as f32 * spacing,
                    z as f32 * spacing,
                    0.0,
                );
                let l = pos.length();
                if l <= radius {// only create atoms within a certain radius to form a sphere
                    if l >= radius * 0.95 { // surface of the sphere to create a hollow aquarium
                        positions.push(pos);
                        colors.push(Color::from(Srgba::rgba_u8(200,200,255, 10))); // light blue with low alpha for glass walls
                    }

                    let wave = (sin((x as f32 + 5.) * 0.1) * cos(z as f32 * 0.2) * 2.) as i32; // Add some waves to the water surface
                    if y >= y_end - number_per_side as i32 / 20 + wave 
                        && y<y_end - number_per_side as i32 / 30 + wave { // Create a "water surface"
                        positions.push(pos);
                        colors.push(Color::from(Srgba::rgba_u8(0, 100, 255, 25))); // darker blue
                    }

                    let wave = (sin((x as f32 + 4.) * 0.1) * cos(z as f32 * 0.2) * 2.) as i32; // Add some waves to the floor surface
                    if y < y_start + number_per_side as i32 / 7 + wave { // Create a "floor" for the aquarium
                        positions.push(pos);
                        let yellow = rand::random_range(150..=255);
                        colors.push(Color::from(Srgba::rgba_u8(yellow, yellow, 100, 255))); // yellow sand floor
                    }
                }
            }
        }
    }
    Atoms4D { positions, colors }
}

/// Create a goldfish shape using an ellipsoid for the body and a tapering tail.
/// number_of_atoms_total_length is the total length of the fish in atom steps, including body and tail.
/// The length of the fish goes along the x-axis, the height along the y-axis, and the width along the z-axis.
pub(crate) fn create_fish_3d(spacing: f32, number_of_atoms_total_length: usize) -> Atoms4D {
    let total_atoms = number_of_atoms_total_length.max(6) as i32;
    let tail_atoms = ((total_atoms as f32) * 0.2).round().max(2.0) as i32;
    let body_atoms = total_atoms - tail_atoms;

    let x_min = -total_atoms / 2;
    let x_max = x_min + total_atoms - 1;
    let tail_max_x = x_min + tail_atoms - 1;
    let body_x_min = tail_max_x + 1;

    let body_half_length = (body_atoms as f32) * 0.5 * spacing;
    let body_center_x = (body_x_min + x_max) as f32 * 0.5 * spacing;
    let body_radius_y = body_half_length * 0.7;
    let body_radius_z = body_half_length * 0.5;
    let tail_length = (tail_atoms as f32) * spacing;
    let tail_max_height = body_radius_y * 0.6;
    let tail_max_width = body_radius_z * 0.4;

    let y_min = -(body_radius_y * 1.2).ceil() as i32;
    let y_max = (body_radius_y * 1.2).ceil() as i32;
    let z_min = -(body_radius_z * 1.4).ceil() as i32;
    let z_max = (body_radius_z * 1.4).ceil() as i32;

    let capacity = (total_atoms * (y_max - y_min + 1) * (z_max - z_min + 1)).max(total_atoms * 10) as usize;
    let mut positions = Vec::with_capacity(capacity);
    let mut colors = Vec::with_capacity(capacity);

    for x in x_min-10..=x_max+10 {
        for y in y_min-10..=y_max+10 {
            for z in z_min-10..=z_max+10 {
                let pos = Vec3::new(x as f32 * spacing, y as f32 * spacing, z as f32 * spacing);

                let body = x >= body_x_min // ellipsoid body
                    && ((pos.x - body_center_x) / body_half_length).powi(2)
                        + (pos.y / body_radius_y).powi(2)
                        + (pos.z / body_radius_z).powi(2)
                        <= 1.0;

                let inside_body = body
                    && ((pos.x - body_center_x) / body_half_length).powi(2)
                        + (pos.y / body_radius_y).powi(2)
                        + (pos.z / body_radius_z).powi(2)
                        <= 0.7;

                let bones = body
                    && (pos.y.abs() < spacing || x%2 == 0 && inside_body) // spine and vertical bones
                    && pos.z.abs() < spacing; // only in the middle of the body

                let tail = x <= tail_max_x && x >= x_min && tail_length > 0.0;
                let tail = tail && {
                    let progress = 1. - ((pos.x - x_min as f32 * spacing) / tail_length).clamp(0.0, 1.0);
                    let height = tail_max_height * progress;
                    let width = tail_max_width * progress;
                    height > 0.0
                        && width > 0.0
                        && (pos.y.abs() / height).powi(2) + (pos.z.abs() / width).powi(2) <= 1.0
                };

                let eye_center_x = body_center_x + body_half_length * 0.3;//0.2;
                let eye_center_y = body_radius_y * 0.5;//0.45;
                let eye_radius_x = body_half_length * 0.3;//0.18;
                let eye_radius_y = body_radius_y * 0.22;
                let eye_radius_z = body_radius_z * 0.3;//0.25;

                let left_eye = /*body
                    &&*/ ((pos.x - eye_center_x) / eye_radius_x).powi(2)
                        + ((pos.y - eye_center_y) / eye_radius_y).powi(2)
                        + ((pos.z + body_radius_z * 0.5) / eye_radius_z).powi(2)
                        <= 1.0
                    && pos.y > 0.0;

                let right_eye = /*body
                    &&*/ ((pos.x - eye_center_x) / eye_radius_x).powi(2)
                        + ((pos.y - eye_center_y) / eye_radius_y).powi(2)
                        + ((pos.z - body_radius_z * 0.5) / eye_radius_z).powi(2)
                        <= 1.0
                    && pos.y > 0.0;

                let eyes = left_eye || right_eye;

                if eyes {
                    positions.push(Vec4::new(pos.x, pos.y, pos.z, 0.0));
                    colors.push(Color::from(Srgba::rgb_u8(0, 70, 0)));// dark green for eyes
                }
                else if body {
                    positions.push(Vec4::new(pos.x, pos.y, pos.z, 0.0));
                    if inside_body {
                        if bones {
                            colors.push(Color::from(Srgba::rgb_u8(255, 255, 255)));// white for bones
                        } else {
                            colors.push(Color::from(Srgba::rgb_u8(255, 180, 50)));// orange for inner body
                        }
                    } else {
                        colors.push(Color::from(Srgba::rgb_u8(255, 150, 50)));// darker orange for surface of the body
                    }
                } else if tail {
                    positions.push(Vec4::new(pos.x, pos.y, pos.z, 0.0));
                    colors.push(Color::from(Srgba::rgb_u8(255, 0, 150)));// pink for tail
                }
            }
        }
    }

    Atoms4D { positions, colors }
}