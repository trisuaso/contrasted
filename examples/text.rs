use contrasted::{Color, MINIMUM_CONTRAST_THRESHOLD};

fn main() {
    let background = Color::from_hex("#34342d");
    let foreground = Color::from_hex("#cfb820");

    let contrast = background.contrast(&foreground);
    let passes = contrast > MINIMUM_CONTRAST_THRESHOLD;
    println!("contrast: {contrast} (passes: {passes})");
}
