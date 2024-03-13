use ordered_float::NotNan;

use crate::color_space::Rgb;

pub trait ColorDistance {
    type Item: std::cmp::Ord;

    /**
        Find distance between two colors.
    */
    fn dist(c1: Rgb, c2: Rgb) -> Self::Item;

    /**
        Find closest value to the `target`.
    
        # Returns
        Index of the closest value from candidates or [`Option::None`] if slice is empty.
    */
    fn find_closest(target: Rgb, candidates: &[Rgb]) -> Option<usize> {
         candidates.iter().enumerate()
            .min_by_key(|(_idx, other)| Self::dist(target, **other))
            .map(|(idx, _)| idx)
    }
}

// Formula: https://www.compuphase.com/cmetric.htm
pub struct EuclideanImproved;

impl ColorDistance for EuclideanImproved {
    type Item = NotNan<f32>;

    fn dist(c1: Rgb, c2: Rgb) -> Self::Item {
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
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::color_space::rgb;

    #[test]
    fn basic_human_eye_test() {
        assert_eq!(EuclideanImproved::find_closest(
            rgb(255, 192, 203),
            &[rgb(255, 0, 0), rgb(255, 105, 180), rgb(250, 128, 114),
            rgb(199, 21, 133), rgb(255, 165, 0),]),
        Some(1));
    }
}
