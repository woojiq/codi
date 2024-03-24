mod args;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = concat!(env!("CARGO_PKG_NAME"), " v", env!("CARGO_PKG_VERSION"));

fn main() {
    let args = args::parse_cli_args().unwrap_or_else(|err| {
        eprintln!("{PKG_NAME}: {err}");
        std::process::exit(1);
    });

    if args.help {
        println!("{}", help_message());
        return;
    }
    if args.version {
        println!("{VERSION}");
        return;
    }
    if args.all_html {
        print_all_html_colors();
        return;
    }

    if let Some(color) = args.color {
        run(color);
    } else {
        eprintln!("{}", help_message());
        std::process::exit(1);
    }
}

pub fn run(orig_color: codi::color_space::Rgb) {
    use codi::html_color::{HtmlColor, find_closest};

    println!("Original color: {orig_color:X} {}", pretty_block(orig_color));

    for algo in codi::color_dist::ALGORITHMS {
        let HtmlColor {name, color} = find_closest(algo, orig_color);
        println!("{algo}: {name} {color:X} {}", pretty_block(color));
    }
}

pub fn print_all_html_colors() {
    use codi::html_color::{MAX_NAME_LEN, COLORS};

    let max_len = MAX_NAME_LEN;
    for color in COLORS {
        println!("{:<0max_len$} {}", color.name, pretty_block(color.color));
    }
}

fn pretty_block(codi::color_space::Rgb {r, g, b}: codi::color_space::Rgb) -> colored::ColoredString {
    use colored::Colorize;
    " ".repeat(2).on_truecolor(r, g, b)
}

fn help_message() -> String {
    format!("\
{VERSION}
Find closest named html color.

Usage:
    codi <color>

Args:
    <color> Hex color, e.g. \"#000000\" or \"ffffff\"

Options:
    --help                  Prints help information
    --version               Prints version
    --all-html              Prints and displays all html color names
    ")
}
