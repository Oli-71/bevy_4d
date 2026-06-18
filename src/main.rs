//! A Bevy application to visualize a 4D scene from different viewpoints and with different transformations,
//! to give an intuition of how a 4D world would look like and how it relates to our 3D world and a 2D world.
//! 
//! The content is incrementally added to the scene, so user can explore the different aspects step by step.
//! 
//! The code is structured in a way that the 
//! - main.rs file contains the general setup of the scene and the controls, while the 
//! - scene4d.rs file contains the logic for the 4D scene and the transformations. 
//! - atoms.rs file contains the logic for generating objects composed of atoms.
//! 
//! Author: Oliver Bringmann, Leipzig, Germany, 2026

use std::f32::consts::PI;

use bevy::{
    camera::SubCameraView, color::palettes::tailwind::*, light::{NotShadowCaster, NotShadowReceiver}, prelude::*
};

use bevy::prelude::Srgba;

mod atoms;
mod scene4d;
use scene4d::*;
mod smooth;

const SCALE: f32 = 6.0; // global scaling to fit the standard screen

const Y_CTR_ROW1: f32 = 9. * SCALE; // y-value for the first row of controls; the second row is below with a small gap
const Y_CTR_ROW2: f32 = 8. * SCALE;

const CAMERA_STANDARD_TARGET: Vec3 = vec3(0., 0., 0.);
const CAMERA_SPACELAND_POSITION: Vec3 = vec3(0., 70., 140.);
const CAMERA_FLATLAND_POSITION: Vec3 = vec3(0., 0., 140.);

const SLIDER_HEIGHT_CONTROL_OFFSET_Y: f32 = 5. * SCALE;

const LABEL_VIEW_POINT: &str = "Top View";
const LABEL_VIEW_POINT_GERMAN: &str = "Draufsicht";
const LABEL_HYPER: &str = "__Hyper";
const LABEL_HYPER_GERMAN: &str = "__Hyper";
const LABEL_PROJECTION: &str = "Projection";
const LABEL_PROJECTION_GERMAN: &str = "Projektion";
const LABEL_SYNC_DRAG: &str = "Synchronized Dragging";
const LABEL_SYNC_DRAG_GERMAN: &str = "Synchrones Drehen";
const LABEL_CONTINUOUS_ROTATION: &str = "Continuous Rotation";
const LABEL_CONTINUOUS_ROTATION_GERMAN: &str = "Kontinuierliche Rotation";
const LABEL_HIGHER_DIMENSION_OFFSET: &str = "Higher Dimension Offset";
const LABEL_HIGHER_DIMENSION_OFFSET_GERMAN: &str = "Offset der höheren Dimension";
const LABEL_FLATLAND: &str = "Flatland";
const LABEL_FLATLAND_GERMAN: &str = "Flachland";
const LABEL_SHOW_MORE: &str = "Show more";
const LABEL_SHOW_MORE_GERMAN: &str = "Mehr anzeigen";
const LABEL_LANGUAGE: &str = "Deutsch/English";
const LABEL_LANGUAGE_GERMAN: &str = "English/Deutsch";

const INSTRUCTIONS_INITIAL: &str = r#"What would our three-dimensional world look like from a location in the fourth dimension? Let’s start with what inhabitants of a two-dimensional world see:
Look into the flatland gap below. Flatlanders can only see what happens between the red lines. Drag the objects... If you understand what they represent, try the gray controls at the top.
The 'Hyper' cone moves your location slowly into the third dimension and back. Wow, From 3D the Flatlanders can look into closed shapes!
If you feel familiar with the Flatlander's understanding of dimension jump, click 'Show more'."#;

const INSTRUCTIONS_INITIAL_GERMAN: &str = r#"Wie würde unsere dreidimensionale Welt von einem Ort in der vierten Dimension aussehen? Beginnen wir damit, was die Bewohner einer zweidimensionalen Welt sehen:
Schauen Sie in die Flachland-Spalte unten. Flachländer können nur sehen, was zwischen den roten Linien passiert. Drehen Sie die Objekte... Wenn Sie verstanden haben, was sie darstellen, probieren Sie alle grauen Steuerelemente oben aus.
Der 'Hyper'-Kegel bewegt Ihren Standort langsam in die dritte Dimension und zurück. Wow, von 3D aus können die Flachländer in geschlossene Formen schauen!
Wenn Sie sich mit dem Verständnis der Dimensionssprünge der Flachländer vertraut fühlen, klicken Sie auf 'Mehr anzeigen'."#;

const INSTRUCTIONS_FLATLAND_COMPLETE: &str = r#"Now we have also two 3D cubes extending into our 2D world.
Try to imagine how difficult it is for the inhabitants of Flatland to grasp their structure. Even if they could jump to the third dimension ('Hyper') it would be a challenge.
BTW: There are a few more options available. Try them all.
Surprised what happens when you combine 'Projection' and 'Hyper'? Add some 'Continuous Rotation' and 'Top View'. Now you will understand...
When you are ready to experience the 3D-to-4D effect, click 'Show more'."#;

const INSTRUCTIONS_FLATLAND_COMPLETE_GERMAN: &str = r#"Nun haben wir zwei 3D-Würfel, die in unsere 2D-Welt hineinragen.
Versuchen Sie sich vorzustellen, wie schwierig es für die Bewohner von Flachland ist, deren Struktur zu erfassen. Selbst wenn sie in die dritte Dimension springen könnten ('Hyper'), wäre es eine Herausforderung.
Übrigens: Es gibt nun noch ein paar weitere Einstellmöglichkeiten. Probieren Sie sie alle aus.
Überrascht, was passiert, wenn Sie 'Projektion' und 'Hyper' kombinieren? Fügen Sie etwas 'Kontinuierliche Rotation' und 'Draufsicht' hinzu... Jetzt werden Sie es verstehen.
Wenn Sie bereit sind, den 3D-zu-4D-Effekt zu erleben, klicken Sie auf 'Mehr anzeigen'."#;

