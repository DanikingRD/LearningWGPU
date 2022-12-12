struct VInput {
    @location(0) pos: vec3<f32>,
    @location(1) uv: vec2<f32>
}

struct VOutput {
    @builtin(position) vertices: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vertex_main(input: VInput) -> VOutput {
    var out: VOutput;
    out.vertices = vec4<f32>(input.pos, 1.0);
    out.uv = input.uv;
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

