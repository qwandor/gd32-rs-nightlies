#[doc = "Register `GUSBCS` reader"]
pub type R = crate::R<GusbcsSpec>;
#[doc = "Register `GUSBCS` writer"]
pub type W = crate::W<GusbcsSpec>;
#[doc = "Field `TOC` reader - timeout calibration"]
pub type TocR = crate::FieldReader;
#[doc = "Field `TOC` writer - timeout calibration"]
pub type TocW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EMBPHY` reader - Embedded PHY selected"]
pub type EmbphyR = crate::BitReader;
#[doc = "Field `EMBPHY` writer - Embedded PHY selected"]
pub type EmbphyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRPCEN` reader - SRP capability enable"]
pub type SrpcenR = crate::BitReader;
#[doc = "Field `SRPCEN` writer - SRP capability enable"]
pub type SrpcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNPCEN` reader - HNP capability enable"]
pub type HnpcenR = crate::BitReader;
#[doc = "Field `HNPCEN` writer - HNP capability enable"]
pub type HnpcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTT` reader - USB turnaround time"]
pub type UttR = crate::FieldReader;
#[doc = "Field `UTT` writer - USB turnaround time"]
pub type UttW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ULPIEVD` reader - ULPI external VBUS driver"]
pub type UlpievdR = crate::BitReader;
#[doc = "Field `ULPIEVD` writer - ULPI external VBUS driver"]
pub type UlpievdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULPIEOI` reader - ULPI external over current indicator"]
pub type UlpieoiR = crate::BitReader;
#[doc = "Field `ULPIEOI` writer - ULPI external over current indicator"]
pub type UlpieoiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FHM` reader - Force host mode"]
pub type FhmR = crate::BitReader;
#[doc = "Field `FHM` writer - Force host mode"]
pub type FhmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDM` reader - Force device mode"]
pub type FdmR = crate::BitReader;
#[doc = "Field `FDM` writer - Force device mode"]
pub type FdmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - timeout calibration"]
    #[inline(always)]
    pub fn toc(&self) -> TocR {
        TocR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - Embedded PHY selected"]
    #[inline(always)]
    pub fn embphy(&self) -> EmbphyR {
        EmbphyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - SRP capability enable"]
    #[inline(always)]
    pub fn srpcen(&self) -> SrpcenR {
        SrpcenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP capability enable"]
    #[inline(always)]
    pub fn hnpcen(&self) -> HnpcenR {
        HnpcenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - USB turnaround time"]
    #[inline(always)]
    pub fn utt(&self) -> UttR {
        UttR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - ULPI external VBUS driver"]
    #[inline(always)]
    pub fn ulpievd(&self) -> UlpievdR {
        UlpievdR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ULPI external over current indicator"]
    #[inline(always)]
    pub fn ulpieoi(&self) -> UlpieoiR {
        UlpieoiR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 29 - Force host mode"]
    #[inline(always)]
    pub fn fhm(&self) -> FhmR {
        FhmR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Force device mode"]
    #[inline(always)]
    pub fn fdm(&self) -> FdmR {
        FdmR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - timeout calibration"]
    #[inline(always)]
    #[must_use]
    pub fn toc(&mut self) -> TocW<GusbcsSpec> {
        TocW::new(self, 0)
    }
    #[doc = "Bit 6 - Embedded PHY selected"]
    #[inline(always)]
    #[must_use]
    pub fn embphy(&mut self) -> EmbphyW<GusbcsSpec> {
        EmbphyW::new(self, 6)
    }
    #[doc = "Bit 8 - SRP capability enable"]
    #[inline(always)]
    #[must_use]
    pub fn srpcen(&mut self) -> SrpcenW<GusbcsSpec> {
        SrpcenW::new(self, 8)
    }
    #[doc = "Bit 9 - HNP capability enable"]
    #[inline(always)]
    #[must_use]
    pub fn hnpcen(&mut self) -> HnpcenW<GusbcsSpec> {
        HnpcenW::new(self, 9)
    }
    #[doc = "Bits 10:13 - USB turnaround time"]
    #[inline(always)]
    #[must_use]
    pub fn utt(&mut self) -> UttW<GusbcsSpec> {
        UttW::new(self, 10)
    }
    #[doc = "Bit 20 - ULPI external VBUS driver"]
    #[inline(always)]
    #[must_use]
    pub fn ulpievd(&mut self) -> UlpievdW<GusbcsSpec> {
        UlpievdW::new(self, 20)
    }
    #[doc = "Bit 21 - ULPI external over current indicator"]
    #[inline(always)]
    #[must_use]
    pub fn ulpieoi(&mut self) -> UlpieoiW<GusbcsSpec> {
        UlpieoiW::new(self, 21)
    }
    #[doc = "Bit 29 - Force host mode"]
    #[inline(always)]
    #[must_use]
    pub fn fhm(&mut self) -> FhmW<GusbcsSpec> {
        FhmW::new(self, 29)
    }
    #[doc = "Bit 30 - Force device mode"]
    #[inline(always)]
    #[must_use]
    pub fn fdm(&mut self) -> FdmW<GusbcsSpec> {
        FdmW::new(self, 30)
    }
}
#[doc = "USB configuration register (GUSBCS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusbcs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GusbcsSpec;
impl crate::RegisterSpec for GusbcsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gusbcs::R`](R) reader structure"]
impl crate::Readable for GusbcsSpec {}
#[doc = "`write(|w| ..)` method takes [`gusbcs::W`](W) writer structure"]
impl crate::Writable for GusbcsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GUSBCS to value 0x0a00"]
impl crate::Resettable for GusbcsSpec {
    const RESET_VALUE: u32 = 0x0a00;
}
