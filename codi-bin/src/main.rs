const PKG_NAME: &str = env!("CARGO_BIN_NAME");
const VERSION: &str = concat!(env!("CARGO_BIN_NAME"), " v", env!("CARGO_PKG_VERSION"));

fn main() {
    let mut stdout = std::io::stdout();

    let args = codi_bin::args::parse_cli_args().unwrap_or_else(|err| {
        eprintln!("{PKG_NAME}: {err}\n{}", help_message());
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
        println!("{}", codi_bin::print_all_html_colors());
        return;
    }

    if let Some(color) = args.color {
        codi_bin::find_closest_all_algs(&mut stdout, color).unwrap();
    } else {
        eprintln!("{}", help_message());
        std::process::exit(1);
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