const INSTRUCTIONS_THREE_DIMENSIONAL: &str = r#"Study how the two new 3D-Objects behave if we go up to a fourth dimension view point (Again: 'Hyper').
Are there are similarities to flatlander's experiences?

'Show more' will add two different 4D-cubes to the scene. It will become crazy ;-)"#;

const INSTRUCTIONS_THREE_DIMENSIONAL_GERMAN: &str = r#"Untersuchen Sie, wie sich die beiden neuen 3D-Objekte verhalten, wenn wir zu einem vierdimensionalen Standpunkt wechseln (nochmals: 'Hyper').
Gibt es Ähnlichkeiten zu den Erfahrungen der Flachländer?

'Mehr anzeigen' fügt der Szene zwei unterschiedliche 4D-Würfel hinzu. Es wird verrückt ;-)"#;

const INSTRUCTIONS_SPACELAND_COMPLETE: &str = r#"Take some time to compare Flatlander's and Spacelander's experiences with an extra dimension. The 'Higher Dimension Offset' slider and 'Synchronized Dragging' can help...
‘Projection’ pushes the atoms from the higher dimensions into the visible range. Note that actually we can never see 4D objects completely.  
'Show more' will display a complex scene, the construction of which now takes a bit longer..."#;

const INSTRUCTIONS_SPACELAND_COMPLETE_GERMAN: &str = r#"Nehmen Sie sich Zeit, um die Erfahrungen der Flachländer und Raumländer mit einer zusätzlichen Dimension zu vergleichen. Der 'Offset der höheren Dimension' Schieber und 'Synchrones Drehen' können helfen...
‘Projektion’ schiebt die Atome aus den höheren Dimensionen in den sichtbaren Bereich. Beachten Sie, dass wir eigentlich 4D-Objekte niemals vollständig sehen können.  
'Mehr anzeigen' zeigt eine komplexe Szene, deren Aufbau nun etwas länger dauert..."#;

const INSTRUCTIONS_SPACELAND_ONLY: &str = r#"Try all Transformations.
Can you figure out what are 
the two segrets of the cube?

The end of the demo is reached.
Thank you for your attention!"#;

const INSTRUCTIONS_SPACELAND_ONLY_GERMAN: &str = r#"Probieren Sie alle Transformationen aus.
Können Sie herausfinden, 
was die beiden Geheimnisse des Würfels sind?

Das Ende der Demo ist erreicht.
Vielen Dank!"#;

fn main() {
    App::new()
        // MeshPickingPlugin is not a default plugin
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .insert_resource(Scene {
            scene_4d: Scene4D::new(),
            viewpoint_is_spaceland: false,
            state: StateScene::Planar,
            language: Language::English,
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
                sync_label_visibility,
                update_move_position_smooth,
            ),
        )
        .run();
}

// Marker component to identify which entities are controls, so we can show/hide them together based on the state of the scene.
#[derive(PartialEq)]
enum OnOffMarkerType {
    Non,
    Projection,
    TopView,
    SynchronizedDrag,
}

// We use the Control component for all control objects, to easily show/hide them together based on the state of the scene.
#[derive(Component)]
struct Control {
    on_off_marker: OnOffMarkerType, // is it an on/off marker; if this is the case: which one?
}

#[derive(Component)]
struct AdvancedControl; // only visible in advanced states

#[derive(Component)]
struct OnlyIn2rowScene; // not visible in complex scene (last state)

#[derive(Component)]
struct HighDimOffset; // not visible in Projection

#[derive(Component)]
struct Rotation{
    rotation: Rotation4d,
}

#[derive(Component)]
struct BackgroundPanel;

// We use the Label component for all labels, to easily show/hide them together based on the state of the scene and to position them above their corresponding entities.
#[derive(Component)]
struct Label {
    entity: Entity,
    offset_y: f32, // an offset to position the label above the entity
}

// We use the LabelId component to identify which label is which, so we can update their text when the language changes.
#[derive(Component)]
struct LabelId {
    id: String,
}

// We use the AngleMonitor component to identify the cone that serves as an angle monitor, 
// so we can rotate it based on the current rotation angle in the 4D scene.
#[derive(Component)]
struct AngleMonitor;

// We use the Atom component for the entities that represent the atoms in the 4D scene, 
// to keep track of which atom they correspond to and whether they are currently visible in principle depending on the states.
#[derive(Component)]
struct Atom {
    index: usize, // index in the Scene4D's atoms vector, to identify which atom this entity corresponds to
    visible: bool,
}

#[derive(Component)]
struct Instructions;

// Content will be incrementally added to the scene, 
// so we have different states of the scene to keep track of which content is currently shown.
#[derive(PartialEq)]
enum StateScene {
    Planar,
    FlatlandComplete,
    ThreeDimensional,
    SpacelandComplete,
    SpacelandOnly,
}

#[derive(PartialEq)]
enum Language {
    English,
    German,
}

// a control to visualize a rotation around a plane (two axis), has actually 4 axes :)
#[derive(Component)]
struct Tripod {
    rotation: Rotation4d,
}

// The main resource that holds the 4D scene and the current state of the application.
#[derive(Resource)]
struct Scene {
    scene_4d: Scene4D,
    viewpoint_is_spaceland: bool,
    state: StateScene,
    language: Language,
}

