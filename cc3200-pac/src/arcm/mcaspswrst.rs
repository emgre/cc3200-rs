#[doc = "Register `MCASPSWRST` reader"]
pub struct R(crate::R<MCASPSWRST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCASPSWRST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MCASPSWRST_SPEC>> for R {
    fn from(reader: crate::R<MCASPSWRST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCASPSWRST` writer"]
pub struct W(crate::W<MCASPSWRST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCASPSWRST_SPEC>;
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
impl core::convert::From<crate::W<MCASPSWRST_SPEC>> for W {
    fn from(writer: crate::W<MCASPSWRST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENSTS` reader - Enable Status"]
pub struct ENSTS_R(crate::FieldReader<bool, bool>);
impl ENSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRST` reader - Software Reset"]
pub struct SWRST_R(crate::FieldReader<bool, bool>);
impl SWRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRST` writer - Software Reset"]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Enable Status"]
    #[inline(always)]
    pub fn ensts(&self) -> ENSTS_R {
        ENSTS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcaspswrst](index.html) module"]
pub struct MCASPSWRST_SPEC;
impl crate::RegisterSpec for MCASPSWRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcaspswrst::R](R) reader structure"]
impl crate::Readable for MCASPSWRST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcaspswrst::W](W) writer structure"]
impl crate::Writable for MCASPSWRST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCASPSWRST to value 0"]
impl crate::Resettable for MCASPSWRST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
