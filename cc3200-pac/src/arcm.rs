#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Camera Clock Enable"]
    pub camclken: crate::Reg<camclken::CAMCLKEN_SPEC>,
    #[doc = "0x08 - Camera Software Reset"]
    pub camswrst: crate::Reg<camswrst::CAMSWRST_SPEC>,
    _reserved2: [u8; 8usize],
    #[doc = "0x14 - I2S Clock Enable"]
    pub mcaspclken: crate::Reg<mcaspclken::MCASPCLKEN_SPEC>,
    #[doc = "0x18 - I2S Software Reset"]
    pub mcaspswrst: crate::Reg<mcaspswrst::MCASPSWRST_SPEC>,
    _reserved4: [u8; 8usize],
    #[doc = "0x24 - SD Host Clock Enable"]
    pub sdiomclken: crate::Reg<sdiomclken::SDIOMCLKEN_SPEC>,
    #[doc = "0x28 - SD Host Software Reset"]
    pub sdiomswrst: crate::Reg<sdiomswrst::SDIOMSWRST_SPEC>,
    _reserved6: [u8; 4usize],
    #[doc = "0x30 - SPI Clock Enable"]
    pub apspiclken: crate::Reg<apspiclken::APSPICLKEN_SPEC>,
    #[doc = "0x34 - SPI Software Reset"]
    pub apspiswrst: crate::Reg<apspiswrst::APSPISWRST_SPEC>,
    _reserved8: [u8; 16usize],
    #[doc = "0x48 - DMA Clock Enable"]
    pub dmaclken: crate::Reg<dmaclken::DMACLKEN_SPEC>,
    #[doc = "0x4c - DMA Software Reset"]
    pub dmaswrst: crate::Reg<dmaswrst::DMASWRST_SPEC>,
    #[doc = "0x50 - GPIO0 Clock Enable"]
    pub gpio0clken: crate::Reg<gpio0clken::GPIO0CLKEN_SPEC>,
    #[doc = "0x54 - GPIO0 Software Reset"]
    pub gpio0swrst: crate::Reg<gpio0swrst::GPIO0SWRST_SPEC>,
    #[doc = "0x58 - GPIO1 Clock Enable"]
    pub gpio1clken: crate::Reg<gpio1clken::GPIO1CLKEN_SPEC>,
    #[doc = "0x5c - GPIO1 Software Reset"]
    pub gpio1swrst: crate::Reg<gpio1swrst::GPIO1SWRST_SPEC>,
    #[doc = "0x60 - GPIO2 Clock Enable"]
    pub gpio2clken: crate::Reg<gpio2clken::GPIO2CLKEN_SPEC>,
    #[doc = "0x64 - GPIO2 Software Reset"]
    pub gpio2swrst: crate::Reg<gpio2swrst::GPIO2SWRST_SPEC>,
    #[doc = "0x68 - GPIO3 Clock Enable"]
    pub gpio3clken: crate::Reg<gpio3clken::GPIO3CLKEN_SPEC>,
    #[doc = "0x6c - GPIO3 Software Reset"]
    pub gpio3swrst: crate::Reg<gpio3swrst::GPIO3SWRST_SPEC>,
    #[doc = "0x70 - GPIO4 Clock Enable"]
    pub gpio4clken: crate::Reg<gpio4clken::GPIO4CLKEN_SPEC>,
    #[doc = "0x74 - GPIO4 Software Reset"]
    pub gpio4swrst: crate::Reg<gpio4swrst::GPIO4SWRST_SPEC>,
    #[doc = "0x78 - Watchdog Clock Enable"]
    pub wdtclken: crate::Reg<wdtclken::WDTCLKEN_SPEC>,
    #[doc = "0x7c - Watchdog Software Reset"]
    pub wdtswrst: crate::Reg<wdtswrst::WDTSWRST_SPEC>,
    #[doc = "0x80 - UART0 Clock Enable"]
    pub uart0clken: crate::Reg<uart0clken::UART0CLKEN_SPEC>,
    #[doc = "0x84 - UART0 Software Reset"]
    pub uart0swrst: crate::Reg<uart0swrst::UART0SWRST_SPEC>,
    #[doc = "0x88 - UART1 Clock Enable"]
    pub uart1clken: crate::Reg<uart1clken::UART1CLKEN_SPEC>,
    #[doc = "0x8c - UART1 Software Reset"]
    pub uart1swrst: crate::Reg<uart1swrst::UART1SWRST_SPEC>,
    #[doc = "0x90 - GPT0 Clock Enable"]
    pub gpt0clken: crate::Reg<gpt0clken::GPT0CLKEN_SPEC>,
    #[doc = "0x94 - GPT0 Software Reset"]
    pub gpt0swrst: crate::Reg<gpt0swrst::GPT0SWRST_SPEC>,
    #[doc = "0x98 - GPT1 Clock Enable"]
    pub gpt1clken: crate::Reg<gpt1clken::GPT1CLKEN_SPEC>,
    #[doc = "0x9c - GPT1 Software Reset"]
    pub gpt1swrst: crate::Reg<gpt1swrst::GPT1SWRST_SPEC>,
    #[doc = "0xa0 - GPT2 Clock Enable"]
    pub gpt2clken: crate::Reg<gpt2clken::GPT2CLKEN_SPEC>,
    #[doc = "0xa4 - GPT2 Software Reset"]
    pub gpt2swrst: crate::Reg<gpt2swrst::GPT2SWRST_SPEC>,
    #[doc = "0xa8 - GPT3 Clock Enable"]
    pub gpt3clken: crate::Reg<gpt3clken::GPT3CLKEN_SPEC>,
    #[doc = "0xac - GPT3 Software Reset"]
    pub gpt3swrst: crate::Reg<gpt3swrst::GPT3SWRST_SPEC>,
    _reserved34: [u8; 40usize],
    #[doc = "0xd8 - I2C Clock Enable"]
    pub i2cclken: crate::Reg<i2cclken::I2CCLKEN_SPEC>,
    #[doc = "0xdc - I2C Software Reset"]
    pub i2cswrst: crate::Reg<i2cswrst::I2CSWRST_SPEC>,
}
#[doc = "CAMCLKEN register accessor: an alias for `Reg<CAMCLKEN_SPEC>`"]
pub type CAMCLKEN = crate::Reg<camclken::CAMCLKEN_SPEC>;
#[doc = "Camera Clock Enable"]
pub mod camclken;
#[doc = "CAMSWRST register accessor: an alias for `Reg<CAMSWRST_SPEC>`"]
pub type CAMSWRST = crate::Reg<camswrst::CAMSWRST_SPEC>;
#[doc = "Camera Software Reset"]
pub mod camswrst;
#[doc = "MCASPCLKEN register accessor: an alias for `Reg<MCASPCLKEN_SPEC>`"]
pub type MCASPCLKEN = crate::Reg<mcaspclken::MCASPCLKEN_SPEC>;
#[doc = "I2S Clock Enable"]
pub mod mcaspclken;
#[doc = "MCASPSWRST register accessor: an alias for `Reg<MCASPSWRST_SPEC>`"]
pub type MCASPSWRST = crate::Reg<mcaspswrst::MCASPSWRST_SPEC>;
#[doc = "I2S Software Reset"]
pub mod mcaspswrst;
#[doc = "SDIOMCLKEN register accessor: an alias for `Reg<SDIOMCLKEN_SPEC>`"]
pub type SDIOMCLKEN = crate::Reg<sdiomclken::SDIOMCLKEN_SPEC>;
#[doc = "SD Host Clock Enable"]
pub mod sdiomclken;
#[doc = "SDIOMSWRST register accessor: an alias for `Reg<SDIOMSWRST_SPEC>`"]
pub type SDIOMSWRST = crate::Reg<sdiomswrst::SDIOMSWRST_SPEC>;
#[doc = "SD Host Software Reset"]
pub mod sdiomswrst;
#[doc = "APSPICLKEN register accessor: an alias for `Reg<APSPICLKEN_SPEC>`"]
pub type APSPICLKEN = crate::Reg<apspiclken::APSPICLKEN_SPEC>;
#[doc = "SPI Clock Enable"]
pub mod apspiclken;
#[doc = "APSPISWRST register accessor: an alias for `Reg<APSPISWRST_SPEC>`"]
pub type APSPISWRST = crate::Reg<apspiswrst::APSPISWRST_SPEC>;
#[doc = "SPI Software Reset"]
pub mod apspiswrst;
#[doc = "DMACLKEN register accessor: an alias for `Reg<DMACLKEN_SPEC>`"]
pub type DMACLKEN = crate::Reg<dmaclken::DMACLKEN_SPEC>;
#[doc = "DMA Clock Enable"]
pub mod dmaclken;
#[doc = "DMASWRST register accessor: an alias for `Reg<DMASWRST_SPEC>`"]
pub type DMASWRST = crate::Reg<dmaswrst::DMASWRST_SPEC>;
#[doc = "DMA Software Reset"]
pub mod dmaswrst;
#[doc = "GPIO0CLKEN register accessor: an alias for `Reg<GPIO0CLKEN_SPEC>`"]
pub type GPIO0CLKEN = crate::Reg<gpio0clken::GPIO0CLKEN_SPEC>;
#[doc = "GPIO0 Clock Enable"]
pub mod gpio0clken;
#[doc = "GPIO0SWRST register accessor: an alias for `Reg<GPIO0SWRST_SPEC>`"]
pub type GPIO0SWRST = crate::Reg<gpio0swrst::GPIO0SWRST_SPEC>;
#[doc = "GPIO0 Software Reset"]
pub mod gpio0swrst;
#[doc = "GPIO1CLKEN register accessor: an alias for `Reg<GPIO1CLKEN_SPEC>`"]
pub type GPIO1CLKEN = crate::Reg<gpio1clken::GPIO1CLKEN_SPEC>;
#[doc = "GPIO1 Clock Enable"]
pub mod gpio1clken;
#[doc = "GPIO1SWRST register accessor: an alias for `Reg<GPIO1SWRST_SPEC>`"]
pub type GPIO1SWRST = crate::Reg<gpio1swrst::GPIO1SWRST_SPEC>;
#[doc = "GPIO1 Software Reset"]
pub mod gpio1swrst;
#[doc = "GPIO2CLKEN register accessor: an alias for `Reg<GPIO2CLKEN_SPEC>`"]
pub type GPIO2CLKEN = crate::Reg<gpio2clken::GPIO2CLKEN_SPEC>;
#[doc = "GPIO2 Clock Enable"]
pub mod gpio2clken;
#[doc = "GPIO2SWRST register accessor: an alias for `Reg<GPIO2SWRST_SPEC>`"]
pub type GPIO2SWRST = crate::Reg<gpio2swrst::GPIO2SWRST_SPEC>;
#[doc = "GPIO2 Software Reset"]
pub mod gpio2swrst;
#[doc = "GPIO3CLKEN register accessor: an alias for `Reg<GPIO3CLKEN_SPEC>`"]
pub type GPIO3CLKEN = crate::Reg<gpio3clken::GPIO3CLKEN_SPEC>;
#[doc = "GPIO3 Clock Enable"]
pub mod gpio3clken;
#[doc = "GPIO3SWRST register accessor: an alias for `Reg<GPIO3SWRST_SPEC>`"]
pub type GPIO3SWRST = crate::Reg<gpio3swrst::GPIO3SWRST_SPEC>;
#[doc = "GPIO3 Software Reset"]
pub mod gpio3swrst;
#[doc = "GPIO4CLKEN register accessor: an alias for `Reg<GPIO4CLKEN_SPEC>`"]
pub type GPIO4CLKEN = crate::Reg<gpio4clken::GPIO4CLKEN_SPEC>;
#[doc = "GPIO4 Clock Enable"]
pub mod gpio4clken;
#[doc = "GPIO4SWRST register accessor: an alias for `Reg<GPIO4SWRST_SPEC>`"]
pub type GPIO4SWRST = crate::Reg<gpio4swrst::GPIO4SWRST_SPEC>;
#[doc = "GPIO4 Software Reset"]
pub mod gpio4swrst;
#[doc = "WDTCLKEN register accessor: an alias for `Reg<WDTCLKEN_SPEC>`"]
pub type WDTCLKEN = crate::Reg<wdtclken::WDTCLKEN_SPEC>;
#[doc = "Watchdog Clock Enable"]
pub mod wdtclken;
#[doc = "WDTSWRST register accessor: an alias for `Reg<WDTSWRST_SPEC>`"]
pub type WDTSWRST = crate::Reg<wdtswrst::WDTSWRST_SPEC>;
#[doc = "Watchdog Software Reset"]
pub mod wdtswrst;
#[doc = "UART0CLKEN register accessor: an alias for `Reg<UART0CLKEN_SPEC>`"]
pub type UART0CLKEN = crate::Reg<uart0clken::UART0CLKEN_SPEC>;
#[doc = "UART0 Clock Enable"]
pub mod uart0clken;
#[doc = "UART0SWRST register accessor: an alias for `Reg<UART0SWRST_SPEC>`"]
pub type UART0SWRST = crate::Reg<uart0swrst::UART0SWRST_SPEC>;
#[doc = "UART0 Software Reset"]
pub mod uart0swrst;
#[doc = "UART1CLKEN register accessor: an alias for `Reg<UART1CLKEN_SPEC>`"]
pub type UART1CLKEN = crate::Reg<uart1clken::UART1CLKEN_SPEC>;
#[doc = "UART1 Clock Enable"]
pub mod uart1clken;
#[doc = "UART1SWRST register accessor: an alias for `Reg<UART1SWRST_SPEC>`"]
pub type UART1SWRST = crate::Reg<uart1swrst::UART1SWRST_SPEC>;
#[doc = "UART1 Software Reset"]
pub mod uart1swrst;
#[doc = "GPT0CLKEN register accessor: an alias for `Reg<GPT0CLKEN_SPEC>`"]
pub type GPT0CLKEN = crate::Reg<gpt0clken::GPT0CLKEN_SPEC>;
#[doc = "GPT0 Clock Enable"]
pub mod gpt0clken;
#[doc = "GPT0SWRST register accessor: an alias for `Reg<GPT0SWRST_SPEC>`"]
pub type GPT0SWRST = crate::Reg<gpt0swrst::GPT0SWRST_SPEC>;
#[doc = "GPT0 Software Reset"]
pub mod gpt0swrst;
#[doc = "GPT1CLKEN register accessor: an alias for `Reg<GPT1CLKEN_SPEC>`"]
pub type GPT1CLKEN = crate::Reg<gpt1clken::GPT1CLKEN_SPEC>;
#[doc = "GPT1 Clock Enable"]
pub mod gpt1clken;
#[doc = "GPT1SWRST register accessor: an alias for `Reg<GPT1SWRST_SPEC>`"]
pub type GPT1SWRST = crate::Reg<gpt1swrst::GPT1SWRST_SPEC>;
#[doc = "GPT1 Software Reset"]
pub mod gpt1swrst;
#[doc = "GPT2CLKEN register accessor: an alias for `Reg<GPT2CLKEN_SPEC>`"]
pub type GPT2CLKEN = crate::Reg<gpt2clken::GPT2CLKEN_SPEC>;
#[doc = "GPT2 Clock Enable"]
pub mod gpt2clken;
#[doc = "GPT2SWRST register accessor: an alias for `Reg<GPT2SWRST_SPEC>`"]
pub type GPT2SWRST = crate::Reg<gpt2swrst::GPT2SWRST_SPEC>;
#[doc = "GPT2 Software Reset"]
pub mod gpt2swrst;
#[doc = "GPT3CLKEN register accessor: an alias for `Reg<GPT3CLKEN_SPEC>`"]
pub type GPT3CLKEN = crate::Reg<gpt3clken::GPT3CLKEN_SPEC>;
#[doc = "GPT3 Clock Enable"]
pub mod gpt3clken;
#[doc = "GPT3SWRST register accessor: an alias for `Reg<GPT3SWRST_SPEC>`"]
pub type GPT3SWRST = crate::Reg<gpt3swrst::GPT3SWRST_SPEC>;
#[doc = "GPT3 Software Reset"]
pub mod gpt3swrst;
#[doc = "I2CCLKEN register accessor: an alias for `Reg<I2CCLKEN_SPEC>`"]
pub type I2CCLKEN = crate::Reg<i2cclken::I2CCLKEN_SPEC>;
#[doc = "I2C Clock Enable"]
pub mod i2cclken;
#[doc = "I2CSWRST register accessor: an alias for `Reg<I2CSWRST_SPEC>`"]
pub type I2CSWRST = crate::Reg<i2cswrst::I2CSWRST_SPEC>;
#[doc = "I2C Software Reset"]
pub mod i2cswrst;
