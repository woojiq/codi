const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = concat!(env!("CARGO_PKG_NAME"), " v", env!("CARGO_PKG_VERSION"));

fn main() {
    let args = codi::args::parse_cli_args().unwrap_or_else(|err| {
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

    if let Some(color) = args.color {
        codi::run(color);
    } else {
        eprintln!("{}", help_message());
        std::process::exit(1);
    }
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
    ")
}
