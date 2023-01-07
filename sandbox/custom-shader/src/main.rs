mod orbit_cam;

use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_resource::{AsBindGroup, ShaderRef},
        settings::{PowerPreference, WgpuSettings},
        RenderPlugin,
    },
};
use orbit_cam::OrbitCamera;

const CLEAR: Color = Color::GRAY;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const HEIGHT: f32 = 600.0;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(CLEAR))
        .add_plugins(
            DefaultPlugins
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
                })
                .set(RenderPlugin {
                    wgpu_settings: WgpuSettings {
                        power_preference: PowerPreference::HighPerformance,
                        ..Default::default()
                    },
                }),
        )
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
            mesh: mesh_assets.add(Mesh::from(shape::Cube::default())).into(),
            material: custom_material_assets.add(MyMaterial {
                color: Color::ORANGE,
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Movable);
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "4a558437-427d-4904-a6f8-c51c5f10fe4e"]
struct MyMaterial {
    #[uniform(0)]
    color: Color,
}

impl Material for MyMaterial {
    // fn vertex_shader() -> ShaderRef {
    //     "my_material.wgsl".into()
    // }

    fn fragment_shader() -> ShaderRef {
        "my_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
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
