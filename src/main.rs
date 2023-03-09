mod axis;
mod axis_vert;
mod font_map;
mod gl_wrap;
mod label_draw;
mod plot;
mod scene;
mod vertices;
use axis::TickStyle;
use plot::Plot;

fn main() {
    let mut plot = Plot::new("test", 800.0, 800.0).unwrap();
    plot.set_background_color([0.05, 0.05, 0.05]);
    plot.axis.set_tick_style(TickStyle::Tick);
    plot.axis.set_border_color([1.0, 1.0, 1.0]);
    plot.axis.set_tick_color([0.5, 0.5, 0.5]);
    plot.set_bounds(1.0, 0.5, 0.7);
    plot.labels
        .set_font_face("./resources/Ubuntu-Regular.ttf")
        .unwrap();
    plot.labels.set_labels("X axis", "Y axis", "Z axis");
    plot.display().unwrap();
}
