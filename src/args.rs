use crate::color_space::Rgb;

pub struct Args {
    pub color: Option<Rgb>,
    pub help: bool,
    pub version: bool,
}

#[allow(clippy::module_name_repetitions, clippy::missing_errors_doc)]
pub fn parse_cli_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut color: Option<Rgb> = None;
    let mut help = false;
    let mut version = false;

    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Long("help") => help = true,
            Long("version") => version = true,
            Value(color_str) => color = Some(color_str.parse()?),
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args {
        color,
        help,
        version
    })
}