impl Scene {
    fn instructions_localized(&self) -> &str {
        match self.language {
            Language::English => match self.state {
                StateScene::Planar => INSTRUCTIONS_INITIAL,
                StateScene::FlatlandComplete => INSTRUCTIONS_FLATLAND_COMPLETE,
                StateScene::ThreeDimensional => INSTRUCTIONS_THREE_DIMENSIONAL,
                StateScene::SpacelandComplete => INSTRUCTIONS_SPACELAND_COMPLETE,
                StateScene::SpacelandOnly => INSTRUCTIONS_SPACELAND_ONLY,
            },
            Language::German => match self.state {
                StateScene::Planar => INSTRUCTIONS_INITIAL_GERMAN,
                StateScene::FlatlandComplete => INSTRUCTIONS_FLATLAND_COMPLETE_GERMAN,
                StateScene::ThreeDimensional => INSTRUCTIONS_THREE_DIMENSIONAL_GERMAN,
                StateScene::SpacelandComplete => INSTRUCTIONS_SPACELAND_COMPLETE_GERMAN,
                StateScene::SpacelandOnly => INSTRUCTIONS_SPACELAND_ONLY_GERMAN,
            }
        }
    }

    fn label_localized<'a>(&self, label_name: &'a str) -> &'a str {
        match self.language { 
            Language::English => label_name,
            Language::German => match label_name {
                LABEL_VIEW_POINT => LABEL_VIEW_POINT_GERMAN,
                LABEL_HYPER => LABEL_HYPER_GERMAN,
                LABEL_PROJECTION => LABEL_PROJECTION_GERMAN,
                LABEL_SYNC_DRAG => LABEL_SYNC_DRAG_GERMAN,
                LABEL_CONTINUOUS_ROTATION => LABEL_CONTINUOUS_ROTATION_GERMAN,
                LABEL_HIGHER_DIMENSION_OFFSET => LABEL_HIGHER_DIMENSION_OFFSET_GERMAN,
                LABEL_SHOW_MORE => LABEL_SHOW_MORE_GERMAN,
                LABEL_FLATLAND => LABEL_FLATLAND_GERMAN,
                LABEL_LANGUAGE => LABEL_LANGUAGE_GERMAN,
                _ => "german localisation missing",
            }
        }
    }
}

