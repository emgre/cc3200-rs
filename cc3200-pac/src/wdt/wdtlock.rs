#[doc = "Register `WDTLOCK` reader"]
pub struct R(crate::R<WDTLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WDTLOCK_SPEC>> for R {
    fn from(reader: crate::R<WDTLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTLOCK` writer"]
pub struct W(crate::W<WDTLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTLOCK_SPEC>;
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
impl core::convert::From<crate::W<WDTLOCK_SPEC>> for W {
    fn from(writer: crate::W<WDTLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTLOCK` reader - Watchdog Lock"]
pub struct WDTLOCK_R(crate::FieldReader<u32, u32>);
impl WDTLOCK_R {
    pub(crate) fn new(bits: u32) -> Self {
        WDTLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTLOCK_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTLOCK` writer - Watchdog Lock"]
pub struct WDTLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Watchdog Lock"]
    #[inline(always)]
    pub fn wdtlock(&self) -> WDTLOCK_R {
        WDTLOCK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Watchdog Lock"]
    #[inline(always)]
    pub fn wdtlock(&mut self) -> WDTLOCK_W {
        WDTLOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Lock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtlock](index.html) module"]
pub struct WDTLOCK_SPEC;
impl crate::RegisterSpec for WDTLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtlock::R](R) reader structure"]
impl crate::Readable for WDTLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtlock::W](W) writer structure"]
impl crate::Writable for WDTLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDTLOCK to value 0"]
impl crate::Resettable for WDTLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
