use std::f32::consts::PI;

use bevy::{
    camera::{SubCameraView, visibility},
    color::palettes::tailwind::*,
    light::{NotShadowCaster, NotShadowReceiver},
    picking::pointer::PointerInteraction,
    prelude::*, 
    //render::render_asset::RenderAsset,
};

use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

mod scene4d;
use scene4d::*;
mod smooth;

const SCALE: f32 = 6.0; // global scaling to fit the standard screen

const CAMERA_STANDARD_TARGET: Vec3 = vec3(0., 0., 0.);
const CAMERA_SPACELAND_POSITION: Vec3 = vec3(0., 70., 140.);
const CAMERA_FLATLAND_POSITION: Vec3 = vec3(0., 0., 140.);

fn main() {
    App::new()
        // MeshPickingPlugin is not a default plugin
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .insert_resource(Scene {
            scene_4d: Scene4D::new(),
            viewpoint_is_spaceland: false,
            state: StateScene::Planar,
        })
        .add_systems(
            Startup, 
            setup_scene
        )
        .add_systems(
            Update,
            (
                //draw_mesh_intersections,
                transform_scene_4d,
                monitor_scene_4d,
                update_labels,
                update_move_position_smooth,
            ),
        )
        .run();
}

/// A marker component for our shapes so we can query them separately from the ground plane.
#[derive(Component)]
struct ControlShape;

#[derive(Component)]
struct AdvancedControlShape;
#[derive(Component)]
struct CoverPanel;

#[derive(Component)]
struct BackgroundPanel;

#[derive(Component)]
struct MainLight;

#[derive(Component)]
struct Label {
    entity: Entity,
    offset_y: f32, // an offset to position the label above the entity
}

#[derive(Component)]
struct AngleMonitor;

#[derive(Component)]
struct Atom {
    index: usize, // index in the Scene4D's atoms vector, to identify which atom this entity corresponds to
    visible: bool,
}

#[derive(Component)]
struct Instructions;

enum StateScene {
    Planar,
    FlatlandComplete,
    ThreeDimensional,
    SpacelandComplete,
}

