extern crate glutin;
extern crate gl;
use glutin::{ContextWrapper, ContextBuilder, GlRequest, Api, CreationError, PossiblyCurrent};
use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::event::{Event, WindowEvent};
use glutin::window::WindowBuilder;

pub struct Window {
    ctx: ContextWrapper<PossiblyCurrent, glutin::window::Window>,
    event_loop: EventLoop<()>
}

impl Window {
    pub unsafe fn new(title: &str) -> Result<Self, CreationError> {
        let window = WindowBuilder::new().with_title(title);
        let event_loop = EventLoop::new();
        let ctx = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
            .build_windowed(window, &event_loop)?;
        let ctx = ctx.make_current().unwrap();
        gl::load_with(|ptr| ctx.get_proc_address(ptr) as *const _);
        Ok(Self { ctx, event_loop })
    }

    pub unsafe fn run<F: Fn() -> () + 'static>(self, draw: F) -> () {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::LoopDestroyed => (),
                Event::WindowEvent {event, ..} => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => ()
                },
                Event::RedrawRequested(_) => {
                    draw();
                    self.ctx.swap_buffers().unwrap();
                },
                _ => ()
            }
        })
    }
}
