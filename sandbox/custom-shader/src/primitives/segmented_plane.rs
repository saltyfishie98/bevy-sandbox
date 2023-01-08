use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use itertools::Itertools;

#[derive(Debug, Copy, Clone, Reflect)]
pub struct SegmentedPlane {
    size: f32,
    axis_subdivision: u32,
    normal_direction: Vec3,
}

impl Default for SegmentedPlane {
    fn default() -> Self {
        Self {
            size: 1.0,
            axis_subdivision: 1,
            normal_direction: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        }
    }
}

impl From<SegmentedPlane> for Mesh {
    fn from(plane: SegmentedPlane) -> Self {
        let extent = f32::sqrt(plane.size);

        let jump = extent / plane.axis_subdivision as f32;

        let vertex_product =
            (0..=plane.axis_subdivision).cartesian_product(0..=plane.axis_subdivision);

        debug!(
            "jump: {}, vertex_product: {:?}",
            jump,
            vertex_product.clone().collect::<Vec<_>>()
        );

        let vertices = vertex_product
            .clone()
            .map(|(x, y)| {
                debug!("");
                let calculate_position = |input: f32| -> f32 {
                    let out = (input * jump) - (0.5 * extent);
                    debug!("position: {}", out);
                    out
                };

                let position = [
                    calculate_position(x as f32),
                    calculate_position(y as f32),
                    0.0,
                ];

                (
                    position,
                    plane.normal_direction,
                    [
                        x as f32 / plane.axis_subdivision as f32,
                        y as f32 / plane.axis_subdivision as f32,
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
