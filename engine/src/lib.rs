use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("First Rust Engine"))
            .expect("failed to create window");
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                // render calls will go here
            }
            _ => {}
        }
    }
}

pub fn run() {
    env_logger::init();

    let event_loop = EventLoop::new().expect("oopsie failed to create EventLoop");
    let mut app = App::default();
    event_loop.run_app(&mut app).expect("oopsie loop err");
}