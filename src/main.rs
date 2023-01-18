extern crate glutin;
extern crate gl;
use glutin::{ContextBuilder, GlRequest, Api};
use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::event::{Event, WindowEvent};
use glutin::window::WindowBuilder;

mod shader;
use shader::{Shader, ShaderProgram};

mod buffer;
use buffer::{Buffer, VertexArray};

type Pos = [f32; 2];
type Col = [f32; 3];
#[repr(C, packed)]
struct Vertex(Pos, Col);
const VERTICES: [Vertex; 3] = [
    Vertex([-0.5, -0.5], [1.0, 0.0, 0.0]),
    Vertex([0.5,  -0.5], [0.0, 1.0, 0.0]),
    Vertex([0.0,   0.5], [0.0, 0.0, 1.0])
];

unsafe fn draw() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("gl");
    let ctx = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_windowed(window, &event_loop)
        .expect("Couldn't create context");
    let ctx = ctx.make_current().expect("Failed to make context current");
    gl::load_with(|ptr| ctx.get_proc_address(ptr) as *const _);

    let vertex_shader = Shader::new("./shaders/vert.glsl", gl::VERTEX_SHADER).unwrap();
    let fragment_shader = Shader::new("./shaders/frag.glsl", gl::FRAGMENT_SHADER).unwrap();
    let program = ShaderProgram::new(&vertex_shader, &fragment_shader).unwrap();
    let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
    vertex_buffer.set_data(&VERTICES, gl::STATIC_DRAW);
    let vertex_array = VertexArray::new();
    let pos_index = program.get_attrib_location("position").unwrap();
    vertex_array.set_attribute::<Vertex>(pos_index, 2, 0);
    let col_index = program.get_attrib_location("color").unwrap();
    vertex_array.set_attribute::<Vertex>(col_index, 3, 2);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent {event, ..} => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => ()
            },
            Event::RedrawRequested(_) => {
                gl::ClearColor(0.1, 0.1, 0.1, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
                program.apply();
                vertex_array.bind();
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
                ctx.swap_buffers().unwrap();
            }
            _ => ()
        }
    });
}

fn main() {
    unsafe { draw(); };
}
