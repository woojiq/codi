#[derive(Default)]
pub struct Args {
    pub color: Option<codi::color_space::Rgb>,
    pub help: bool,
    pub version: bool,
    pub all_html: bool,
}

#[allow(clippy::module_name_repetitions, clippy::missing_errors_doc)]
pub fn parse_cli_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut args = Args::default();

    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Long("help") => args.help = true,
            Long("version") => args.version = true,
            Long("all-html") => args.all_html = true,
            Value(color_str) => args.color = Some(color_str.parse()?),
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(args)
}
