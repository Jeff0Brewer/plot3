mod axis;
mod gl_wrap;
mod plot;
mod scene;
mod text;
mod ticks;
mod vertices;
use plot::Plot;

fn main() {
    let mut plot = Plot::new("test", 800.0, 800.0).unwrap();
    plot.set_background_color([0.05, 0.05, 0.05]);
    plot.display().unwrap();
}
