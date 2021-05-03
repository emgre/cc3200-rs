#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 160usize],
    #[doc = "0xa0 - GPIO0 / (ANA) (Pin 50) Pad Config"]
    pub gpio_pad_config_0: crate::Reg<gpio_pad_config_0::GPIO_PAD_CONFIG_0_SPEC>,
    _reserved1: [u8; 2usize],
    #[doc = "0xa4 - GPIO1 (Pin 55) Pad Config"]
    pub gpio_pad_config_1: crate::Reg<gpio_pad_config_1::GPIO_PAD_CONFIG_1_SPEC>,
    _reserved2: [u8; 2usize],
    #[doc = "0xa8 - GPIO2 (Pin 57) Pad Config"]
    pub gpio_pad_config_2: crate::Reg<gpio_pad_config_2::GPIO_PAD_CONFIG_2_SPEC>,
    _reserved3: [u8; 2usize],
    #[doc = "0xac - GPIO3 (Pin 58) Pad Config"]
    pub gpio_pad_config_3: crate::Reg<gpio_pad_config_3::GPIO_PAD_CONFIG_3_SPEC>,
    _reserved4: [u8; 2usize],
    #[doc = "0xb0 - GPIO4 (Pin 59) Pad Config"]
    pub gpio_pad_config_4: crate::Reg<gpio_pad_config_4::GPIO_PAD_CONFIG_4_SPEC>,
    _reserved5: [u8; 2usize],
    #[doc = "0xb4 - GPIO5 (Pin 60) Pad Config"]
    pub gpio_pad_config_5: crate::Reg<gpio_pad_config_5::GPIO_PAD_CONFIG_5_SPEC>,
    _reserved6: [u8; 2usize],
    #[doc = "0xb8 - GPIO6 (Pin 61) Pad Config"]
    pub gpio_pad_config_6: crate::Reg<gpio_pad_config_6::GPIO_PAD_CONFIG_6_SPEC>,
    _reserved7: [u8; 2usize],
    #[doc = "0xbc - GPIO7 (Pin 62) Pad Config"]
    pub gpio_pad_config_7: crate::Reg<gpio_pad_config_7::GPIO_PAD_CONFIG_7_SPEC>,
    _reserved8: [u8; 2usize],
    #[doc = "0xc0 - GPIO8 (Pin 63) Pad Config"]
    pub gpio_pad_config_8: crate::Reg<gpio_pad_config_8::GPIO_PAD_CONFIG_8_SPEC>,
    _reserved9: [u8; 2usize],
    #[doc = "0xc4 - GPIO9 (Pin 64) Pad Config"]
    pub gpio_pad_config_9: crate::Reg<gpio_pad_config_9::GPIO_PAD_CONFIG_9_SPEC>,
    _reserved10: [u8; 2usize],
    #[doc = "0xc8 - GPIO10 (Pin 1) Pad Config"]
    pub gpio_pad_config_10: crate::Reg<gpio_pad_config_10::GPIO_PAD_CONFIG_10_SPEC>,
    _reserved11: [u8; 2usize],
    #[doc = "0xcc - GPIO11 (Pin 2) Pad Config"]
    pub gpio_pad_config_11: crate::Reg<gpio_pad_config_11::GPIO_PAD_CONFIG_11_SPEC>,
    _reserved12: [u8; 2usize],
    #[doc = "0xd0 - GPIO12 (Pin 3) Pad Config"]
    pub gpio_pad_config_12: crate::Reg<gpio_pad_config_12::GPIO_PAD_CONFIG_12_SPEC>,
    _reserved13: [u8; 2usize],
    #[doc = "0xd4 - GPIO13 (Pin 4) Pad Config"]
    pub gpio_pad_config_13: crate::Reg<gpio_pad_config_13::GPIO_PAD_CONFIG_13_SPEC>,
    _reserved14: [u8; 2usize],
    #[doc = "0xd8 - GPIO14 (Pin 5) Pad Config"]
    pub gpio_pad_config_14: crate::Reg<gpio_pad_config_14::GPIO_PAD_CONFIG_14_SPEC>,
    _reserved15: [u8; 2usize],
    #[doc = "0xdc - GPIO15 (Pin 6) Pad Config"]
    pub gpio_pad_config_15: crate::Reg<gpio_pad_config_15::GPIO_PAD_CONFIG_15_SPEC>,
    _reserved16: [u8; 2usize],
    #[doc = "0xe0 - GPIO16 (Pin 7) Pad Config"]
    pub gpio_pad_config_16: crate::Reg<gpio_pad_config_16::GPIO_PAD_CONFIG_16_SPEC>,
    _reserved17: [u8; 2usize],
    #[doc = "0xe4 - GPIO17 (Pin 8) Pad Config"]
    pub gpio_pad_config_17: crate::Reg<gpio_pad_config_17::GPIO_PAD_CONFIG_17_SPEC>,
    _reserved18: [u8; 2usize],
    #[doc = "0xe8 - GPIO18 (SPI_FLASH_CLK) (Pin 11) Pad Config"]
    pub gpio_pad_config_18: crate::Reg<gpio_pad_config_18::GPIO_PAD_CONFIG_18_SPEC>,
    _reserved19: [u8; 2usize],
    #[doc = "0xec - GPIO19 (SPI_FLASH_DOUT) (Pin 12) Pad Config"]
    pub gpio_pad_config_19: crate::Reg<gpio_pad_config_19::GPIO_PAD_CONFIG_19_SPEC>,
    _reserved20: [u8; 2usize],
    #[doc = "0xf0 - GPIO20 (SPI_FLASH_DIN) (Pin 13) Pad Config"]
    pub gpio_pad_config_20: crate::Reg<gpio_pad_config_20::GPIO_PAD_CONFIG_20_SPEC>,
    _reserved21: [u8; 2usize],
    #[doc = "0xf4 - GPIO21 (SPI_FLASH_CS) (Pin 14) Pad Config"]
    pub gpio_pad_config_21: crate::Reg<gpio_pad_config_21::GPIO_PAD_CONFIG_21_SPEC>,
    _reserved22: [u8; 2usize],
    #[doc = "0xf8 - GPIO22 (Pin 15) Pad Config"]
    pub gpio_pad_config_22: crate::Reg<gpio_pad_config_22::GPIO_PAD_CONFIG_22_SPEC>,
    _reserved23: [u8; 2usize],
    #[doc = "0xfc - GPIO23 (TDI) (Pin 16) Pad Config"]
    pub gpio_pad_config_23: crate::Reg<gpio_pad_config_23::GPIO_PAD_CONFIG_23_SPEC>,
    _reserved24: [u8; 2usize],
    #[doc = "0x100 - GPIO24 (TDO) (Pin 17) Pad Config"]
    pub gpio_pad_config_24: crate::Reg<gpio_pad_config_24::GPIO_PAD_CONFIG_24_SPEC>,
    _reserved25: [u8; 2usize],
    #[doc = "0x104 - GPIO25 (SOP2) (Pin 21) Pad Config"]
    pub gpio_pad_config_25: crate::Reg<gpio_pad_config_25::GPIO_PAD_CONFIG_25_SPEC>,
    _reserved26: [u8; 2usize],
    #[doc = "0x108 - GPIO26 (ANTSEL1) (Pin 29) Pad Config"]
    pub gpio_pad_config_26: crate::Reg<gpio_pad_config_26::GPIO_PAD_CONFIG_26_SPEC>,
    _reserved27: [u8; 2usize],
    #[doc = "0x10c - GPIO27 (ANTSEL2) (Pin 30) Pad Config"]
    pub gpio_pad_config_27: crate::Reg<gpio_pad_config_27::GPIO_PAD_CONFIG_27_SPEC>,
    _reserved28: [u8; 2usize],
    #[doc = "0x110 - GPIO28 (TCK) (Pin 19) Pad Config"]
    pub gpio_pad_config_28: crate::Reg<gpio_pad_config_28::GPIO_PAD_CONFIG_28_SPEC>,
    _reserved29: [u8; 2usize],
    #[doc = "0x114 - GPIO29 (TMS) (Pin 20) Pad Config"]
    pub gpio_pad_config_29: crate::Reg<gpio_pad_config_29::GPIO_PAD_CONFIG_29_SPEC>,
    _reserved30: [u8; 2usize],
    #[doc = "0x118 - GPIO30 (ANA) (Pin 53) Pad Config"]
    pub gpio_pad_config_30: crate::Reg<gpio_pad_config_30::GPIO_PAD_CONFIG_30_SPEC>,
    _reserved31: [u8; 2usize],
    #[doc = "0x11c - GPIO31 (DCDC_ANA2_SW_P) (Pin 45) Pad Config"]
    pub gpio_pad_config_31: crate::Reg<gpio_pad_config_31::GPIO_PAD_CONFIG_31_SPEC>,
    _reserved32: [u8; 2usize],
    #[doc = "0x120 - GPIO32 (RTC_XTAL_N) (Pin 52) Pad Config"]
    pub gpio_pad_config_32: crate::Reg<gpio_pad_config_32::GPIO_PAD_CONFIG_32_SPEC>,
}
#[doc = "GPIO_PAD_CONFIG_0 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_0_SPEC>`"]
pub type GPIO_PAD_CONFIG_0 = crate::Reg<gpio_pad_config_0::GPIO_PAD_CONFIG_0_SPEC>;
#[doc = "GPIO0 / (ANA) (Pin 50) Pad Config"]
pub mod gpio_pad_config_0;
#[doc = "GPIO_PAD_CONFIG_1 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_1_SPEC>`"]
pub type GPIO_PAD_CONFIG_1 = crate::Reg<gpio_pad_config_1::GPIO_PAD_CONFIG_1_SPEC>;
#[doc = "GPIO1 (Pin 55) Pad Config"]
pub mod gpio_pad_config_1;
#[doc = "GPIO_PAD_CONFIG_2 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_2_SPEC>`"]
pub type GPIO_PAD_CONFIG_2 = crate::Reg<gpio_pad_config_2::GPIO_PAD_CONFIG_2_SPEC>;
#[doc = "GPIO2 (Pin 57) Pad Config"]
pub mod gpio_pad_config_2;
#[doc = "GPIO_PAD_CONFIG_3 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_3_SPEC>`"]
pub type GPIO_PAD_CONFIG_3 = crate::Reg<gpio_pad_config_3::GPIO_PAD_CONFIG_3_SPEC>;
#[doc = "GPIO3 (Pin 58) Pad Config"]
pub mod gpio_pad_config_3;
#[doc = "GPIO_PAD_CONFIG_4 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_4_SPEC>`"]
pub type GPIO_PAD_CONFIG_4 = crate::Reg<gpio_pad_config_4::GPIO_PAD_CONFIG_4_SPEC>;
#[doc = "GPIO4 (Pin 59) Pad Config"]
pub mod gpio_pad_config_4;
#[doc = "GPIO_PAD_CONFIG_5 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_5_SPEC>`"]
pub type GPIO_PAD_CONFIG_5 = crate::Reg<gpio_pad_config_5::GPIO_PAD_CONFIG_5_SPEC>;
#[doc = "GPIO5 (Pin 60) Pad Config"]
pub mod gpio_pad_config_5;
#[doc = "GPIO_PAD_CONFIG_6 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_6_SPEC>`"]
pub type GPIO_PAD_CONFIG_6 = crate::Reg<gpio_pad_config_6::GPIO_PAD_CONFIG_6_SPEC>;
#[doc = "GPIO6 (Pin 61) Pad Config"]
pub mod gpio_pad_config_6;
#[doc = "GPIO_PAD_CONFIG_7 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_7_SPEC>`"]
pub type GPIO_PAD_CONFIG_7 = crate::Reg<gpio_pad_config_7::GPIO_PAD_CONFIG_7_SPEC>;
#[doc = "GPIO7 (Pin 62) Pad Config"]
pub mod gpio_pad_config_7;
#[doc = "GPIO_PAD_CONFIG_8 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_8_SPEC>`"]
pub type GPIO_PAD_CONFIG_8 = crate::Reg<gpio_pad_config_8::GPIO_PAD_CONFIG_8_SPEC>;
#[doc = "GPIO8 (Pin 63) Pad Config"]
pub mod gpio_pad_config_8;
#[doc = "GPIO_PAD_CONFIG_9 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_9_SPEC>`"]
pub type GPIO_PAD_CONFIG_9 = crate::Reg<gpio_pad_config_9::GPIO_PAD_CONFIG_9_SPEC>;
#[doc = "GPIO9 (Pin 64) Pad Config"]
pub mod gpio_pad_config_9;
#[doc = "GPIO_PAD_CONFIG_10 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_10_SPEC>`"]
pub type GPIO_PAD_CONFIG_10 = crate::Reg<gpio_pad_config_10::GPIO_PAD_CONFIG_10_SPEC>;
#[doc = "GPIO10 (Pin 1) Pad Config"]
pub mod gpio_pad_config_10;
#[doc = "GPIO_PAD_CONFIG_11 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_11_SPEC>`"]
pub type GPIO_PAD_CONFIG_11 = crate::Reg<gpio_pad_config_11::GPIO_PAD_CONFIG_11_SPEC>;
#[doc = "GPIO11 (Pin 2) Pad Config"]
pub mod gpio_pad_config_11;
#[doc = "GPIO_PAD_CONFIG_12 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_12_SPEC>`"]
pub type GPIO_PAD_CONFIG_12 = crate::Reg<gpio_pad_config_12::GPIO_PAD_CONFIG_12_SPEC>;
#[doc = "GPIO12 (Pin 3) Pad Config"]
pub mod gpio_pad_config_12;
#[doc = "GPIO_PAD_CONFIG_13 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_13_SPEC>`"]
pub type GPIO_PAD_CONFIG_13 = crate::Reg<gpio_pad_config_13::GPIO_PAD_CONFIG_13_SPEC>;
#[doc = "GPIO13 (Pin 4) Pad Config"]
pub mod gpio_pad_config_13;
#[doc = "GPIO_PAD_CONFIG_14 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_14_SPEC>`"]
pub type GPIO_PAD_CONFIG_14 = crate::Reg<gpio_pad_config_14::GPIO_PAD_CONFIG_14_SPEC>;
#[doc = "GPIO14 (Pin 5) Pad Config"]
pub mod gpio_pad_config_14;
#[doc = "GPIO_PAD_CONFIG_15 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_15_SPEC>`"]
pub type GPIO_PAD_CONFIG_15 = crate::Reg<gpio_pad_config_15::GPIO_PAD_CONFIG_15_SPEC>;
#[doc = "GPIO15 (Pin 6) Pad Config"]
pub mod gpio_pad_config_15;
#[doc = "GPIO_PAD_CONFIG_16 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_16_SPEC>`"]
pub type GPIO_PAD_CONFIG_16 = crate::Reg<gpio_pad_config_16::GPIO_PAD_CONFIG_16_SPEC>;
#[doc = "GPIO16 (Pin 7) Pad Config"]
pub mod gpio_pad_config_16;
#[doc = "GPIO_PAD_CONFIG_17 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_17_SPEC>`"]
pub type GPIO_PAD_CONFIG_17 = crate::Reg<gpio_pad_config_17::GPIO_PAD_CONFIG_17_SPEC>;
#[doc = "GPIO17 (Pin 8) Pad Config"]
pub mod gpio_pad_config_17;
#[doc = "GPIO_PAD_CONFIG_18 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_18_SPEC>`"]
pub type GPIO_PAD_CONFIG_18 = crate::Reg<gpio_pad_config_18::GPIO_PAD_CONFIG_18_SPEC>;
#[doc = "GPIO18 (SPI_FLASH_CLK) (Pin 11) Pad Config"]
pub mod gpio_pad_config_18;
#[doc = "GPIO_PAD_CONFIG_19 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_19_SPEC>`"]
pub type GPIO_PAD_CONFIG_19 = crate::Reg<gpio_pad_config_19::GPIO_PAD_CONFIG_19_SPEC>;
#[doc = "GPIO19 (SPI_FLASH_DOUT) (Pin 12) Pad Config"]
pub mod gpio_pad_config_19;
#[doc = "GPIO_PAD_CONFIG_20 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_20_SPEC>`"]
pub type GPIO_PAD_CONFIG_20 = crate::Reg<gpio_pad_config_20::GPIO_PAD_CONFIG_20_SPEC>;
#[doc = "GPIO20 (SPI_FLASH_DIN) (Pin 13) Pad Config"]
pub mod gpio_pad_config_20;
#[doc = "GPIO_PAD_CONFIG_21 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_21_SPEC>`"]
pub type GPIO_PAD_CONFIG_21 = crate::Reg<gpio_pad_config_21::GPIO_PAD_CONFIG_21_SPEC>;
#[doc = "GPIO21 (SPI_FLASH_CS) (Pin 14) Pad Config"]
pub mod gpio_pad_config_21;
#[doc = "GPIO_PAD_CONFIG_22 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_22_SPEC>`"]
pub type GPIO_PAD_CONFIG_22 = crate::Reg<gpio_pad_config_22::GPIO_PAD_CONFIG_22_SPEC>;
#[doc = "GPIO22 (Pin 15) Pad Config"]
pub mod gpio_pad_config_22;
#[doc = "GPIO_PAD_CONFIG_23 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_23_SPEC>`"]
pub type GPIO_PAD_CONFIG_23 = crate::Reg<gpio_pad_config_23::GPIO_PAD_CONFIG_23_SPEC>;
#[doc = "GPIO23 (TDI) (Pin 16) Pad Config"]
pub mod gpio_pad_config_23;
#[doc = "GPIO_PAD_CONFIG_24 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_24_SPEC>`"]
pub type GPIO_PAD_CONFIG_24 = crate::Reg<gpio_pad_config_24::GPIO_PAD_CONFIG_24_SPEC>;
#[doc = "GPIO24 (TDO) (Pin 17) Pad Config"]
pub mod gpio_pad_config_24;
#[doc = "GPIO_PAD_CONFIG_25 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_25_SPEC>`"]
pub type GPIO_PAD_CONFIG_25 = crate::Reg<gpio_pad_config_25::GPIO_PAD_CONFIG_25_SPEC>;
#[doc = "GPIO25 (SOP2) (Pin 21) Pad Config"]
pub mod gpio_pad_config_25;
#[doc = "GPIO_PAD_CONFIG_26 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_26_SPEC>`"]
pub type GPIO_PAD_CONFIG_26 = crate::Reg<gpio_pad_config_26::GPIO_PAD_CONFIG_26_SPEC>;
#[doc = "GPIO26 (ANTSEL1) (Pin 29) Pad Config"]
pub mod gpio_pad_config_26;
#[doc = "GPIO_PAD_CONFIG_27 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_27_SPEC>`"]
pub type GPIO_PAD_CONFIG_27 = crate::Reg<gpio_pad_config_27::GPIO_PAD_CONFIG_27_SPEC>;
#[doc = "GPIO27 (ANTSEL2) (Pin 30) Pad Config"]
pub mod gpio_pad_config_27;
#[doc = "GPIO_PAD_CONFIG_28 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_28_SPEC>`"]
pub type GPIO_PAD_CONFIG_28 = crate::Reg<gpio_pad_config_28::GPIO_PAD_CONFIG_28_SPEC>;
#[doc = "GPIO28 (TCK) (Pin 19) Pad Config"]
pub mod gpio_pad_config_28;
#[doc = "GPIO_PAD_CONFIG_29 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_29_SPEC>`"]
pub type GPIO_PAD_CONFIG_29 = crate::Reg<gpio_pad_config_29::GPIO_PAD_CONFIG_29_SPEC>;
#[doc = "GPIO29 (TMS) (Pin 20) Pad Config"]
pub mod gpio_pad_config_29;
#[doc = "GPIO_PAD_CONFIG_30 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_30_SPEC>`"]
pub type GPIO_PAD_CONFIG_30 = crate::Reg<gpio_pad_config_30::GPIO_PAD_CONFIG_30_SPEC>;
#[doc = "GPIO30 (ANA) (Pin 53) Pad Config"]
pub mod gpio_pad_config_30;
#[doc = "GPIO_PAD_CONFIG_31 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_31_SPEC>`"]
pub type GPIO_PAD_CONFIG_31 = crate::Reg<gpio_pad_config_31::GPIO_PAD_CONFIG_31_SPEC>;
#[doc = "GPIO31 (DCDC_ANA2_SW_P) (Pin 45) Pad Config"]
pub mod gpio_pad_config_31;
#[doc = "GPIO_PAD_CONFIG_32 register accessor: an alias for `Reg<GPIO_PAD_CONFIG_32_SPEC>`"]
pub type GPIO_PAD_CONFIG_32 = crate::Reg<gpio_pad_config_32::GPIO_PAD_CONFIG_32_SPEC>;
#[doc = "GPIO32 (RTC_XTAL_N) (Pin 52) Pad Config"]
pub mod gpio_pad_config_32;
