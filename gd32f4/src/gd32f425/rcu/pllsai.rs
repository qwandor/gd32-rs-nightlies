#[doc = "Register `PLLSAI` reader"]
pub type R = crate::R<PllsaiSpec>;
#[doc = "Register `PLLSAI` writer"]
pub type W = crate::W<PllsaiSpec>;
#[doc = "Field `PLLSAIN` reader - The PLLSAI VCO clock multi factor"]
pub type PllsainR = crate::FieldReader<u16>;
#[doc = "Field `PLLSAIN` writer - The PLLSAI VCO clock multi factor"]
pub type PllsainW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PLLSAIP` reader - The PLLSAI P output frequency division factor from PLLSAI VCO clock"]
pub type PllsaipR = crate::FieldReader;
#[doc = "Field `PLLSAIP` writer - The PLLSAI P output frequency division factor from PLLSAI VCO clock"]
pub type PllsaipW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLLSAIQ` reader - The PLLI2S Q output frequency division factor from PLLI2S VCO clock"]
pub type PllsaiqR = crate::FieldReader;
#[doc = "Field `PLLSAIQ` writer - The PLLI2S Q output frequency division factor from PLLI2S VCO clock"]
pub type PllsaiqW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PLLSAIR` reader - The PLLSAI R output frequency division factor from PLLSAI VCO clock"]
pub type PllsairR = crate::FieldReader;
#[doc = "Field `PLLSAIR` writer - The PLLSAI R output frequency division factor from PLLSAI VCO clock"]
pub type PllsairW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 6:14 - The PLLSAI VCO clock multi factor"]
    #[inline(always)]
    pub fn pllsain(&self) -> PllsainR {
        PllsainR::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:17 - The PLLSAI P output frequency division factor from PLLSAI VCO clock"]
    #[inline(always)]
    pub fn pllsaip(&self) -> PllsaipR {
        PllsaipR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:27 - The PLLI2S Q output frequency division factor from PLLI2S VCO clock"]
    #[inline(always)]
    pub fn pllsaiq(&self) -> PllsaiqR {
        PllsaiqR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - The PLLSAI R output frequency division factor from PLLSAI VCO clock"]
    #[inline(always)]
    pub fn pllsair(&self) -> PllsairR {
        PllsairR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 6:14 - The PLLSAI VCO clock multi factor"]
    #[inline(always)]
    #[must_use]
    pub fn pllsain(&mut self) -> PllsainW<PllsaiSpec> {
        PllsainW::new(self, 6)
    }
    #[doc = "Bits 16:17 - The PLLSAI P output frequency division factor from PLLSAI VCO clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllsaip(&mut self) -> PllsaipW<PllsaiSpec> {
        PllsaipW::new(self, 16)
    }
    #[doc = "Bits 24:27 - The PLLI2S Q output frequency division factor from PLLI2S VCO clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllsaiq(&mut self) -> PllsaiqW<PllsaiSpec> {
        PllsaiqW::new(self, 24)
    }
    #[doc = "Bits 28:30 - The PLLSAI R output frequency division factor from PLLSAI VCO clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllsair(&mut self) -> PllsairW<PllsaiSpec> {
        PllsairW::new(self, 28)
    }
}
#[doc = "PLLSAI register (RCU_PLLSAI)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllsai::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllsai::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllsaiSpec;
impl crate::RegisterSpec for PllsaiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllsai::R`](R) reader structure"]
impl crate::Readable for PllsaiSpec {}
#[doc = "`write(|w| ..)` method takes [`pllsai::W`](W) writer structure"]
impl crate::Writable for PllsaiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLSAI to value 0x016e_41c2"]
impl crate::Resettable for PllsaiSpec {
    const RESET_VALUE: u32 = 0x016e_41c2;
}
