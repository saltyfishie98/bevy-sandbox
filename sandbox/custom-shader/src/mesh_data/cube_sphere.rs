use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use bevy_inspector_egui::prelude::*;
use itertools::Itertools;

// #[derive(Component, Reflect)]
// pub struct CubeSphere;

#[derive(Debug, Component, Reflect, InspectorOptions, PartialEq, Copy, Clone)]
#[reflect(Component, InspectorOptions)]
pub struct CubeSphere {
    #[inspector(min = 1)]
    pub resolution: u32,
    #[inspector(min = 0.0)]
    pub radius: f32,
}

impl Default for CubeSphere {
    fn default() -> Self {
        Self {
            resolution: 10,
            radius: 0.5,
        }
    }
}

impl From<&CubeSphere> for Mesh {
    fn from(plane: &CubeSphere) -> Self {
        let faces: Vec<Vec3> = vec![
            [0.0, 0.0, 1.0].into(),  // OUT
            [0.0, 0.0, -1.0].into(), // IN
            [0.0, 1.0, 0.0].into(),  // UP
            [0.0, -1.0, 0.0].into(), // DOWN
            [1.0, 0.0, 0.0].into(),  // RIGHT
            [-1.0, 0.0, 0.0].into(), // LEFT
        ];

        let mut vertices_vec: VertexData = [].into();
        let mut indices_vec: Vec<u32> = [].into();
        let mut index_offset = 0;

        let vertex_template = (0..=plane.resolution).cartesian_product(0..=plane.resolution);

        for face_direction in faces {
            let mut face_vertices =
                create_face_vertices(&plane, face_direction, vertex_template.clone());

            let mut face_indices =
                create_face_indices(&plane, index_offset, vertex_template.clone());

            index_offset = face_indices.iter().max().unwrap().clone() + 1;

            vertices_vec.append(&mut face_vertices);
            indices_vec.append(&mut face_indices);
        }

        // debug!("vertices count: {}", vertices_vec.len());
        // debug!("indices count: {}", indices_vec.len());

        // format and set mesh attributes
        let positions: Vec<_> = vertices_vec.iter().map(|(p, _, _)| *p).collect();
        let normals: Vec<_> = vertices_vec.iter().map(|(_, n, _)| *n).collect();
        let uvs: Vec<_> = vertices_vec.iter().map(|(_, _, uv)| *uv).collect();

        let indices = Indices::U32(indices_vec);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}

//// Plugins ///////////////////////////////////////////////////////////////////////////////////////

pub mod plugin {
    use super::*;
    use bevy::pbr::wireframe::{Wireframe, WireframePlugin};

    #[allow(dead_code)]
    pub struct CubeSphere;

    impl Plugin for CubeSphere {
        fn build(&self, app: &mut App) {
            app.register_type::<super::CubeSphere>()
                .add_startup_system_to_stage(StartupStage::PostStartup, init_cube_sphere);
        }
    }

    fn init_cube_sphere(
        mut meshes: ResMut<Assets<Mesh>>,
        query: Query<(&super::CubeSphere, &Handle<Mesh>)>,
    ) {
        debug!("Init CubeSphere");
        for (cube_sphere, mesh_handle) in query.iter() {
            let mesh_opt = meshes.get_mut(mesh_handle);
            match mesh_opt {
                Some(cube) => {
                    *cube = Mesh::from(cube_sphere);
                }
                None => {}
            }
        }
    }

    #[allow(dead_code)]
    pub struct DebugCubeSphere;

    impl Plugin for DebugCubeSphere {
        fn build(&self, app: &mut App) {
            app.register_type::<CubeSphereDebugInfo>()
                .add_plugin(WireframePlugin)
                .add_startup_system_to_stage(StartupStage::PostStartup, insert_debug_components)
                .add_system(toggle_wireframe)
                .add_system(update_square_cube);
        }
    }

    #[derive(Component, Debug, Reflect, InspectorOptions)]
    #[reflect(Component, Default)]
    struct CubeSphereDebugInfo {
        show_wireframe: bool,
        outdated: bool,

        #[reflect(ignore)]
        old_data: super::CubeSphere,
    }

    impl Default for CubeSphereDebugInfo {
        fn default() -> Self {
            Self {
                show_wireframe: true,
                outdated: true,
                old_data: super::CubeSphere::default(),
            }
        }
    }

    fn insert_debug_components(
        mut commands: Commands,
        query: Query<Entity, With<super::CubeSphere>>,
    ) {
        // println!("{:?}", query);
        info!("Enabled CubeSphere debugging!");
        for entt in query.iter() {
            if let Some(mut entity) = commands.get_entity(entt) {
                entity.insert(CubeSphereDebugInfo::default());
            };
        }
    }

