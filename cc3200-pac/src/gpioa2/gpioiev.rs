#[doc = "Register `GPIOIEV` reader"]
pub struct R(crate::R<GPIOIEV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOIEV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPIOIEV_SPEC>> for R {
    fn from(reader: crate::R<GPIOIEV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOIEV` writer"]
pub struct W(crate::W<GPIOIEV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOIEV_SPEC>;
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
impl core::convert::From<crate::W<GPIOIEV_SPEC>> for W {
    fn from(writer: crate::W<GPIOIEV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IEV` reader - GPIO Interrupt Event"]
pub struct IEV_R(crate::FieldReader<u8, u8>);
impl IEV_R {
    pub(crate) fn new(bits: u8) -> Self {
        IEV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEV` writer - GPIO Interrupt Event"]
pub struct IEV_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPIO Interrupt Event"]
    #[inline(always)]
    pub fn iev(&self) -> IEV_R {
        IEV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO Interrupt Event"]
    #[inline(always)]
    pub fn iev(&mut self) -> IEV_W {
        IEV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Interrupt Event\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioiev](index.html) module"]
pub struct GPIOIEV_SPEC;
impl crate::RegisterSpec for GPIOIEV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioiev::R](R) reader structure"]
impl crate::Readable for GPIOIEV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpioiev::W](W) writer structure"]
impl crate::Writable for GPIOIEV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOIEV to value 0"]
impl crate::Resettable for GPIOIEV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
