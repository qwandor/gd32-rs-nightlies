#[doc = "Register `AHBEN` reader"]
pub type R = crate::R<AhbenSpec>;
#[doc = "Register `AHBEN` writer"]
pub type W = crate::W<AhbenSpec>;
#[doc = "DMA clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaen {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<Dmaen> for bool {
    #[inline(always)]
    fn from(variant: Dmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA clock enable"]
pub type DmaenR = crate::BitReader<Dmaen>;
impl DmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaen {
        match self.bits {
            false => Dmaen::Disabled,
            true => Dmaen::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmaen::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmaen::Enabled
    }
}
#[doc = "Field `DMAEN` writer - DMA clock enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG, Dmaen>;
impl<'a, REG> DmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::Enabled)
    }
}
#[doc = "Field `SRAMEN` reader - SRAM interface clock enable"]
pub use DmaenR as SramenR;
#[doc = "Field `FMCEN` reader - FMC clock enable"]
pub use DmaenR as FmcenR;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub use DmaenR as CrcenR;
#[doc = "Field `PAEN` reader - GPIO port A clock enable"]
pub use DmaenR as PaenR;
#[doc = "Field `PBEN` reader - GPIO port B clock enable"]
pub use DmaenR as PbenR;
#[doc = "Field `PCEN` reader - GPIO port C clock enable"]
pub use DmaenR as PcenR;
#[doc = "Field `PDEN` reader - GPIO port D clock enable"]
pub use DmaenR as PdenR;
#[doc = "Field `PFEN` reader - GPIO port F clock enable"]
pub use DmaenR as PfenR;
#[doc = "Field `TSIEN` reader - TSI clock enable"]
pub use DmaenR as TsienR;
#[doc = "Field `SRAMEN` writer - SRAM interface clock enable"]
pub use DmaenW as SramenW;
#[doc = "Field `FMCEN` writer - FMC clock enable"]
pub use DmaenW as FmcenW;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub use DmaenW as CrcenW;
#[doc = "Field `PAEN` writer - GPIO port A clock enable"]
pub use DmaenW as PaenW;
#[doc = "Field `PBEN` writer - GPIO port B clock enable"]
pub use DmaenW as PbenW;
#[doc = "Field `PCEN` writer - GPIO port C clock enable"]
pub use DmaenW as PcenW;
#[doc = "Field `PDEN` writer - GPIO port D clock enable"]
pub use DmaenW as PdenW;
#[doc = "Field `PFEN` writer - GPIO port F clock enable"]
pub use DmaenW as PfenW;
#[doc = "Field `TSIEN` writer - TSI clock enable"]
pub use DmaenW as TsienW;
impl R {
    #[doc = "Bit 0 - DMA clock enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sramen(&self) -> SramenR {
        SramenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FMC clock enable"]
    #[inline(always)]
    pub fn fmcen(&self) -> FmcenR {
        FmcenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO port A clock enable"]
    #[inline(always)]
    pub fn paen(&self) -> PaenR {
        PaenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO port B clock enable"]
    #[inline(always)]
    pub fn pben(&self) -> PbenR {
        PbenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO port C clock enable"]
    #[inline(always)]
    pub fn pcen(&self) -> PcenR {
        PcenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO port D clock enable"]
    #[inline(always)]
    pub fn pden(&self) -> PdenR {
        PdenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO port F clock enable"]
    #[inline(always)]
    pub fn pfen(&self) -> PfenR {
        PfenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - TSI clock enable"]
    #[inline(always)]
    pub fn tsien(&self) -> TsienR {
        TsienR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DmaenW<AhbenSpec> {
        DmaenW::new(self, 0)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sramen(&mut self) -> SramenW<AhbenSpec> {
        SramenW::new(self, 2)
    }
    #[doc = "Bit 4 - FMC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn fmcen(&mut self) -> FmcenW<AhbenSpec> {
        FmcenW::new(self, 4)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CrcenW<AhbenSpec> {
        CrcenW::new(self, 6)
    }
    #[doc = "Bit 17 - GPIO port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn paen(&mut self) -> PaenW<AhbenSpec> {
        PaenW::new(self, 17)
    }
    #[doc = "Bit 18 - GPIO port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pben(&mut self) -> PbenW<AhbenSpec> {
        PbenW::new(self, 18)
    }
    #[doc = "Bit 19 - GPIO port C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcen(&mut self) -> PcenW<AhbenSpec> {
        PcenW::new(self, 19)
    }
    #[doc = "Bit 20 - GPIO port D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pden(&mut self) -> PdenW<AhbenSpec> {
        PdenW::new(self, 20)
    }
    #[doc = "Bit 22 - GPIO port F clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pfen(&mut self) -> PfenW<AhbenSpec> {
        PfenW::new(self, 22)
    }
    #[doc = "Bit 24 - TSI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsien(&mut self) -> TsienW<AhbenSpec> {
        TsienW::new(self, 24)
    }
}
#[doc = "AHB enable register (RCU_AHBEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbenSpec;
impl crate::RegisterSpec for AhbenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahben::R`](R) reader structure"]
impl crate::Readable for AhbenSpec {}
#[doc = "`write(|w| ..)` method takes [`ahben::W`](W) writer structure"]
impl crate::Writable for AhbenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBEN to value 0x14"]
impl crate::Resettable for AhbenSpec {
    const RESET_VALUE: u32 = 0x14;
}
