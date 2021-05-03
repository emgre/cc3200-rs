#[doc = "Register `GPIOIBE` reader"]
pub struct R(crate::R<GPIOIBE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOIBE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPIOIBE_SPEC>> for R {
    fn from(reader: crate::R<GPIOIBE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOIBE` writer"]
pub struct W(crate::W<GPIOIBE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOIBE_SPEC>;
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
impl core::convert::From<crate::W<GPIOIBE_SPEC>> for W {
    fn from(writer: crate::W<GPIOIBE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IBE` reader - GPIO Interrupt Both Edges"]
pub struct IBE_R(crate::FieldReader<u8, u8>);
impl IBE_R {
    pub(crate) fn new(bits: u8) -> Self {
        IBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBE` writer - GPIO Interrupt Both Edges"]
pub struct IBE_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPIO Interrupt Both Edges"]
    #[inline(always)]
    pub fn ibe(&self) -> IBE_R {
        IBE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO Interrupt Both Edges"]
    #[inline(always)]
    pub fn ibe(&mut self) -> IBE_W {
        IBE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Interrupt Both Edges\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioibe](index.html) module"]
pub struct GPIOIBE_SPEC;
impl crate::RegisterSpec for GPIOIBE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioibe::R](R) reader structure"]
impl crate::Readable for GPIOIBE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpioibe::W](W) writer structure"]
impl crate::Writable for GPIOIBE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOIBE to value 0"]
impl crate::Resettable for GPIOIBE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
