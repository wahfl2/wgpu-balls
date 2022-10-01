use cgmath::{Vector2, Quaternion, Vector3};
use winit::{window::Window, event_loop::ControlFlow};

use crate::{render_state::RenderState, input_handler::InputHandler, physics::{Physics, Ball}, util::Vec2, instance::Instance};

pub struct State {
    pub(crate) render_state: RenderState,
    pub(crate) input_handler: InputHandler,
    pub(crate) physics: Physics,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        Self {
            render_state: RenderState::new(window).await,
            input_handler: InputHandler::new(),
            physics: Physics::new(),
        }
    }

    pub fn update(&mut self, control_flow: &mut ControlFlow) {
        self.input_handler.handle_input();
        self.add_input_balls();
        self.physics.update();
        self.sync_balls();

        match self.render_state.render() {
            Ok(_) => {}
            // Reconfigure the surface if lost
            Err(wgpu::SurfaceError::Lost) => self.render_state.resize(self.render_state.size),
            // The system is out of memory, we should probably quit
            Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
            // All other errors (Outdated, Timeout) should be resolved by the next frame
            Err(e) => eprintln!("{:?}", e),
        }
    }

    pub fn add_input_balls(&mut self) {
        let balls_to_add = self.input_handler.balls_to_add.clone();
        self.input_handler.balls_to_add.clear();

        for pos in balls_to_add.iter() {
            self.physics.add_ball(Ball::new(pos.x, pos.y, 10.0));

            let instance = Instance {
                position: Vector2::new(pos.x, pos.y),
                rotation: Quaternion::from_sv(1.0, Vector3::unit_y()),
                scale: 10.0,
            };

            self.render_state.add_instance(instance);
        }
    }

    pub fn sync_balls(&mut self) {
        for (i, ball) in self.physics.balls.iter().enumerate() {
            self.render_state.instances[i].position = ball.pos.into();
        }
        self.render_state.recreate_instance_buffer();
    }
}