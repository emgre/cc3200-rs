#[doc = "Register `WDTRIS` reader"]
pub struct R(crate::R<WDTRIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTRIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WDTRIS_SPEC>> for R {
    fn from(reader: crate::R<WDTRIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDTRIS` reader - Watchdog Raw Interrupt Status"]
pub struct WDTRIS_R(crate::FieldReader<bool, bool>);
impl WDTRIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDTRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog Raw Interrupt Status"]
    #[inline(always)]
    pub fn wdtris(&self) -> WDTRIS_R {
        WDTRIS_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Watchdog Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtris](index.html) module"]
pub struct WDTRIS_SPEC;
impl crate::RegisterSpec for WDTRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtris::R](R) reader structure"]
impl crate::Readable for WDTRIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WDTRIS to value 0"]
impl crate::Resettable for WDTRIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
