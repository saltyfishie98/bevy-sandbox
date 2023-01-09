mod common;
mod material;
mod orbit_cam;
mod primitives;

use bevy::{
    log::LogPlugin,
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use material::MyMaterial;
use orbit_cam::OrbitCamera;

const CLEAR: Color = Color::GRAY;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const HEIGHT: f32 = 600.0;

fn main() {
    let mut application = App::new();

    application
        .register_type::<MyMaterial>()
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
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(WireframePlugin)
        .add_plugin(MaterialPlugin::<MyMaterial>::default())
        .add_plugin(OrbitCamera::default())
        .add_startup_system(setup)
        .add_system(move_components)
        .run();
}

fn setup(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut custom_material_assets: ResMut<Assets<MyMaterial>>,
) {
    commands
        .spawn(MaterialMeshBundle {
            mesh: mesh_assets
                // .add(Mesh::from(shape::Quad::default()))
                .add(Mesh::from(primitives::CubeFace::default()))
                .into(),
            material: custom_material_assets.add(MyMaterial {
                color: Color::ORANGE,
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Movable)
        .insert(Wireframe);

    commands.spawn(MaterialMeshBundle {
        mesh: mesh_assets
            .add(Mesh::from(shape::Cube { size: 0.25 }))
            .into(),
        material: custom_material_assets.add(MyMaterial::default()),
        ..Default::default()
    });
}

#[derive(Component)]
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

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct Resizable {}
