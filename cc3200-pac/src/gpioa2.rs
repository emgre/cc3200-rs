#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1020usize],
    #[doc = "0x3fc - GPIO Data"]
    pub gpiodata: crate::Reg<gpiodata::GPIODATA_SPEC>,
    #[doc = "0x400 - GPIO Data Direction"]
    pub gpiodir: crate::Reg<gpiodir::GPIODIR_SPEC>,
    #[doc = "0x404 - GPIO Interrupt Sense"]
    pub gpiois: crate::Reg<gpiois::GPIOIS_SPEC>,
    #[doc = "0x408 - GPIO Interrupt Both Edges"]
    pub gpioibe: crate::Reg<gpioibe::GPIOIBE_SPEC>,
    #[doc = "0x40c - GPIO Interrupt Event"]
    pub gpioiev: crate::Reg<gpioiev::GPIOIEV_SPEC>,
    #[doc = "0x410 - GPIO Interrupt Mask Enable"]
    pub gpioim: crate::Reg<gpioim::GPIOIM_SPEC>,
    #[doc = "0x414 - GPIO Interrupt Raw Status"]
    pub gpioris: crate::Reg<gpioris::GPIORIS_SPEC>,
    #[doc = "0x418 - GPIO Masked Interrupt Status"]
    pub gpiomis: crate::Reg<gpiomis::GPIOMIS_SPEC>,
    #[doc = "0x41c - GPIO Interrupt Clear"]
    pub gpioicr: crate::Reg<gpioicr::GPIOICR_SPEC>,
}
#[doc = "GPIODATA register accessor: an alias for `Reg<GPIODATA_SPEC>`"]
pub type GPIODATA = crate::Reg<gpiodata::GPIODATA_SPEC>;
#[doc = "GPIO Data"]
pub mod gpiodata;
#[doc = "GPIODIR register accessor: an alias for `Reg<GPIODIR_SPEC>`"]
pub type GPIODIR = crate::Reg<gpiodir::GPIODIR_SPEC>;
#[doc = "GPIO Data Direction"]
pub mod gpiodir;
#[doc = "GPIOIS register accessor: an alias for `Reg<GPIOIS_SPEC>`"]
pub type GPIOIS = crate::Reg<gpiois::GPIOIS_SPEC>;
#[doc = "GPIO Interrupt Sense"]
pub mod gpiois;
#[doc = "GPIOIBE register accessor: an alias for `Reg<GPIOIBE_SPEC>`"]
pub type GPIOIBE = crate::Reg<gpioibe::GPIOIBE_SPEC>;
#[doc = "GPIO Interrupt Both Edges"]
pub mod gpioibe;
#[doc = "GPIOIEV register accessor: an alias for `Reg<GPIOIEV_SPEC>`"]
pub type GPIOIEV = crate::Reg<gpioiev::GPIOIEV_SPEC>;
#[doc = "GPIO Interrupt Event"]
pub mod gpioiev;
#[doc = "GPIOIM register accessor: an alias for `Reg<GPIOIM_SPEC>`"]
pub type GPIOIM = crate::Reg<gpioim::GPIOIM_SPEC>;
#[doc = "GPIO Interrupt Mask Enable"]
pub mod gpioim;
#[doc = "GPIORIS register accessor: an alias for `Reg<GPIORIS_SPEC>`"]
pub type GPIORIS = crate::Reg<gpioris::GPIORIS_SPEC>;
#[doc = "GPIO Interrupt Raw Status"]
pub mod gpioris;
#[doc = "GPIOMIS register accessor: an alias for `Reg<GPIOMIS_SPEC>`"]
pub type GPIOMIS = crate::Reg<gpiomis::GPIOMIS_SPEC>;
#[doc = "GPIO Masked Interrupt Status"]
pub mod gpiomis;
#[doc = "GPIOICR register accessor: an alias for `Reg<GPIOICR_SPEC>`"]
pub type GPIOICR = crate::Reg<gpioicr::GPIOICR_SPEC>;
#[doc = "GPIO Interrupt Clear"]
pub mod gpioicr;
