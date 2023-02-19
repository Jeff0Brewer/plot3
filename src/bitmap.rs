use crate::vertices::{BitmapVert, bmp_vert};

struct Bitmap {
    vertices: [BitmapVert; 4]
}

impl Bitmap {
    pub fn new() -> Self {
        let vertices = bmp_vert![
            [1.0, 1.0, 1.0, 1.0],
            [1.0, -1.0, 1.0, 0.0],
            [-1.0, -1.0, 0.0, 0.0],
            [-1.0, 1.0, 0.0, 1.0]
        ];
        Self { vertices }
    }
}
