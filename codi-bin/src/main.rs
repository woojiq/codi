mod args;

const PKG_NAME: &str = env!("CARGO_BIN_NAME");
const VERSION: &str = concat!(env!("CARGO_BIN_NAME"), " v", env!("CARGO_PKG_VERSION"));

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
        println!("{}", print_all_html_colors());
        return;
    }

    if let Some(color) = args.color {
        println!("{}", find_closest_all_algs(color));
    } else {
        eprintln!("{}", help_message());
        std::process::exit(1);
    }
}

pub fn find_closest_all_algs(orig_color: codi_core::color_space::Rgb) -> tabled::Table {
    use codi_core::html_color::{find_closest, HtmlColor};
    use tabled::builder::Builder;

    let mut table = Builder::default();
    table.push_record(["Algorithm", "HTML color", "Hex", ""]);
    table.push_record([
        "> Original color".into(),
        "unknown".into(),
        format!("{orig_color:X}"),
        rgb_block(orig_color),
    ]);

    for algo in codi_core::color_dist::ALGORITHMS {
        let HtmlColor { name, color } = find_closest(algo, orig_color);
        table.push_record([
            algo.to_string(),
            name.into(),
            format!("{color:X}"),
            rgb_block(color),
        ]);
    }
    table.build()
}

pub fn print_all_html_colors() -> tabled::Table {
    use tabled::builder::Builder;

    let mut table = Builder::default();
    for color in codi_core::html_color::COLORS {
        table.push_record([color.name, &rgb_block(color.color)]);
    }
    table.build()
}

fn rgb_block(codi_core::color_space::Rgb { r, g, b }: codi_core::color_space::Rgb) -> String {
    use std::io::IsTerminal;
    if std::io::stdout().is_terminal() {
        format!("\x1b[48;2;{r};{g};{b}m  \x1b[0m")
    } else {
        String::from("  ")
    }
}

fn help_message() -> String {
    format!(
        "\
{VERSION}
Find closest named html color.

Usage:
    codi <color>

Args:
    <color> Hex color, e.g. \"#000000\" or \"ffffff\"

Options:
    --help                  Prints help information
    --version               Prints version
    --all-html              Displays all named html color
    "
    )
}
