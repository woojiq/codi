use libm::{hypotf, powf, sqrtf};
use ordered_float::NotNan;

use crate::color_space::{Cielab, Rgb};

pub trait ColorDistance: core::fmt::Display {
    /**
        Find distance between two colors.
    */
    fn dist(&self, c1: Rgb, c2: Rgb) -> NotNan<f32>;

    /**
        Find closest value to the `target`.

        # Returns
        Index of the closest value from candidates or [`Option::None`] if slice is empty.
    */
    fn find_closest(&self, target: Rgb, candidates: &[Rgb]) -> Option<usize> {
        candidates
            .iter()
            .enumerate()
            .min_by_key(|(_idx, other)| self.dist(target, **other))
            .map(|(idx, _)| idx)
    }
}

pub const ALGORITHMS: [&'static dyn ColorDistance; 3] = [&Euclidean, &EuclideanImproved, &CIE94];

#[derive(Debug, Clone, Copy)]
pub struct Euclidean;

impl ColorDistance for Euclidean {
    fn dist(&self, c1: Rgb, c2: Rgb) -> NotNan<f32> {
        let dist = powf(f32::from(c1.r) - f32::from(c2.r), 2.0)
            + powf(f32::from(c1.g) - f32::from(c2.g), 2.0)
            + powf(f32::from(c1.b) - f32::from(c2.b), 2.0);
        NotNan::new(dist).unwrap()
    }
}

impl core::fmt::Display for Euclidean {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Euclidean")
    }
}

// Formula: https://www.compuphase.com/cmetric.htm
#[derive(Debug, Clone, Copy)]
pub struct EuclideanImproved;

impl ColorDistance for EuclideanImproved {
    fn dist(&self, c1: Rgb, c2: Rgb) -> NotNan<f32> {
        let red_mean = (f32::from(c1.r) + f32::from(c2.r)) / 2.0;
        let (d_r, d_g, d_b) = (
            (f32::from(c1.r) - f32::from(c2.r)),
            (f32::from(c1.g) - f32::from(c2.g)),
            (f32::from(c1.b) - f32::from(c2.b)),
        );
        let dist: f32 = powf((2.0 + red_mean / 256.0) * d_r, 2.0)
            + 4.0 * powf(d_g, 2.0)
            + powf((2.0 + (255.0 - red_mean) / 256.0) * d_b, 2.0);
        NotNan::new(dist).unwrap()
    }
}

impl core::fmt::Display for EuclideanImproved {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Euclidean Improved")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CIE94;

impl ColorDistance for CIE94 {
    #[allow(non_upper_case_globals, non_snake_case)]
    fn dist(&self, c1: Rgb, c2: Rgb) -> NotNan<f32> {
        let (lab1, lab2) = (Cielab::from(c1), Cielab::from(c2));

        let (kL, K1, K2) = (1.0, 0.045, 0.015);
        let (kC, kH) = (1.0, 1.0);

        let (delta_l, delta_a, delta_b) = (lab1.l - lab2.l, lab1.a - lab2.a, lab1.b - lab2.b);

        let C1 = hypotf(*lab1.a, *lab1.b);
        let C2 = hypotf(*lab2.a, *lab2.b);

        let Cab = C1 - C2;
        let Sl = 1.0;
        let Sc = 1.0 + K1 * C1;
        let Sh = 1.0 + K2 * C1;
        // https://github.com/zschuessler/DeltaE/issues/9
        let Hab = sqrtf((powf(*delta_a, 2.0) + powf(*delta_b, 2.0) - powf(Cab, 2.0)).max(0.0));

        let dist = powf(*delta_l / (kL * Sl), 2.0)
            + powf(Cab / (kC * Sc), 2.0)
            + powf(Hab / (kH * Sh), 2.0);
        NotNan::new(dist).unwrap()
    }
}

impl core::fmt::Display for CIE94 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CIE94")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::color_space::Rgb;
    use crate::html_color::find_closest;

    #[test]
    fn all_algs_same_result() -> Result<(), Box<dyn std::error::Error>> {
        let tests = [(Rgb::try_from("#81818d")?, Rgb::try_from("#808080")?)];

        for (input, expected) in tests {
            for alg in ALGORITHMS {
                assert_eq!(
                    find_closest(alg, input).color,
                    expected,
                    "closest html color for {input} shoud be {expected}"
                );
            }
        }
        Ok(())
    }
}