#[derive(Resource)]
struct Scene {
    scene_4d: Scene4D,
    viewpoint_is_spaceland: bool,
    state: StateScene,
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    scene: ResMut<Scene>,
    asset_server: Res<AssetServer>,
    //mut images: ResMut<Assets<Image>>,
) {
    // Set up the materials.
    let white_matl = materials.add(Color::WHITE);
    //let ground_matl = materials.add(Color::from(GRAY_300));
    let hover_matl = materials.add(Color::from(CYAN_300));
    let pressed_matl = materials.add(Color::from(YELLOW_300));

    let size_of_controls = 0.2 * SCALE;
    let y_ctr_row1 = 9. * SCALE;
    let y_ctr_row2 = 8. * SCALE;

    // Gray Control Objects
    //////////////////////////////
    
    // Sphere to trigger view point
    let view_point_control_entity = commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(size_of_controls))),
            MeshMaterial3d(white_matl.clone()),
            Transform::from_xyz(-3. * SCALE, y_ctr_row1, 0.),
            ControlShape,
            AdvancedControlShape,
            Visibility::Hidden, 
        ))
        .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
        .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
        .observe(update_material_on::<Pointer<Press>>(pressed_matl.clone()))
        .observe(update_material_on::<Pointer<Release>>(hover_matl.clone()))
        .observe(toggle_view_point_on_press)
        .id();

    // Cone to trigger 4d view
    // also suits as Angle Monitor
    let angle_monitor_entity = commands.spawn((
        Mesh3d(meshes.add(Cone::new(size_of_controls, size_of_controls * 2.0))),
        MeshMaterial3d(white_matl.clone()),
        Transform::from_xyz(0., y_ctr_row1, 0.).with_rotation(Quat::from_rotation_y(PI/2.)),
        ControlShape,
        AngleMonitor,
    ))
    .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
    .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
    .observe(update_material_on::<Pointer<Press>>(pressed_matl.clone()))
    .observe(update_material_on::<Pointer<Release>>(hover_matl.clone()))
    .observe(toggle_4d_on_press)
    .id();

    // Sphere to trigger projection view
    let projection_control_entity = commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(size_of_controls))),
            MeshMaterial3d(white_matl.clone()),
            Transform::from_xyz(3. * SCALE, y_ctr_row1, 0.),
            ControlShape,
            AdvancedControlShape,
            Visibility::Hidden,
        ))
        .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
        .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
        .observe(update_material_on::<Pointer<Press>>(pressed_matl.clone()))
        .observe(update_material_on::<Pointer<Release>>(hover_matl.clone()))
        .observe(toggle_projection_on_press)
        .id();

    // Cube to rotate all objects in the scene by dragging.
    let drag_all_objects_entity = commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(0.5 * SCALE, 0.5 * SCALE, 0.5 * SCALE))),
            MeshMaterial3d(white_matl.clone()),
            Transform::from_xyz(6. * SCALE, y_ctr_row1, 0.),
            ControlShape,
        ))
        .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
        .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
        .observe(update_material_on::<Pointer<Press>>(pressed_matl.clone()))
        .observe(update_material_on::<Pointer<Release>>(hover_matl.clone()))
        .observe(drag_all_objects)
        .id();

    // Slider to adjust speed of 3d rotation
    let slider_3d_rotation_entity = commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(size_of_controls))),
            MeshMaterial3d(white_matl.clone()),
            Transform::from_xyz(-3. * SCALE, y_ctr_row2, 0.),
            ControlShape,
        ))
        .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
        .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
        .observe(update_material_on::<Pointer<Press>>(pressed_matl.clone()))
        .observe(update_material_on::<Pointer<Release>>(hover_matl.clone()))
        .observe(drag_to_adjust_speed)
        .id();

    commands.spawn((
        Mesh3d(meshes.add(Segment3d::new(
            vec3(-3. * SCALE, y_ctr_row2, 0.),
            vec3(3. * SCALE, y_ctr_row2, 0.),
        ))),
        MeshMaterial3d(white_matl.clone()),
    ));

    // Sphere to trigger "see more"
    let show_more_control_entity = commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(size_of_controls))),
            MeshMaterial3d(white_matl.clone()),
            Transform::from_xyz(6. * SCALE, y_ctr_row2, 0.),
            ControlShape,
        ))
        .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
        .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
        .observe(update_material_on::<Pointer<Press>>(pressed_matl.clone()))
        .observe(update_material_on::<Pointer<Release>>(hover_matl.clone()))
        .observe(show_more_on_press)
        .id();

    // slider to adjust higher dimension height (w in Spaceland, y in Flatland)
    let left = -10. * SCALE;
    let slider_height_entity = commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(size_of_controls))),
            MeshMaterial3d(white_matl.clone()),
            Transform::from_xyz(left, 2. * SCALE, 0.),
            ControlShape,
            AdvancedControlShape,
            Visibility::Hidden,
        ))
        .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
        .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
        .observe(update_material_on::<Pointer<Press>>(pressed_matl.clone()))
        .observe(update_material_on::<Pointer<Release>>(hover_matl.clone()))
        .observe(drag_to_adjust_higher_dimension_height)
        .id();

    commands.spawn((
        Mesh3d(meshes.add(Segment3d::new(
            vec3(left, -2. * SCALE, 0.),
            vec3(left, 2. * SCALE, 0.),
        ))),
        MeshMaterial3d(white_matl.clone()),
        AdvancedControlShape,
        Visibility::Hidden,
    ));

    // 4D Scene
    /////////////////////////
    // We spawn an entity for each atom in the 4D scene, and use the Atom component to link them 
    // to their corresponding atoms in the Scene4D. This way, we can easily update their positions and 
    // visibilities based on the state of the Scene4D.
    for (atom_index, position) in scene.scene_4d.atoms.positions.iter().enumerate() {
        commands
            .spawn((
                Mesh3d(meshes.add(Sphere::new(scene.scene_4d.size_of_atom * 0.8 * SCALE))),
                MeshMaterial3d(materials.add(scene.scene_4d.atoms.colors[atom_index])),
                Transform::from_translation(vec3(
                    position.x * SCALE,
                    position.y * SCALE,
                    position.z * SCALE,
                )),
                Atom { index: atom_index, visible: scene.scene_4d.is_planar(atom_index) },
            ))
            .observe(rotate_object_on_drag);
    }

    // Cover Panel to hide invisible 3D-Space for flatland
    let size_of_panel = 30. * SCALE;
    let color = 200;
    let offset_atom_thickness = scene.scene_4d.size_of_atom * 0.5 * SCALE;
    let z_offset =
        scene.scene_4d.size_of_atom * 1.1 * SCALE * scene.scene_4d.number_of_atoms_per_side as f32;

    // top horizontal panel 
    // (high alpha to mark the 2D slice when looking at it from the spaceland view)
    commands.spawn((
        Mesh3d(
            meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(size_of_panel, size_of_panel)
                    .subdivisions(10),
            ),
        ),
        MeshMaterial3d(materials.add(Color::srgba_u8(color, color, color, 200))),
        Transform::from_translation(vec3(
            0.,
            0. + offset_atom_thickness,
            z_offset - size_of_panel / 2.,
        )),
        Pickable::IGNORE,
        NotShadowReceiver,
        CoverPanel,
    ));

    // bottom horizontal panel
    commands.spawn((
        Mesh3d(
            meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(size_of_panel, size_of_panel)
                    .subdivisions(10),
            ),
        ),
        MeshMaterial3d(materials.add(Color::srgba_u8(color, color, color, 200))),
        Transform::from_translation(vec3(
            0.,
            0. - offset_atom_thickness,
            z_offset - size_of_panel / 2.,
        )),
        Pickable::IGNORE,
        NotShadowReceiver,
        CoverPanel,
    ));

    let y_size = z_offset / 2.9;
    // vertical top panel
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(
            vec3(0., 0., 1.),
            vec2(size_of_panel, y_size),
        ))),
        MeshMaterial3d(materials.add(Color::srgba_u8(color, color, color, 100))),
        Transform::from_translation(vec3(
            0.,
            0. + y_size + offset_atom_thickness,
            z_offset,
        )),
        Pickable::IGNORE,
        NotShadowReceiver,
        CoverPanel,
    ));

    // vertical bottom panel
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(
            vec3(0., 0., 1.),
            vec2(size_of_panel, y_size),
        ))),
        MeshMaterial3d(materials.add(Color::srgba_u8(color, color, color, 100))),
        Transform::from_translation(vec3(
            0.,
            0. - y_size - offset_atom_thickness,
            z_offset,
        )),
        Pickable::IGNORE,
        NotShadowReceiver,
        CoverPanel,
    ));

    // A helper closure to spawn a thick line between two points.
    let mut spawn_thick_line = |start: Vec3, end: Vec3, thickness: f32| -> Entity {
        let direction = end - start;
        let length = direction.length();
        
        // Calculate the rotation needed to align the cylinder with the direction vector.
        let rotation = Quat::from_rotation_arc(Vec3::Y, direction.normalize());

        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(thickness, length))),
            MeshMaterial3d(materials.add(Color::srgba_u8(255, 0, 0, 200))),
            Transform::from_translation(start + direction / 2.0)
                .with_rotation(rotation),
        )).id()
    };

    // Flatland Indicator Lines
    let flatland_top_line_entity = spawn_thick_line(
        vec3(- size_of_panel / 2., offset_atom_thickness, z_offset),
        vec3( size_of_panel / 2., offset_atom_thickness, z_offset),
        0.02 * SCALE,
    );
    
    let _flatland_bottom_line_entity = spawn_thick_line(
        vec3(- size_of_panel / 2., - offset_atom_thickness, z_offset),
        vec3( size_of_panel / 2., - offset_atom_thickness, z_offset),
        0.02 * SCALE,
    );

    // Background Panel
    
    /* // 1. Schachbrett-Textur erstellen (z.B. 2x2 oder 8x8)
    let size = 8;
    let mut image = create_checkerboard_image(size);
    // WICHTIG: Damit die Linien scharf bleiben
    //image.sampler = bevy::render::texture::ImageSampler::nearest();
    let texture_handle = images.add(image);

    // 2. Material mit der Textur erstellen
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        // Optional: unlit: true, // Wenn keine Beleuchtung gewünscht ist
        ..default()
    }); */

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(
            vec3(0., 0., 1.),
            vec2(150., 95.),
        ))),
        //MeshMaterial3d(material_handle),
        MeshMaterial3d(materials.add(Color::srgba_u8(10, 10, 5, 255))),
        Transform::from_translation(vec3(0., 0., -150.)),
        Pickable::IGNORE,
        NotShadowReceiver,
        BackgroundPanel,
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 50_000_000. * SCALE,
            range: 500.0 * SCALE,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0 * SCALE, 16.0 * SCALE, 8.0 * SCALE),
        MainLight,
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 50_000_000. * SCALE,
            range: 500.0 * SCALE,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0 * SCALE, -16.0 * SCALE, 8.0 * SCALE),
        MainLight,
    ));

    // for flatland
    commands.spawn((
        PointLight {//DirectionalLight
            shadows_enabled: true,
            intensity: 50_000_000. * SCALE,
            range: 500.0 * SCALE,
            ..default()
        },
        NotShadowCaster, // this light should not cast shadows to avoid too dark shadows in the flatland view
        //Transform::from_xyz(8.0 * SCALE, 0.0 * SCALE, 8.0 * SCALE),
        Transform::from_xyz(0.0 * SCALE, 0.0 * SCALE, 16.0 * SCALE),
    ));

    // Camera
    // size of window; only the ratio is important
    let x = 16;
    let y = 14;
    commands.spawn((
        Camera3d::default(),
        Camera {
            // lens shift for nice flatland -> we look within the plane
            sub_camera_view: Some(SubCameraView {
                full_size: UVec2::new(x, y),
                offset: Vec2::ZERO,
                size: UVec2::new(x, y * 7 / 10),
            }),
            order: 3,
            ..default()
        },
        Transform::from_translation(CAMERA_FLATLAND_POSITION)
            .looking_at(CAMERA_STANDARD_TARGET, Vec3::Y),
        smooth::PositionTarget::new(CAMERA_FLATLAND_POSITION), // can be moved smoothly
    ));

    // Text Labels
    let text_style = TextFont {
        font: asset_server.load("fonts/CenturyGothicPaneuropeanThin.ttf"),
        ..default()
    };

    let label_text_style = (text_style.clone(), TextColor(Color::srgb_u8(200, 200, 200)));

    // A helper closure to add labels to the control objects.
    // We use a closure here to avoid repeating the same code for each label,
    // since they all have the same structure (a parent node with absolute positioning and a child text node with the label).
    let mut spawn_label = |entity: Entity, label: &str, offset: f32, visibility: Visibility| {
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                ..default()
            },
            Label { entity, offset_y: offset },
            AdvancedControlShape, // to show/hide labels together with advanced controls
            visibility,
            children![(
                Text::new(label),
                label_text_style.clone(),
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::ZERO,
                    ..default()
                },
                TextLayout::default().with_no_wrap(),
            )],
        ));
    };

    spawn_label(view_point_control_entity, "View Point", 0.9, Visibility::Hidden);
    spawn_label(angle_monitor_entity, "Hyper", 0.9, Visibility::Visible);
    spawn_label(projection_control_entity, "Projection", 0.9, Visibility::Hidden);
    spawn_label(drag_all_objects_entity, "______synchronize objects", 0.9, Visibility::Visible);
    spawn_label(slider_3d_rotation_entity, "Rotation Speed", 0.9, Visibility::Visible);
    spawn_label(slider_height_entity, "Higher Dimension Height", 0.9, Visibility::Hidden);
    spawn_label(flatland_top_line_entity, "Flatland", 0.0, Visibility::Visible);
    spawn_label(show_more_control_entity, "Show more", 0.9, Visibility::Visible);

    // Instructions
    commands.spawn((
        Text::new("What would our three-dimensional world look like from a location in the fourth dimension? Let’s start with what inhabitants of a two-dimensional world see...
Look into the flatland gap below. Rotate the objects... Then feel free to try all the options (above). The 'Hyper' cone moves your location slowly into the third dimension and back.
If you feel familiar with the flatlander's understanding of dimension jump, click 'Show more.'"),
        Node {
            position_type: PositionType::Absolute,
            top: px(170),
            left: px(12),
            ..default()
        },
        Instructions,
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
    mut scene: ResMut<Scene>,
) {
    // get updated positions for all atoms in the 4D scene based on the current time (for animation)
    let new_positions = scene.scene_4d.transform_scene(time.elapsed_secs());

    // update the transforms of the atom entities based on the rotated positions
    for (mut transform, mut visibility, atom_entity) in &mut query {
        if !atom_entity.visible {
            *visibility = Visibility::Hidden;
            continue;
        }
        let index = atom_entity.index;
        if let Some(position) = new_positions.get(index) {
            *visibility = if scene.scene_4d.is_atom_visible(*position) {
                transform.translation =
                    vec3(position.x * SCALE, position.y * SCALE, position.z * SCALE);
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

/// smooth: A system to smoothly move an entity (with PositionTarget component)
fn update_move_position_smooth(
    time: Res<Time>,
    mut position_query: Query<(&mut smooth::PositionTarget, &mut Transform)>,
) {
    for (mut position, mut trafo) in &mut position_query {
        // calculate the position motion based on the difference between where the object is
        // and where it should be; the greater the distance, the faster the motion;
        // smooth out the object movement using the frame time
        if position.has_been_reached() {
            continue;
        }

        let t = position.get_next_translation(trafo.translation, time.delta_secs());
        *trafo = trafo
            .looking_at(CAMERA_STANDARD_TARGET, Vec3::Y)
            .with_translation(t);
    }
}

/// A system to update label positions and orientations 
/// based on the position of the camera and the labeled entities
fn update_labels(
    labeled: Query<&GlobalTransform>,
    mut labels: Query<(&mut Node, &Label)>,
    camera3d: Single<( &mut Camera, &GlobalTransform), With<Camera3d>>,
) {
    let (camera, camera_global_transform) = camera3d.into_inner();

    for (mut node, label) in &mut labels {
        let world_position = labeled.get(label.entity).unwrap().translation() + label.offset_y * Vec3::Y;

        let viewport_position = camera
            .world_to_viewport(camera_global_transform, world_position)
            .unwrap();

        // position the label's node in the viewport based on the world position of the labeled entity
        node.top = px(viewport_position.y);
        node.left = px(viewport_position.x);
    }
}

/// A system to reflect the Scene4d state in the the general scene
fn monitor_scene_4d(
    mut visibilities: Query<&mut Visibility, With<BackgroundPanel>>,
    mut trafos: Query<&mut Transform, With<AngleMonitor>>,
    //mut trafos_background: Query<&mut Transform, With<BackgroundPanel>>,
    scene: Res<Scene>,
) {
    // show background if 4D rotation is Zero
    for mut visibility in &mut visibilities {
        let angle = scene.scene_4d.get_angle_high_dimension().abs();
        *visibility = if angle < 0.2 || (PI - angle).abs() < 0.2 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    //visualize 4D rotation by cone
    for mut trafo in &mut trafos {
       *trafo = Transform::from_rotation(Quat::from_rotation_x(
           PI / 2.0 + scene.scene_4d.get_angle_high_dimension(),
       ));
       trafo.translation = vec3(0., 9. * SCALE, 0.);
    }
}

/// An observer to rotate an entity when it is dragged
fn drag_all_objects(
    drag: On<Pointer<Drag>>,
    mut transforms: Query<&mut Transform, Without<Camera3d>>,
    mut scene: ResMut<Scene>,
) {
    let mut transform = transforms.get_mut(drag.entity).unwrap();
    transform.rotate_y(drag.delta.x * 0.01);
    transform.rotate_x(drag.delta.y * 0.01);

    scene.scene_4d.drag_all_objects(drag.delta);
}

/// An observer to switch global view
fn toggle_view_point_on_press(
    _press: On<Pointer<Press>>,
    mut scene: ResMut<Scene>,
    mut camera3ds: Query<&mut smooth::PositionTarget, With<Camera3d>>,
    //mut text: Query<&mut Text, With<Instructions>>,
) {
    scene.viewpoint_is_spaceland = !scene.viewpoint_is_spaceland;
    for mut camera in &mut camera3ds {
        if scene.viewpoint_is_spaceland {
            camera.set_target(CAMERA_SPACELAND_POSITION);
        } else {
            camera.set_target(CAMERA_FLATLAND_POSITION);
        }
    }
}

/// An observer to switch to the next scene state.
fn show_more_on_press(
    _press: On<Pointer<Press>>,
    mut text: Query<&mut Text, With<Instructions>>,
    mut atoms: Query<&mut Atom>,
    mut scene: ResMut<Scene>,
    mut visibilities: Query<&mut Visibility, With<AdvancedControlShape>>,
) {
    scene.state = match scene.state {
        StateScene::Planar => StateScene::FlatlandComplete,
        StateScene::FlatlandComplete => StateScene::ThreeDimensional,
        StateScene::ThreeDimensional => StateScene::SpacelandComplete,
        StateScene::SpacelandComplete => StateScene::SpacelandComplete, // no further state
    };

    scene.scene_4d.reset_view();

    match scene.state {
        StateScene::Planar => {}, // initial state, nothing to do
        StateScene::FlatlandComplete => {
            // add projection
            // add view point
            // add height slider
            for mut visibility in &mut visibilities {
                *visibility = Visibility::Visible;
            }

            for mut text in &mut text {
                text.0 = "Now we have also two three-dimensional cubes extending into our two-dimensional world.\nTry to imagine how difficult it is for the inhabitants of Flatland to grasp their structure. Even if they could jump to the third dimension it would be a challenge?!...\n\nBTW: There are a few more options now available. Try and combine them.\nTo experience the 3D-to-4D effect, click 'Show more'.".to_string();
            }

            for mut atom in &mut atoms {
                if scene.scene_4d.is_2d(atom.index) {
                    atom.visible = true;
                }
            }
        },
        StateScene::ThreeDimensional => {
            for mut text in &mut text {
                text.0 = "Study how 3D-Object behave if we go up to a fourth dimension view point (Press 'Hyper').\n'Show more' will add actual 4D objects to the scene. It becomes crazy ;-)".to_string();
            }

            for mut atom in &mut atoms {
                if !scene.scene_4d.is_4d(atom.index) {
                    atom.visible = true;
                }
            }
        },
        StateScene::SpacelandComplete => {
            // hide show more button and instructions
            for mut text in &mut text {
                text.0 = "Take a few Minutes to compare Flatlander's and Spacelander's experiences with an extra dimension. (A key is the 'Higher Dimension Height' slider...)".to_string();
            }

            for mut atom in &mut atoms {
                atom.visible = true;
            }
        },
    }
}

/// An observer to adjust the speed of 3D rotation in the 4D scene.
fn drag_to_adjust_speed(
    drag: On<Pointer<Drag>>,
    mut transforms: Query<&mut Transform>,
    mut scene: ResMut<Scene>,
) {
    let mut transform = transforms.get_mut(drag.entity).unwrap();

    let sensitivity = 0.02;
    let x = transform.translation.x + drag.delta.x * SCALE * sensitivity;
    let bound = 3.0 * SCALE; // Set a x coordinate bound for how far the control can be dragged
    if (-bound..=bound).contains(&x) {
        transform.translation.x = x;
        // map x to 0..1
        let speed = (x + bound) / (2.0 * bound);
        scene.scene_4d.adjust_3d_rotation_speed(speed);
    }
}

/// An observer to adjust the higher dimension height.
fn drag_to_adjust_higher_dimension_height(
    drag: On<Pointer<Drag>>,
    mut transforms: Query<&mut Transform>,
    mut scene: ResMut<Scene>,
) {
    if scene.scene_4d.is_projection_view {
        return; // In projection view, the higher dimension height has no effect, so we don't allow adjusting it.
    }

    let mut transform = transforms.get_mut(drag.entity).unwrap();

    let sensitivity = 0.02;
    let y = transform.translation.y - drag.delta.y * SCALE * sensitivity;
    let bound = 2.0 * SCALE; // Set a y coordinate bound for how far the control can be dragged
    if (-bound..=bound).contains(&y) {
        transform.translation.y = y;
        // map y to -1..0
        let height = (y - bound) / (2.0 * bound);
        scene.scene_4d.adjust_higher_dimension_height(height);
    }
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

/// An observer to trigger toggle_4d when the ControlShape is pressed.
fn toggle_4d_on_press(_press: On<Pointer<Press>>, mut scene: ResMut<Scene>, time: Res<Time>) {
    scene
        .scene_4d
        .toggle_high_dimension_view(time.elapsed_secs());
}

/// An observer to trigger toggle_projection when the ControlShape is pressed.
fn toggle_projection_on_press(_press: On<Pointer<Press>>, mut scene: ResMut<Scene>) {
    scene.scene_4d.toggle_projection_view();
}

fn create_checkerboard_image(size: u32) -> Image {
    let mut pixels = vec![0u8; (size * size * 4) as usize];
    for y in 0..size {
        for x in 0..size {
            let color = if (x + y) % 2 == 0 {
                [255, 255, 255, 255] // Weiß
            } else {
                [0, 0, 0, 255] // Schwarz
            };
            let i = (y * size + x) as usize * 4;
            pixels[i..i + 4].copy_from_slice(&color);
        }
    }

    Image::new(
        Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        pixels,
        TextureFormat::Rgba8UnormSrgb,
        bevy::asset::RenderAssetUsages::default()//RenderAssetUsages::default(),
        //RenderAssetUsages::default(),
    )
}