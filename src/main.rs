#![allow(dead_code)] // for text module testing

mod vertices;
mod plot;
mod axis_vert;
mod axis;
mod scene;
mod gl_wrap;
use gl_wrap::{Window};
use scene::{Scene};
extern crate gl;

fn main() {
    let test_window = Window::new("text testing", 800.0, 800.0).unwrap();
    let scene = Scene::new_empty();
    unsafe { gl::ClearColor(0.0, 0.0, 0.1, 1.0); }

    test_window.run(vec![scene]);
}

//use plot::Plot;
//let plot = Plot::new("test", 800.0, 800.0).unwrap();
//plot.display().unwrap();
