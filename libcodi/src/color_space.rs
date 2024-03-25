#![allow(clippy::upper_case_acronyms)]

use ordered_float::NotNan;

const HEX_COLOR_LEN: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    HexColorWrongLen(usize),
    NotAsciiHexDigit(u8),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Self::HexColorWrongLen(_len) => write!(
                f,
                "The length of hex string must be {} or {} if the first character is '#'",
                HEX_COLOR_LEN,
                HEX_COLOR_LEN + 1
            ),
            Self::NotAsciiHexDigit(dig) => write!(f, "{dig} is not ascii hexadecimal digit"),
        }
    }
}

impl std::error::Error for Error {}

type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            r: red,
            g: green,
            b: blue,
        }
    }

    pub fn for_each<F: Fn(Self)>(func: F) {
        for r in 0..=u8::MAX {
            for g in 0..=u8::MAX {
                for b in 0..=u8::MAX {
                    func(Self::new(r, g, b));
                }
            }
        }
    }
}

impl core::str::FromStr for Rgb {
    type Err = Error;

    fn from_str(value: &str) -> core::result::Result<Self, Self::Err> {
        value.as_bytes().try_into()
    }
}

impl TryFrom<&str> for Rgb {
    type Error = Error;

    /**
        Convert hex colors into Rgb.

        Hex strings format can be either with leading '#' symbol or not.

        # Example

        ```
        use codi::color_space::Rgb;
        assert_eq!("#00a0f0".try_into(), Ok(Rgb::new(0, 160, 240)));
        assert_eq!("fFfFfF".try_into(), Ok(Rgb::new(255, 255, 255)));
        ```
    */
    fn try_from(value: &str) -> core::result::Result<Self, Self::Error> {
        value.as_bytes().try_into()
    }
}

impl TryFrom<&[u8]> for Rgb {
    type Error = Error;

    fn try_from(value: &[u8]) -> core::result::Result<Self, Self::Error> {
        let start_from = usize::from(value.first() == Some(&b'#'));

        let Ok(arr) = <[u8; HEX_COLOR_LEN]>::try_from(&value[start_from..]) else {
            return Err(Error::HexColorWrongLen(value.len()));
        };

        arr.try_into()
    }
}

impl TryFrom<[u8; 6]> for Rgb {
    type Error = Error;

    fn try_from(value: [u8; 6]) -> core::result::Result<Self, Self::Error> {
        Ok(Self {
            r: u8_from_two_hex(value[0], value[1])?,
            g: u8_from_two_hex(value[2], value[3])?,
            b: u8_from_two_hex(value[4], value[5])?,
        })
    }
}

