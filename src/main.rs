use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod camera;
mod state;
mod texture;
mod vertex;

const MIN_WINDOW_SIZE: PhysicalSize<i32> = PhysicalSize::new(400, 400);

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Learning WGPU")
        .with_min_inner_size(MIN_WINDOW_SIZE)
        .with_inner_size(MIN_WINDOW_SIZE)
        .build(&event_loop)
        .unwrap();

    let mut state = state::State::new(&window).await;
    event_loop.run(move |event, _, flow| {
        match event {
            Event::RedrawRequested(id) => {
                if id == window.id() {
                    state.update(&window);
                    match state.render() {
                        Ok(_) => {}
                        // Reconfigure the surface if lost
                        Err(wgpu::SurfaceError::Lost) => state.resize(state.get_size()),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *flow = ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
            }
            Event::WindowEvent {
                window_id,
                ref event,
            } => {
                if window_id == window.id() && !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested => *flow = ControlFlow::Exit,
                        WindowEvent::Resized(size) => {
                            state.resize(*size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            state.resize(**new_inner_size);
                        }
                        _ => (),
                    }
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => (),
        }
    });
}
