use ordered_float::NotNan;

use crate::color_space::Rgb;

// Formula: https://www.compuphase.com/cmetric.htm
//
pub fn euclidean_improved(c1: Rgb, c2: Rgb) -> NotNan<f32> {
    let red_mean = (c1.r as f32 + c2.r as f32) / 2.0;
    let (d_r, d_g, d_b) = (
        (c1.r as f32 - c2.r as f32),
        (c1.g as f32 - c2.g as f32),
        (c1.b as f32 - c2.b as f32)
    );
    let dist: f32 =
        (2.0 + red_mean / 256.0) * d_r.powi(2) +
        4.0 * d_g.powi(2) +
        (2.0 + (255.0 - red_mean) / 256.0) * d_b.powi(2);
    NotNan::new(dist).unwrap()
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