impl core::fmt::Display for Rgb {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

impl core::fmt::LowerHex for Rgb {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl core::fmt::UpperHex for Rgb {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

pub const fn rgb(red: u8, green: u8, blue: u8) -> Rgb {
    Rgb::new(red, green, blue)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Xyz {
    pub x: NotNan<f32>,
    pub y: NotNan<f32>,
    pub z: NotNan<f32>,
}

#[allow(clippy::fallible_impl_from)]
impl From<Rgb> for Xyz {
    /// <https://en.wikipedia.org/wiki/SRgb#From_sRgb_to_CIE_XYZ>
    fn from(value: Rgb) -> Self {
        // http://www.brucelindbloom.com/index.html?Eqn_Rgb_XYZ_Matrix.html
        const COEF: [[f32; 3]; 3] = [
            [0.4124, 0.3576, 0.1805],
            [0.2126, 0.7152, 0.0722],
            [0.0193, 0.1192, 0.9505],
        ];

        let (r, g, b) = (
            f32::from(value.r) / 255.0,
            f32::from(value.g) / 255.0,
            f32::from(value.b) / 255.0,
        );

        let to_linear = |col: f32| {
            if col <= 0.04045 {
                col / 12.92
            } else {
                ((col + 0.055) / 1.055).powf(2.4)
            }
        };

        let lin_col: [[f32; 1]; 3] = [[to_linear(r)], [to_linear(g)], [to_linear(b)]];
        assert_eq!(lin_col.len(), COEF[0].len());

        let mut res: [[f32; 1]; 3] = [[0.0], [0.0], [0.0]];

        // TODO: separate function for matrix multiplication
        #[allow(clippy::needless_range_loop)]
        for i in 0..COEF.len() {
            for j in 0..lin_col[0].len() {
                for k in 0..COEF[0].len() {
                    res[i][j] += COEF[i][k] * lin_col[k][j];
                }
            }
        }

        Self {
            x: NotNan::new(res[0][0] * 100.0).unwrap(),
            y: NotNan::new(res[1][0] * 100.0).unwrap(),
            z: NotNan::new(res[2][0] * 100.0).unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cielab {
    pub l: NotNan<f32>,
    pub a: NotNan<f32>,
    pub b: NotNan<f32>,
}

#[allow(clippy::fallible_impl_from)]
impl From<Xyz> for Cielab {
    #[allow(non_upper_case_globals, non_snake_case)]
    #[allow(clippy::many_single_char_names)]
    fn from(value: Xyz) -> Self {
        const Xn: f32 = 95.0489;
        const Yn: f32 = 100.0;
        const Zn: f32 = 108.884;

        const DELTA: f32 = 6.0 / 29.0;
        let DELTA_3 = DELTA.powi(3);
        let DELTA_M2 = DELTA.powi(-2);

        let f = |t: f32| {
            if t > DELTA_3 {
                t.cbrt()
            } else {
                1.0 / 3.0 * t * DELTA_M2 + 4.0 / 29.0
            }
        };

        let (x, y, z) = (
            value.x.into_inner(),
            value.y.into_inner(),
            value.z.into_inner(),
        );

        let (L, a, b) = (
            116.0 * f(y / Yn) - 16.0,
            500.0 * (f(x / Xn) - f(y / Yn)),
            200.0 * (f(y / Yn) - f(z / Zn)),
        );

        Self {
            l: NotNan::new(L).unwrap(),
            a: NotNan::new(a).unwrap(),
            b: NotNan::new(b).unwrap(),
        }
    }
}

impl From<Rgb> for Cielab {
    fn from(value: Rgb) -> Self {
        Xyz::from(value).into()
    }
}

const fn u8_from_two_hex(hex1: u8, hex2: u8) -> Result<u8> {
    match (hex_byte_to_dec(hex1), hex_byte_to_dec(hex2)) {
        (Ok(v1), Ok(v2)) => Ok(v1 * 16 + v2),
        (Err(err), _) | (_, Err(err)) => Err(err),
    }
}

/**
    Converts ascii hex digit to the decimal number.
*/
const fn hex_byte_to_dec(hex: u8) -> Result<u8> {
    match hex {
        b'0'..=b'9' => Ok(hex - b'0'),
        b'a'..=b'f' => Ok(hex - b'a' + 10),
        b'A'..=b'F' => Ok(hex - b'A' + 10),
        _ => Err(Error::NotAsciiHexDigit(hex)),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Check with and without "#" prefix, lowercase and uppercase. The behavior
    // should be the same.
    #[test]
    fn test_rgb_from_hex_some() {
        let tests = [
            ("#000000", rgb(0, 0, 0)),
            ("#FFfFff", rgb(255, 255, 255)),
            ("#010100", rgb(1, 1, 0)),
            ("#010a00", rgb(1, 10, 0)),
            ("#00fF00", rgb(0, 255, 0)),
            ("#abcdef", rgb(171, 205, 239)),
        ];
        for (case, expected) in tests {
            from_hex_assert_ok(case, expected);
            from_hex_assert_ok(&case.to_ascii_lowercase(), expected);
            from_hex_assert_ok(&case.to_ascii_uppercase(), expected);
            from_hex_assert_ok(&case[1..].to_ascii_lowercase(), expected);
            from_hex_assert_ok(&case[1..].to_ascii_uppercase(), expected);
        }
    }

    fn from_hex_assert_ok(input: &str, expected: Rgb) {
        assert_eq!(
            Rgb::try_from(input.as_bytes()),
            Ok(expected),
            "input: {input}"
        );
    }

    #[test]
    fn test_rgb_from_hex_error() {
        let tests = [
            "0000000", "#00000", "#00000g", "-000000", "#123 45", "123456 ",
        ];
        for test in tests {
            assert!(Rgb::try_from(test.as_bytes()).is_err(), "input: {test}");
        }
    }

    #[test]
    fn test_hex_byte_to_dec_ok() {
        assert_eq!(hex_byte_to_dec(b'0'), Ok(0));
        assert_eq!(hex_byte_to_dec(b'1'), Ok(1));
        assert_eq!(hex_byte_to_dec(b'9'), Ok(9));
        assert_eq!(hex_byte_to_dec(b'a'), Ok(10));
        assert_eq!(hex_byte_to_dec(b'A'), Ok(10));
        assert_eq!(hex_byte_to_dec(b'f'), Ok(15));
        assert_eq!(hex_byte_to_dec(b'F'), Ok(15));
    }

    #[test]
    fn test_hex_byte_to_dec_error() {
        assert!(hex_byte_to_dec(0).is_err());
        assert!(hex_byte_to_dec(9).is_err());
        assert!(hex_byte_to_dec(b'0' - 1).is_err());
        assert!(hex_byte_to_dec(b'a' - 1).is_err());
        assert!(hex_byte_to_dec(b'A' - 1).is_err());
        assert!(hex_byte_to_dec(b'f' + 1).is_err());
        assert!(hex_byte_to_dec(b'F' + 1).is_err());
    }

    #[test]
    fn test_u8_from_two_hex_ok() {
        assert_eq!(u8_from_two_hex(b'0', b'0'), Ok(0));
        assert_eq!(u8_from_two_hex(b'0', b'1'), Ok(1));
        assert_eq!(u8_from_two_hex(b'1', b'0'), Ok(16));
        assert_eq!(u8_from_two_hex(b'0', b'a'), Ok(10));
        assert_eq!(u8_from_two_hex(b'a', b'0'), Ok(160));
        assert_eq!(u8_from_two_hex(b'f', b'f'), Ok(255));
    }

    #[test]
    #[ignore]
    fn test_lab_from_rgb_not_panic() {
        Rgb::for_each(|color: Rgb| {
            let _ = Cielab::from(color);
        });
    }
}
