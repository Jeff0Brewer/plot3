#[repr(C)]
pub struct PosVert(pub [f32; 3]);
// convert Nx3 array into PosVert vec
macro_rules! pos_vert {
    ($($pos:expr),*) => {
        vec![$(PosVert($pos),)*]
    }
}
pub(crate) use pos_vert;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct BitmapVert(pub [f32; 2], pub [f32; 2]);
// convert Nx4 array into BitmapVert arr
macro_rules! bmp_arr {
    ($([$a:expr, $b:expr, $c:expr, $d:expr]),*) => {
        [$(BitmapVert([$a, $b], [$c, $d]),)*]
    }
}
pub(crate) use bmp_arr;

macro_rules! bmp_vert {
    ($([$a:expr, $b:expr, $c:expr, $d:expr]),*) => {
        vec![$(BitmapVert([$a, $b], [$c, $d]),)*]
    }
}
pub(crate) use bmp_vert;
