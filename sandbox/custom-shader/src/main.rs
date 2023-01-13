mod common;
mod material;
mod mesh_data;
mod utils;

use std::f32::consts::PI;

use bevy::{log::LogPlugin, prelude::*};
use material::MyMaterial;
use mesh_data::CubeSphere;
use utils::OrbitCamera;

#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const CLEAR: Color = Color::GRAY;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const HEIGHT: f32 = 600.0;

fn main() {
    let mut application = App::new();

    application
        .register_type::<MyMaterial>()
        .register_type::<Movable>()
        .insert_resource(ClearColor(CLEAR))
        //
        // App settings
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    filter: "winit=info,bevy_render=info,custom_shader=debug".into(),
                    level: bevy::log::Level::ERROR,
                })
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "test-window".into(),
                        resizable: false,
                        width: HEIGHT * ASPECT_RATIO,
                        height: HEIGHT,
                        scale_factor_override: Some(1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
        )
        //
        // Plugins
        .add_plugin(MaterialPlugin::<MyMaterial>::default())
        .add_plugin(OrbitCamera::default())
        .add_plugin(mesh_data::CubeSpherePlugin)
        //
        // Systems
        .add_startup_system(setup)
        .add_system(move_components);

    #[cfg(debug_assertions)]
    application
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(mesh_data::InspectPlugin::<CubeSphere>::default());

    application.run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands
        .spawn(MaterialMeshBundle {
            material: materials.add(Color::ORANGE.into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(mesh_data::CubeSphere::default())
        .insert(mesh_data::MeshNoise::<CubeSphere>::default())
        .insert(Movable)
        .insert(Name::new("Planet"));

    commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 5000.0,
                ..Default::default()
            },
            transform: Transform {
                rotation: Quat::from_axis_angle([1.0, 0.0, 0.0].into(), -PI / 2.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Movable)
        .insert(Name::new("Light"));
}

//// Misc Plugins //////////////////////////////////////////////////////////////////////////////////

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct Movable;

fn move_components(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Movable>>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if input.pressed(KeyCode::W) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::S) {
            direction.y -= 1.0;
        }
        if input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }
        if input.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }

        transform.translation += time.delta_seconds() * direction;
    }
}
