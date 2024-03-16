#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

    pub fn from_hex(hex: &[u8]) -> Option<Self> {
        const HEX_COLOR_LEN: usize = 6;

        let start_from: usize = usize::from(hex.first() == Some(&b'#'));

        if hex.len() != HEX_COLOR_LEN + start_from {
            return None;
        }

        Some(Self {
            r: u8_from_two_hex(*hex.get(start_from)?, *hex.get(start_from + 1)?)?,
            g: u8_from_two_hex(*hex.get(start_from + 2)?, *hex.get(start_from + 3)?)?,
            b: u8_from_two_hex(*hex.get(start_from + 4)?, *hex.get(start_from + 5)?)?,
        })
    }
}

impl std::fmt::Display for Rgb {
    fn fmt(& self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

impl std::fmt::LowerHex for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl std::fmt::UpperHex for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

pub const fn rgb(red: u8, green: u8, blue: u8) -> Rgb {
    Rgb::new(red, green, blue)
}

const fn u8_from_two_hex(hex1: u8, hex2: u8) -> Option<u8> {
    match (hex_byte_to_dec(hex1), hex_byte_to_dec(hex2)) {
        (Some(v1), Some(v2)) => Some(v1 * 16 + v2),
        _ => None
    }
}

/**
    Converts hex byte to the decimal number.

    The hex byte must be a **byte** instead of a number. `b'0'` and not `0`.
*/
const fn hex_byte_to_dec(hex: u8) -> Option<u8> {
    match hex {
        b'0'..=b'9' => Some(hex - b'0'),
        b'a'..=b'f' => Some(hex - b'a' + 10),
        b'A'..=b'F' => Some(hex - b'A' + 10),
        _ => None,
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
            from_hex_assert_some(&case, expected);
            from_hex_assert_some(&case.to_ascii_lowercase(), expected);
            from_hex_assert_some(&case.to_ascii_uppercase(), expected);
            from_hex_assert_some(&case[1..].to_ascii_lowercase(), expected);
            from_hex_assert_some(&case[1..].to_ascii_uppercase(), expected);
        }
    }

    fn from_hex_assert_some(input: &str, expected: Rgb) {
        assert_eq!(Rgb::from_hex(input.as_bytes()), Some(expected), "input: {input}");
    }

    #[test]
    fn test_rgb_from_hex_none() {
        let tests = [
            "0000000", "#00000", "#00000g", "-000000",
            "#123 45", "123456 "
        ];
        for test in tests {
            assert!(Rgb::from_hex(test.as_bytes()).is_none(), "input: {test}");
        }
    }

    #[test]
    fn test_hex_byte_to_dec_some() {
        assert_eq!(hex_byte_to_dec(b'0'), Some(0));
        assert_eq!(hex_byte_to_dec(b'1'), Some(1));
        assert_eq!(hex_byte_to_dec(b'9'), Some(9));
        assert_eq!(hex_byte_to_dec(b'a'), Some(10));
        assert_eq!(hex_byte_to_dec(b'A'), Some(10));
        assert_eq!(hex_byte_to_dec(b'f'), Some(15));
        assert_eq!(hex_byte_to_dec(b'F'), Some(15));
    }

    #[test]
    fn test_hex_byte_to_dec_none() {
        assert!(hex_byte_to_dec(0).is_none());
        assert!(hex_byte_to_dec(9).is_none());
        assert!(hex_byte_to_dec(b'0' - 1).is_none());
        assert!(hex_byte_to_dec(b'a' - 1).is_none());
        assert!(hex_byte_to_dec(b'A' - 1).is_none());
        assert!(hex_byte_to_dec(b'f' + 1).is_none());
        assert!(hex_byte_to_dec(b'F' + 1).is_none());
    }

    #[test]
    fn test_u8_from_two_hex() {
        assert_eq!(u8_from_two_hex(b'0', b'0'), Some(0));
        assert_eq!(u8_from_two_hex(b'0', b'1'), Some(1));
        assert_eq!(u8_from_two_hex(b'1', b'0'), Some(16));
        assert_eq!(u8_from_two_hex(b'0', b'a'), Some(10));
        assert_eq!(u8_from_two_hex(b'a', b'0'), Some(160));
        assert_eq!(u8_from_two_hex(b'f', b'f'), Some(255));
    }
}
