use crate::vertices::{TexVert, tex_vert};

struct Bitmap {
    vertices: [TexVert; 4]
}

impl Bitmap {
    pub fn new() -> Self {
        let vertices = tex_vert![
            [1.0, 1.0, 0.0, 1.0, 1.0],
            [1.0, -1.0, 0.0, 1.0, 0.0],
            [-1.0, -1.0, 0.0, 0.0, 0.0],
            [-1.0, 1.0, 0.0, 0.0, 1.0]
        ];
        Self { vertices }
    }
}
