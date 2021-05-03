#[doc = "Register `GPIOIM` reader"]
pub struct R(crate::R<GPIOIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPIOIM_SPEC>> for R {
    fn from(reader: crate::R<GPIOIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOIM` writer"]
pub struct W(crate::W<GPIOIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOIM_SPEC>;
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
impl core::convert::From<crate::W<GPIOIM_SPEC>> for W {
    fn from(writer: crate::W<GPIOIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IM` reader - GPIO Interrupt Mask Enable"]
pub struct IM_R(crate::FieldReader<u8, u8>);
impl IM_R {
    pub(crate) fn new(bits: u8) -> Self {
        IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM` writer - GPIO Interrupt Mask Enable"]
pub struct IM_W<'a> {
    w: &'a mut W,
}
impl<'a> IM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPIO Interrupt Mask Enable"]
    #[inline(always)]
    pub fn im(&self) -> IM_R {
        IM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO Interrupt Mask Enable"]
    #[inline(always)]
    pub fn im(&mut self) -> IM_W {
        IM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Interrupt Mask Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioim](index.html) module"]
pub struct GPIOIM_SPEC;
impl crate::RegisterSpec for GPIOIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioim::R](R) reader structure"]
impl crate::Readable for GPIOIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpioim::W](W) writer structure"]
impl crate::Writable for GPIOIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOIM to value 0"]
impl crate::Resettable for GPIOIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
