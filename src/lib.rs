pub mod args;
pub mod color_dist;
pub mod color_space;
pub mod html_color;

const BLOCK: &str = "☐☐";

pub fn run(color: color_space::Rgb) {
    println!("Original color: {color:X} {}", pretty_block(color));

    let (name, color) = html_color::find_closest(color_dist::EuclideanImproved, color);
    println!("Closest html: {name} {color:X} {}", pretty_block(color));
}

fn pretty_block(color_space::Rgb {r, g, b}: color_space::Rgb) -> colored::ColoredString {
    use colored::Colorize;
    BLOCK.truecolor(r, g, b).on_truecolor(r, g, b)
}
