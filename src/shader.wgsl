// Vertex shader

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
};

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,

    @location(9) color: vec3<f32>,
};

struct VpSizeUniform {
    viewport_size: vec2<f32>,
}
@group(0) @binding(0) // 1.
var<uniform> vp_size: VpSizeUniform;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    var out: VertexOutput;
    out.color = instance.color * instance.color;
    out.clip_position = ((model_matrix * vec4<f32>(model.position, 0.0, 1.0)) / vec4<f32>((vp_size.viewport_size / 2.0), 1.0, 1.0))
         * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(-1.0, 1.0, 1.0, 0.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}