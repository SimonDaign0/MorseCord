use rppal::gpio::{Gpio, InputPin, OutputPin};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::u8;
pub struct Board {
    pub btn: InputPin,
    pub led: Arc<Mutex<OutputPin>>,
}
impl Board {
    pub fn new(btn_pin: u8, led_pin: u8) -> Board {
        let gpio = Gpio::new().expect("failed to init GPIO");
        let btn = gpio
            .get(btn_pin) //
            .expect("failed to get button")
            .into_input_pullup();
        let led = gpio
            .get(led_pin) //
            .expect("failed to get LED")
            .into_output();
        Board {
            btn,
            led: Arc::new(Mutex::new(led)),
        }
    }
    pub fn blink(&mut self, n: u64) {
        let mut led_lock = self.led.lock().unwrap();
        for _ in 0..n {
            led_lock.set_high();
            thread::sleep(Duration::from_millis(300));
            led_lock.set_low();
            thread::sleep(Duration::from_millis(300));
        }
    }

    pub fn _cleanup(&mut self) {
        let mut led_lock = self.led.lock().unwrap();
        led_lock.set_low();
    }
}
