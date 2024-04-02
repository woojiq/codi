use crate::color_dist::ColorDistance;
use crate::color_space::{rgb, Rgb};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HtmlColor {
    pub name: &'static str,
    pub color: Rgb,
}

// Generates consts like `YELLOWGREEN: Rgb`
// and `COLORS` array that contains all of them.
include!(concat!(env!("OUT_DIR"), "/contrib/html-color-names.txt.rs"));

pub const ONLY_NAMES: [&str; COLORS.len()] = __split_colors_arr().0;
pub const ONLY_COLORS: [Rgb; COLORS.len()] = __split_colors_arr().1;

// Modify it when adding new entries to the file.
const _: () = assert!(COLORS.len() == 138);
const _: () = assert!(ONLY_COLORS.len() == COLORS.len());
const _: () = assert!(ONLY_NAMES.len() == COLORS.len());

/**
    Splits array of tuples into tuple of arrays.

    Unfortunately, I couldn't write generic solution using function declaration:
    `<I, J, const N: usize>(arr: [(I, J); N]) -> ([I; N], [J; N)])`

    I tried two different methods:
    * Make I and J implement [`Default`] and [`Copy`], but `[T::default()]` is
      not a constant function, so we cannot write `[T::default(); N]`.
    * Use `[MaybeUninit<I>; N]`, but in this case we cannot convert
      [`std::mem::MaybeUninit`]s array to array we need. Because generic array
      transmutes don't currently work in rustc:
      <https://github.com/rust-lang/rust/issues/61956>
*/
const fn __split_colors_arr<const N: usize>() -> ([&'static str; N], [Rgb; N]) {
    let mut names = [""; N];
    let mut colors = [Rgb::new(0, 0, 0); N];
    let mut idx = 0;
    while idx < N {
        names[idx] = COLORS[idx].name;
        colors[idx] = COLORS[idx].color;
        idx += 1;
    }
    (names, colors)
}

/**
    Find closest to target color from list of named html colors.
*/
#[allow(clippy::missing_panics_doc)]
pub fn find_closest<T: ColorDistance + ?Sized>(alg: &T, target: Rgb) -> HtmlColor {
    let idx = alg
        .find_closest(target, &ONLY_COLORS)
        .expect("SAFETY: we have assert for const array ONLY_COLORS length");
    COLORS[idx]
}

pub fn find_exact(target: Rgb) -> Option<&'static str> {
    for HtmlColor { name, color } in COLORS {
        if target == color {
            return Some(name);
        }
    }
    None
}
