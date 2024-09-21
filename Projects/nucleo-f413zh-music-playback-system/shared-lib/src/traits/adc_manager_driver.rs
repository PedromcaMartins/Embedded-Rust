// Define a trait for any analog input, removing the AdcPin dependency
pub trait AnalogInput {
    fn read_value(&mut self) -> u16;
}