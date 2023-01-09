use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use itertools::Itertools;

#[derive(Debug)]
pub struct CubeFace {
    axis_subdivision: u32,
    normal_direction: Vec3,
}

impl Default for CubeFace {
    fn default() -> Self {
        Self {
            axis_subdivision: 10,
            normal_direction: Vec3::from([1.0, 0.0, 0.0]),
        }
    }
}

impl From<CubeFace> for Mesh {
    fn from(plane: CubeFace) -> Self {
        let normal_direction = if !plane.normal_direction.is_normalized() {
            plane
                .normal_direction
                .try_normalize()
                .expect("normalizing SegmentedPlane::normal_direction")
        } else {
            plane.normal_direction
        };

        let x_unit_vector =
            Vec3::from([normal_direction.z, normal_direction.x, normal_direction.y]);
        let y_unit_vector = normal_direction.cross(x_unit_vector);

        debug!(
            "plane axis direction: [ {}, {}, {} ]",
            x_unit_vector, y_unit_vector, normal_direction
        );

        let vertex_product =
            (0..=plane.axis_subdivision).cartesian_product(0..=plane.axis_subdivision);

        debug!(
            "vertex product: {:?}",
            vertex_product.clone().collect::<Vec<_>>()
        );

        let vertices: Vec<([f32; 3], [f32; 3], [f32; 2])> = vertex_product
            .clone()
            .map(|(magnitude_x, magnitude_y)| {
                let percent = Vec2::from([magnitude_x as f32, magnitude_y as f32])
                    / plane.axis_subdivision as f32;

                let position = normal_direction / 2.0
                    + x_unit_vector * (percent.x - 0.5)
                    + y_unit_vector * (percent.y - 0.5);

                debug!("position: {}", position);

                (
                    position.normalize().to_array(),
                    plane.normal_direction.to_array(),
                    [
                        magnitude_x as f32 / plane.axis_subdivision as f32,
                        magnitude_y as f32 / plane.axis_subdivision as f32,
                    ],
                )
            })
            .collect::<Vec<_>>();

        let indices = Indices::U32(
            vertex_product
                .enumerate()
                .filter_map(|(index, (x, y))| {
                    if y >= plane.axis_subdivision {
                        None
                    } else if x >= plane.axis_subdivision {
                        None
                    } else {
                        Some([
                            [
                                index as u32,
                                index as u32 + 1 + 1 + plane.axis_subdivision,
                                index as u32 + 1,
                            ],
                            [
                                index as u32,
                                index as u32 + 1 + plane.axis_subdivision,
                                index as u32 + plane.axis_subdivision + 1 + 1,
                            ],
                        ])
                    }
                })
                .flatten()
                .flatten()
                .collect::<Vec<_>>(),
        );

        let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
        let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
        let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}
