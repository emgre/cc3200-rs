use cc3200_pac::SYST;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};

pub struct Delay {
    syst: SYST,
}

impl Delay {
    pub fn new(mut syst: SYST) -> Self {
        syst.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
        Self { syst }
    }

    pub fn free(self) -> SYST {
        self.syst
    }
}

impl DelayMs<u8> for Delay {
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(ms as u32);
    }
}

impl DelayMs<u16> for Delay {
    fn delay_ms(&mut self, ms: u16) {
        self.delay_ms(ms as u32);
    }
}

impl DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        self.delay_us(ms * 1_000);
    }
}

impl DelayUs<u8> for Delay {
    fn delay_us(&mut self, us: u8) {
        self.delay_us(us as u32);
    }
}

impl DelayUs<u16> for Delay {
    fn delay_us(&mut self, us: u16) {
        self.delay_us(us as u32);
    }
}

impl DelayUs<u32> for Delay {
    fn delay_us(&mut self, mut us: u32) {
        // CC3200 clocks runs at 80 MHz
        const SYSTICK_FREQ_MHZ: u32 = 80;
        // SysTick is 24-bit wide
        const MAX_SYSTICK_VALUE: u32 = 0xFFFFFF;
        // This is an exact value of 131,071 us
        const MAX_DELAY_US: u32 = MAX_SYSTICK_VALUE / SYSTICK_FREQ_MHZ;

        // If the value is greater than what can be represented in SysTick register
        if us > MAX_DELAY_US {
            // Configure timer with maximum value
            self.syst.set_reload(MAX_SYSTICK_VALUE);
            self.syst.clear_current();
            self.syst.enable_counter();

            // Repeat until the remaining value is within the range of SysTick register
            while us > MAX_DELAY_US {
                // Wait for the timer to elapse
                while !self.syst.has_wrapped() {}
                us -= MAX_DELAY_US;
            }

            self.syst.disable_counter();
        }

        // We should now be within range
        assert!(us < MAX_DELAY_US);

        // Exit immediatly if delay is 0
        if us == 0 {
            return;
        }

        // Set the value properly
        self.syst.set_reload(us * SYSTICK_FREQ_MHZ);
        self.syst.clear_current();
        self.syst.enable_counter();

        // Wait for the timer to elapse
        while !self.syst.has_wrapped() {}

        self.syst.disable_counter();
    }
}
