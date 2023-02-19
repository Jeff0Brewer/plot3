type Position = [f32; 3];
type TexCoord = [f32; 2];

#[repr(C)]
pub struct PosVert(pub Position);
// convert Nx3 array into PosVert vec
macro_rules! pos_vert {
    ($($pos:expr),*) => {
        vec![$(PosVert($pos),)*]
    }
}
pub(crate) use pos_vert;

#[repr(C, packed)]
pub struct TexVert(pub Position, pub TexCoord);
// convert Nx5 array into TexVert arr
macro_rules! tex_vert {
    ($([$a:expr, $b:expr, $c:expr, $d:expr, $e:expr]),*) => {
        [$(
            TexVert([$a, $b, $c], [$d, $e]),
        )*]
    }
}
pub(crate) use tex_vert;
