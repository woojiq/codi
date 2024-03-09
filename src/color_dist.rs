use ordered_float::NotNan;

use crate::color_space::Rgb;

// Formula: https://www.compuphase.com/cmetric.htm
//
#[allow(clippy::missing_panics_doc)]
pub fn euclidean_improved(c1: Rgb, c2: Rgb) -> NotNan<f32> {
    let red_mean = (f32::from(c1.r) + f32::from(c2.r)) / 2.0;
    let (d_r, d_g, d_b) = (
        (f32::from(c1.r) - f32::from(c2.r)),
        (f32::from(c1.g) - f32::from(c2.g)),
        (f32::from(c1.b) - f32::from(c2.b))
    );
    let dist: f32 =
        (2.0 + (255.0 - red_mean) / 256.0).mul_add(
            d_b.powi(2),
            (2.0 + red_mean / 256.0).mul_add(d_r.powi(2), 4.0 * d_g.powi(2))
        );
    NotNan::new(dist)
        .expect("SAFETY: We divide by non-zero const, the input data are integers.")
}

pub fn find_closest(target: Rgb, candidates: &[Rgb]) -> Option<Rgb> {
    candidates.iter()
        .min_by_key(|&&other| euclidean_improved(target, other))
        .copied()
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::color_space::rgb;

    #[test]
    fn basic_human_eye_test() {
        let answer = rgb(255, 105, 180);
        assert_eq!(find_closest(rgb(255, 192, 203), &[
            rgb(255, 0, 0), answer, rgb(250, 128, 114),
            rgb(199, 21, 133), rgb(255, 165, 0),
        ]), Some(answer));
    }
}
