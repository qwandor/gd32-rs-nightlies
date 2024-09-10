#[doc = "Register `SDRSCTL` reader"]
pub type R = crate::R<SdrsctlSpec>;
#[doc = "Register `SDRSCTL` writer"]
pub type W = crate::W<SdrsctlSpec>;
#[doc = "Field `RSEN` reader - Read sample enable"]
pub type RsenR = crate::BitReader;
#[doc = "Field `RSEN` writer - Read sample enable"]
pub type RsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSCR` reader - Select sample cycle of read data"]
pub type SscrR = crate::BitReader;
#[doc = "Field `SSCR` writer - Select sample cycle of read data"]
pub type SscrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDSC` reader - Select the delayed sample clock of read data"]
pub type SdscR = crate::FieldReader;
#[doc = "Field `SDSC` writer - Select the delayed sample clock of read data"]
pub type SdscW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Read sample enable"]
    #[inline(always)]
    pub fn rsen(&self) -> RsenR {
        RsenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select sample cycle of read data"]
    #[inline(always)]
    pub fn sscr(&self) -> SscrR {
        SscrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Select the delayed sample clock of read data"]
    #[inline(always)]
    pub fn sdsc(&self) -> SdscR {
        SdscR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Read sample enable"]
    #[inline(always)]
    #[must_use]
    pub fn rsen(&mut self) -> RsenW<SdrsctlSpec> {
        RsenW::new(self, 0)
    }
    #[doc = "Bit 1 - Select sample cycle of read data"]
    #[inline(always)]
    #[must_use]
    pub fn sscr(&mut self) -> SscrW<SdrsctlSpec> {
        SscrW::new(self, 1)
    }
    #[doc = "Bits 4:7 - Select the delayed sample clock of read data"]
    #[inline(always)]
    #[must_use]
    pub fn sdsc(&mut self) -> SdscW<SdrsctlSpec> {
        SdscW::new(self, 4)
    }
}
#[doc = "SDRAM read sample control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdrsctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdrsctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdrsctlSpec;
impl crate::RegisterSpec for SdrsctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdrsctl::R`](R) reader structure"]
impl crate::Readable for SdrsctlSpec {}
#[doc = "`write(|w| ..)` method takes [`sdrsctl::W`](W) writer structure"]
impl crate::Writable for SdrsctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDRSCTL to value 0"]
impl crate::Resettable for SdrsctlSpec {
    const RESET_VALUE: u32 = 0;
}
