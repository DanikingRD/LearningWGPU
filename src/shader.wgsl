struct VInput {
    @location(0) pos: vec3<f32>,
    @location(1) color: vec3<f32>
}

struct VOutput {
    @builtin(position) vertices: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vertex_main(input: VInput) -> VOutput {
    var out: VOutput;
    out.vertices = vec4<f32>(input.pos, 1.0);
    out.color = input.color;
    return out;
}

@fragment
fn fragment_main(input: VOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.color, 1.0);
}

