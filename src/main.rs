use std::f32::consts::PI;

use bevy::{
    camera::{SubCameraView},
    color::palettes::tailwind::*,
    light::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};

mod atoms;
mod scene4d;
use scene4d::*;
mod smooth;

const SCALE: f32 = 6.0; // global scaling to fit the standard screen

const CAMERA_STANDARD_TARGET: Vec3 = vec3(0., 0., 0.);
const CAMERA_SPACELAND_POSITION: Vec3 = vec3(0., 70., 140.);
const CAMERA_FLATLAND_POSITION: Vec3 = vec3(0., 0., 140.);

const SLIDER_HEIGHT_CONTROL_OFFSET_Y: f32 = 5. * SCALE;

const LABEL_VIEW_POINT: &str = "View Point";
const LABEL_HYPER: &str = "Hyper";
const LABEL_PROJECTION: &str = "Projection";
const LABEL_SYNC_DRAG: &str = "______Synchronized Dragging";
const LABEL_HORIZONTAL_ROTATION: &str = "Horizontal Rotation";
const LABEL_HIGHER_DIMENSION_OFFSET: &str = "Higher Dimension Offset";
const LABEL_FLATLAND: &str = "Flatland";
const LABEL_SHOW_MORE: &str = "Show more";

const INSTRUCTIONS_INITIAL: &str = r#"What would our three-dimensional world look like from a location in the fourth dimension? Let’s start with what inhabitants of a two-dimensional world see:
Look into the flatland gap below. Flatlanders can only see what happens between the red lines. Rotate the objects... Then try the gray controls at the top.
The 'Hyper' cone moves your location slowly into the third dimension and back. Wow, From 3D the Flatlanders can look into closed shapes!
If you feel familiar with the Flatlander's understanding of dimension jump, click 'Show more'."#;

const INSTRUCTIONS_FLATLAND_COMPLETE: &str = r#"Now we have also two 3D cubes extending into our two-dimensional world.
Try to imagine how difficult it is for the inhabitants of Flatland to grasp their structure. Even if they could jump to the third dimension ('Hyper') it would be a challenge.

BTW: There are a few more options available. Try them all.
Surprised what happens when you combine 'Projection' and 'Hyper'? Add some 'Horizontal Rotation' and 'View Point'. Now you will understand...
When you are ready to experience the 3D-to-4D effect, click 'Show more'."#;

const INSTRUCTIONS_THREE_DIMENSIONAL: &str = r#"Study how the two new 3D-Objects behave if we go up to a fourth dimension view point (Again: 'Hyper').
Are there are similarities to flatlander's experiences?

'Show more' will add 4D-cubes to the scene. It will become crazy ;-)"#;

const INSTRUCTIONS_SPACELAND_COMPLETE: &str = r#"Take a few minutes to compare Flatlander's and Spacelander's experiences with an extra dimension. The 'Higher Dimension Offset' slider and 'Synchronized Dragging' can help...
‘Projection’ pushes the atoms from the higher dimensions into the visible range. But note that actually we can never see 4D objects completely.  
...Imagine a human being seen from 4D."#;

const INSTRUCTIONS_SPACELAND_ONLY: &str = r#"Try all Transformations."#;

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
                transform_scene_4d,
                monitor_scene_4d,
                update_labels,
                update_move_position_smooth,
            ),
        )
        .run();
}

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

