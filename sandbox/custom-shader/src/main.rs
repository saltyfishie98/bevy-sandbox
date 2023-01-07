mod material;
mod orbit_cam;

use bevy::{
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
    render::{
        settings::{PowerPreference, WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
};
use material::MyMaterial;
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
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        ..Default::default()
                    },
                }),
        )
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
            mesh: mesh_assets.add(Mesh::from(shape::Cube::default())).into(),
            material: custom_material_assets.add(MyMaterial {
                color: Color::ORANGE,
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Movable)
        .insert(Wireframe);
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
