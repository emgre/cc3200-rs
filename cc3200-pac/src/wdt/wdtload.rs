#[doc = "Register `WDTLOAD` reader"]
pub struct R(crate::R<WDTLOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTLOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WDTLOAD_SPEC>> for R {
    fn from(reader: crate::R<WDTLOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTLOAD` writer"]
pub struct W(crate::W<WDTLOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTLOAD_SPEC>;
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
impl core::convert::From<crate::W<WDTLOAD_SPEC>> for W {
    fn from(writer: crate::W<WDTLOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTLOAD` reader - Watchdog Load Value"]
pub struct WDTLOAD_R(crate::FieldReader<u32, u32>);
impl WDTLOAD_R {
    pub(crate) fn new(bits: u32) -> Self {
        WDTLOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTLOAD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTLOAD` writer - Watchdog Load Value"]
pub struct WDTLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTLOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Watchdog Load Value"]
    #[inline(always)]
    pub fn wdtload(&self) -> WDTLOAD_R {
        WDTLOAD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Watchdog Load Value"]
    #[inline(always)]
    pub fn wdtload(&mut self) -> WDTLOAD_W {
        WDTLOAD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Load\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtload](index.html) module"]
pub struct WDTLOAD_SPEC;
impl crate::RegisterSpec for WDTLOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtload::R](R) reader structure"]
impl crate::Readable for WDTLOAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtload::W](W) writer structure"]
impl crate::Writable for WDTLOAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDTLOAD to value 0xffff_ffff"]
impl crate::Resettable for WDTLOAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
