use crate::plot::Bounds;
use crate::axis::{BorderStyle, TickStyle};
use crate::vertices::{PosVert, pos_vert};

pub fn get_axis_border(style: &BorderStyle, bounds: &Bounds) -> (Vec<PosVert>, Vec<PosVert>) {
    match style {
        BorderStyle::Arrow => get_arrow_border(bounds),
        BorderStyle::Box => get_box_border(bounds)
    }
}

pub fn get_arrow_border(b: &Bounds) -> (Vec<PosVert>, Vec<PosVert>) {
    const S: f32 = 0.02;
    let lines = pos_vert![
        [0.0, 0.0, 0.0],
        [b.x, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        [0.0, b.y, 0.0],
        [0.0, 0.0, 0.0],
        [0.0, 0.0, b.z]
    ];
    let tris = pos_vert![
        [b.x, 0.0, 0.0],
        [b.x-S, S, 0.0],
        [b.x-S, -S, 0.0],
        [0.0, b.y, 0.0],
        [S, b.y-S, 0.0],
        [-S, b.y-S, 0.0],
        [0.0, 0.0, b.z],
        [S, 0.0, b.z-S],
        [-S, 0.0, b.z-S]
    ];
    (lines, tris)
}

pub fn get_box_border(b: &Bounds) -> (Vec<PosVert>, Vec<PosVert>) {
    let lines = pos_vert![
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
    ];
    let tris = pos_vert![];
    (lines, tris)
}

pub fn get_axis_ticks(style: &TickStyle, border: &BorderStyle, bounds: &Bounds, count: &i32) -> Vec<PosVert> {
    let spacing = bounds.max() / (*count as f32);
    match style {
        TickStyle::Blank => pos_vert![],
        TickStyle::Grid => get_grid(bounds, spacing),
        TickStyle::Tick => {
            // separate funcs for diff border styles since ticks placed on border
            match border {
                BorderStyle::Arrow => get_arrow_ticks(bounds, spacing),
                BorderStyle::Box => get_box_ticks(bounds, spacing)
            }
        }
    }
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

const TICK_SIZE: f32 = 0.02;

pub fn get_arrow_ticks(b: &Bounds, spacing: f32) -> Vec<PosVert> {
    let mut lines = Vec::<PosVert>::new();
    const S: f32 = -TICK_SIZE;
    for i in 0..((b.x / spacing) as i32) {
        let x = spacing * (i as f32);
        lines.append(&mut pos_vert![
            [x, 0.0, 0.0],
            [x, S, 0.0],
            [x, 0.0, 0.0],
            [x, 0.0, S]
        ]);
    }
    for i in 0..((b.y / spacing) as i32) {
        let y = spacing * (i as f32);
        lines.append(&mut pos_vert![
            [0.0, y, 0.0],
            [S, y, 0.0],
            [0.0, y, 0.0],
            [0.0, y, S]
        ]);
    }
    for i in 0..((b.z / spacing) as i32) {
        let z = spacing * (i as f32);
        lines.append(&mut pos_vert![
            [0.0, 0.0, z],
            [S, 0.0, z],
            [0.0, 0.0, z],
            [0.0, S, z]
        ]);
    }

    lines
}

pub fn get_box_ticks(b: &Bounds, spacing: f32) -> Vec<PosVert> {
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
