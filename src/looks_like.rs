/// LooksLikePostalCode is a trait to use for types that could be used for postal codes. This
/// implementation can convert any signed or unsigned integer, any float, String or &str to u32.
pub trait LooksLikePostalCode {
    fn as_u32(&self) -> u32;
}

impl LooksLikePostalCode for u8 {
    fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl LooksLikePostalCode for u16 {
    fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl LooksLikePostalCode for u64 {
    fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl LooksLikePostalCode for i8 {
    fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl LooksLikePostalCode for i16 {
    fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl LooksLikePostalCode for i32 {
    fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl LooksLikePostalCode for i64 {
    fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl LooksLikePostalCode for f32 {
    fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl LooksLikePostalCode for f64 {
    fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl LooksLikePostalCode for String {
    fn as_u32(&self) -> u32 {
        match self.parse::<u32>() {
            Ok(v) => v,
            Err(_) => 0u32,
        }
    }
}

impl LooksLikePostalCode for &str {
    fn as_u32(&self) -> u32 {
        match self.parse::<u32>() {
            Ok(v) => v,
            Err(_) => 0u32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traits() {
        assert_eq!(10u8.as_u32(), 10u32);
        assert_eq!(10u16.as_u32(), 10u32);
        assert_eq!(10u64.as_u32(), 10u32);
        assert_eq!(10i8.as_u32(), 10u32);
        assert_eq!(10i16.as_u32(), 10u32);
        assert_eq!(10i32.as_u32(), 10u32);
        assert_eq!(10i64.as_u32(), 10u32);
        assert_eq!(10.0f32.as_u32(), 10u32);
        assert_eq!(10.0f64.as_u32(), 10u32);
        assert_eq!("10".as_u32(), 10u32);
        assert_eq!(String::from("10").as_u32(), 10u32);
    }
}
