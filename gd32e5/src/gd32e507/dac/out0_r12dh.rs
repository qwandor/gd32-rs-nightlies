#[doc = "Register `OUT0_R12DH` reader"]
pub type R = crate::R<OUT0_R12DH_SPEC>;
#[doc = "Register `OUT0_R12DH` writer"]
pub type W = crate::W<OUT0_R12DH_SPEC>;
#[doc = "Field `OUT0_DH` reader - DAC_OUT0 12-bit right-aligned data"]
pub type OUT0_DH_R = crate::FieldReader<u16>;
#[doc = "Field `OUT0_DH` writer - DAC_OUT0 12-bit right-aligned data"]
pub type OUT0_DH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC_OUT0 12-bit right-aligned data"]
    #[inline(always)]
    pub fn out0_dh(&self) -> OUT0_DH_R {
        OUT0_DH_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC_OUT0 12-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn out0_dh(&mut self) -> OUT0_DH_W<OUT0_R12DH_SPEC, 0> {
        OUT0_DH_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DAC_OUT0 12-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out0_r12dh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out0_r12dh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT0_R12DH_SPEC;
impl crate::RegisterSpec for OUT0_R12DH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out0_r12dh::R`](R) reader structure"]
impl crate::Readable for OUT0_R12DH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out0_r12dh::W`](W) writer structure"]
impl crate::Writable for OUT0_R12DH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT0_R12DH to value 0"]
impl crate::Resettable for OUT0_R12DH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}