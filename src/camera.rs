use cgmath::{Vector3, Matrix4};
use winit::window::Window;

pub struct Camera {
    // eye: cgmath::Point3<f32>,
    // target: cgmath::Point3<f32>,
    // up: cgmath::Vector3<f32>,
    // aspect: f32,
    // fovy: f32,
    // znear: f32,
    // zfar: f32,
    eye: cgmath::Point3<f32>,
}

impl Camera {
    pub fn new(eye: cgmath::Point3<f32>) -> Self {
        Self { eye }
    }
    fn build_view_proj(&self, window: &Window) -> cgmath::Matrix4<f32> {
        let size = window.inner_size();
        let w = size.width;
        let h = size.height;
        let w_ratio = w as f32 / 100.0;
        let h_ratio = h as f32 / 100.0;
        let proj = OPENGL_TO_WGPU_MATRIX
        * cgmath::ortho(
            -w_ratio,
            w_ratio,
            -h_ratio,
            h_ratio,
            -1.0,
            1.0,
        );
        let vec = cgmath::point3(-100., 0., 0.);
        // let view: Matrix4<f32> = cgmath::Matrix4 {
        //     x: 1.0,
        //     y: 1.0,
        //     z: todo!(),
        //     w: todo!(),
        // };
        return OPENGL_TO_WGPU_MATRIX * proj;
    }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    // We can't use cgmath with bytemuck directly so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera, window: &Window) {
        self.view_proj = camera.build_view_proj(window).into();
    }
}
