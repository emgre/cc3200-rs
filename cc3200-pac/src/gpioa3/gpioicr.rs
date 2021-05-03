#[doc = "Register `GPIOICR` writer"]
pub struct W(crate::W<GPIOICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOICR_SPEC>;
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
impl core::convert::From<crate::W<GPIOICR_SPEC>> for W {
    fn from(writer: crate::W<GPIOICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICR` writer - GPIO Interrupt Clear"]
pub struct ICR_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO Interrupt Clear"]
    #[inline(always)]
    pub fn icr(&mut self) -> ICR_W {
        ICR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Interrupt Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioicr](index.html) module"]
pub struct GPIOICR_SPEC;
impl crate::RegisterSpec for GPIOICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gpioicr::W](W) writer structure"]
impl crate::Writable for GPIOICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOICR to value 0"]
impl crate::Resettable for GPIOICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
