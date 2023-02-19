// sized position vertex with C repr for gl buffering
type Pos = [f32; 3];
#[repr(C, packed)]
pub struct PosVert(pub Pos);

// macro to remove init boilerplate
macro_rules! pos_vert {
    ($($pos:expr),*) => {
        vec![$(PosVert($pos),)*]
    }
}

pub(crate) use pos_vert;
