#[doc = "Register `WDTVALUE` reader"]
pub struct R(crate::R<WDTVALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTVALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WDTVALUE_SPEC>> for R {
    fn from(reader: crate::R<WDTVALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDTVALUE` reader - Watchdog Value"]
pub struct WDTVALUE_R(crate::FieldReader<u32, u32>);
impl WDTVALUE_R {
    pub(crate) fn new(bits: u32) -> Self {
        WDTVALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTVALUE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Watchdog Value"]
    #[inline(always)]
    pub fn wdtvalue(&self) -> WDTVALUE_R {
        WDTVALUE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Watchdog Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtvalue](index.html) module"]
pub struct WDTVALUE_SPEC;
impl crate::RegisterSpec for WDTVALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtvalue::R](R) reader structure"]
impl crate::Readable for WDTVALUE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WDTVALUE to value 0xffff_ffff"]
impl crate::Resettable for WDTVALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
