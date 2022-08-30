#[doc = "Register `VREF_TRIM1` reader"]
pub struct R(crate::R<VREF_TRIM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREF_TRIM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREF_TRIM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREF_TRIM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREF_TRIM1` writer"]
pub struct W(crate::W<VREF_TRIM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREF_TRIM1_SPEC>;
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
impl From<crate::W<VREF_TRIM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREF_TRIM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VREF_TEMPCO_TRIM` reader - N/A"]
pub type VREF_TEMPCO_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREF_TEMPCO_TRIM` writer - N/A"]
pub type VREF_TEMPCO_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, VREF_TRIM1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn vref_tempco_trim(&self) -> VREF_TEMPCO_TRIM_R {
        VREF_TEMPCO_TRIM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn vref_tempco_trim(&mut self) -> VREF_TEMPCO_TRIM_W<0> {
        VREF_TEMPCO_TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VREF Trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vref_trim1](index.html) module"]
pub struct VREF_TRIM1_SPEC;
impl crate::RegisterSpec for VREF_TRIM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vref_trim1::R](R) reader structure"]
impl crate::Readable for VREF_TRIM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vref_trim1::W](W) writer structure"]
impl crate::Writable for VREF_TRIM1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VREF_TRIM1 to value 0"]
impl crate::Resettable for VREF_TRIM1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
