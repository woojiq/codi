use crate::color_space::{Rgb, rgb};

pub type HtmlColor = (&'static str, Rgb);

// Generates consts like `YELLOWGREEN: Rgb`
// and `HTML_NAMED_COLORS` array that contains all of them.
include!(concat!(env!("OUT_DIR"), "/contrib/html-color-names.txt.rs"));
