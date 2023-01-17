extern crate glutin;
extern crate gl;
use glutin::{ContextBuilder, GlRequest, Api};
use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::event::{Event, WindowEvent};
use glutin::window::WindowBuilder;

mod shader;
use shader::{Shader, ShaderProgram};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("gl");
    let ctx = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_windowed(window, &event_loop)
        .expect("Couldn't create context");
    let ctx = unsafe {
        ctx
            .make_current()
            .expect("Failed to make context current")
    };
    gl::load_with(|ptr| ctx.get_proc_address(ptr) as *const _);

    unsafe {
        let vertex_shader = Shader::new("./shaders/vert.glsl", gl::VERTEX_SHADER).unwrap();
        let fragment_shader = Shader::new("./shaders/frag.glsl", gl::FRAGMENT_SHADER).unwrap();
        let program = ShaderProgram::new(&vertex_shader, &fragment_shader).unwrap();
        program.apply();
    }


    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent {event, ..} => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => ()
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    gl::ClearColor(0.0, 0.0, 1.0, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                ctx.swap_buffers().unwrap();
            }
            _ => ()
        }
    });
}
