#[doc = "Register `GPIODATA` reader"]
pub struct R(crate::R<GPIODATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIODATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPIODATA_SPEC>> for R {
    fn from(reader: crate::R<GPIODATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIODATA` writer"]
pub struct W(crate::W<GPIODATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIODATA_SPEC>;
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
impl core::convert::From<crate::W<GPIODATA_SPEC>> for W {
    fn from(writer: crate::W<GPIODATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - GPIO Data"]
pub struct DATA_R(crate::FieldReader<u8, u8>);
impl DATA_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA` writer - GPIO Data"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPIO Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiodata](index.html) module"]
pub struct GPIODATA_SPEC;
impl crate::RegisterSpec for GPIODATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiodata::R](R) reader structure"]
impl crate::Readable for GPIODATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpiodata::W](W) writer structure"]
impl crate::Writable for GPIODATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIODATA to value 0"]
impl crate::Resettable for GPIODATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
