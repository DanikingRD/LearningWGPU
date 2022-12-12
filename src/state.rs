use wgpu::util::DeviceExt;
use winit::{
    event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
    window::Window,
};

use crate::vertex::{
    RenderShape, Vertex, OCTAGON_INDICES, OCTAGON_VERTICES, SQUARE_INDICES, SQUARE_VERTICES,
};

#[derive(Debug)]
pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    clear_color: wgpu::Color,
    render_pipeline: wgpu::RenderPipeline,
    octagon_vertex_buffer: wgpu::Buffer,
    octagon_index_buffer: wgpu::Buffer,
    square_vertex_buffer: wgpu::Buffer,
    square_index_buffer: wgpu::Buffer,
    shape: RenderShape,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let instace = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instace.create_surface(&window) };
        // Handle to the graphics card
        let adapter = instace
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();
        //  println!("{:?}", &adapter.features());
        //  println!("{:?}", &surface.get_supported_formats(&adapter));

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &config);

        let clear_color = wgpu::Color::BLACK;
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                push_constant_ranges: &[],
                bind_group_layouts: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vertex_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fragment_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let octagon_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Octagon Vertex Buffer"),
            contents: bytemuck::cast_slice(OCTAGON_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let octagon_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Octagon Index Buffer"),
            contents: bytemuck::cast_slice(OCTAGON_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let square_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Square Vertex Buffer"),
            contents: bytemuck::cast_slice(SQUARE_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let square_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Square Index Buffer"),
            contents: bytemuck::cast_slice(SQUARE_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let shape = RenderShape::OCTAGON;
        Self {
            surface,
            device,
            queue,
            config,
            size,
            clear_color,
            render_pipeline,
            octagon_vertex_buffer,
            octagon_index_buffer,
            square_vertex_buffer,
            square_index_buffer,
            shape,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        if let WindowEvent::CursorMoved { position, .. } = event {
            println!("Capturing mouse events");
            println!("Red {}", position.x as f64 / self.size.width as f64);
            println!("Green {}", position.y as f64 / self.size.height as f64);
            self.clear_color = wgpu::Color {
                r: position.x as f64 / self.size.width as f64,
                g: position.y as f64 / self.size.height as f64,
                b: 1.0,
                a: 1.0,
            };
            return true;
        }
        if let WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Space),
                    ..
                },
            ..
        } = event
        {
            self.shape = match self.shape {
                RenderShape::OCTAGON => RenderShape::SQUARE,
                RenderShape::SQUARE => RenderShape::OCTAGON,
            };
        }
        false
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let surface = self.surface.get_current_texture()?;
        let view = surface
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Command Enconder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            match self.shape {
                RenderShape::OCTAGON => {
                    render_pass.set_vertex_buffer(0, self.octagon_vertex_buffer.slice(..));
                    render_pass.set_index_buffer(
                        self.octagon_index_buffer.slice(..),
                        wgpu::IndexFormat::Uint16,
                    );
                    render_pass.draw_indexed(0..OCTAGON_INDICES.len() as u32, 0, 0..1);
                }
                RenderShape::SQUARE => {
                    render_pass.set_vertex_buffer(0, self.square_vertex_buffer.slice(..));
                    render_pass.set_index_buffer(
                        self.square_index_buffer.slice(..),
                        wgpu::IndexFormat::Uint16,
                    );
                    render_pass.draw_indexed(0..SQUARE_INDICES.len() as u32, 0, 0..1);
                }
            }
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        surface.present();
        Ok(())
    }

    pub fn get_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }
}
