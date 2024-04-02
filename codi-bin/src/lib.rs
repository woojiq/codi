pub mod args;

/**
    Find closest named html color to target color using all algorithms.

    # Errors

    Returns [`Err`] if writing to `writer` fails.
*/
pub fn find_closest_all_algs<T: std::io::Write>(
    writer: &mut T,
    orig_color: codi_core::color_space::Rgb,
) -> std::io::Result<()> {
    use codi_core::html_color::{find_closest, find_exact, HtmlColor};
    use tabled::builder::Builder;

    let mut table = Builder::default();
    table.push_record(["Algorithm", "HTML color", "Hex", ""]);
    table.push_record([
        "> Original color".into(),
        find_exact(orig_color).unwrap_or("unknown").into(),
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

    let table = table.build().to_string() + "\n";
    writer.write_all(table.as_bytes())
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
