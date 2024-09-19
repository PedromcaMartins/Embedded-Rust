#[derive(PartialEq, Eq)]
pub struct Percentage(u8);

impl Percentage {
    #[allow(unused)]
    pub fn from_ratio_u32(numerator: u32, denominator: u32) -> Self {
        let ratio = (numerator as f32 / denominator as f32) * 100.0;
        Percentage(ratio as u8)
    }

    #[allow(unused)]
    pub fn zero() -> Self {
        Percentage(0)
    }

    #[allow(unused)]
    pub fn fifty() -> Self {
        Percentage(50)
    }

    #[allow(unused)]
    pub fn hundred() -> Self {
        Percentage(100)
    }

    pub fn value(self) -> u8 {
        self.0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_ratio_u32() {
        assert_eq!(Percentage::from_ratio_u32(1, 2).value(), 50);
        assert_eq!(Percentage::from_ratio_u32(1, 4).value(), 25);
        assert_eq!(Percentage::from_ratio_u32(3, 4).value(), 75);
        assert_eq!(Percentage::from_ratio_u32(0, 1).value(), 0);
        assert_eq!(Percentage::from_ratio_u32(1, 1).value(), 100);
    }

    #[test]
    fn test_zero() {
        assert_eq!(Percentage::zero().value(), 0);
    }

    #[test]
    fn test_fifty() {
        assert_eq!(Percentage::fifty().value(), 50);
    }

    #[test]
    fn test_hundred() {
        assert_eq!(Percentage::hundred().value(), 100);
    }

    #[test]
    fn test_value() {
        let p = Percentage(42);
        assert_eq!(p.value(), 42);
    }
}
