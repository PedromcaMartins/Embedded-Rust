pub trait LedDriver {
    fn toggle(&mut self);
    fn turn_on(&mut self);
    fn turn_off(&mut self);
}