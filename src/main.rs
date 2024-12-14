//! A simple 3D scene with light shining over a cube sitting on a plane.

mod loading;
mod settings;

use bevy::{
      diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*
};
use bevy_asset::embedded_asset;
use bevy_egui::EguiPlugin;
use bevy_obj::ObjPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use loading::{unload_current_visualization, LoadingData, LoadingScreenPlugin, VisualizzationComponents};
use settings::{Resolution, SettingsPlugin};

struct EmbeddedAssetPlugin;

impl Plugin for EmbeddedAssetPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "embedded_assets/logo.png");
        embedded_asset!(app, "embedded_assets/alessio.obj");
        embedded_asset!(app, "embedded_assets/alessio.mtl");
        embedded_asset!(app, "embedded_assets/alessio.png");
    }
}

fn main() {
    let window = WindowPlugin {
        primary_window: Some(Window {
            title: "Cyber Bevy".to_string(),
            ..default()
        }),
        ..default()
    };
    App::new()
        // default plugin
        .add_plugins(DefaultPlugins.set(window))
        
        // libraries plugins
        .add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()))
        .add_plugins(ObjPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(PanOrbitCameraPlugin)

        // custom plugins
        .add_plugins(EmbeddedAssetPlugin)
        .add_plugins(SettingsPlugin)
        .add_plugins(LoadingScreenPlugin{
            img_path: "embedded://cyber_bevy/embedded_assets/logo.png".to_string(),
        })

        // insert setup function 
        
        .add_systems(
            Update,
            (unload_current_visualization, setup )
                .chain()
                .run_if(resource_changed::<Resolution>),
        )
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: ResMut<AssetServer>,
    mut loading_data: ResMut<LoadingData>,
    set: Res<Resolution>
) {
    // add a circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        VisualizzationComponents,
        //Visibility::Hidden,
    ));
    if *set == Resolution::High {
    
        // ask to load a 3d model
        let model =
            asset_server.load("embedded://cyber_bevy/embedded_assets/alessio.obj");
        // add it to the loading queue
        loading_data.add_asset(&model);
        commands.spawn((
            SceneRoot(model),
            MeshMaterial3d(materials.add(Color::WHITE)),
        // pos,
            VisualizzationComponents,
            Visibility::Hidden,
        ));
    }else{
        let model: Handle<Image> =
            asset_server.load("embedded://cyber_bevy/embedded_assets/logo.png");
            loading_data.add_asset(&model);
    }
    //world.add_entity(e);

    // cube
    /*let mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        VisualizzationComponents,
        Visibility::Hidden,
    ));*/

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(2.0, 8.0, 4.0),
        VisualizzationComponents,
        Visibility::Hidden,
    ));

    // camera
    commands.spawn((
        PanOrbitCamera {
            pitch_lower_limit: Some(0.0),
            ..default()
        },
        Camera {
            is_active: false,
            ..default()
        },
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        VisualizzationComponents,
        //PanOrbitCamera::default(),
    ));
}
