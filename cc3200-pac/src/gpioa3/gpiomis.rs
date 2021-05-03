#[doc = "Register `GPIOMIS` reader"]
pub struct R(crate::R<GPIOMIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOMIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPIOMIS_SPEC>> for R {
    fn from(reader: crate::R<GPIOMIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MIS` reader - GPIO Masked Interrupt Status"]
pub struct MIS_R(crate::FieldReader<u8, u8>);
impl MIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        MIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - GPIO Masked Interrupt Status"]
    #[inline(always)]
    pub fn mis(&self) -> MIS_R {
        MIS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "GPIO Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiomis](index.html) module"]
pub struct GPIOMIS_SPEC;
impl crate::RegisterSpec for GPIOMIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiomis::R](R) reader structure"]
impl crate::Readable for GPIOMIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIOMIS to value 0"]
impl crate::Resettable for GPIOMIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
