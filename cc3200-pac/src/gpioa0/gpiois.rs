#[doc = "Register `GPIOIS` reader"]
pub struct R(crate::R<GPIOIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPIOIS_SPEC>> for R {
    fn from(reader: crate::R<GPIOIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOIS` writer"]
pub struct W(crate::W<GPIOIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOIS_SPEC>;
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
impl core::convert::From<crate::W<GPIOIS_SPEC>> for W {
    fn from(writer: crate::W<GPIOIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IS` reader - GPIO Interrupt Sense"]
pub struct IS_R(crate::FieldReader<u8, u8>);
impl IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS` writer - GPIO Interrupt Sense"]
pub struct IS_W<'a> {
    w: &'a mut W,
}
impl<'a> IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPIO Interrupt Sense"]
    #[inline(always)]
    pub fn is(&self) -> IS_R {
        IS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO Interrupt Sense"]
    #[inline(always)]
    pub fn is(&mut self) -> IS_W {
        IS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Interrupt Sense\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiois](index.html) module"]
pub struct GPIOIS_SPEC;
impl crate::RegisterSpec for GPIOIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiois::R](R) reader structure"]
impl crate::Readable for GPIOIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpiois::W](W) writer structure"]
impl crate::Writable for GPIOIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOIS to value 0"]
impl crate::Resettable for GPIOIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
