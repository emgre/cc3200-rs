#[doc = "Register `GPIODIR` reader"]
pub struct R(crate::R<GPIODIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIODIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPIODIR_SPEC>> for R {
    fn from(reader: crate::R<GPIODIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIODIR` writer"]
pub struct W(crate::W<GPIODIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIODIR_SPEC>;
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
impl core::convert::From<crate::W<GPIODIR_SPEC>> for W {
    fn from(writer: crate::W<GPIODIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIR` reader - GPIO Data Direction"]
pub struct DIR_R(crate::FieldReader<u8, u8>);
impl DIR_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIR` writer - GPIO Data Direction"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPIO Data Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO Data Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Data Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiodir](index.html) module"]
pub struct GPIODIR_SPEC;
impl crate::RegisterSpec for GPIODIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiodir::R](R) reader structure"]
impl crate::Readable for GPIODIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpiodir::W](W) writer structure"]
impl crate::Writable for GPIODIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIODIR to value 0"]
impl crate::Resettable for GPIODIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
