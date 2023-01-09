use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use crate::common::rad_to_deg;
use itertools::Itertools;

#[derive(Reflect, FromReflect, Debug, Clone)]
#[reflect(Default, Debug)]
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
        let normal_direction = if !plane.normal_direction.is_normalized() {
            plane
                .normal_direction
                .try_normalize()
                .expect("normalizing SegmentedPlane::normal_direction")
        } else {
            plane.normal_direction
        };

        let extent = f32::sqrt(plane.size);
        let jump = extent / plane.axis_subdivision as f32;
        let vertex_product =
            (0..=plane.axis_subdivision).cartesian_product(0..=plane.axis_subdivision);

        // debug!(
        //     "jump: {}, vertex_product: {:?}",
        //     jump,
        //     vertex_product.clone().collect::<Vec<_>>()
        // );

        let src_vec = Vec3::from([0.0, 0.0, 1.0]);
        let rotation_matrix = rotation_matrix(src_vec, normal_direction);

        let vertices: Vec<([f32; 3], [f32; 3], [f32; 2])> = vertex_product
            .clone()
            .map(|(x, y)| {
                debug!("");
                let calculate_position = |input: f32| -> f32 {
                    let out = (input * jump) - (0.5 * extent);
                    debug!("position: {}", out);
                    out
                };

                let positions = [
                    calculate_position(x as f32),
                    calculate_position(y as f32),
                    0.0,
                ];

                let rotated = rotation_matrix.mul_vec3(positions.into()).to_array();
                debug!("rotated: {:?}", rotated);

                (
                    rotated,
                    plane.normal_direction.to_array(),
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

fn rotation_matrix(src: Vec3, dst: Vec3) -> Mat3 {
    fn angle_between(src: [f32; 2], dst: [f32; 2]) -> f32 {
        let src_vec = Vec2::from(src);
        let dst_vec = Vec2::from(dst);
        let ans = dst_vec.angle_between(src_vec);

        if !ans.is_nan() {
            ans
        } else {
            0.0
        }
    }

    // (zeta) rotate about x
    let z = angle_between([src.y, src.z], [dst.y, dst.z]);

    // (beta) rotate about y
    let b = angle_between([src.x, src.z], [dst.x, dst.z]);

    // (alpha) rotate about z
    let a = angle_between([src.x, src.y], [dst.x, dst.y]);

    debug!(
        "angle between normals: ( {} deg, {} deg, {} deg )",
        rad_to_deg(z),
        rad_to_deg(b),
        rad_to_deg(a)
    );

    let about_x = Mat3::from_cols(
        [1.0, 0.0, 0.0].into(),
        [0.0, f32::cos(z), -f32::sin(z)].into(),
        [0.0, f32::sin(z), f32::cos(z)].into(),
    );

    let about_y = Mat3::from_cols(
        [f32::cos(b), 0.0, -f32::sin(b)].into(),
        [0.0, 1.0, 0.0].into(),
        [f32::sin(b), 0.0, f32::cos(b)].into(),
    );

    let about_z = Mat3::from_cols(
        [f32::cos(a), f32::sin(a), 0.0].into(),
        [-f32::sin(a), f32::cos(a), 0.0].into(),
        [0.0, 0.0, 1.0].into(),
    );

    Mat3::IDENTITY
        .mul_mat3(&about_z)
        .mul_mat3(&about_y)
        .mul_mat3(&about_x)
}

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::ApproxEq;

    const DEFAULT_NORMAL_DIRECTION: Vec3 = Vec3::from_array([0.0, 0.0, 1.0]);
    const PRECISION: f32 = 0.0000001;
    const ULPS: i32 = 2;

    #[test]
    fn test_rotation_matrix() {
        log_result_header("test_rotation_matrix");

        transform_to(Vec3::from([0.0, 0.0, 1.0]));
        transform_to(Vec3::from([0.0, 1.0, 0.0]));
        transform_to(Vec3::from([1.0, 0.0, 0.0]));

        transform_to(Vec3::from([0.0, 0.0, -1.0])); // Failling
        transform_to(Vec3::from([0.0, -1.0, 0.0]));
        transform_to(Vec3::from([-1.0, 0.0, 0.0]));

        log_result_end();
    }

    #[test]
    #[should_panic]
    fn test_normalizing_zero() {
        transform_to(Vec3::from([0.0, 0.0, 0.0]));
    }

    fn log_result_header(title: &'_ str) {
        println!("===== {} =====", title);
    }

    fn log_result_end() {
        println!("\n");
    }

    fn transform_to(target: Vec3) {
        let transformation = rotation_matrix(DEFAULT_NORMAL_DIRECTION, target);
        let Vec3 { x, y, z } = transformation.mul_vec3(DEFAULT_NORMAL_DIRECTION);

        println!(
            "expect: {:?}, result: {:?}",
            [target.x, target.y, target.z],
            [x, y, z]
        );

        assert!(x.approx_eq(target.x, (PRECISION, ULPS)));
        assert!(y.approx_eq(target.y, (PRECISION, ULPS)));
        assert!(z.approx_eq(target.z, (PRECISION, ULPS)));
    }
}
