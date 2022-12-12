#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}
pub const OCTAGON_VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.0, 0.0], // 0
        uv: [1.0, 0.0],
    }, // A  10
    Vertex {
        position: [-0.5, -0.5, 0.0], // 1
        uv: [0.0, 1.0],
    }, // B
    Vertex {
        position: [0.0, -1.0, 0.0], // 2
        uv: [0.0, 0.0],
    }, // C
    Vertex {
        position: [-1.0, 0.0, 0.0], // 3
        uv: [0.0, 0.0],
    }, // C
    Vertex {
        position: [-0.5, 0.5, 0.0],
        uv: [0.0, 0.0],
    }, // C
    Vertex {
        position: [0.0, 1.0, 0.0],
        uv: [0.0, 0.0],
    }, // C
    Vertex {
        position: [0.5, 0.5, 0.0],
        uv: [0.0, 0.0],
    }, // C
    Vertex {
        position: [1.0, 0.0, 0.0],
        uv: [0.0, 0.0],
    }, // C
    Vertex {
        position: [0.5, -0.5, 0.0],
        uv: [0.0, 0.0],
    }, // C
];
pub const SQUARE_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.8, 0.8, 0.0], // top right
        // top left
        // bottom left
        uv: [0.0, 1.0],
    }, // top left
    Vertex {
        position: [0.8, 0.8, 0.0],
        uv: [1.0, 1.0],
    }, // top right
    Vertex {
        position: [0.8, -0.8, 0.0],
        uv: [0.0, 0.0],
    }, // bottom right
    Vertex {
        position: [-0.8, -0.8, 0.0],
        uv: [0.0, 0.0],
    }, // bottom left
];
pub const SQUARE_INDICES: &[u16] = &[1, 0, 3, 3, 2, 1];
pub const OCTAGON_INDICES: &[u16] = &[
    0, 1, 2, 0, 3, 1, 0, 4, 3, 0, 5, 4, 6, 5, 0, 7, 6, 0, 7, 0, 2,
];

#[derive(Debug)]
pub enum RenderShape {
    OCTAGON = 0,
    SQUARE = 1,
}
