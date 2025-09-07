use dlopen::raw::Library;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                if let Some(window) = &self.window {
                    // call function normally (works)
                    winit_dll_bug_macos::access_window_functions(window);

                    // Dll function (crashes)
                    let dll = Library::open("./libwinit_dll_bug_macos.dylib")
                        .expect("could not open dll");
                    let func = unsafe {
                        dll.symbol::<fn(&winit::window::Window)>("access_window_functions")
                    }
                    .unwrap();
                    func(window);

                    window.request_redraw();
                }
            }
            _ => (),
        }
    }
}

fn main() {
    let mut app = App { window: None };
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run_app(&mut app)
        .expect("error running event loop");
}
