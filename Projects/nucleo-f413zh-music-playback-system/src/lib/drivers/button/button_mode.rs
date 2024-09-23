use core::future::Future;

pub trait ButtonMode {
    fn is_pressed_down(&self) -> bool;
    fn is_released(&self) -> bool {
        !self.is_pressed_down()
    }
}

pub trait InterruptMode: ButtonMode {
    fn wait_for_press_down(&mut self) -> impl Future<Output = ()>;
    fn wait_for_release(&mut self) -> impl Future<Output = ()>;
}