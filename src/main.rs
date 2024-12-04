//! A simple 3D scene with light shining over a cube sitting on a plane.

mod loading;

use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use loading::{unload_current_visualization, LoadingData, VisualizzationComponents};
//use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

#[derive(Default, Debug, Resource)]
pub enum Resolution {
    #[default]
    Cube,
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Resolution>()
        .add_systems(Update, (setup, unload_current_visualization).run_if(resource_changed::<Resolution>))
        //.add_plugins(WorldInspectorPlugin::new())
        .add_plugins(LogDiagnosticsPlugin::default())
        //.add_plugins(PanOrbitCameraPlugin)
        .add_plugins(loading::LoadingScreenPlugin)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: ResMut<AssetServer>,
    mut loading_data: ResMut<LoadingData>,
) {


    // circular base
    //let model: Handle<Scene> = asset_server.load("untitled.gltf#Scene0");
    /*commands.spawn((SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("untitled.glb"))),
        //Transform::from_xyz(0.0, 0.0, 0.0),
        VisualizzationComponents,
        Visibility::Hidden
));*/

    let model: Handle<Scene> = asset_server.load(GltfAssetLabel::Scene(0).from_asset("untitled.glb"));
    //println!(" {}", model.id());
    //let mesh = meshes.add(Circle::new(4.0));
    loading_data.add_asset(&model);
    commands
        .spawn((
            SceneRoot(model),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            VisualizzationComponents,
            Visibility::Hidden
        ));
    //world.add_entity(e);

    // cube
    let mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    commands
        .spawn((
            Mesh3d(mesh),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::from_xyz(0.0, 0.5, 0.0),
            VisualizzationComponents,
            Visibility::Hidden
        ));


    // light
    commands
        .spawn((
            PointLight {
                shadows_enabled: true,
                ..default()
            },
            Transform::from_xyz(4.0, 8.0, 4.0),
            VisualizzationComponents,
            Visibility::Hidden
        ));

    // camera
    commands
        .spawn((
            Camera3d::default(),
            Camera{
                is_active: false,
                ..default()
            },
            
            //Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            VisualizzationComponents,
            //PanOrbitCamera::default(),
        ));
}
