use cc3200_pac::ARCM;

pub struct Sysctl {
    arcm: ARCM,
}

/// The majority of method takes a mutable reference because no bit-banding
/// facility is available for these registers.
impl Sysctl {
    pub fn new(arcm: ARCM) -> Self {
        Self { arcm }
    }

    pub fn free(self) -> ARCM {
        self.arcm
    }

    /// Activate or deactivate clocks and resets to the given peripheral
    /// in the given run mode.
    pub fn control_power(&mut self, peripheral: Peripheral, mode: RunMode, state: PowerState) {
        match mode {
            RunMode::Run => self.control_run_power(peripheral, state),
            RunMode::Sleep => self.control_sleep_power(peripheral, state),
            RunMode::DeepSleep => self.control_deepsleep_power(peripheral, state),
        }
    }

    fn control_run_power(&mut self, peripheral: Peripheral, state: PowerState) {
        match peripheral {
            Peripheral::Camera => self
                .arcm
                .camclken
                .modify(|_, w| w.runclken().bit(state.into())),
            Peripheral::I2s => self
                .arcm
                .mcaspclken
                .modify(|_, w| w.runclken().bit(state.into())),
            Peripheral::SdHost => self
                .arcm
                .sdiomclken
                .modify(|_, w| w.runclken().bit(state.into())),
            Peripheral::Spi => self
                .arcm
                .apspiclken
                .modify(|_, w| w.runclken().bit(state.into())),
            Peripheral::Dma => self
                .arcm
                .dmaclken
                .modify(|_, w| w.runclken().bit(state.into())),
            Peripheral::Gpio0 => self
                .arcm
                .gpio0clken
                .modify(|_, w| w.runclken().bit(state.into())),
            Peripheral::Gpio1 => self
                .arcm
                .gpio1clken
                .modify(|_, w| w.runclken().bit(state.into())),
            Peripheral::Gpio2 => self
                .arcm
                .gpio2clken
                .modify(|_, w| w.runclken().bit(state.into())),
            Peripheral::Gpio3 => self
                .arcm
                .gpio3clken
                .modify(|_, w| w.runclken().bit(state.into())),
            Peripheral::Gpio4 => self
                .arcm
                .gpio4clken
                .modify(|_, w| w.runclken().bit(state.into())),
            Peripheral::Watchdog => self
                .arcm
                .wdtclken
                .modify(|_, w| w.runclken().bit(state.into())),
            Peripheral::Uart0 => self
                .arcm
                .uart0clken
                .modify(|_, w| w.runclken().bit(state.into())),
            Peripheral::Uart1 => self
                .arcm
                .uart1clken
                .modify(|_, w| w.runclken().bit(state.into())),
            Peripheral::Timer0 => self
                .arcm
                .gpt0clken
                .modify(|_, w| w.runclken().bit(state.into())),
            Peripheral::Timer1 => self
                .arcm
                .gpt1clken
                .modify(|_, w| w.runclken().bit(state.into())),
            Peripheral::Timer2 => self
                .arcm
                .gpt2clken
                .modify(|_, w| w.runclken().bit(state.into())),
            Peripheral::Timer3 => self
                .arcm
                .gpt3clken
                .modify(|_, w| w.runclken().bit(state.into())),
            Peripheral::I2c => self
                .arcm
                .i2cclken
                .modify(|_, w| w.runclken().bit(state.into())),
        }
    }

    fn control_sleep_power(&mut self, peripheral: Peripheral, state: PowerState) {
        match peripheral {
            Peripheral::Camera => self
                .arcm
                .camclken
                .modify(|_, w| w.slpclken().bit(state.into())),
            Peripheral::I2s => self
                .arcm
                .mcaspclken
                .modify(|_, w| w.slpclken().bit(state.into())),
            Peripheral::SdHost => self
                .arcm
                .sdiomclken
                .modify(|_, w| w.slpclken().bit(state.into())),
            Peripheral::Spi => self
                .arcm
                .apspiclken
                .modify(|_, w| w.slpclken().bit(state.into())),
            Peripheral::Dma => self
                .arcm
                .dmaclken
                .modify(|_, w| w.slpclken().bit(state.into())),
            Peripheral::Gpio0 => self
                .arcm
                .gpio0clken
                .modify(|_, w| w.slpclken().bit(state.into())),
            Peripheral::Gpio1 => self
                .arcm
                .gpio1clken
                .modify(|_, w| w.slpclken().bit(state.into())),
            Peripheral::Gpio2 => self
                .arcm
                .gpio2clken
                .modify(|_, w| w.slpclken().bit(state.into())),
            Peripheral::Gpio3 => self
                .arcm
                .gpio3clken
                .modify(|_, w| w.slpclken().bit(state.into())),
            Peripheral::Gpio4 => self
                .arcm
                .gpio4clken
                .modify(|_, w| w.slpclken().bit(state.into())),
            Peripheral::Watchdog => self
                .arcm
                .wdtclken
                .modify(|_, w| w.slpclken().bit(state.into())),
            Peripheral::Uart0 => self
                .arcm
                .uart0clken
                .modify(|_, w| w.slpclken().bit(state.into())),
            Peripheral::Uart1 => self
                .arcm
                .uart1clken
                .modify(|_, w| w.slpclken().bit(state.into())),
            Peripheral::Timer0 => self
                .arcm
                .gpt0clken
                .modify(|_, w| w.slpclken().bit(state.into())),
            Peripheral::Timer1 => self
                .arcm
                .gpt1clken
                .modify(|_, w| w.slpclken().bit(state.into())),
            Peripheral::Timer2 => self
                .arcm
                .gpt2clken
                .modify(|_, w| w.slpclken().bit(state.into())),
            Peripheral::Timer3 => self
                .arcm
                .gpt3clken
                .modify(|_, w| w.slpclken().bit(state.into())),
            Peripheral::I2c => self
                .arcm
                .i2cclken
                .modify(|_, w| w.slpclken().bit(state.into())),
        }
    }

