struct VertexInfo {
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @builtin(position) coordinate: vec4<f32>,
};

struct MyMaterial {
    color: vec4<f32>,
}

@group(1) @binding(0)
var<uniform> uniform_data: MyMaterial;

// @vertex
// fn vertex(@builtin(vertex_index) in_vertex_index: u32) -> VertexInfo {

//     let x = f32(i32(in_vertex_index)) * 0.5 - 0.5;
//     let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;

//     var out: VertexInfo;
//     out.coordinate = vec4<f32>(x, y, 0.0, 1.0);

//     return out;
// }

@fragment
fn fragment(input: VertexInfo) -> @location(0) vec4<f32> {
    var output_color = vec4<f32>(input.coordinate, 1.0);

    return output_color;
}



