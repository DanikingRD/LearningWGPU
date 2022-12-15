struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
}

struct CameraUniform {
    proj: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VInput {
    @location(0) pos: vec3<f32>,
    @location(1) uv: vec2<f32>
}

struct VOutput {
    @builtin(position) vertices: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vertex_main(input: VInput, instance: InstanceInput) -> VOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );
    var out: VOutput;
    out.uv = input.uv;
    out.vertices = camera.proj * model_matrix * vec4<f32>(input.pos, 1.0);
    return out;
}

@group(0) @binding(0)
var texture: texture_2d<f32>;
@group(0)@binding(1)
var texture_sampler: sampler;

@fragment
fn fragment_main(input: VOutput) -> @location(0) vec4<f32> {
    return textureSample(texture, texture_sampler, input.uv);
} 

