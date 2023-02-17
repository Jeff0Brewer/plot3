mod plot;
mod axis;
mod scene;
mod gl_wrap;
use plot::Plot;
use axis::BorderType;

fn main() {
    let mut plot = Plot::new("test", 500.0, 500.0).unwrap();
    plot.axis.set_border_type(BorderType::Arrow);
    plot.axis.set_bounds([1.0, 0.5, 1.0]);
    plot.axis.set_border_color([1.0, 0.7, 1.0, 1.0]);
    plot.display().unwrap();
}