/// A system to set up the initial scene, including the 3D objects, controls, camera, and instructions.
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    scene: ResMut<Scene>,
    asset_server: Res<AssetServer>,
) {
    // Set up the materials.
    let white_matl = materials.add(Color::WHITE);
    let hover_matl = materials.add(Color::from(CYAN_300));
    let pressed_matl = materials.add(Color::from(YELLOW_300));
    let activated_matl =materials.add(Color::linear_rgb(0.8, 0.2, 0.8));

    let size_of_controls = 0.2 * SCALE;

    // Gray Control Objects
    //////////////////////////////
    let left = -10. * SCALE;
    // Sphere to switch language (english/german)
    let language_control_entity = commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(size_of_controls))),
            MeshMaterial3d(white_matl.clone()),
            Transform::from_xyz(left, Y_CTR_ROW1, 0.),
            Control {on_off_marker: OnOffMarkerType::Non},
            Visibility::Visible, 
        ))
        .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
        .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
        .observe(update_material_on::<Pointer<Press>>(pressed_matl.clone()))
        .observe(update_material_on::<Pointer<Release>>(hover_matl.clone()))
        .observe(toggle_language_on_press)
        .id();
    
    // Sphere to trigger view point
    let view_point_control_entity = commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(size_of_controls))),
            MeshMaterial3d(white_matl.clone()),
            Transform::from_xyz(0. * SCALE, Y_CTR_ROW1, 0.),
            Control { on_off_marker: OnOffMarkerType::Non},
            Visibility::Hidden,
            AdvancedControl,
        ))
        .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
        .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
        .observe(update_material_on::<Pointer<Press>>(pressed_matl.clone()))
        .observe(update_material_on::<Pointer<Release>>(hover_matl.clone()))
        .observe(toggle_view_point_on_press)
        .id();
    
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(size_of_controls*1.2))),
        MeshMaterial3d(activated_matl.clone()),
        Transform::from_xyz(0. * SCALE, Y_CTR_ROW1, 0.),
        Control { on_off_marker: OnOffMarkerType::TopView },
        Visibility::Hidden,
        Pickable::IGNORE,
    ));

    // Cone with red point to trigger 4d view
    // also suits as Angle Monitor
    let angle_monitor_entity = commands.spawn((
        Mesh3d(meshes.add(Cone::new(size_of_controls, size_of_controls * 2.0))),
        MeshMaterial3d(white_matl.clone()),
        Transform::from_xyz(0., Y_CTR_ROW1, 0.).with_rotation(Quat::from_rotation_y(PI/2.)),
        Control { on_off_marker: OnOffMarkerType::Non},
        AngleMonitor,
        AdvancedControl,
    )).with_children(|parent| {
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(size_of_controls/5.))),
            MeshMaterial3d(materials.add(Color::from(Srgba::RED))),
            Transform::from_xyz(0.,size_of_controls,0.),
        ));})
    .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
    .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
    .observe(update_material_on::<Pointer<Press>>(pressed_matl.clone()))
    .observe(update_material_on::<Pointer<Release>>(hover_matl.clone()))
    .observe(toggle_4d_on_press)
    .id();

    // Sphere to rotate all objects synchronized in the scene by dragging.
    let drag_all_objects_entity = commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(size_of_controls))),
            MeshMaterial3d(white_matl.clone()),
            Transform::from_xyz(6. * SCALE, Y_CTR_ROW1, 0.),
            Control {on_off_marker: OnOffMarkerType::Non},
            Visibility::Visible,
        ))
        .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
        .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
        .observe(update_material_on::<Pointer<Press>>(pressed_matl.clone()))
        .observe(update_material_on::<Pointer<Release>>(hover_matl.clone()))
        .observe(toggle_drag_all_on_press)
        .id();
    
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(size_of_controls*1.2))),
        MeshMaterial3d(activated_matl.clone()),
        Transform::from_xyz(6. * SCALE, Y_CTR_ROW1, 0.),
        Control { on_off_marker: OnOffMarkerType::SynchronizedDrag },
        Visibility::Hidden,
        Pickable::IGNORE,
    ));

    // Slider to adjust speed of continuous rotation
    let slider_3d_rotation_entity = commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(size_of_controls))),
            MeshMaterial3d(white_matl.clone()),
            Transform::from_xyz(-3. * SCALE, Y_CTR_ROW2, 0.),
        ))
        .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
        .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
        .observe(update_material_on::<Pointer<Press>>(pressed_matl.clone()))
        .observe(update_material_on::<Pointer<Release>>(hover_matl.clone()))
        .observe(drag_to_adjust_speed)
        .id();

    commands.spawn((
        Mesh3d(meshes.add(Segment3d::new(
            vec3(-3. * SCALE, Y_CTR_ROW2, 0.),
            vec3(3. * SCALE, Y_CTR_ROW2, 0.),
        ))),
        MeshMaterial3d(white_matl.clone()),
    ));

    // Sphere to trigger "see more"
    let show_more_control_entity = commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(size_of_controls))),
            MeshMaterial3d(white_matl.clone()),
            Transform::from_xyz(6. * SCALE, Y_CTR_ROW2, 0.),
            Control {on_off_marker: OnOffMarkerType::Non},
            OnlyIn2rowScene,
        ))
        .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
        .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
        .observe(update_material_on::<Pointer<Press>>(pressed_matl.clone()))
        .observe(update_material_on::<Pointer<Release>>(hover_matl.clone()))
        .observe(show_more_on_press)
        .id();

    // Sphere to trigger projection view
    let projection_control_entity = commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(size_of_controls))),
            MeshMaterial3d(white_matl.clone()),
            Transform::from_xyz(left, Y_CTR_ROW2, 0.),
            Control {on_off_marker: OnOffMarkerType::Non},
            Visibility::Hidden,
            AdvancedControl,
        ))
        .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
        .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
        .observe(update_material_on::<Pointer<Press>>(pressed_matl.clone()))
        .observe(update_material_on::<Pointer<Release>>(hover_matl.clone()))
        .observe(toggle_projection_on_press)
        .id();

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(size_of_controls*1.2))),
        MeshMaterial3d(activated_matl.clone()),
        Transform::from_xyz(left, Y_CTR_ROW2, 0.),
        Control {on_off_marker: OnOffMarkerType::Projection },
        Visibility::Hidden,
        Pickable::IGNORE,
    ));

    // slider to adjust higher dimension height (w in Spaceland, y in Flatland)
    let slider_height_entity = commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(size_of_controls))),
            MeshMaterial3d(white_matl.clone()),
            Transform::from_xyz(left, 2. * SCALE + SLIDER_HEIGHT_CONTROL_OFFSET_Y, 0.),
            Control {on_off_marker: OnOffMarkerType::Non},
            Visibility::Hidden,
            AdvancedControl,
            HighDimOffset,
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
        Control {on_off_marker: OnOffMarkerType::Non},
        Visibility::Hidden,
        AdvancedControl,
        HighDimOffset,
    ));

    // 4D-Scene: Flatland and Spaceland
    spawn_scene(&mut commands, &mut meshes, &mut materials, &scene);

    // Cover Panel to hide invisible 3D-Space for flatland
    let size_of_panel = 30. * SCALE;
    let color = 200;
    let atom_size_at_panel_plane = 0.5 * scene.scene_4d.spacing * SCALE; // factor, because panel is closer to camera
    let z_offset =
        scene.scene_4d.spacing * SCALE * scene.scene_4d.number_of_atoms_per_side as f32;

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
            0. + atom_size_at_panel_plane,
            z_offset - size_of_panel / 2.,
        )),
        Pickable::IGNORE,
        Control {on_off_marker: OnOffMarkerType::Non},
        NotShadowReceiver,
        OnlyIn2rowScene,
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
            0. - atom_size_at_panel_plane,
            z_offset - size_of_panel / 2.,
        )),
        Pickable::IGNORE,
        Control {on_off_marker: OnOffMarkerType::Non},
        NotShadowReceiver,
        OnlyIn2rowScene,
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
            0. + y_size + atom_size_at_panel_plane,
            z_offset,
        )),
        Pickable::IGNORE,
        Control {on_off_marker: OnOffMarkerType::Non},
        NotShadowReceiver,
        OnlyIn2rowScene,
    ));

    // vertical bottom panel
    let y_size_of_bottom_panel = y_size*2.;
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(
            vec3(0., 0., 1.),
            vec2(size_of_panel, y_size_of_bottom_panel),
        ))),
        MeshMaterial3d(materials.add(Color::srgba_u8(color, color, color, 100))),
        Transform::from_translation(vec3(
            0.,
            0. - y_size_of_bottom_panel - atom_size_at_panel_plane,
            z_offset,
        )),
        Pickable::IGNORE,
        Control { on_off_marker: OnOffMarkerType::Non},
        NotShadowReceiver,
        OnlyIn2rowScene,
    ));

    // A helper closure to spawn a thick line between two points.
    let mut spawn_thick_red_line = |start: Vec3, end: Vec3, thickness: f32| -> Entity {
        let direction = end - start;
        let length = direction.length();
        
        // Calculate the rotation needed to align the cylinder with the direction vector.
        let rotation = Quat::from_rotation_arc(Vec3::Y, direction.normalize());

        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(thickness, length))),
            MeshMaterial3d(materials.add(Color::srgba_u8(255, 0, 0, 200))),
            Transform::from_translation(start + direction / 2.0)
                .with_rotation(rotation),
            Control {  on_off_marker: OnOffMarkerType::Non},
            OnlyIn2rowScene,
        )).id()
    };

    // Flatland Indicator Lines
    let _flatland_top_line_entity = spawn_thick_red_line(
        vec3(- size_of_panel / 2., atom_size_at_panel_plane, z_offset),
        vec3( size_of_panel / 2., atom_size_at_panel_plane, z_offset),
        0.02 * SCALE,
    );
    
    let _flatland_bottom_line_entity = spawn_thick_red_line(
        vec3(- size_of_panel / 2., - atom_size_at_panel_plane, z_offset),
        vec3( size_of_panel / 2., - atom_size_at_panel_plane, z_offset),
        0.02 * SCALE,
    );

    let flatland_bottom =commands.spawn((
            Mesh3d(meshes.add(Sphere::new(1.))),
            MeshMaterial3d(materials.add(Color::srgba_u8(255, 0, 0, 0))),
            Transform::from_translation(vec3(-9.5 * SCALE, - atom_size_at_panel_plane, z_offset)),
            Control {on_off_marker: OnOffMarkerType::Non},
            OnlyIn2rowScene,
        )).id();

    // Background Panel - indicates that your viewpoint is in 3D-space (no hyper)
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(
            vec3(0., 0., 1.),
            vec2(150., 95.),
        ))),
        MeshMaterial3d(materials.add(Color::srgba_u8(30, 30, 5, 255))),
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
    ));

    // Light for flatland
    commands.spawn((
        PointLight {//DirectionalLight
            shadows_enabled: true,
            intensity: 50_000_000. * SCALE,
            range: 500.0 * SCALE,
            ..default()
        },
        NotShadowCaster, // this light should not cast shadows to avoid too dark shadows in the flatland view
        Transform::from_xyz(0.0 * SCALE, 0.0 * SCALE, 16.0 * SCALE),
        Control { on_off_marker: OnOffMarkerType::Non},
        OnlyIn2rowScene,
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
    let mut spawn_label = |entity: Entity, label: &str, offset: f32, /*visibility: Visibility, only_in_2row_scene: bool, high_dim_offset: bool*/| {
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                ..default()
            },
            Label { entity, offset_y: offset },
            children![(
                Text::new(label),
                label_text_style.clone(),
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::ZERO,
                    ..default()
                },
                TextLayout::default().with_no_wrap(),
                LabelId { id: label.to_string() },
            )],
        ));
    };

    spawn_label(view_point_control_entity, LABEL_VIEW_POINT, 0.9);
    spawn_label(angle_monitor_entity, LABEL_HYPER, 0.9);
    spawn_label(projection_control_entity, LABEL_PROJECTION, 0.9);
    spawn_label(drag_all_objects_entity, LABEL_SYNC_DRAG, 0.9);
    spawn_label(slider_3d_rotation_entity, LABEL_CONTINUOUS_ROTATION, 0.9);
    spawn_label(slider_height_entity, LABEL_HIGHER_DIMENSION_OFFSET, 0.9);
    spawn_label(flatland_bottom, LABEL_FLATLAND, 0.0);
    spawn_label(show_more_control_entity, LABEL_SHOW_MORE, 0.9);
    spawn_label(language_control_entity, LABEL_LANGUAGE, 0.9);

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
        if !atom_entity.visible { // hide the atom is not visible based on the current state
            *visibility = Visibility::Hidden;
            continue;
        }
        if let Some(position) = new_positions.get(atom_entity.index) {
            *visibility = if scene.scene_4d.is_atom_visible(*position) {// w component is close to zero 
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

        let translation = position.get_next_translation(trafo.translation, time.delta_secs());
        *trafo = trafo
            .looking_at(CAMERA_STANDARD_TARGET, Vec3::Y)
            .with_translation(translation);
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

/// A system that synchronizes the visibility of label entities with their referenced control entities
fn sync_label_visibility(
    vis_of_all_entities: Query<&Visibility, Without<Label>>,
    mut labels: Query<(&mut Visibility, &Label), Without<Control>>,
) {
    for (mut label_vis, label) in &mut labels {
        if let Ok(control_vis) = vis_of_all_entities.get(label.entity) { // interesting filtering
            *label_vis = *control_vis;
        }
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
       trafo.translation = vec3(-3. * SCALE, 9. * SCALE, 0.);
    }
}

/// An observer to trigger synchronized dragging of all objects.
fn toggle_drag_all_on_press(
    _press: On<Pointer<Press>>,
    mut scene: ResMut<Scene>,
    on_off: Query<(&mut Visibility, &mut Control)>,
) {
    scene.scene_4d.is_synchronized_drag = !scene.scene_4d.is_synchronized_drag;

    for (mut vis, control) in on_off {
        if control.on_off_marker == OnOffMarkerType::SynchronizedDrag {
            *vis = if scene.scene_4d.is_synchronized_drag {Visibility::Visible} else {Visibility::Hidden}; 
        }
    }
}

fn toggle_language_on_press(_press: On<Pointer<Press>>,
    mut scene: ResMut<Scene>,
    mut instruction_texts: Query<&mut Text, (With<Instructions>, Without<Label>)>,
    mut label_texts: Query<(&mut Text, &LabelId), Without<Instructions>>,
) {
    scene.language = match scene.language {
        Language::English => Language::German,
        Language::German => Language::English,
    };
    // switch between english and german instructions
    for mut instruction_text in &mut instruction_texts {
        instruction_text.0 = scene.instructions_localized().to_string();
    }
    // switch between english and german label texts
    for (mut label_text, label_id) in &mut label_texts {
        label_text.0 = scene.label_localized(&label_id.id).to_string();
    }
}

/// An observer to switch global view
fn toggle_view_point_on_press(
    _press: On<Pointer<Press>>,
    mut scene: ResMut<Scene>,
    mut camera3ds: Query<&mut smooth::PositionTarget, With<Camera3d>>,
    on_off: Query<(&mut Visibility, &mut Control)>
) {
    scene.viewpoint_is_spaceland = !scene.viewpoint_is_spaceland;

    for (mut vis, control) in on_off {
        if control.on_off_marker == OnOffMarkerType::TopView {
            *vis = if scene.viewpoint_is_spaceland {Visibility::Visible} else {Visibility::Hidden}; 
        }
    }

    for mut camera in &mut camera3ds {
        if scene.viewpoint_is_spaceland {
            camera.set_target(CAMERA_SPACELAND_POSITION);
        } else {
            camera.set_target(CAMERA_FLATLAND_POSITION);
        }
    }
}

/// An observer to trigger toggle_projection when the ControlShape is pressed.
fn toggle_projection_on_press(_press: On<Pointer<Press>>, 
    mut scene: ResMut<Scene>, 
    vis: Query<(&mut Visibility, Option<&Control>, Option<&HighDimOffset>)> // could be both
) {
    scene.scene_4d.toggle_projection_view();
    for (mut vis, control_opt, hd_opt) in vis {
        if let Some(control) = control_opt {
            if control.on_off_marker == OnOffMarkerType::Projection {
                *vis = if scene.scene_4d.is_projection_view {Visibility::Visible} else {Visibility::Hidden}; 
            }
        }
        // Hide Slider for High Dim Offset
        if let Some(_hd) = hd_opt {
            *vis = if scene.scene_4d.is_projection_view {Visibility::Hidden} else {Visibility::Visible};
        }
    }
}

/// An observer to toggle_rotation when the tripod is pressed.
fn toggle_rotation_on_press(
    press: On<Pointer<Press>>,
    mut tripods: Query<&mut Tripod>,
    mut scene: ResMut<Scene>,
    on_off: Query<(&mut Visibility, &Rotation)>,
    time: Res<Time>,
) {
    let tripod = tripods.get_mut(press.entity).unwrap();

    if scene.scene_4d.rotation != tripod.rotation
    {//switch and start new rotation
        scene.scene_4d.rotation = tripod.rotation;

        // show active rotation by on/off marker visibility
        for (mut vis,rot) in on_off {
                *vis = if rot.rotation == tripod.rotation {Visibility::Visible} else {Visibility::Hidden};
        }

        // start hyper jump
        scene
            .scene_4d
            .force_high_dimension_view(time.elapsed_secs());
    } else if scene.scene_4d.is_high_dimension_view {
        // turn off hyper jump
        scene
            .scene_4d
            .toggle_high_dimension_view(time.elapsed_secs());
    }
}

/// An observer to trigger toggle_4d when the ControlShape is pressed.
fn toggle_4d_on_press(_press: On<Pointer<Press>>, mut scene: ResMut<Scene>, time: Res<Time>) {
    scene
    .scene_4d
    .toggle_high_dimension_view(time.elapsed_secs());
}

/// An observer to switch to the next scene state.
fn show_more_on_press(
    _press: On<Pointer<Press>>,
    mut text: Query<(&mut Text, &mut Node), With<Instructions>>,
    mut atoms: Query<(Entity, &mut Atom)>,
    mut scene: ResMut<Scene>,
    mut visibility_set: ParamSet<(
        Query<(&mut Visibility, &Control)>,
        Query<(&mut Visibility, &AdvancedControl)>,
        Query<(&mut Visibility, &OnlyIn2rowScene)>,
        Query<(&mut Visibility, &HighDimOffset)>,
    )>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut camera3ds: Query<&mut smooth::PositionTarget, With<Camera3d>>,
) {
    scene.state = match scene.state {
        StateScene::Planar => StateScene::FlatlandComplete,
        StateScene::FlatlandComplete => StateScene::ThreeDimensional,
        StateScene::ThreeDimensional => StateScene::SpacelandComplete,
        StateScene::SpacelandComplete => StateScene::SpacelandOnly,
        StateScene::SpacelandOnly => StateScene::SpacelandOnly, // no further state
    };

    // good settings to start exploring a new state
    scene.scene_4d.reset_view();

    // hide all on/off marker
    for (mut visibility, control) in visibility_set.p0().iter_mut() {
        if control.on_off_marker != OnOffMarkerType::Non {
            *visibility = Visibility::Hidden;
        }
    }

    // show high_dim_offset slider
    for (mut visibility, _high_dim_offset_control) in visibility_set.p3().iter_mut() {
        *visibility = Visibility::Visible;
    }

    // reset to flatland view point
    scene.viewpoint_is_spaceland = false;
    for mut camera in &mut camera3ds {
        camera.set_target(CAMERA_FLATLAND_POSITION);
    }

    let instructions_string = scene.instructions_localized();

    match scene.state {
        StateScene::Planar => {}, // initial state, nothing to do
        StateScene::FlatlandComplete => {
            // add advanced controls
            for (mut visibility, _advanced) in visibility_set.p1().iter_mut(){
                *visibility = Visibility::Visible;
            }

            for (mut text, mut node) in &mut text {
                text.0 = instructions_string.to_string();
                //middle right position
                node.top = percent(22.);
                node.left = percent(43.);
                node.right = percent(5.);
            }

            for (_entity, mut atom) in &mut atoms {
                if scene.scene_4d.is_2d(atom.index) {
                    atom.visible = true;
                }
            }
        },
        StateScene::ThreeDimensional => {
            for (mut text, mut node) in &mut text {
                text.0 = instructions_string.to_string();
                //middle more right position
                node.top = percent(22.);
                node.left = percent(60.);
                node.right = percent(5.);
            }

            for (_entity, mut atom) in &mut atoms {
                if !scene.scene_4d.is_4d(atom.index) {
                    atom.visible = true;
                }
            }
        },
        StateScene::SpacelandComplete => {
            for (mut text, mut node) in &mut text {
                text.0 = instructions_string.to_string();
                //left bottom position
                node.top = percent(80.);
                node.left = percent(3.);
                node.right = percent(25.);
            }

            for (_entity, mut atom) in &mut atoms {
                atom.visible = true;
            }
        },
        StateScene::SpacelandOnly => {// complex scene
            // instructions
            for (mut text, mut node) in &mut text {
                text.0 = instructions_string.to_string();
                //left bottom position
                node.top = percent(80.);
                node.left = percent(3.);
                node.right = percent(46.);
            }

            //remove all atoms
            for (entity, _atom) in atoms {
                commands.entity(entity).despawn();
            }

            // hide flatlands panels etc.
            for (mut visibility, _only2row) in visibility_set.p2().iter_mut() {
                    *visibility = Visibility::Hidden;
            }

            // create a new Scene4d
            scene.scene_4d = Scene4D::new_complex_scene();

            // create the new atoms from new scene4d
            spawn_scene(&mut commands, &mut meshes, &mut materials, &scene);

            // tripods for control of Hyper rotation
            spawn_tripods(&mut commands, &mut meshes, &mut materials, scene.scene_4d.rotation);

            // Light
            commands.spawn((
                PointLight {
                    shadows_enabled: true,
                    intensity: 50_000_000. * SCALE,
                    range: 500.0 * SCALE,
                    shadow_depth_bias: 0.2,
                    ..default()
                },
                Transform::from_xyz(8.0 * SCALE, 2.0 * SCALE, 8.0 * SCALE),
            ));

            // Light inside aquarium
            commands.spawn((
                PointLight {
                    shadows_enabled: true,
                    intensity: 1_000_000. * SCALE,
                    range: 50.0 * SCALE,
                    shadow_depth_bias: 0.2,
                    ..default()
                },
                Transform::from_xyz(-3.0 * SCALE, 2.0 * SCALE, 1.0 * SCALE),
            ));

            // synchronized dragging is standard in complex scene
            scene.scene_4d.is_synchronized_drag = true;  
            for (mut visibility, control) in visibility_set.p0().iter_mut() {
                if control.on_off_marker == OnOffMarkerType::SynchronizedDrag {
                    *visibility = Visibility::Visible;
                }
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
fn drag_to_rotate_object(
    drag: On<Pointer<Drag>>,
    mut atoms: Query<&mut Atom>,
    mut scene: ResMut<Scene>,
){
    // Update the drag state in the 4D scene based on the drag delta and the atom index
    let atom = atoms.get_mut(drag.entity).unwrap();
    scene.scene_4d.drag_object_from_atom(atom.index, drag.delta);
}

/// A helper function to spawn the 3D scene based on the current state of the Scene4D. 
/// We call this both at startup and when resetting the scene.
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
    let sphere_shape = scene.scene_4d.is_2row_structured_scene;
    let radius = scene.scene_4d.size_of_atom() * 0.5 * SCALE;
    let _cube_size = 1.5 * scene.scene_4d.size_of_atom() * SCALE;
    // Create a single Handle<Mesh> so both branches have the same type
    let atom_mesh_handle = if sphere_shape {
        meshes.add(Sphere::new(radius))
        //meshes.add(Cuboid::new(cube_size, cube_size, cube_size))
    } else {
        //meshes.add(Cuboid::new(cube_size, cube_size, cube_size))
        meshes.add(Sphere::new(radius))
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
            .observe(drag_to_rotate_object);
    }
}

/// A helper to spawn tripods to control Hyper rotation
fn spawn_tripods(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    active_rotation: Rotation4d,
){
    let size = 0.5 * SCALE;
    spawn_tripod(commands,meshes,materials,Rotation4d::Xy,active_rotation,vec3(-7. * SCALE, Y_CTR_ROW1, 0.),size);
    spawn_tripod(commands,meshes,materials,Rotation4d::Xz,active_rotation,vec3(-6. * SCALE, Y_CTR_ROW1, 0.),size);
    spawn_tripod(commands,meshes,materials,Rotation4d::Xw,active_rotation,vec3(-5. * SCALE, Y_CTR_ROW1, 0.),size);
    spawn_tripod(commands,meshes,materials,Rotation4d::Yz,active_rotation,vec3(-7. * SCALE, Y_CTR_ROW2, 0.),size);
    spawn_tripod(commands,meshes,materials,Rotation4d::Yw,active_rotation,vec3(-6. * SCALE, Y_CTR_ROW2, 0.),size);
    spawn_tripod(commands,meshes,materials,Rotation4d::Zw,active_rotation,vec3(-5. * SCALE, Y_CTR_ROW2, 0.),size);
}

/// A helpe to spawn one tripod
fn spawn_tripod (
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    rotation: Rotation4d,
    activate_rotation: Rotation4d,
    position: Vec3,
    scale: f32
) {
    let length = 1.0;
    let radius = 0.05 * length;

    // set colors depending rotation 
    let gray = Color::from(Srgba::gray(0.1));
    let gray_transparent = Color::from(Srgba::rgba_u8(25, 25, 25, 160));
    let yellow_transparent = Color::from(Srgba::rgba_u8(255, 255, 0, 160));
    let (x_color, y_color, z_color, w_color) = match rotation {
        Rotation4d::Xy => (
            Color::from(Srgba::RED),
            Color::from(Srgba::GREEN),
            gray,
            gray_transparent,
        ),
        Rotation4d::Xz => (
            Color::from(Srgba::RED),
            gray,
            Color::from(Srgba::BLUE),
            gray_transparent,
        ),
        Rotation4d::Xw => (
            Color::from(Srgba::RED),
            gray,
            gray,
            yellow_transparent,
        ),
        Rotation4d::Yz => (
            gray,
            Color::from(Srgba::GREEN),
            Color::from(Srgba::BLUE),
            gray_transparent,
        ),
        Rotation4d::Yw => (
            gray,
            Color::from(Srgba::GREEN),
            gray,
            yellow_transparent,
        ),
        Rotation4d::Zw => (
            gray,
            gray,
            Color::from(Srgba::BLUE),
            yellow_transparent,
        ),
    };

    // Set up the hover materials.
    let white_matl = materials.add(Color::WHITE);
    let hover_matl = materials.add(Color::from(CYAN_300));
    let pressed_matl = materials.add(Color::from(YELLOW_300));
    let activated_matl =materials.add(Color::linear_rgb(0.8, 0.2, 0.8));

    // Parent Entity (Holds position/transform)
    commands.spawn((
        Name::new("Tripod"),
        Transform::from_scale(vec3(scale,scale,scale)).with_translation(position),
        Visibility::default(),
        Tripod {rotation},
    )).observe(toggle_rotation_on_press)
    .with_children(|parent| {

        //Center gray
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(5. * radius))),
            MeshMaterial3d(white_matl.clone()),
        )).observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
        .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
        .observe(update_material_on::<Pointer<Press>>(pressed_matl.clone()))
        .observe(update_material_on::<Pointer<Release>>(hover_matl.clone()));

        // On/Off marker
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(5.2 * radius))),
            MeshMaterial3d(activated_matl.clone()),
            Rotation {rotation},
            if activate_rotation == rotation {Visibility::Visible} else {Visibility::Hidden},
            Pickable::IGNORE,
        ));

        //y-Axis green
        parent.spawn((
            Mesh3d(meshes.add(Cylinder::new(radius, length))),
            MeshMaterial3d(materials.add(y_color)),
            Transform::from_translation(vec3(0.0, 0.5*length, 0.0)),
        ));
        parent.spawn((
            Mesh3d(meshes.add(Cone::new(2.*radius, 3.*radius))),
            MeshMaterial3d(materials.add(y_color)),
            Transform::from_translation(vec3(0.0, length, 0.0)),
        ));

        //z-Axis blue
        parent.spawn((
            Mesh3d(meshes.add(Cylinder::new(radius, length))),
            MeshMaterial3d(materials.add(z_color)),
            Transform::from_rotation(Quat::from_rotation_x(PI/2.)).with_translation(vec3(0.0, 0.0, 0.5*length)),
        ));
        parent.spawn((
            Mesh3d(meshes.add(Cone::new(2.*radius, 3.*radius))),
            MeshMaterial3d(materials.add(z_color)),
            Transform::from_rotation(Quat::from_rotation_x(PI/2.)).with_translation(vec3(0.0, 0.0, length)),
        ));

        //x-Axis red
        parent.spawn((
            Mesh3d(meshes.add(Cylinder::new(radius, length))),
            MeshMaterial3d(materials.add(x_color)),
            Transform::from_rotation(Quat::from_rotation_z(PI/2.)).with_translation(vec3(0.5*length, 0.0, 0.0)),
        ));
        parent.spawn((
            Mesh3d(meshes.add(Cone::new(2.*radius, 3.*radius))),
            MeshMaterial3d(materials.add(x_color)),
            Transform::from_rotation(Quat::from_rotation_z(PI/-2.)).with_translation(vec3(length, 0.0, 0.0)),
        ));

        //w-Axis yellow
        let mut trafo = Transform::from_translation(vec3(0.,0.5*length,0.));
        trafo = Transform::from_rotation(Quat::from_euler(EulerRot::ZXY,PI/4.,PI/4.,0.)).mul_transform(trafo);
        parent.spawn((
            Mesh3d(meshes.add(Cylinder::new(radius, length))),
            MeshMaterial3d(materials.add(w_color)),//yellow
            trafo,
        ));
        let mut trafo = Transform::from_translation(vec3(0.,length,0.));
        trafo = Transform::from_rotation(Quat::from_euler(EulerRot::ZXY,PI/4.,PI/4.,0.)).mul_transform(trafo);
        parent.spawn((
            Mesh3d(meshes.add(Cone::new(2.*radius, 3.*radius))),
            MeshMaterial3d(materials.add(w_color)),//yellow
            trafo,
        ));
    });
}