    fn control_deepsleep_power(&mut self, peripheral: Peripheral, state: PowerState) {
        match peripheral {
            Peripheral::Camera => self
                .arcm
                .camclken
                .modify(|_, w| w.dslpclken().bit(state.into())),
            Peripheral::I2s => self
                .arcm
                .mcaspclken
                .modify(|_, w| w.dslpclken().bit(state.into())),
            Peripheral::SdHost => self
                .arcm
                .sdiomclken
                .modify(|_, w| w.dslpclken().bit(state.into())),
            Peripheral::Spi => self
                .arcm
                .apspiclken
                .modify(|_, w| w.dslpclken().bit(state.into())),
            Peripheral::Dma => self
                .arcm
                .dmaclken
                .modify(|_, w| w.dslpclken().bit(state.into())),
            Peripheral::Gpio0 => self
                .arcm
                .gpio0clken
                .modify(|_, w| w.dslpclken().bit(state.into())),
            Peripheral::Gpio1 => self
                .arcm
                .gpio1clken
                .modify(|_, w| w.dslpclken().bit(state.into())),
            Peripheral::Gpio2 => self
                .arcm
                .gpio2clken
                .modify(|_, w| w.dslpclken().bit(state.into())),
            Peripheral::Gpio3 => self
                .arcm
                .gpio3clken
                .modify(|_, w| w.dslpclken().bit(state.into())),
            Peripheral::Gpio4 => self
                .arcm
                .gpio4clken
                .modify(|_, w| w.dslpclken().bit(state.into())),
            Peripheral::Watchdog => self
                .arcm
                .wdtclken
                .modify(|_, w| w.dslpclken().bit(state.into())),
            Peripheral::Uart0 => self
                .arcm
                .uart0clken
                .modify(|_, w| w.dslpclken().bit(state.into())),
            Peripheral::Uart1 => self
                .arcm
                .uart1clken
                .modify(|_, w| w.dslpclken().bit(state.into())),
            Peripheral::Timer0 => self
                .arcm
                .gpt0clken
                .modify(|_, w| w.dslpclken().bit(state.into())),
            Peripheral::Timer1 => self
                .arcm
                .gpt1clken
                .modify(|_, w| w.dslpclken().bit(state.into())),
            Peripheral::Timer2 => self
                .arcm
                .gpt2clken
                .modify(|_, w| w.dslpclken().bit(state.into())),
            Peripheral::Timer3 => self
                .arcm
                .gpt3clken
                .modify(|_, w| w.dslpclken().bit(state.into())),
            Peripheral::I2c => self
                .arcm
                .i2cclken
                .modify(|_, w| w.dslpclken().bit(state.into())),
        }
    }
}

/// Peripheral enum that can be activated
pub enum Peripheral {
    /// Parallel Camera Interface Module
    Camera,
    /// Inter-integrated Sound Multi-Channel Audio Serial Port
    I2s,
    /// SD Host Controller Interface
    SdHost,
    /// Serial Peripheral Interface
    Spi,
    /// Direct Memory Access
    Dma,
    /// General-Purpose Input/Output 0
    Gpio0,
    /// General-Purpose Input/Output 1
    Gpio1,
    /// General-Purpose Input/Output 2
    Gpio2,
    /// General-Purpose Input/Output 3
    Gpio3,
    /// General-Purpose Input/Output 4
    Gpio4,
    /// Watchdog Timer
    Watchdog,
    /// Universal Asynchronous Receiver/Transmitter 0
    Uart0,
    /// Universal Asynchronous Receiver/Transmitter 1
    Uart1,
    /// General-Purpose Timer 0
    Timer0,
    /// General-Purpose Timer 1
    Timer1,
    /// General-Purpose Timer 2
    Timer2,
    /// General-Purpose Timer 3
    Timer3,
    /// Inter-Integrated Circuit
    I2c,
}

pub enum RunMode {
    /// Normal running mode
    Run,
    /// Sleep running mode
    Sleep,
    /// Deep sleep running mode
    DeepSleep,
}

pub enum PowerState {
    On,
    Off,
}

impl From<PowerState> for bool {
    fn from(from: PowerState) -> Self {
        match from {
            PowerState::On => true,
            PowerState::Off => false,
        }
    }
}
