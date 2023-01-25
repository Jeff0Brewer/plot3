extern crate gl;
mod gl_wrap;
use gl_wrap::{Window, Shader, Program, Buffer, VertexArray};

type Pos = [f32; 2];
type Col = [f32; 3];
#[repr(C, packed)]
struct Vertex(Pos, Col);
const VERTICES: [Vertex; 3] = [
    Vertex([-0.5, -0.5], [1.0, 0.0, 0.0]),
    Vertex([0.5,  -0.5], [0.0, 1.0, 0.0]),
    Vertex([0.0,   0.5], [0.0, 0.0, 1.0])
];

fn main() {
    let window = Window::new("gl").unwrap();

    let vertex_shader = Shader::new("./shaders/vert.glsl", gl::VERTEX_SHADER).unwrap();
    let fragment_shader = Shader::new("./shaders/frag.glsl", gl::FRAGMENT_SHADER).unwrap();
    let program = Program::new(&vertex_shader, &fragment_shader).unwrap();

    let vertex_buffer = Buffer::new();
    vertex_buffer.set_data(&VERTICES, gl::STATIC_DRAW);

    let vertex_array = VertexArray::new();
    let pos_index = program.get_attrib_location("position").unwrap();
    vertex_array.set_attribute::<Vertex>(pos_index, 2, 0);
    let col_index = program.get_attrib_location("color").unwrap();
    vertex_array.set_attribute::<Vertex>(col_index, 3, 2);

    program.apply();
    vertex_array.bind();
    let draw = || {
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    };
    let cleanup = move || {
        vertex_shader.drop();
        fragment_shader.drop();
        program.drop();
        vertex_buffer.drop();
        vertex_array.drop();
    };

    window.run(draw, cleanup);
}
