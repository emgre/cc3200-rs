#[doc = "Register `GPIO_PAD_CONFIG_20` reader"]
pub struct R(crate::R<GPIO_PAD_CONFIG_20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_PAD_CONFIG_20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPIO_PAD_CONFIG_20_SPEC>> for R {
    fn from(reader: crate::R<GPIO_PAD_CONFIG_20_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_PAD_CONFIG_20` writer"]
pub struct W(crate::W<GPIO_PAD_CONFIG_20_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_PAD_CONFIG_20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<GPIO_PAD_CONFIG_20_SPEC>> for W {
    fn from(writer: crate::W<GPIO_PAD_CONFIG_20_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Configuration Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CONFMODE_A {
    #[doc = "1: Data From SPI Serial Flash (Fixed Default)"]
    FLASH_SPI_DIN = 1,
}
impl From<CONFMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CONFMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CONFMODE` reader - Configuration Mode"]
pub struct CONFMODE_R(crate::FieldReader<u8, CONFMODE_A>);
impl CONFMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CONFMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CONFMODE_A> {
        match self.bits {
            1 => Some(CONFMODE_A::FLASH_SPI_DIN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_SPI_DIN`"]
    #[inline(always)]
    pub fn is_flash_spi_din(&self) -> bool {
        **self == CONFMODE_A::FLASH_SPI_DIN
    }
}
impl core::ops::Deref for CONFMODE_R {
    type Target = crate::FieldReader<u8, CONFMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONFMODE` writer - Configuration Mode"]
pub struct CONFMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONFMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Data From SPI Serial Flash (Fixed Default)"]
    #[inline(always)]
    pub fn flash_spi_din(self) -> &'a mut W {
        self.variant(CONFMODE_A::FLASH_SPI_DIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u16 & 0x0f);
        self.w
    }
}
#[doc = "Field `OPENDRAIN` reader - Open-drain Mode"]
pub struct OPENDRAIN_R(crate::FieldReader<bool, bool>);
impl OPENDRAIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPENDRAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPENDRAIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPENDRAIN` writer - Open-drain Mode"]
pub struct OPENDRAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPENDRAIN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Drive Strength\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DRIVESTRENGTH_A {
    #[doc = "0: Output driver not enabled"]
    DISABLED = 0,
    #[doc = "1: 2 mA"]
    _2MA = 1,
    #[doc = "2: 4 mA"]
    _4MA = 2,
    #[doc = "3: 6 mA"]
    _6MA = 3,
    #[doc = "4: 8 mA"]
    _8MA = 4,
    #[doc = "5: 10 mA"]
    _10MA = 5,
    #[doc = "6: 12 mA"]
    _12MA = 6,
    #[doc = "7: 14 mA"]
    _14MA = 7,
}
impl From<DRIVESTRENGTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DRIVESTRENGTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DRIVESTRENGTH` reader - Drive Strength"]
pub struct DRIVESTRENGTH_R(crate::FieldReader<u8, DRIVESTRENGTH_A>);
impl DRIVESTRENGTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        DRIVESTRENGTH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRIVESTRENGTH_A {
        match self.bits {
            0 => DRIVESTRENGTH_A::DISABLED,
            1 => DRIVESTRENGTH_A::_2MA,
            2 => DRIVESTRENGTH_A::_4MA,
            3 => DRIVESTRENGTH_A::_6MA,
            4 => DRIVESTRENGTH_A::_8MA,
            5 => DRIVESTRENGTH_A::_10MA,
            6 => DRIVESTRENGTH_A::_12MA,
            7 => DRIVESTRENGTH_A::_14MA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DRIVESTRENGTH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `_2MA`"]
    #[inline(always)]
    pub fn is_2m_a(&self) -> bool {
        **self == DRIVESTRENGTH_A::_2MA
    }
    #[doc = "Checks if the value of the field is `_4MA`"]
    #[inline(always)]
    pub fn is_4m_a(&self) -> bool {
        **self == DRIVESTRENGTH_A::_4MA
    }
    #[doc = "Checks if the value of the field is `_6MA`"]
    #[inline(always)]
    pub fn is_6m_a(&self) -> bool {
        **self == DRIVESTRENGTH_A::_6MA
    }
    #[doc = "Checks if the value of the field is `_8MA`"]
    #[inline(always)]
    pub fn is_8m_a(&self) -> bool {
        **self == DRIVESTRENGTH_A::_8MA
    }
    #[doc = "Checks if the value of the field is `_10MA`"]
    #[inline(always)]
    pub fn is_10m_a(&self) -> bool {
        **self == DRIVESTRENGTH_A::_10MA
    }
    #[doc = "Checks if the value of the field is `_12MA`"]
    #[inline(always)]
    pub fn is_12m_a(&self) -> bool {
        **self == DRIVESTRENGTH_A::_12MA
    }
    #[doc = "Checks if the value of the field is `_14MA`"]
    #[inline(always)]
    pub fn is_14m_a(&self) -> bool {
        **self == DRIVESTRENGTH_A::_14MA
    }
}
impl core::ops::Deref for DRIVESTRENGTH_R {
    type Target = crate::FieldReader<u8, DRIVESTRENGTH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRIVESTRENGTH` writer - Drive Strength"]
pub struct DRIVESTRENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVESTRENGTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRIVESTRENGTH_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output driver not enabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DRIVESTRENGTH_A::DISABLED)
    }
    #[doc = "2 mA"]
    #[inline(always)]
    pub fn _2m_a(self) -> &'a mut W {
        self.variant(DRIVESTRENGTH_A::_2MA)
    }
    #[doc = "4 mA"]
    #[inline(always)]
    pub fn _4m_a(self) -> &'a mut W {
        self.variant(DRIVESTRENGTH_A::_4MA)
    }
    #[doc = "6 mA"]
    #[inline(always)]
    pub fn _6m_a(self) -> &'a mut W {
        self.variant(DRIVESTRENGTH_A::_6MA)
    }
    #[doc = "8 mA"]
    #[inline(always)]
    pub fn _8m_a(self) -> &'a mut W {
        self.variant(DRIVESTRENGTH_A::_8MA)
    }
    #[doc = "10 mA"]
    #[inline(always)]
    pub fn _10m_a(self) -> &'a mut W {
        self.variant(DRIVESTRENGTH_A::_10MA)
    }
    #[doc = "12 mA"]
    #[inline(always)]
    pub fn _12m_a(self) -> &'a mut W {
        self.variant(DRIVESTRENGTH_A::_12MA)
    }
    #[doc = "14 mA"]
    #[inline(always)]
    pub fn _14m_a(self) -> &'a mut W {
        self.variant(DRIVESTRENGTH_A::_14MA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u16 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `PULLUP` reader - Enable internal weak pullup"]
pub struct PULLUP_R(crate::FieldReader<bool, bool>);
impl PULLUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULLUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULLUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULLUP` writer - Enable internal weak pullup"]
pub struct PULLUP_W<'a> {
    w: &'a mut W,
}
impl<'a> PULLUP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `PULLDOWN` reader - Enable internal weak pulldown"]
pub struct PULLDOWN_R(crate::FieldReader<bool, bool>);
impl PULLDOWN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULLDOWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULLDOWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULLDOWN` writer - Enable internal weak pulldown"]
pub struct PULLDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> PULLDOWN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `OUTPUT_ENABLE_OVERRIDE` reader - Pad output enable override value"]
pub struct OUTPUT_ENABLE_OVERRIDE_R(crate::FieldReader<bool, bool>);
impl OUTPUT_ENABLE_OVERRIDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUTPUT_ENABLE_OVERRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTPUT_ENABLE_OVERRIDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTPUT_ENABLE_OVERRIDE` writer - Pad output enable override value"]
pub struct OUTPUT_ENABLE_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPUT_ENABLE_OVERRIDE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u16 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `OUTPUT_BUFFER_OVERRIDE` reader - Pad output buffer override enable"]
pub struct OUTPUT_BUFFER_OVERRIDE_R(crate::FieldReader<bool, bool>);
impl OUTPUT_BUFFER_OVERRIDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUTPUT_BUFFER_OVERRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTPUT_BUFFER_OVERRIDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTPUT_BUFFER_OVERRIDE` writer - Pad output buffer override enable"]
pub struct OUTPUT_BUFFER_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPUT_BUFFER_OVERRIDE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u16 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Configuration Mode"]
    #[inline(always)]
    pub fn confmode(&self) -> CONFMODE_R {
        CONFMODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Open-drain Mode"]
    #[inline(always)]
    pub fn opendrain(&self) -> OPENDRAIN_R {
        OPENDRAIN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Drive Strength"]
    #[inline(always)]
    pub fn drivestrength(&self) -> DRIVESTRENGTH_R {
        DRIVESTRENGTH_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Enable internal weak pullup"]
    #[inline(always)]
    pub fn pullup(&self) -> PULLUP_R {
        PULLUP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable internal weak pulldown"]
    #[inline(always)]
    pub fn pulldown(&self) -> PULLDOWN_R {
        PULLDOWN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Pad output enable override value"]
    #[inline(always)]
    pub fn output_enable_override(&self) -> OUTPUT_ENABLE_OVERRIDE_R {
        OUTPUT_ENABLE_OVERRIDE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Pad output buffer override enable"]
    #[inline(always)]
    pub fn output_buffer_override(&self) -> OUTPUT_BUFFER_OVERRIDE_R {
        OUTPUT_BUFFER_OVERRIDE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Configuration Mode"]
    #[inline(always)]
    pub fn confmode(&mut self) -> CONFMODE_W {
        CONFMODE_W { w: self }
    }
    #[doc = "Bit 4 - Open-drain Mode"]
    #[inline(always)]
    pub fn opendrain(&mut self) -> OPENDRAIN_W {
        OPENDRAIN_W { w: self }
    }
    #[doc = "Bits 5:7 - Drive Strength"]
    #[inline(always)]
    pub fn drivestrength(&mut self) -> DRIVESTRENGTH_W {
        DRIVESTRENGTH_W { w: self }
    }
    #[doc = "Bit 8 - Enable internal weak pullup"]
    #[inline(always)]
    pub fn pullup(&mut self) -> PULLUP_W {
        PULLUP_W { w: self }
    }
    #[doc = "Bit 9 - Enable internal weak pulldown"]
    #[inline(always)]
    pub fn pulldown(&mut self) -> PULLDOWN_W {
        PULLDOWN_W { w: self }
    }
    #[doc = "Bit 10 - Pad output enable override value"]
    #[inline(always)]
    pub fn output_enable_override(&mut self) -> OUTPUT_ENABLE_OVERRIDE_W {
        OUTPUT_ENABLE_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 11 - Pad output buffer override enable"]
    #[inline(always)]
    pub fn output_buffer_override(&mut self) -> OUTPUT_BUFFER_OVERRIDE_W {
        OUTPUT_BUFFER_OVERRIDE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO20 (SPI_FLASH_DIN) (Pin 13) Pad Config\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_20](index.html) module"]
pub struct GPIO_PAD_CONFIG_20_SPEC;
impl crate::RegisterSpec for GPIO_PAD_CONFIG_20_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gpio_pad_config_20::R](R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_20_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_20::W](W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_20_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_PAD_CONFIG_20 to value 0x0c61"]
impl crate::Resettable for GPIO_PAD_CONFIG_20_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c61
    }
}
