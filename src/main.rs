mod vertices;
mod plot;
mod axis_vert;
mod axis;
mod scene;
mod gl_wrap;
mod bitmap;
use plot::Plot;
use bitmap::Bitmap;

fn main() {
    let plot = Plot::new("test", 800.0, 800.0).unwrap();
    let bmp = Bitmap::new().unwrap();
    bmp.gen_font_map("./resources/Roboto-Regular.ttf").unwrap();
    plot.display().unwrap();
}
