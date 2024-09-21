use mockall::automock;
use shared_lib::traits::LedDriver;

pub struct Led;

#[automock]
impl LedDriver for Led {
    fn toggle(&mut self) {}
    fn turn_on(&mut self) {}
    fn turn_off(&mut self) {}
}