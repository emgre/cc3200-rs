#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_semihosting as _;

use cc3200_hal::delay::Delay;
use cc3200_hal::gpio::*;
use cc3200_hal::sysctl::*;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::*;

#[entry]
fn main() -> ! {
    hprintln!("Hello world!").unwrap();

    let core = cc3200_hal::CorePeripherals::take().unwrap();
    let p = cc3200_hal::Peripherals::take().unwrap();

    // Enable peripheral clocks
    let mut sysctl = cc3200_hal::sysctl::Sysctl::new(p.ARCM);
    sysctl.control_power(Peripheral::Gpio1, RunMode::Run, PowerState::On);

    let gpioa1 = p.GPIOA1.split(&mut sysctl);
    let mut red_led = gpioa1.gpio9.into_push_pull_output();
    let mut yellow_led = gpioa1.gpio10.into_push_pull_output();
    let mut _green_led = gpioa1.gpio11.into_push_pull_output();

    let mut delay = Delay::new(core.SYST);

    const FULL_DUTY: u32 = 10;
    let mut current_duty = 0;
    loop {
        for _ in 0..10 {
            // Light the LEDs
            red_led.set_high().unwrap();
            delay.delay_ms(current_duty);

            // Turn off the LEDs
            red_led.set_low().unwrap();
            delay.delay_ms(FULL_DUTY - current_duty);
        }

        current_duty += 1;
        if current_duty > FULL_DUTY {
            current_duty = 0;
            yellow_led.toggle().unwrap();
        }
    }

    // Configure the pads
    /*let shared = p.OCP_SHARED;
    shared.gpio_pad_config_9.write(|w| w
        .confmode().gpio9()
        .drivestrength()._2m_a()
        .output_enable_override().clear_bit()
        .output_buffer_override().clear_bit()
    );
    shared.gpio_pad_config_10.write(|w| w
        .confmode().gpio10()
        .drivestrength()._2m_a()
        .output_enable_override().clear_bit()
        .output_buffer_override().clear_bit()
    );
    shared.gpio_pad_config_11.write(|w| w
        .confmode().gpio11()
        .drivestrength()._2m_a()
        .output_enable_override().clear_bit()
        .output_buffer_override().clear_bit()
    );

    // Configure the GPIO modes
    let gpio = p.GPIOA1;
    gpio.gpiodir.write(|w| unsafe { w.bits(0x0E) });

    let mut delay = Delay::new(core.SYST);

    const FULL_DUTY: u32 = 10;
    let mut current_duty = 0;
    loop {
        for _ in 0..10 {
            // Light the LEDs
            gpio.gpiodata.write(|w| unsafe { w.bits(0x0E) });
            delay.delay_ms(current_duty);

            // Turn off the LEDs
            gpio.gpiodata.write(|w| unsafe { w.bits(0x00) });
            delay.delay_ms(FULL_DUTY - current_duty);
        }

        current_duty += 1;
        if current_duty > FULL_DUTY {
            current_duty = 0;
        }
    }*/
}
