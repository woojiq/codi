use ordered_float::NotNan;

use crate::color_space::{Rgb, Lab};

pub trait ColorDistance {
    /**
        Find distance between two colors.
    */
    fn dist(c1: Rgb, c2: Rgb) -> NotNan<f32>;

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
#[derive(Debug, Clone, Copy)]
pub struct EuclideanImproved;

impl ColorDistance for EuclideanImproved {
    fn dist(c1: Rgb, c2: Rgb) -> NotNan<f32> {
        let red_mean = (f32::from(c1.r) + f32::from(c2.r)) / 2.0;
        let (d_r, d_g, d_b) = (
            (f32::from(c1.r) - f32::from(c2.r)),
            (f32::from(c1.g) - f32::from(c2.g)),
            (f32::from(c1.b) - f32::from(c2.b))
        );
        let dist: f32 =
            (2.0 + red_mean / 256.0) * d_r.powi(2) +
            4.0 * d_g.powi(2) +
            (2.0 + (255.0 - red_mean) / 256.0) * d_b.powi(2);
        NotNan::new(dist).unwrap()
    }
}

impl std::fmt::Display for EuclideanImproved {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Euclidean Improved")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Euclidean;

impl ColorDistance for Euclidean {
    fn dist(c1: Rgb, c2: Rgb) -> NotNan<f32> {
        let dist = (f32::from(c1.r) - f32::from(c2.r)).powi(2) +
                   (f32::from(c1.g) - f32::from(c2.g)).powi(2) +
                   (f32::from(c1.b) - f32::from(c2.b)).powi(2);
        NotNan::new(dist).unwrap()
    }
}

impl std::fmt::Display for Euclidean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Euclidean")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CIE94;

impl ColorDistance for CIE94 {
    #[allow(non_upper_case_globals, non_snake_case)]
    fn dist(c1: Rgb, c2: Rgb) -> NotNan<f32> {
        let (lab1, lab2) = (Lab::from(c1), Lab::from(c2));

        let (kL, K1, K2) = (1.0, 0.045, 0.015);
        let (kC, kH) = (1.0, 1.0);

        let (delta_l, delta_a, delta_b) = (
            lab1.l - lab2.l,
            lab1.a - lab2.a,
            lab1.b - lab2.b,
        );

        let C1 = lab1.a.hypot(lab1.b.into_inner());
        let C2 = lab2.a.hypot(lab2.b.into_inner());

        let Cab = C1 - C2;
        let Sl = 1.0;
        let Sc = 1.0 + K1 * C1;
        let Sh = 1.0 + K2 * C1;
        // https://github.com/zschuessler/DeltaE/issues/9
        let Hab = (delta_a.powi(2) + delta_b.powi(2) - Cab.powi(2)).max(0.0).sqrt();

        let E94 = (delta_l / (kL * Sl)).powi(2) +
                  (Cab / (kC * Sc)).powi(2) +
                  (Hab / (kH * Sh)).powi(2);
        NotNan::new(E94).unwrap()
    }
}

impl std::fmt::Display for CIE94 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CIE94")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::color_space::{Rgb, rgb};

    #[test]
    fn test_basic_human_eye() {
        assert_eq!(EuclideanImproved::find_closest(
            rgb(255, 192, 203),
            &[rgb(255, 0, 0), rgb(255, 105, 180), rgb(250, 128, 114),
            rgb(199, 21, 133), rgb(255, 165, 0),]),
        Some(1));
    }

    #[test]
    fn test_cie_94_not_panic() {
        CIE94::dist(rgb(0, 0, 0), rgb(0, 0, 1));
    }
}
