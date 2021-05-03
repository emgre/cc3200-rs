#![no_std]

pub use cc3200_pac::{self, CorePeripherals, Peripherals};

//pub mod bb;
pub mod delay;
pub mod gpio;
pub mod sysctl;
