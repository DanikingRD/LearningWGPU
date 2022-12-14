use cgmath::{Deg, Vector3, vec3, Matrix4, vec4};
use winit::window::Window;

pub struct Camera {
    /// Field Of View in degress
    pub fov: f32,
    /// Aspect Ratio
    pub ratio: f32,
    // Z near distance
    pub znear: f32,
    // Z far distance
    pub zfar: f32,
}

impl Camera {
    fn build_view_proj(&self, window: &Window) -> cgmath::Matrix4<f32> {
        let translation = vec3(0.0, 0.0, 3.0);
        let size = window.inner_size();
        let proj = cgmath::perspective(Deg(self.fov), self.ratio, self.znear, self.zfar);
        let view = Matrix4::from_translation(translation);
        return OPENGL_TO_WGPU_MATRIX * proj * view;
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
