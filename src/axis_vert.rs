use crate::plot::Bounds;
use crate::axis::TickStyle;
use crate::vertices::{PosVert, pos_vert};

pub fn get_axis_verts(bounds: &Bounds, style: &TickStyle, tick_count: i32)
-> (Vec<PosVert>, Vec<PosVert>) {
    let mut border = get_axis_border(bounds);
    let mut ticks = get_blank(bounds);
    let spacing = bounds.max() / (tick_count as f32);
    match style {
        // when style == tick append ticks to border to match with border color
        TickStyle::Tick => border.append(&mut get_ticks(bounds, spacing)),
        TickStyle::Grid => ticks.append(&mut get_grid(bounds, spacing)),
        _ => ()
    }
    (border, ticks)
}

pub fn get_axis_border(b: &Bounds) -> Vec<PosVert> {
    pos_vert![
        [b.x, b.y, 0.0],
        [b.x, 0.0, 0.0],
        [b.x, 0.0, 0.0],
        [b.x, 0.0, b.z],
        [b.x, 0.0, b.z],
        [0.0, 0.0, b.z],
        [0.0, 0.0, b.z],
        [0.0, b.y, b.z],
        [0.0, b.y, b.z],
        [0.0, b.y, 0.0],
        [0.0, b.y, 0.0],
        [b.x, b.y, 0.0]
    ]
}

pub fn get_blank(b: &Bounds) -> Vec<PosVert> {
    pos_vert![
        [0.0, 0.0, 0.0],
        [b.x, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        [0.0, b.y, 0.0],
        [0.0, 0.0, 0.0],
        [0.0, 0.0, b.z]
    ]
}

const TICK_SIZE: f32 = 0.02;
pub fn get_ticks(b: &Bounds, spacing: f32) -> Vec<PosVert> {
    let mut lines = Vec::<PosVert>::new();
    const S: f32 = TICK_SIZE;
    for i in 0..((b.x / spacing) as i32) {
        let x = spacing * (i as f32);
        lines.append(&mut pos_vert![
            [x, 0.0, b.z],
            [x, 0.0, b.z + S]
        ]);
    }
    for i in 0..((b.y / spacing) as i32) {
        let y = spacing * (i as f32);
        lines.append(&mut pos_vert![
            [b.x, y, 0.0],
            [b.x + S, y, 0.0]
        ]);
    }
    for i in 0..((b.z / spacing) as i32) {
        let z = spacing * (i as f32);
        lines.append(&mut pos_vert![
            [b.x, 0.0, z],
            [b.x + S, 0.0, z]
        ]);
    }
    lines
}

pub fn get_grid(b: &Bounds, spacing: f32) -> Vec<PosVert> {
    let mut lines = Vec::<PosVert>::new();
    for i in 0..((b.x / spacing) as i32) {
        let x = spacing * (i as f32);
        lines.append(&mut pos_vert![
            [x, 0.0, 0.0],
            [x, b.y, 0.0],
            [x, 0.0, 0.0],
            [x, 0.0, b.z]
        ]);
    }
    for i in 0..((b.y / spacing) as i32) {
        let y = spacing * (i as f32);
        lines.append(&mut pos_vert![
            [0.0, y, 0.0],
            [b.x, y, 0.0],
            [0.0, y, 0.0],
            [0.0, y, b.z]
        ]);
    }
    for i in 0..((b.z / spacing) as i32) {
        let z = spacing * (i as f32);
        lines.append(&mut pos_vert![
            [0.0, 0.0, z],
            [b.x, 0.0, z],
            [0.0, 0.0, z],
            [0.0, b.y, z]
        ]);
    }
    lines
}
