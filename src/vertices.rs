#[repr(C)]
pub struct PosVert {
    pub position: [f32; 3]
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct BitmapVert {
    pub position: [f32; 2],
    pub texcoord: [f32; 2]
}

// convert Nx3 array into PosVert vec
macro_rules! pos_vert {
    ($($pos:expr),*) => {
        vec![$(PosVert{
            position: $pos
        },)*]
    }
}

// convert Nx4 array into BitmapVert arr
macro_rules! bmp_arr {
    ($([$a:expr, $b:expr, $c:expr, $d:expr]),*) => {
        [$(BitmapVert{
            position: [$a, $b],
            texcoord: [$c, $d]
        },)*]
    }
}

// convert Nx4 array into BitmapVert vec
macro_rules! bmp_vert {
    ($([$a:expr, $b:expr, $c:expr, $d:expr]),*) => {
        vec![$(BitmapVert{
            position: [$a, $b],
            texcoord: [$c, $d]
        },)*]
    }
}

pub(crate) use pos_vert;
pub(crate) use bmp_arr;
pub(crate) use bmp_vert;
