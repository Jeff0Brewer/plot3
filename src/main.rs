#![allow(dead_code)] // for text module testing

extern crate gl;
extern crate fontdue;
mod vertices;
mod plot;
mod axis_vert;
mod axis;
mod scene;
mod gl_wrap;
use gl_wrap::{Window};
use scene::{Scene};
use fontdue::{Font, FontSettings};

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    //let test_window = Window::new("text testing", 800.0, 800.0).unwrap();
    //let scene = Scene::new_empty();
    //unsafe { gl::ClearColor(0.0, 0.0, 0.1, 1.0); }
    //test_window.run(vec![scene]);
    let font_bytes = include_bytes!("../resources/Roboto-Regular.ttf") as &[u8];
    let font = Font::from_bytes(font_bytes, FontSettings::default()).unwrap();
    let (metrics, bitmap) = font.rasterize('O', 15.0);
    print_type_of(&bitmap);
    println!("{}", bitmap.len());
}

//use plot::Plot;
//let plot = Plot::new("test", 800.0, 800.0).unwrap();
//plot.display().unwrap();
