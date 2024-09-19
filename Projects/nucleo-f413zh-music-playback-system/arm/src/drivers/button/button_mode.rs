pub trait ButtonMode {
    fn is_pressed_down(&self) -> bool;
    fn is_released(&self) -> bool {
        !self.is_pressed_down()
    }
}

pub trait InterruptMode: ButtonMode {
    async fn wait_for_press_down(&mut self);
    async fn wait_for_release(&mut self);
}