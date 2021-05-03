use core::marker::PhantomData;

pub trait GpioExt {
    /// The type to split the GPIO into
    type Parts;

    /// Splits the GPIO port into independent pins and registers
    fn split(self, sysctl: &mut crate::sysctl::Sysctl) -> Self::Parts;
}

/// Unconfigured pin
pub struct Tristate;

/// GPIO in input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (type state)
pub struct Floating;

/// Pulled down input (type state)
pub struct PullDown;

/// Pulled up input (type state)
pub struct PullUp;

/// GPIO in output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push-pull output (type state)
pub struct PushPull;

/// Open drain input or output (type state)
pub struct OpenDrain;

/// Drive strength of an output
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum DriveStrength {
    TwoMilliAmps,
    FourMilliAmps,
    SixMilliAmps,
    EightMilliAmps,
    TenMilliAmps,
    TwelveMilliAmps,
    FourteenMilliAmps,
}

#[macro_export]
macro_rules! gpio_macro {
    (
        // The GPIO identifier in the PAC crate
        $GPIOAX:ident,
        // The created GPIO module
        $gpiox:ident,
        // The sysctl::Peripheral enum value
        $iopd:ident,
        // Erased pin struct identifier
        $PXx:ident,
        // List of pins of this port
        [$(
            // {Pin struct identifier}: ({Pad identifier}, {Pin identifier}, {GPIO number in port (0 to 7)}, {Boot mode})
            $Pxx:ident: ($gpio_pad_config_xx:ident, $gpioxx:ident, $i:expr, $MODE:ty),
        )+]
    ) => {
        pub mod $gpiox {
            use super::*;
            use crate::sysctl;
            use cc3200_pac::$GPIOAX;
            use cc3200_pac::OCP_SHARED;
            use core::convert::Infallible;
            use embedded_hal::digital::v2::*;

            pub struct Parts {
                $(
                    pub $gpioxx: $Pxx<$MODE>,
                )+
            }

            impl GpioExt for $GPIOAX {
                type Parts = Parts;

                fn split(self, sysctl: &mut sysctl::Sysctl) -> Parts {
                    // Enable GPIO peripheral
                    sysctl.control_power(
                        sysctl::Peripheral::$iopd,
                        sysctl::RunMode::Run,
                        sysctl::PowerState::On
                    );

                    Parts {
                        $(
                            $gpioxx: $Pxx { _mode: PhantomData },
                        )+
                    }
                }
            }

            $(
                pub struct $Pxx<MODE> {
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> $Pxx<MODE> {
                    /// Configures the pin to operate as a floating input pin
                    pub fn into_floating_input(self) -> $Pxx<Input<Floating>> {
                        self.pads().$gpio_pad_config_xx.write(|w| w
                            .confmode().$gpioxx()
                            .opendrain().clear_bit()
                            .pullup().clear_bit()
                            .pulldown().clear_bit()
                            .output_enable_override().clear_bit()
                            .output_buffer_override().clear_bit()
                        );
                        self.set_input_dir();

                        $Pxx { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled down input pin
                    pub fn into_pull_down_input(self) -> $Pxx<Input<Floating>> {
                        self.pads().$gpio_pad_config_xx.write(|w| w
                            .confmode().$gpioxx()
                            .opendrain().clear_bit()
                            .pullup().clear_bit()
                            .pulldown().set_bit()
                            .output_enable_override().clear_bit()
                            .output_buffer_override().clear_bit()
                        );
                        self.set_input_dir();

                        $Pxx { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled up input pin
                    pub fn into_pull_up_input(self) -> $Pxx<Input<Floating>> {
                        self.pads().$gpio_pad_config_xx.write(|w| w
                            .confmode().$gpioxx()
                            .opendrain().clear_bit()
                            .pullup().set_bit()
                            .pulldown().clear_bit()
                            .output_enable_override().clear_bit()
                            .output_buffer_override().clear_bit()
                        );
                        self.set_input_dir();

                        $Pxx { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an open drain output pin
                    pub fn into_open_drain_output(self) -> $Pxx<Output<OpenDrain>> {
                        self.pads().$gpio_pad_config_xx.write(|w| w
                            .confmode().$gpioxx()
                            .opendrain().set_bit()
                            .pullup().clear_bit()
                            .pulldown().clear_bit()
                            .output_enable_override().clear_bit()
                            .output_buffer_override().clear_bit()
                        );
                        self.set_output_dir();

                        $Pxx { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an push pull output pin
                    pub fn into_push_pull_output(self) -> $Pxx<Output<PushPull>> {
                        self.pads().$gpio_pad_config_xx.write(|w| w
                            .confmode().$gpioxx()
                            .opendrain().clear_bit()
                            .pullup().clear_bit()
                            .pulldown().clear_bit()
                            .output_enable_override().clear_bit()
                            .output_buffer_override().clear_bit()
                        );
                        self.set_output_dir();

                        $Pxx { _mode: PhantomData }
                    }

                    fn pads(&self) -> &cc3200_pac::ocp_shared::RegisterBlock {
                        unsafe { &(*OCP_SHARED::ptr()) }
                    }

                    fn set_input_dir(&self) {
                        let gpio = unsafe { &(*$GPIOAX::ptr()) };

                        // TODO: use bit-banding for atomic change instead
                        gpio.gpiodir.modify(|r, w| unsafe {
                            w.bits(r.bits() & !(1 << $i))
                        });
                    }

                    fn set_output_dir(&self) {
                        let gpio = unsafe { &(*$GPIOAX::ptr()) };

                        // TODO: use bit-banding for atomic change instead
                        gpio.gpiodir.modify(|r, w| unsafe {
                            w.bits(r.bits() | (1 << $i))
                        });
                    }
                }

                impl<MODE> $Pxx<Output<MODE>> {
                    pub fn drive_strength(self, strength: DriveStrength) -> Self {
                        type PacDriveStrength = cc3200_pac::ocp_shared::$gpio_pad_config_xx::DRIVESTRENGTH_A;
                        let variant = match strength {
                            DriveStrength::TwoMilliAmps => PacDriveStrength::_2MA,
                            DriveStrength::FourMilliAmps => PacDriveStrength::_4MA,
                            DriveStrength::SixMilliAmps => PacDriveStrength::_6MA,
                            DriveStrength::EightMilliAmps => PacDriveStrength::_8MA,
                            DriveStrength::TenMilliAmps => PacDriveStrength::_10MA,
                            DriveStrength::TwelveMilliAmps => PacDriveStrength::_12MA,
                            DriveStrength::FourteenMilliAmps => PacDriveStrength::_14MA,
                        };

                        self.pads().$gpio_pad_config_xx.modify(|_, w| w
                            .drivestrength().variant(variant)
                        );

                        self
                    }
                }

                impl<MODE> InputPin for $Pxx<Input<MODE>> {
                    type Error = Infallible;

                    fn is_low(&self) -> Result<bool, Self::Error> {
                        Ok(!self.is_high()?)
                    }

                    fn is_high(&self) -> Result<bool, Self::Error> {
                        // TODO: use bit-banding for atomic read instead
                        unsafe {
                            Ok(&(*$GPIOAX::ptr()).gpiodata.read().bits() & (1 << $i) != 0)
                        }
                    }
                }

                impl<MODE> OutputPin for $Pxx<Output<MODE>> {
                    type Error = Infallible;

                    fn set_low(&mut self) -> Result<(), Self::Error> {
                        unsafe {
                            // TODO: use bit-banding for atomic change instead
                            &(*$GPIOAX::ptr()).gpiodata.modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });
                        }
                        Ok(())
                    }

                    fn set_high(&mut self) -> Result<(), Self::Error> {
                        unsafe {
                            // TODO: use bit-banding for atomic change instead
                            &(*$GPIOAX::ptr()).gpiodata.modify(|r, w| {
                                w.bits(r.bits() | (1 << $i))
                            });
                        }
                        Ok(())
                    }
                }

                impl<MODE> StatefulOutputPin for $Pxx<Output<MODE>> {
                    fn is_set_low(&self) -> Result<bool, Self::Error> {
                        Ok(!self.is_set_high()?)
                    }

                    fn is_set_high(&self) -> Result<bool, Self::Error> {
                        // TODO: use bit-banding for atomic read instead
                        unsafe {
                            Ok(&(*$GPIOAX::ptr()).gpiodata.read().bits() & (1 << $i) != 0)
                        }
                    }
                }

                impl<MODE> ToggleableOutputPin for $Pxx<Output<MODE>> {
                    type Error = Infallible;

                    fn toggle(&mut self) -> Result<(), Self::Error> {
                        unsafe {
                            // TODO: use bit-banding for atomic change instead
                            &(*$GPIOAX::ptr()).gpiodata.modify(|r, w| {
                                w.bits(r.bits() ^ (1 << $i))
                            });
                        }
                        Ok(())
                    }
                }
            )+
        }
    }
}

gpio_macro!(GPIOA0, gpio0, Gpio0, P0x, [
    P00: (gpio_pad_config_0, gpio0, 0, Tristate),
    P01: (gpio_pad_config_1, gpio1, 1, Tristate),
    P02: (gpio_pad_config_2, gpio2, 2, Tristate),
    P03: (gpio_pad_config_3, gpio3, 3, Tristate),
    P04: (gpio_pad_config_4, gpio4, 4, Tristate),
    P05: (gpio_pad_config_5, gpio5, 5, Tristate),
    P06: (gpio_pad_config_6, gpio6, 6, Tristate),
    P07: (gpio_pad_config_7, gpio7, 7, Tristate),
]);

gpio_macro!(GPIOA1, gpio1, Gpio1, P1x, [
    P08: (gpio_pad_config_8, gpio8, 0, Tristate),
    P09: (gpio_pad_config_9, gpio9, 1, Tristate),
    P10: (gpio_pad_config_10, gpio10, 2, Tristate),
    P11: (gpio_pad_config_11, gpio11, 3, Tristate),
    P12: (gpio_pad_config_12, gpio12, 4, Tristate),
    P13: (gpio_pad_config_13, gpio13, 5, Tristate),
    P14: (gpio_pad_config_14, gpio14, 6, Tristate),
    P15: (gpio_pad_config_15, gpio15, 7, Tristate),
]);

gpio_macro!(GPIOA2, gpio2, Gpio2, P2x, [
    P16: (gpio_pad_config_16, gpio16, 0, Tristate),
    P17: (gpio_pad_config_17, gpio17, 1, Tristate),
    //P18: (gpio_pad_config_18, gpio18, 2, Tristate),
    //P19: (gpio_pad_config_19, gpio19, 3, Tristate),
    //P20: (gpio_pad_config_20, gpio20, 4, Tristate),
    //P21: (gpio_pad_config_21, gpio21, 5, Tristate),
    P22: (gpio_pad_config_22, gpio22, 6, Tristate),
    P23: (gpio_pad_config_23, gpio23, 7, Tristate),
]);

gpio_macro!(GPIOA3, gpio3, Gpio3, P3x, [
    P24: (gpio_pad_config_24, gpio24, 0, Tristate),
    P25: (gpio_pad_config_25, gpio25, 1, Tristate),
    //P26: (gpio_pad_config_26, gpio26, 2, Tristate),
    //P27: (gpio_pad_config_27, gpio27, 3, Tristate),
    P28: (gpio_pad_config_28, gpio28, 4, Tristate),
    P29: (gpio_pad_config_29, gpio29, 5, Tristate),
    P30: (gpio_pad_config_30, gpio30, 6, Tristate),
    P31: (gpio_pad_config_31, gpio31, 7, Tristate),
]);
