pub mod args;
pub mod color_dist;
pub mod color_space;
pub mod html_color;

pub fn run(color: color_space::Rgb) {
    println!("Original color: {color:X} {}", pretty_block(color));

    let algo = color_dist::Euclidean;
    let (name, color) = html_color::find_closest(algo, color);
    println!("{algo}: {name} {color:X} {}", pretty_block(color));

    let algo = color_dist::EuclideanImproved;
    let (name, color) = html_color::find_closest(algo, color);
    println!("{algo}: {name} {color:X} {}", pretty_block(color));

    let algo = color_dist::CIE94;
    let (name, color) = html_color::find_closest(algo, color);
    println!("{algo}: {name} {color:X} {}", pretty_block(color));
}

fn pretty_block(color_space::Rgb {r, g, b}: color_space::Rgb) -> colored::ColoredString {
    use colored::Colorize;
    " ".repeat(2).on_truecolor(r, g, b)
}
