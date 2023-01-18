extern crate gl;
mod window;
use window::Window;
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

unsafe fn run() {
    let window = Window::new("gl").unwrap();

    let vertex_shader = Shader::new("./shaders/vert.glsl", gl::VERTEX_SHADER).unwrap();
    let fragment_shader = Shader::new("./shaders/frag.glsl", gl::FRAGMENT_SHADER).unwrap();
    let program = ShaderProgram::new(&vertex_shader, &fragment_shader).unwrap();

    let vertex_buffer = Buffer::new();
    vertex_buffer.set_data(&VERTICES, gl::STATIC_DRAW);

    let vertex_array = VertexArray::new();
    let pos_index = program.get_attrib_location("position").unwrap();
    vertex_array.set_attribute::<Vertex>(pos_index, 2, 0);
    let col_index = program.get_attrib_location("color").unwrap();
    vertex_array.set_attribute::<Vertex>(col_index, 3, 2);

    window.run(move || {
        gl::ClearColor(0.1, 0.1, 0.1, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        program.apply();
        vertex_array.bind();
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
    });
}

fn main() {
    unsafe { run(); };
}
