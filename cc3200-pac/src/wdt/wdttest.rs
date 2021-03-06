#[doc = "Register `WDTTEST` reader"]
pub struct R(crate::R<WDTTEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTTEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WDTTEST_SPEC>> for R {
    fn from(reader: crate::R<WDTTEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTTEST` writer"]
pub struct W(crate::W<WDTTEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTTEST_SPEC>;
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
impl core::convert::From<crate::W<WDTTEST_SPEC>> for W {
    fn from(writer: crate::W<WDTTEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STALL` reader - Watchdog Stall Enable"]
pub struct STALL_R(crate::FieldReader<bool, bool>);
impl STALL_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL` writer - Watchdog Stall Enable"]
pub struct STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - Watchdog Stall Enable"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Watchdog Stall Enable"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Test Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdttest](index.html) module"]
pub struct WDTTEST_SPEC;
impl crate::RegisterSpec for WDTTEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdttest::R](R) reader structure"]
impl crate::Readable for WDTTEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdttest::W](W) writer structure"]
impl crate::Writable for WDTTEST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDTTEST to value 0"]
impl crate::Resettable for WDTTEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
