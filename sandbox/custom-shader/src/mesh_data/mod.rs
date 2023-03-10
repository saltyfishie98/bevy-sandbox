mod cube_sphere;
pub use cube_sphere::*;

mod mesh_noise;
pub use mesh_noise::*;

mod inspect;
pub use inspect::*;

pub trait ProceduralShape {
    fn generate_data(&self) -> (VerticesData, IndicesData);
}

pub struct VertexData {
    position: [f32; 3],
    normal: [f32; 3],
    uv: [f32; 2],
}

pub type VerticesData = Vec<VertexData>;
pub type IndicesData = Vec<u32>;