#[derive(PartialEq)]
enum StateScene {
    Planar,
    FlatlandComplete,
    ThreeDimensional,
    SpacelandComplete,
    SpacelandOnly,
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
            AdvancedControlShape,
            Visibility::Hidden, 
        ))
        .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
        .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
        .observe(update_material_on::<Pointer<Press>>(pressed_matl.clone()))
        .observe(update_material_on::<Pointer<Release>>(hover_matl.clone()))
        .observe(toggle_view_point_on_press)
        .id();

    //debug
    spawn_tripod(&mut commands, &mut meshes, &mut materials,
        vec3(-5. * SCALE, y_ctr_row2, 0.), 
        SCALE
    );

    // Cone to trigger 4d view
    // also suits as Angle Monitor
    let angle_monitor_entity = commands.spawn((
        Mesh3d(meshes.add(Cone::new(size_of_controls, size_of_controls * 2.0))),
        MeshMaterial3d(white_matl.clone()),
        Transform::from_xyz(0., y_ctr_row1, 0.).with_rotation(Quat::from_rotation_y(PI/2.)),
        AdvancedControlShape,
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
            Transform::from_xyz(left, 2. * SCALE + SLIDER_HEIGHT_CONTROL_OFFSET_Y, 0.),
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
            vec3(left, -2. * SCALE + SLIDER_HEIGHT_CONTROL_OFFSET_Y, 0.),
            vec3(left, 2. * SCALE + SLIDER_HEIGHT_CONTROL_OFFSET_Y, 0.),
        ))),
        MeshMaterial3d(white_matl.clone()),
        AdvancedControlShape,
        Visibility::Hidden,
    ));

    // 4D-Scene: Flatland and Spaceland
    spawn_scene(&mut commands, &mut meshes, &mut materials, &scene);

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

    spawn_label(view_point_control_entity, LABEL_VIEW_POINT, 0.9, Visibility::Hidden);
    spawn_label(angle_monitor_entity, LABEL_HYPER, 0.9, Visibility::Visible);
    spawn_label(projection_control_entity, LABEL_PROJECTION, 0.9, Visibility::Hidden);
    spawn_label(drag_all_objects_entity, LABEL_SYNC_DRAG, 0.9, Visibility::Visible);
    spawn_label(slider_3d_rotation_entity, LABEL_HORIZONTAL_ROTATION, 0.9, Visibility::Visible);
    spawn_label(slider_height_entity, LABEL_HIGHER_DIMENSION_OFFSET, 0.9, Visibility::Hidden);
    spawn_label(flatland_top_line_entity, LABEL_FLATLAND, 0.0, Visibility::Visible);
    spawn_label(show_more_control_entity, LABEL_SHOW_MORE, 0.9, Visibility::Visible);

    // Instructions
    commands.spawn((
        Text::new(INSTRUCTIONS_INITIAL),
        Node {
            position_type: PositionType::Absolute,
            top: percent(20.),
            left: percent(10.),
            right: percent(40.),
            ..default()
        },
        TextFont {
            font: asset_server.load("fonts/CenturyGothicPaneuropeanThin.ttf"),
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        BackgroundColor(Color::srgba_u8(10, 10, 10, 50)),
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
    mut text: Query<(&mut Text, &mut Node), With<Instructions>>,
    mut atoms: Query<(Entity, &mut Atom)>,
    mut scene: ResMut<Scene>,
    mut visibilities: Query<&mut Visibility, With<AdvancedControlShape>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    scene.state = match scene.state {
        StateScene::Planar => StateScene::FlatlandComplete,
        StateScene::FlatlandComplete => StateScene::ThreeDimensional,
        StateScene::ThreeDimensional => StateScene::SpacelandComplete,
        StateScene::SpacelandComplete => StateScene::SpacelandOnly,
        StateScene::SpacelandOnly => StateScene::SpacelandOnly, // no further state
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

            for (mut text, mut node) in &mut text {
                text.0 = INSTRUCTIONS_FLATLAND_COMPLETE.to_string();
                //middle right position
                node.top = percent(22.);
                node.left = percent(43.);
                node.right = percent(5.);
            }

            for (entity, mut atom) in &mut atoms {
                if scene.scene_4d.is_2d(atom.index) {
                    atom.visible = true;
                }
            }
        },
        StateScene::ThreeDimensional => {
            for (mut text, mut node) in &mut text {
                text.0 = INSTRUCTIONS_THREE_DIMENSIONAL.to_string();
                //middle more right position
                node.top = percent(22.);
                node.left = percent(60.);
                node.right = percent(5.);
            }

            for (entity, mut atom) in &mut atoms {
                if !scene.scene_4d.is_4d(atom.index) {
                    atom.visible = true;
                }
            }
        },
        StateScene::SpacelandComplete => {
            for (mut text, mut node) in &mut text {
                text.0 = INSTRUCTIONS_SPACELAND_COMPLETE.to_string();
                //left bottom position
                node.top = percent(80.);
                node.left = percent(3.);
                node.right = percent(46.);
            }

            for (entity, mut atom) in &mut atoms {
                atom.visible = true;
            }
        },
        StateScene::SpacelandOnly => {
            // hide show more button and instructions
            for (mut text, mut node) in &mut text {
                text.0 = INSTRUCTIONS_SPACELAND_ONLY.to_string();
                //left bottom position
                node.top = percent(80.);
                node.left = percent(3.);
                node.right = percent(46.);
            }

            //remove all atoms
            for (entity, atom) in atoms {
                commands.entity(entity).despawn();
            }

            scene.scene_4d = Scene4D::new_complex_scene();

            spawn_scene(&mut commands, &mut meshes, &mut materials, &scene);
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
    let y = transform.translation.y - drag.delta.y * SCALE * sensitivity - SLIDER_HEIGHT_CONTROL_OFFSET_Y;
    let bound = 2.0 * SCALE; // Set a y coordinate bound for how far the control can be dragged
    if (-bound..=bound).contains(&y) {
        transform.translation.y = y + SLIDER_HEIGHT_CONTROL_OFFSET_Y;
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

fn spawn_scene (
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    scene: & ResMut<Scene>,
) {
    // 4D Scene
    /////////////////////////
    // We spawn an entity for each atom in the 4D scene, and use the Atom component to link them 
    // to their corresponding atoms in the Scene4D. This way, we can easily update their positions and 
    // visibilities based on the state of the Scene4D.
    let sphere_shape = scene.scene_4d.is_2row_structure;
    let radius = scene.scene_4d.size_of_atom * 0.8 * SCALE;
    // Create a single Handle<Mesh> so both branches have the same type
    let atom_mesh_handle = if sphere_shape {
        meshes.add(Sphere::new(radius))
    } else {
        meshes.add(Cuboid::new(2.0 * radius, 2.0 * radius, 2.0 * radius))
        //meshes.add(Sphere::new(radius))
    };
    for (atom_index, position) in scene.scene_4d.atoms.positions.iter().enumerate() {
        commands
            .spawn((
                Mesh3d(atom_mesh_handle.clone()),
                MeshMaterial3d(materials.add(scene.scene_4d.atoms.colors[atom_index])),
                Transform::from_translation(vec3(
                    position.x * SCALE,
                    position.y * SCALE,
                    position.z * SCALE,
                )),
                Atom { index: atom_index, visible: scene.state == StateScene::SpacelandOnly || scene.scene_4d.is_planar(atom_index) },
            ))
            .observe(rotate_object_on_drag);
    }
}

fn spawn_tripod (
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    scale: f32
) {
    let length = 1.0;
    let radius = 0.05 * length;
    // Parent Entity (Holds position/transform)
    commands.spawn((
        Name::new("Tripod"),
        Transform::from_scale(vec3(scale,scale,scale)).with_translation(position),
        Visibility::default(), // Erforderlich, damit Kinder sichtbar sind
    ))
    .with_children(|parent| {
        //Center gray
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(2.*radius))),
            MeshMaterial3d(materials.add(Color::from(Srgba::gray(0.1)))),
        ));

        //y-Axis green
        parent.spawn((
            Mesh3d(meshes.add(Cylinder::new(radius, length))),
            MeshMaterial3d(materials.add(Color::from(Srgba::GREEN))),
            Transform::from_translation(vec3(0.0, 0.5*length, 0.0)),
            //Visibility::Hidden, // Dieses Kind ist unsichtbar
        ));
        parent.spawn((
            Mesh3d(meshes.add(Cone::new(2.*radius, 3.*radius))),
            MeshMaterial3d(materials.add(Color::from(Srgba::GREEN))),
            Transform::from_translation(vec3(0.0, length, 0.0)),
        ));

        //z-Axis blue
        parent.spawn((
            Mesh3d(meshes.add(Cylinder::new(radius, length))),
            MeshMaterial3d(materials.add(Color::from(Srgba::BLUE))),
            Transform::from_rotation(Quat::from_rotation_x(PI/2.)).with_translation(vec3(0.0, 0.0, 0.5*length)),
        ));
        parent.spawn((
            Mesh3d(meshes.add(Cone::new(2.*radius, 3.*radius))),
            MeshMaterial3d(materials.add(Color::from(Srgba::BLUE))),
            Transform::from_rotation(Quat::from_rotation_x(PI/2.)).with_translation(vec3(0.0, 0.0, length)),
        ));

        //x-Axis red
        parent.spawn((
            Mesh3d(meshes.add(Cylinder::new(radius, length))),
            MeshMaterial3d(materials.add(Color::from(Srgba::RED))),
            Transform::from_rotation(Quat::from_rotation_z(PI/2.)).with_translation(vec3(0.5*length, 0.0, 0.0)),
        ));
        parent.spawn((
            Mesh3d(meshes.add(Cone::new(2.*radius, 3.*radius))),
            MeshMaterial3d(materials.add(Color::from(Srgba::RED))),
            Transform::from_rotation(Quat::from_rotation_z(PI/-2.)).with_translation(vec3(length, 0.0, 0.0)),
        ));

        //w-Axis yellow
        let mut trafo = Transform::from_translation(vec3(0.,0.5*length,0.));
        trafo = Transform::from_rotation(Quat::from_euler(EulerRot::ZXY,PI/4.,PI/4.,0.)).mul_transform(trafo);
        parent.spawn((
            Mesh3d(meshes.add(Cylinder::new(radius, length))),
            MeshMaterial3d(materials.add(Color::from(Srgba::rgba_u8(255, 255, 0, 160)))),//yellow
            trafo,
        ));
        let mut trafo = Transform::from_translation(vec3(0.,length,0.));
        trafo = Transform::from_rotation(Quat::from_euler(EulerRot::ZXY,PI/4.,PI/4.,0.)).mul_transform(trafo);
        parent.spawn((
            Mesh3d(meshes.add(Cone::new(2.*radius, 3.*radius))),
            MeshMaterial3d(materials.add(Color::from(Srgba::rgba_u8(255, 255, 0, 160)))),//yellow
            trafo,
        ));
    });
}