    fn toggle_wireframe(mut commands: Commands, query: Query<(Entity, &mut CubeSphereDebugInfo)>) {
        // println!("{:?}", query);
        for (entt, debug_info) in query.iter() {
            if let Some(mut entity) = commands.get_entity(entt) {
                if debug_info.show_wireframe {
                    entity.insert(Wireframe);
                } else {
                    entity.remove::<Wireframe>();
                }
            };
        }
    }

    fn update_square_cube(
        mut meshes: ResMut<Assets<Mesh>>,
        mut query: Query<(
            &super::CubeSphere,
            &mut CubeSphereDebugInfo,
            &mut Handle<Mesh>,
        )>,
    ) {
        // println!("{:?}", query);
        for (cube_sphere_data, mut debug_info, mut cube_mesh_handle) in query.iter_mut() {
            if *cube_sphere_data != debug_info.old_data {
                debug_info.outdated = true;
                debug_info.old_data = cube_sphere_data.clone();
            }

            if !debug_info.outdated {
                return;
            }

            let cube_opt = meshes.get_mut(&cube_mesh_handle);
            match cube_opt {
                Some(cube) => {
                    *cube = Mesh::from(cube_sphere_data);
                    debug_info.outdated = false;
                }
                None => {
                    *cube_mesh_handle = meshes.add(Mesh::from(cube_sphere_data));
                }
            }
        }
    }
}

//// Helpers ///////////////////////////////////////////////////////////////////////////////////////

type VertexData = Vec<([f32; 3], [f32; 3], [f32; 2])>;
type VertexTemplate =
    itertools::Product<std::ops::RangeInclusive<u32>, std::ops::RangeInclusive<u32>>;

fn create_face_vertices(
    cube_sphere: &CubeSphere,
    face_direction: Vec3,
    vertex_template: VertexTemplate,
) -> VertexData {
    fn convert_to_sphere_position(pos: Vec3) -> Vec3 {
        let Vec3 { x, y, z } = pos;

        let x2 = x * x;
        let y2 = y * y;
        let z2 = z * z;

        Vec3 {
            x: x * f32::sqrt(1.0 - (y2 + z2) / 2.0 + (y2 * z2) / 3.0),
            y: y * f32::sqrt(1.0 - (z2 + x2) / 2.0 + (z2 * x2) / 3.0),
            z: z * f32::sqrt(1.0 - (x2 + y2) / 2.0 + (x2 * y2) / 3.0),
        }
    }

    let x_unit_vector = Vec3::from([face_direction.z, face_direction.x, face_direction.y]);
    let y_unit_vector = face_direction.cross(x_unit_vector);

    let out = vertex_template
        .clone()
        .map(|(magnitude_x, magnitude_y)| {
            let percent = Vec2::from([magnitude_x as f32, magnitude_y as f32])
                / cube_sphere.resolution as f32;

            let cube_positions = face_direction
                + (percent.x - 0.5) * 2.0 * x_unit_vector
                + (percent.y - 0.5) * 2.0 * y_unit_vector;

            let sphere_positions = convert_to_sphere_position(cube_positions) * cube_sphere.radius;

            // debug!("magnitude: {}", position.length());

            // static mut INDEX: u32 = 0;
            // unsafe {
            //     debug!("[{}] position: {}", INDEX, position);
            //     INDEX += 1;
            // }

            (
                sphere_positions.to_array(),
                sphere_positions.to_array(),
                [
                    magnitude_x as f32 / cube_sphere.resolution as f32,
                    magnitude_y as f32 / cube_sphere.resolution as f32,
                ],
            )
        })
        .collect::<Vec<_>>();

    // debug!(
    //     "plane axis direction: [ {}, {}, {} ]\n",
    //     x_unit_vector, y_unit_vector, face_direction
    // );

    out
}

fn create_face_indices(
    face: &CubeSphere,
    index_offset: u32,
    vertex_template: VertexTemplate,
) -> Vec<u32> {
    let out = vertex_template
        .enumerate()
        .filter_map(|(i, (x, y))| {
            if y >= face.resolution {
                None
            } else if x >= face.resolution {
                None
            } else {
                let index = i as u32 + index_offset;
                Some([
                    [index, index + 1 + 1 + face.resolution, index + 1],
                    [
                        index,
                        index + 1 + face.resolution,
                        index + face.resolution + 1 + 1,
                    ],
                ])
            }
        })
        .flatten()
        .flatten()
        .collect::<Vec<_>>();

    // debug!("face indices {:?}\n", out);
    out
}
