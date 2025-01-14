use embassy_stm32::{time::Hertz, timer::{simple_pwm::SimplePwm, CaptureCompare16bitInstance, Channel}};
use embassy_time::{Duration, Timer};

pub struct BuzzerEncoder<'d, T>
where 
    T: CaptureCompare16bitInstance
{
    pwm: SimplePwm<'d, T>, 
    channel: Channel, 
}

impl <'d, T> BuzzerEncoder<'d, T>
where 
    T: CaptureCompare16bitInstance
{
    pub fn new(mut pwm: SimplePwm<'d, T> , channel: Channel) -> Self {
        let duty_cycle = pwm.get_max_duty()/2;
        pwm.set_duty(channel, duty_cycle);
        pwm.enable(channel);

        Self { 
            pwm, 
            channel, 
        }
    }

    pub async fn set_freq_during_duration(&mut self, freq: Hertz, duration: Duration) {
        self.pwm.set_frequency(freq);
        Timer::after(duration).await;
    }

    pub fn set_duty_cycle(&mut self, duty_cycle: u16) {
        self.pwm.set_duty(self.channel, duty_cycle);
    }
}
