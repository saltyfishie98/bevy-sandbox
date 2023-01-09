use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};

#[derive(AsBindGroup, Reflect, FromReflect, Debug, Clone, TypeUuid, Default)]
#[reflect(Default, Debug)]
#[uuid = "4a558437-427d-4904-a6f8-c51c5f10fe4e"]
pub struct MyMaterial {
    #[uniform(0)]
    pub color: Color,
}

impl Material for MyMaterial {
    fn vertex_shader() -> ShaderRef {
        "my_material.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "my_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }
}
