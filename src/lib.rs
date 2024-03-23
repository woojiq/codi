pub mod args;
pub mod color_dist;
pub mod color_space;
pub mod html_color;

pub fn run(orig_color: color_space::RGB) {
    use html_color::HtmlColor;

    println!("Original color: {orig_color:X} {}", pretty_block(orig_color));

    for algo in color_dist::ALGORITHMS {
        let HtmlColor {name, color} = html_color::find_closest(algo, orig_color);
        println!("{algo}: {name} {color:X} {}", pretty_block(color));
    }
}

pub fn print_all_html_colors() {
    let max_len = html_color::MAX_NAME_LEN;
    for color in html_color::COLORS {
        println!("{:<0max_len$} {}", color.name, pretty_block(color.color));
    }
}

fn pretty_block(color_space::RGB {r, g, b}: color_space::RGB) -> colored::ColoredString {
    use colored::Colorize;
    " ".repeat(2).on_truecolor(r, g, b)
}
