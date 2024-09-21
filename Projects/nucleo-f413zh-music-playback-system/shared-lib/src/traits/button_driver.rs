use core::future::Future;

pub trait ButtonDriverPolling {
    fn is_pressed_down(&self) -> bool;
    fn is_released(&self) -> bool;
}

pub trait ButtonDriverInterrupt: ButtonDriverPolling {
    fn wait_for_press_down(&mut self) -> impl Future<Output = ()>;
    fn wait_for_release(&mut self) -> impl Future<Output = ()>;
}