#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Load"]
    pub wdtload: crate::Reg<wdtload::WDTLOAD_SPEC>,
    #[doc = "0x04 - Watchdog Value"]
    pub wdtvalue: crate::Reg<wdtvalue::WDTVALUE_SPEC>,
    #[doc = "0x08 - Watchdog Control"]
    pub wdtctl: crate::Reg<wdtctl::WDTCTL_SPEC>,
    #[doc = "0x0c - Watchdog Interrupt Clear"]
    pub wdticr: crate::Reg<wdticr::WDTICR_SPEC>,
    #[doc = "0x10 - Watchdog Raw Interrupt Status"]
    pub wdtris: crate::Reg<wdtris::WDTRIS_SPEC>,
    _reserved5: [u8; 1028usize],
    #[doc = "0x418 - Watchdog Test Mode"]
    pub wdttest: crate::Reg<wdttest::WDTTEST_SPEC>,
    _reserved6: [u8; 2020usize],
    #[doc = "0xc00 - Watchdog Lock"]
    pub wdtlock: crate::Reg<wdtlock::WDTLOCK_SPEC>,
}
#[doc = "WDTLOAD register accessor: an alias for `Reg<WDTLOAD_SPEC>`"]
pub type WDTLOAD = crate::Reg<wdtload::WDTLOAD_SPEC>;
#[doc = "Watchdog Load"]
pub mod wdtload;
#[doc = "WDTVALUE register accessor: an alias for `Reg<WDTVALUE_SPEC>`"]
pub type WDTVALUE = crate::Reg<wdtvalue::WDTVALUE_SPEC>;
#[doc = "Watchdog Value"]
pub mod wdtvalue;
#[doc = "WDTCTL register accessor: an alias for `Reg<WDTCTL_SPEC>`"]
pub type WDTCTL = crate::Reg<wdtctl::WDTCTL_SPEC>;
#[doc = "Watchdog Control"]
pub mod wdtctl;
#[doc = "WDTICR register accessor: an alias for `Reg<WDTICR_SPEC>`"]
pub type WDTICR = crate::Reg<wdticr::WDTICR_SPEC>;
#[doc = "Watchdog Interrupt Clear"]
pub mod wdticr;
#[doc = "WDTRIS register accessor: an alias for `Reg<WDTRIS_SPEC>`"]
pub type WDTRIS = crate::Reg<wdtris::WDTRIS_SPEC>;
#[doc = "Watchdog Raw Interrupt Status"]
pub mod wdtris;
#[doc = "WDTTEST register accessor: an alias for `Reg<WDTTEST_SPEC>`"]
pub type WDTTEST = crate::Reg<wdttest::WDTTEST_SPEC>;
#[doc = "Watchdog Test Mode"]
pub mod wdttest;
#[doc = "WDTLOCK register accessor: an alias for `Reg<WDTLOCK_SPEC>`"]
pub type WDTLOCK = crate::Reg<wdtlock::WDTLOCK_SPEC>;
#[doc = "Watchdog Lock"]
pub mod wdtlock;
