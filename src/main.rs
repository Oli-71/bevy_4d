use std::f32::consts::PI;

use bevy::{
    color::palettes::tailwind::*, picking::pointer::PointerInteraction,
    prelude::*,
};

mod scene4d;
use scene4d::*;

fn main() {
    App::new()
        // MeshPickingPlugin is not a default plugin
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .insert_resource(Scene {
            scene_4d: Scene4D::new(),
        })
        .add_systems(Startup, setup_scene)
        .add_systems(
            Update,
            (
                draw_mesh_intersections,
                transform_scene_4d,
            ),
        )
        .run();
}

/// A marker component for our shapes so we can query them separately from the ground plane.
#[derive(Component)]
struct ControlShape;

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct Atom {
    index: usize,
}

#[derive(Resource)]
struct Scene {
    scene_4d: Scene4D,
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    scene: ResMut<Scene>,
) {
    // Set up the materials.
    let white_matl = materials.add(Color::WHITE);
    let ground_matl = materials.add(Color::from(GRAY_300));
    let hover_matl = materials.add(Color::from(CYAN_300));
    let pressed_matl = materials.add(Color::from(YELLOW_300));

    // cube to rotate and trigger 4d view
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(white_matl.clone()),
            Transform::from_xyz(0.0, 5.0, 0.0).with_rotation(Quat::from_rotation_x(-PI / 4.)),
            ControlShape,
        ))
        .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
        .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
        .observe(update_material_on::<Pointer<Press>>(pressed_matl.clone()))
        .observe(update_material_on::<Pointer<Release>>(hover_matl.clone()))
        .observe(rotate_on_drag)
        .observe(goto_4d_on_press);
    
    // 4d scene
    for (index, position) in scene.scene_4d.atoms.positions.iter().enumerate() {
        commands
            .spawn((
                Mesh3d(meshes.add(Sphere::new(scene.scene_4d.size_of_atom * 0.8))),
                MeshMaterial3d(materials.add(scene.scene_4d.atoms.colors[index])),
                Transform::from_translation(vec3(position.x, position.y, position.z)),
                Atom { index }, // to identify these entities
            ))
            .observe(rotate_object_on_drag);
    }

    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(ground_matl.clone()),
        Transform::from_translation(vec3(0., -5., 0.)),
        Pickable::IGNORE, // Disable picking for the ground plane.
        Ground,
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    ));

    // Instructions
    commands.spawn((
        Text::new("Hover over the shapes to pick them\nDrag to rotate"),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));
}

/// Returns an observer that updates the entity's material to the one specified.
fn update_material_on<E: EntityEvent>(
    new_material: Handle<StandardMaterial>,
) -> impl Fn(On<E>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    // An observer closure that captures `new_material`. We do this to avoid needing to write four
    // versions of this observer, each triggered by a different event and with a different hardcoded
    // material. Instead, the event type is a generic, and the material is passed in.
    move |event, mut query| {
        if let Ok(mut material) = query.get_mut(event.event_target()) {
            material.0 = new_material.clone();
        }
    }
}

/// A system that draws hit indicators for every pointer.
fn draw_mesh_intersections(pointers: Query<&PointerInteraction>, mut gizmos: Gizmos) {
    for (point, normal) in pointers
        .iter()
        .filter_map(|interaction| interaction.get_nearest_hit())
        .filter_map(|(_entity, hit)| hit.position.zip(hit.normal))
    {
        gizmos.sphere(point, 0.05, RED_500);
        gizmos.arrow(point, point + normal.normalize() * 0.5, PINK_100);
    }
}

/// A system that transforms the 4D scene and updates the positions of the corresponding entities in the 3D world.
fn transform_scene_4d(
    mut query: Query<(&mut Transform, &mut Visibility, &Atom)>,
    time: Res<Time>,
    scene: Res<Scene>,
) {
    // get updated positions for all atoms in the 4D scene based on the current time (for animation)
    let new_positions = scene.scene_4d.transform_scene(time.elapsed_secs());

    // update the transforms of the atom entities based on the rotated positions
    for (mut transform, mut visibility, atom_entity) in &mut query {
        let index = atom_entity.index;
        if let Some(position) = new_positions.get(index) {
            transform.translation = vec3(position.x, position.y, position.z);
            *visibility = if scene.scene_4d.is_atom_visible(*position) {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

/// An observer to rotate an entity when it is dragged
fn rotate_on_drag(drag: On<Pointer<Drag>>, mut transforms: Query<&mut Transform>) {
    let mut transform = transforms.get_mut(drag.entity).unwrap();
    transform.rotate_y(drag.delta.x * 0.02);
    transform.rotate_x(drag.delta.y * 0.02);
}

/// An observer to rotate an Object4d entity.
fn rotate_object_on_drag(
    drag: On<Pointer<Drag>>,
    mut atoms: Query<&mut Atom>,
    mut scene: ResMut<Scene>,
) {
    // Update the drag state in the 4D scene based on the drag delta and the atom index
    let atom = atoms.get_mut(drag.entity).unwrap();
    scene.scene_4d.drag_object_from_atom(atom.index, drag.delta); 
    
}

/// An observer to trigger goto_4d when a ControlShape is pressed.
fn goto_4d_on_press(
    _press: On<Pointer<Press>>,
    mut vis: Query<&mut Visibility , With<Ground>>,
    mut scene: ResMut<Scene>,
    time: Res<Time>,
) {
    scene.scene_4d.toggle_4d_view(time.elapsed_secs());
    
    for mut v in &mut vis {
        *v = if scene.scene_4d.is_4d_view() {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
    }
}