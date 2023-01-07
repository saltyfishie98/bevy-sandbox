#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_view_bindings

// MUST BE AFTER BINDINGS
#import bevy_pbr::mesh_functions

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,  
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct FragmentInput {
    @location(0) world_position: vec4<f32>,  
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct MyMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> uniform_data: MyMaterial;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    var model = mesh.model;

    out.world_normal = mesh_normal_local_to_world(vertex.normal);
    out.world_position = mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
    out.clip_position = mesh_position_world_to_clip(out.world_position);
    out.uv = vertex.uv;

    return out;
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.world_position.xyz, 1.0);
} 
