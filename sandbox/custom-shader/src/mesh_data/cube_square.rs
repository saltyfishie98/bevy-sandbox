use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use bevy_inspector_egui::prelude::*;

use itertools::Itertools;

#[derive(Debug, Component, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct SquareCube {
    #[inspector(min = 1)]
    pub resolution: u32,
}

impl Default for SquareCube {
    fn default() -> Self {
        Self { resolution: 10 }
    }
}

pub fn update_square_cube(
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(&SquareCube, &Handle<Mesh>), With<Transform>>,
) {
    // println!("{:?}", query);
    for (cube_data, cube_mesh_handle) in query.iter() {
        let cube_opt = meshes.get_mut(&cube_mesh_handle);
        match cube_opt {
            Some(cube) => {
                *cube = Mesh::from(cube_data);
                debug!("updated cube mesh")
            }
            None => {
                debug!("no cube mesh")
            }
        }
    }
}

impl From<&SquareCube> for Mesh {
    fn from(plane: &SquareCube) -> Self {
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

        let vertex_template = (0..=plane.resolution).cartesian_product(0..=plane.resolution);

        let mut index_offset = 0;

        for face_direction in faces {
            let mut face_vertices =
                create_face_vertices(&plane, face_direction, vertex_template.clone());

            let mut face_indices =
                create_face_indices(&plane, index_offset, vertex_template.clone());

            index_offset = face_indices.iter().max().unwrap().clone() + 1;

            vertices_vec.append(&mut face_vertices);
            indices_vec.append(&mut face_indices);
        }

        debug!("vertices count: {}", vertices_vec.len());
        debug!("indices count: {}", indices_vec.len());

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

//// Helpers ///////////////////////////////////////////////////////////////////////////////////////

type VertexData = Vec<([f32; 3], [f32; 3], [f32; 2])>;
type VertexTemplate =
    itertools::Product<std::ops::RangeInclusive<u32>, std::ops::RangeInclusive<u32>>;

fn create_face_vertices(
    face: &SquareCube,
    face_direction: Vec3,
    vertex_template: VertexTemplate,
) -> VertexData {
    let x_unit_vector = Vec3::from([face_direction.z, face_direction.x, face_direction.y]);
    let y_unit_vector = face_direction.cross(x_unit_vector);

    let out = vertex_template
        .clone()
        .map(|(magnitude_x, magnitude_y)| {
            let percent =
                Vec2::from([magnitude_x as f32, magnitude_y as f32]) / face.resolution as f32;

            let position = (face_direction / 2.0
                + x_unit_vector * (percent.x - 0.5)
                + y_unit_vector * (percent.y - 0.5))
                .normalize()
                / 2.0;

            // debug!("magnitude: {}", position.length());

            // static mut INDEX: u32 = 0;
            // unsafe {
            //     debug!("[{}] position: {}", INDEX, position);
            //     INDEX += 1;
            // }

            (
                position.to_array(),
                position.to_array(),
                [
                    magnitude_x as f32 / face.resolution as f32,
                    magnitude_y as f32 / face.resolution as f32,
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
    face: &SquareCube,
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
