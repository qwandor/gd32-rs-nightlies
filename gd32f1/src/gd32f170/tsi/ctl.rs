#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `TSIEN` reader - TSI enable"]
pub type TsienR = crate::BitReader;
#[doc = "Field `TSIEN` writer - TSI enable"]
pub type TsienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIS` reader - TSI start"]
pub type TsisR = crate::BitReader;
#[doc = "Field `TSIS` writer - TSI start"]
pub type TsisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGMOD` reader - Trigger mode selection"]
pub type TrgmodR = crate::BitReader;
#[doc = "Field `TRGMOD` writer - Trigger mode selection"]
pub type TrgmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EGSEL` reader - Edge selection"]
pub type EgselR = crate::BitReader;
#[doc = "Field `EGSEL` writer - Edge selection"]
pub type EgselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINMOD` reader - Pin mode"]
pub type PinmodR = crate::BitReader;
#[doc = "Field `PINMOD` writer - Pin mode"]
pub type PinmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCN` reader - Max cycle number of a sequence"]
pub type McnR = crate::FieldReader;
#[doc = "Field `MCN` writer - Max cycle number of a sequence"]
pub type McnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CTCDIV` reader - CTCLK clock division factor"]
pub type CtcdivR = crate::FieldReader;
#[doc = "Field `CTCDIV` writer - CTCLK clock division factor"]
pub type CtcdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ECDIV` reader - ECCLK clock division factor"]
pub type EcdivR = crate::BitReader;
#[doc = "Field `ECDIV` writer - ECCLK clock division factor"]
pub type EcdivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECEN` reader - Extend Charge State Enable"]
pub type EcenR = crate::BitReader;
#[doc = "Field `ECEN` writer - Extend Charge State Enable"]
pub type EcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECDT` reader - Extend Charge State Maximum Duration Time"]
pub type EcdtR = crate::FieldReader;
#[doc = "Field `ECDT` writer - Extend Charge State Maximum Duration Time"]
pub type EcdtW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CTDT` reader - Charge Transfer State Duration Time"]
pub type CtdtR = crate::FieldReader;
#[doc = "Field `CTDT` writer - Charge Transfer State Duration Time"]
pub type CtdtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CDT` reader - Charge State Duration Time"]
pub type CdtR = crate::FieldReader;
#[doc = "Field `CDT` writer - Charge State Duration Time"]
pub type CdtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - TSI enable"]
    #[inline(always)]
    pub fn tsien(&self) -> TsienR {
        TsienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TSI start"]
    #[inline(always)]
    pub fn tsis(&self) -> TsisR {
        TsisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trigger mode selection"]
    #[inline(always)]
    pub fn trgmod(&self) -> TrgmodR {
        TrgmodR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Edge selection"]
    #[inline(always)]
    pub fn egsel(&self) -> EgselR {
        EgselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin mode"]
    #[inline(always)]
    pub fn pinmod(&self) -> PinmodR {
        PinmodR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Max cycle number of a sequence"]
    #[inline(always)]
    pub fn mcn(&self) -> McnR {
        McnR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 12:14 - CTCLK clock division factor"]
    #[inline(always)]
    pub fn ctcdiv(&self) -> CtcdivR {
        CtcdivR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - ECCLK clock division factor"]
    #[inline(always)]
    pub fn ecdiv(&self) -> EcdivR {
        EcdivR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Extend Charge State Enable"]
    #[inline(always)]
    pub fn ecen(&self) -> EcenR {
        EcenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - Extend Charge State Maximum Duration Time"]
    #[inline(always)]
    pub fn ecdt(&self) -> EcdtR {
        EcdtR::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bits 24:27 - Charge Transfer State Duration Time"]
    #[inline(always)]
    pub fn ctdt(&self) -> CtdtR {
        CtdtR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Charge State Duration Time"]
    #[inline(always)]
    pub fn cdt(&self) -> CdtR {
        CdtR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TSI enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsien(&mut self) -> TsienW<CtlSpec> {
        TsienW::new(self, 0)
    }
    #[doc = "Bit 1 - TSI start"]
    #[inline(always)]
    #[must_use]
    pub fn tsis(&mut self) -> TsisW<CtlSpec> {
        TsisW::new(self, 1)
    }
    #[doc = "Bit 2 - Trigger mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn trgmod(&mut self) -> TrgmodW<CtlSpec> {
        TrgmodW::new(self, 2)
    }
    #[doc = "Bit 3 - Edge selection"]
    #[inline(always)]
    #[must_use]
    pub fn egsel(&mut self) -> EgselW<CtlSpec> {
        EgselW::new(self, 3)
    }
    #[doc = "Bit 4 - Pin mode"]
    #[inline(always)]
    #[must_use]
    pub fn pinmod(&mut self) -> PinmodW<CtlSpec> {
        PinmodW::new(self, 4)
    }
    #[doc = "Bits 5:7 - Max cycle number of a sequence"]
    #[inline(always)]
    #[must_use]
    pub fn mcn(&mut self) -> McnW<CtlSpec> {
        McnW::new(self, 5)
    }
    #[doc = "Bits 12:14 - CTCLK clock division factor"]
    #[inline(always)]
    #[must_use]
    pub fn ctcdiv(&mut self) -> CtcdivW<CtlSpec> {
        CtcdivW::new(self, 12)
    }
    #[doc = "Bit 15 - ECCLK clock division factor"]
    #[inline(always)]
    #[must_use]
    pub fn ecdiv(&mut self) -> EcdivW<CtlSpec> {
        EcdivW::new(self, 15)
    }
    #[doc = "Bit 16 - Extend Charge State Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ecen(&mut self) -> EcenW<CtlSpec> {
        EcenW::new(self, 16)
    }
    #[doc = "Bits 17:23 - Extend Charge State Maximum Duration Time"]
    #[inline(always)]
    #[must_use]
    pub fn ecdt(&mut self) -> EcdtW<CtlSpec> {
        EcdtW::new(self, 17)
    }
    #[doc = "Bits 24:27 - Charge Transfer State Duration Time"]
    #[inline(always)]
    #[must_use]
    pub fn ctdt(&mut self) -> CtdtW<CtlSpec> {
        CtdtW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Charge State Duration Time"]
    #[inline(always)]
    #[must_use]
    pub fn cdt(&mut self) -> CdtW<CtlSpec> {
        CdtW::new(self, 28)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
