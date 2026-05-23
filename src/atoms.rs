use bevy::color::palettes::css::GOLD;
use bevy::prelude::*;

/// Sequence of atoms, represented by equal numbered positions and colors.
pub struct Atoms4D {
    pub positions: Vec<Vec4>,
    pub colors: Vec<Color>,
}

// Helper functions to create different 3D and 4D objects as sequences of atoms.
// Objects are centered around the origin.

/// Creates a 3D cube composed of atoms.
/// `size` is the length of an edge of the cube, and `number_per_side` is how many smaller cubes there are along each edge.
pub(crate) fn create_cube_surface_colorful(size_atom: f32, number_per_side: usize) -> Atoms4D {
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

pub(crate) fn create_cube_4d(size_atom: f32, number_per_side: usize) -> Atoms4D {
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

pub(crate) fn create_cube_4d_surface(size_atom: f32, number_per_side: usize) -> Atoms4D {
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

pub(crate) fn create_cube_4d_corners(size_atom: f32, number_per_side: usize) -> Atoms4D {
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
pub(crate) fn create_cube_4d_edges(size_atom: f32, number_per_side: usize) -> Atoms4D {
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

pub(crate) fn create_heart_3d(size_atom: f32, number_per_side: usize) -> Atoms4D {
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

pub(crate) fn create_sphere_4d(radius: f32, number_per_side: usize) -> Atoms4D {
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

pub(crate) fn create_square(size_atom: f32, number_per_side: usize) -> Atoms4D {
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

pub(crate) fn create_circle(size_atom: f32, number_per_side: usize) -> Atoms4D {
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

pub(crate) fn create_cube_surface(size_atom: f32, number_per_side: usize) -> Atoms4D {
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

pub(crate) fn create_square_surface(size_atom: f32, number_per_side: usize) -> Atoms4D {
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

pub(crate) fn create_cube_edges(size_atom: f32, number_per_side: usize) -> Atoms4D {
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
}
