use state::State;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub mod vertex;
pub mod state;
pub mod instance;
pub mod render_state;
pub mod input_handler;
pub mod util;

pub fn main() {
    pollster::block_on(run());
}

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(&window).await;

    let mut render_state = state.render_state;
    let mut input_handler = state.input_handler;

    event_loop.run(move |event, _, control_flow| match event {
        // Window events
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            // Close window on close or ESC
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,

            WindowEvent::KeyboardInput { device_id, input, is_synthetic } => {
                input_handler.handle_kb_input(input);
            },

            WindowEvent::CursorMoved { device_id, position, modifiers } => {

            }

            WindowEvent::Resized(physical_size) => {
                render_state.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                render_state.resize(**new_inner_size);
            }
            _ => {}
        },

        Event::DeviceEvent { 
            device_id, 
            event
        } => match event {
            DeviceEvent::Button { button, state } => {
                input_handler.handle_mouse_button(button, state);
            }
            _ => {}
        }

        Event::RedrawRequested(window_id) if window_id == window.id() => {
            // state.update();
            match render_state.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => render_state.resize(render_state.size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        },
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            window.request_redraw();
        },
        _ => {}
    });
}
