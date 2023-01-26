extern crate gl;
mod plot;
mod axis;
mod scene;
mod gl_wrap;
use plot::Plot;

fn main() {
    let plot = Plot::new("test", 500.0, 500.0).unwrap();
    plot.display();
}
