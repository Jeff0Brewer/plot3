mod plot;
mod axis;
mod scene;
mod gl_wrap;
use plot::Plot;

fn main() {
    let plot = Plot::new("test", 800.0, 800.0).unwrap();
    plot.display().unwrap();
}
