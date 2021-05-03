#[doc = "Register `GPIORIS` reader"]
pub struct R(crate::R<GPIORIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIORIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPIORIS_SPEC>> for R {
    fn from(reader: crate::R<GPIORIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIORIS` writer"]
pub struct W(crate::W<GPIORIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIORIS_SPEC>;
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
impl core::convert::From<crate::W<GPIORIS_SPEC>> for W {
    fn from(writer: crate::W<GPIORIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RIS` reader - GPIO Interrupt Raw Status"]
pub struct RIS_R(crate::FieldReader<u8, u8>);
impl RIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RIS` writer - GPIO Interrupt Raw Status"]
pub struct RIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPIO Interrupt Raw Status"]
    #[inline(always)]
    pub fn ris(&self) -> RIS_R {
        RIS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO Interrupt Raw Status"]
    #[inline(always)]
    pub fn ris(&mut self) -> RIS_W {
        RIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Interrupt Raw Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioris](index.html) module"]
pub struct GPIORIS_SPEC;
impl crate::RegisterSpec for GPIORIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioris::R](R) reader structure"]
impl crate::Readable for GPIORIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpioris::W](W) writer structure"]
impl crate::Writable for GPIORIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIORIS to value 0"]
impl crate::Resettable for GPIORIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
