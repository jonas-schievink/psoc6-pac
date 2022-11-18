#[doc = "Register `B1_DATA_REG[%s]` reader"]
pub struct R(crate::R<B1_DATA_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<B1_DATA_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<B1_DATA_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<B1_DATA_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `B1_DATA_REG[%s]` writer"]
pub struct W(crate::W<B1_DATA_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<B1_DATA_REG_SPEC>;
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
impl From<crate::W<B1_DATA_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<B1_DATA_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1_DATA` reader - Programmable B1 Data register"]
pub type B1_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `B1_DATA` writer - Programmable B1 Data register"]
pub type B1_DATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, B1_DATA_REG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Programmable B1 Data register"]
    #[inline(always)]
    pub fn b1_data(&self) -> B1_DATA_R {
        B1_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Programmable B1 Data register"]
    #[inline(always)]
    pub fn b1_data(&mut self) -> B1_DATA_W<0> {
        B1_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Programmable B1 Data register (0-3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b1_data_reg](index.html) module"]
pub struct B1_DATA_REG_SPEC;
impl crate::RegisterSpec for B1_DATA_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [b1_data_reg::R](R) reader structure"]
impl crate::Readable for B1_DATA_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [b1_data_reg::W](W) writer structure"]
impl crate::Writable for B1_DATA_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets B1_DATA_REG[%s]
to value 0"]
impl crate::Resettable for B1_DATA_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
