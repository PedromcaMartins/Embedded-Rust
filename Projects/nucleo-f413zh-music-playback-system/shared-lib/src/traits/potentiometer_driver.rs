pub trait PotentiometerDriver {
    fn read_raw_value(&mut self) -> u16;
    fn read_position(&mut self) -> u8;
}