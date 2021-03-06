#[doc = "Register `WDTICR` writer"]
pub struct W(crate::W<WDTICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTICR_SPEC>;
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
impl core::convert::From<crate::W<WDTICR_SPEC>> for W {
    fn from(writer: crate::W<WDTICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTINTCLR` writer - Watchdog Interrupt Clear"]
pub struct WDTINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTINTCLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Watchdog Interrupt Clear"]
    #[inline(always)]
    pub fn wdtintclr(&mut self) -> WDTINTCLR_W {
        WDTINTCLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Interrupt Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdticr](index.html) module"]
pub struct WDTICR_SPEC;
impl crate::RegisterSpec for WDTICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wdticr::W](W) writer structure"]
impl crate::Writable for WDTICR_SPEC {
    type Writer = W;
}
