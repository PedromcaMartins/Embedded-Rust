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
        Percentage(100)
    }

    #[allow(unused)]
    pub fn hundred() -> Self {
        Percentage(100)
    }

    pub fn value(self) -> u8 {
        self.0
    }
}

impl defmt::Format for Percentage {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{:02}%", self.0)
    }
}