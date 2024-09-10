#[doc = "Register `AHB1RST` reader"]
pub type R = crate::R<Ahb1rstSpec>;
#[doc = "Register `AHB1RST` writer"]
pub type W = crate::W<Ahb1rstSpec>;
#[doc = "Field `PARST` reader - GPIO port A reset"]
pub type ParstR = crate::BitReader;
#[doc = "Field `PARST` writer - GPIO port A reset"]
pub type ParstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBRST` reader - GPIO port B reset"]
pub type PbrstR = crate::BitReader;
#[doc = "Field `PBRST` writer - GPIO port B reset"]
pub type PbrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCRST` reader - GPIO port C reset"]
pub type PcrstR = crate::BitReader;
#[doc = "Field `PCRST` writer - GPIO port C reset"]
pub type PcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDRST` reader - GPIO port D reset"]
pub type PdrstR = crate::BitReader;
#[doc = "Field `PDRST` writer - GPIO port D reset"]
pub type PdrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERST` reader - GPIO port E reset"]
pub type PerstR = crate::BitReader;
#[doc = "Field `PERST` writer - GPIO port E reset"]
pub type PerstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFRST` reader - GPIO port F reset"]
pub type PfrstR = crate::BitReader;
#[doc = "Field `PFRST` writer - GPIO port F reset"]
pub type PfrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGRST` reader - GPIO port G reset"]
pub type PgrstR = crate::BitReader;
#[doc = "Field `PGRST` writer - GPIO port G reset"]
pub type PgrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHRST` reader - GPIO port H reset"]
pub type PhrstR = crate::BitReader;
#[doc = "Field `PHRST` writer - GPIO port H reset"]
pub type PhrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIRST` reader - GPIO port I reset"]
pub type PirstR = crate::BitReader;
#[doc = "Field `PIRST` writer - GPIO port I reset"]
pub type PirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCRST` reader - CRC reset"]
pub type CrcrstR = crate::BitReader;
#[doc = "Field `CRCRST` writer - CRC reset"]
pub type CrcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA0RST` reader - DMA0 reset"]
pub type Dma0rstR = crate::BitReader;
#[doc = "Field `DMA0RST` writer - DMA0 reset"]
pub type Dma0rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1RST` reader - DMA1 reset"]
pub type Dma1rstR = crate::BitReader;
#[doc = "Field `DMA1RST` writer - DMA1 reset"]
pub type Dma1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPARST` reader - IPA reset"]
pub type IparstR = crate::BitReader;
#[doc = "Field `IPARST` writer - IPA reset"]
pub type IparstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENETRST` reader - Ethernet reset"]
pub type EnetrstR = crate::BitReader;
#[doc = "Field `ENETRST` writer - Ethernet reset"]
pub type EnetrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBHSRST` reader - USBHS reset"]
pub type UsbhsrstR = crate::BitReader;
#[doc = "Field `USBHSRST` writer - USBHS reset"]
pub type UsbhsrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO port A reset"]
    #[inline(always)]
    pub fn parst(&self) -> ParstR {
        ParstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO port B reset"]
    #[inline(always)]
    pub fn pbrst(&self) -> PbrstR {
        PbrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO port C reset"]
    #[inline(always)]
    pub fn pcrst(&self) -> PcrstR {
        PcrstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO port D reset"]
    #[inline(always)]
    pub fn pdrst(&self) -> PdrstR {
        PdrstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO port E reset"]
    #[inline(always)]
    pub fn perst(&self) -> PerstR {
        PerstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO port F reset"]
    #[inline(always)]
    pub fn pfrst(&self) -> PfrstR {
        PfrstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO port G reset"]
    #[inline(always)]
    pub fn pgrst(&self) -> PgrstR {
        PgrstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO port H reset"]
    #[inline(always)]
    pub fn phrst(&self) -> PhrstR {
        PhrstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO port I reset"]
    #[inline(always)]
    pub fn pirst(&self) -> PirstR {
        PirstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn crcrst(&self) -> CrcrstR {
        CrcrstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA0 reset"]
    #[inline(always)]
    pub fn dma0rst(&self) -> Dma0rstR {
        Dma0rstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA1 reset"]
    #[inline(always)]
    pub fn dma1rst(&self) -> Dma1rstR {
        Dma1rstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - IPA reset"]
    #[inline(always)]
    pub fn iparst(&self) -> IparstR {
        IparstR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Ethernet reset"]
    #[inline(always)]
    pub fn enetrst(&self) -> EnetrstR {
        EnetrstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - USBHS reset"]
    #[inline(always)]
    pub fn usbhsrst(&self) -> UsbhsrstR {
        UsbhsrstR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn parst(&mut self) -> ParstW<Ahb1rstSpec> {
        ParstW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst(&mut self) -> PbrstW<Ahb1rstSpec> {
        PbrstW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst(&mut self) -> PcrstW<Ahb1rstSpec> {
        PcrstW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO port D reset"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst(&mut self) -> PdrstW<Ahb1rstSpec> {
        PdrstW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO port E reset"]
    #[inline(always)]
    #[must_use]
    pub fn perst(&mut self) -> PerstW<Ahb1rstSpec> {
        PerstW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO port F reset"]
    #[inline(always)]
    #[must_use]
    pub fn pfrst(&mut self) -> PfrstW<Ahb1rstSpec> {
        PfrstW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO port G reset"]
    #[inline(always)]
    #[must_use]
    pub fn pgrst(&mut self) -> PgrstW<Ahb1rstSpec> {
        PgrstW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO port H reset"]
    #[inline(always)]
    #[must_use]
    pub fn phrst(&mut self) -> PhrstW<Ahb1rstSpec> {
        PhrstW::new(self, 7)
    }
    #[doc = "Bit 8 - GPIO port I reset"]
    #[inline(always)]
    #[must_use]
    pub fn pirst(&mut self) -> PirstW<Ahb1rstSpec> {
        PirstW::new(self, 8)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CrcrstW<Ahb1rstSpec> {
        CrcrstW::new(self, 12)
    }
    #[doc = "Bit 21 - DMA0 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma0rst(&mut self) -> Dma0rstW<Ahb1rstSpec> {
        Dma0rstW::new(self, 21)
    }
    #[doc = "Bit 22 - DMA1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma1rst(&mut self) -> Dma1rstW<Ahb1rstSpec> {
        Dma1rstW::new(self, 22)
    }
    #[doc = "Bit 23 - IPA reset"]
    #[inline(always)]
    #[must_use]
    pub fn iparst(&mut self) -> IparstW<Ahb1rstSpec> {
        IparstW::new(self, 23)
    }
    #[doc = "Bit 25 - Ethernet reset"]
    #[inline(always)]
    #[must_use]
    pub fn enetrst(&mut self) -> EnetrstW<Ahb1rstSpec> {
        EnetrstW::new(self, 25)
    }
    #[doc = "Bit 29 - USBHS reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbhsrst(&mut self) -> UsbhsrstW<Ahb1rstSpec> {
        UsbhsrstW::new(self, 29)
    }
}
#[doc = "AHB1 reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb1rstSpec;
impl crate::RegisterSpec for Ahb1rstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1rst::R`](R) reader structure"]
impl crate::Readable for Ahb1rstSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb1rst::W`](W) writer structure"]
impl crate::Writable for Ahb1rstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB1RST to value 0"]
impl crate::Resettable for Ahb1rstSpec {
    const RESET_VALUE: u32 = 0;
}
