mod common;
mod material;
mod mesh_data;
mod utils;

use bevy::{log::LogPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use material::MyMaterial;
use utils::OrbitCamera;

const CLEAR: Color = Color::GRAY;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const HEIGHT: f32 = 600.0;

fn main() {
    let mut application = App::new();

    application
        .register_type::<MyMaterial>()
        .register_type::<Movable>()
        .insert_resource(ClearColor(CLEAR))
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
        // Debug
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(mesh_data::plugin::DebugCubeSphere)
        //
        // Settings
        .add_plugin(MaterialPlugin::<MyMaterial>::default())
        .add_plugin(OrbitCamera::default())
        .add_startup_system(setup)
        .add_system(move_components)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(&mesh_data::CubeSphereData::default())),
            material: materials.add(Color::ORANGE.into()),
            transform: Transform::from_xyz(-1.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(mesh_data::CubeSphere)
        .insert(Movable)
        .insert(Name::new("Planet"));

    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                radius: 2.0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Movable)
        .insert(Name::new("Light"));

    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes
                .add(Mesh::from(shape::UVSphere {
                    radius: 0.5,
                    ..Default::default()
                }))
                .into(),
            material: materials.add(Color::ORANGE.into()),
            transform: Transform::from_xyz(1.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Movable)
        .insert(Name::new("Sphere"));
}

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
