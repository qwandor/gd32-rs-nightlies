#[doc = "Register `PCF1` reader"]
pub type R = crate::R<Pcf1Spec>;
#[doc = "Register `PCF1` writer"]
pub type W = crate::W<Pcf1Spec>;
#[doc = "Field `TIMER8_REMAP` reader - TIMER8 remapping"]
pub type Timer8RemapR = crate::BitReader;
#[doc = "Field `TIMER8_REMAP` writer - TIMER8 remapping"]
pub type Timer8RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXMC_NADV` reader - EXMC_NADV connect/disconnect"]
pub type ExmcNadvR = crate::BitReader;
#[doc = "Field `EXMC_NADV` writer - EXMC_NADV connect/disconnect"]
pub type ExmcNadvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTC_REMAP` reader - CTC remapping"]
pub type CtcRemapR = crate::FieldReader;
#[doc = "Field `CTC_REMAP` writer - CTC remapping"]
pub type CtcRemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 5 - TIMER8 remapping"]
    #[inline(always)]
    pub fn timer8_remap(&self) -> Timer8RemapR {
        Timer8RemapR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - EXMC_NADV connect/disconnect"]
    #[inline(always)]
    pub fn exmc_nadv(&self) -> ExmcNadvR {
        ExmcNadvR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - CTC remapping"]
    #[inline(always)]
    pub fn ctc_remap(&self) -> CtcRemapR {
        CtcRemapR::new(((self.bits >> 11) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - TIMER8 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn timer8_remap(&mut self) -> Timer8RemapW<Pcf1Spec> {
        Timer8RemapW::new(self, 5)
    }
    #[doc = "Bit 10 - EXMC_NADV connect/disconnect"]
    #[inline(always)]
    #[must_use]
    pub fn exmc_nadv(&mut self) -> ExmcNadvW<Pcf1Spec> {
        ExmcNadvW::new(self, 10)
    }
    #[doc = "Bits 11:12 - CTC remapping"]
    #[inline(always)]
    #[must_use]
    pub fn ctc_remap(&mut self) -> CtcRemapW<Pcf1Spec> {
        CtcRemapW::new(self, 11)
    }
}
#[doc = "AFIO port configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcf1Spec;
impl crate::RegisterSpec for Pcf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcf1::R`](R) reader structure"]
impl crate::Readable for Pcf1Spec {}
#[doc = "`write(|w| ..)` method takes [`pcf1::W`](W) writer structure"]
impl crate::Writable for Pcf1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCF1 to value 0"]
impl crate::Resettable for Pcf1Spec {
    const RESET_VALUE: u32 = 0;
}
