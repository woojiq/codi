/**
    Generic matrix multiplication.
*/
pub fn matrix_mul<T, const I: usize, const J: usize, const K: usize>(
    m1: &[[T; K]; I],
    m2: &[[T; J]; K],
) -> [[T; J]; I]
where
    T: core::ops::Mul<Output = T> + core::ops::Add<Output = T> + Default + Copy,
{
    let mut res: [[T; J]; I] = [[T::default(); J]; I];

    #[allow(clippy::needless_range_loop)]
    for i in 0..I {
        for j in 0..J {
            for k in 0..K {
                res[i][j] = res[i][j] + m1[i][k] * m2[k][j];
            }
        }
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn matrix_mul_zero_size_dimension() {
        let empty = [[0u32; 0]; 0];
        assert_eq!(matrix_mul(&empty, &empty), empty);

        // 1x0 X 0x2 = 1x2
        assert_eq!(matrix_mul(&[[1u32; 0]; 1], &[[1; 2]; 0]), [[0; 2]; 1]);
    }

    #[test]
    fn matrix_mul_same_squares() {
        let matrix = [[1u32, 2, 3], [4, 5, 6], [7, 8, 9]];
        let answer = [[30, 36, 42], [66, 81, 96], [102, 126, 150]];
        assert_eq!(matrix_mul(&matrix, &matrix), answer);
    }
}
