mod vertices;
mod plot;
mod axis_vert;
mod axis;
mod scene;
mod gl_wrap;
mod font_map;
mod label_draw;
use plot::Plot;

fn main() {
    let mut plot = Plot::new("test", 800.0, 800.0).unwrap();
    plot.set_background_color([0.05, 0.05, 0.05]);
    plot.labels.set_font_face("./resources/Ubuntu-Regular.ttf").unwrap();
    plot.labels.set_label("Testing 19AZ");
    plot.display().unwrap();
}
