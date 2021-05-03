#[doc = "Register `GPT3CLKEN` reader"]
pub struct R(crate::R<GPT3CLKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPT3CLKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPT3CLKEN_SPEC>> for R {
    fn from(reader: crate::R<GPT3CLKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPT3CLKEN` writer"]
pub struct W(crate::W<GPT3CLKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPT3CLKEN_SPEC>;
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
impl core::convert::From<crate::W<GPT3CLKEN_SPEC>> for W {
    fn from(writer: crate::W<GPT3CLKEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSLPCLKEN` reader - Deepsleep Clock Enable"]
pub struct DSLPCLKEN_R(crate::FieldReader<bool, bool>);
impl DSLPCLKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSLPCLKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSLPCLKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSLPCLKEN` writer - Deepsleep Clock Enable"]
pub struct DSLPCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSLPCLKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `SLPCLKEN` reader - Sleep Clock Enable"]
pub struct SLPCLKEN_R(crate::FieldReader<bool, bool>);
impl SLPCLKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLPCLKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLPCLKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLPCLKEN` writer - Sleep Clock Enable"]
pub struct SLPCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLPCLKEN_W<'a> {
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
#[doc = "Field `RUNCLKEN` reader - Run Clock Enable"]
pub struct RUNCLKEN_R(crate::FieldReader<bool, bool>);
impl RUNCLKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RUNCLKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUNCLKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUNCLKEN` writer - Run Clock Enable"]
pub struct RUNCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNCLKEN_W<'a> {
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
    #[doc = "Bit 16 - Deepsleep Clock Enable"]
    #[inline(always)]
    pub fn dslpclken(&self) -> DSLPCLKEN_R {
        DSLPCLKEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Sleep Clock Enable"]
    #[inline(always)]
    pub fn slpclken(&self) -> SLPCLKEN_R {
        SLPCLKEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Run Clock Enable"]
    #[inline(always)]
    pub fn runclken(&self) -> RUNCLKEN_R {
        RUNCLKEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Deepsleep Clock Enable"]
    #[inline(always)]
    pub fn dslpclken(&mut self) -> DSLPCLKEN_W {
        DSLPCLKEN_W { w: self }
    }
    #[doc = "Bit 8 - Sleep Clock Enable"]
    #[inline(always)]
    pub fn slpclken(&mut self) -> SLPCLKEN_W {
        SLPCLKEN_W { w: self }
    }
    #[doc = "Bit 0 - Run Clock Enable"]
    #[inline(always)]
    pub fn runclken(&mut self) -> RUNCLKEN_W {
        RUNCLKEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPT3 Clock Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpt3clken](index.html) module"]
pub struct GPT3CLKEN_SPEC;
impl crate::RegisterSpec for GPT3CLKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpt3clken::R](R) reader structure"]
impl crate::Readable for GPT3CLKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpt3clken::W](W) writer structure"]
impl crate::Writable for GPT3CLKEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPT3CLKEN to value 0"]
impl crate::Resettable for GPT3CLKEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
