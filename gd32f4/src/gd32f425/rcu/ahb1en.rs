#[doc = "Register `AHB1EN` reader"]
pub type R = crate::R<Ahb1enSpec>;
#[doc = "Register `AHB1EN` writer"]
pub type W = crate::W<Ahb1enSpec>;
#[doc = "Field `PAEN` reader - GPIO port A clock enable"]
pub type PaenR = crate::BitReader;
#[doc = "Field `PAEN` writer - GPIO port A clock enable"]
pub type PaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBEN` reader - GPIO port B clock enable"]
pub type PbenR = crate::BitReader;
#[doc = "Field `PBEN` writer - GPIO port B clock enable"]
pub type PbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCEN` reader - GPIO port C clock enable"]
pub type PcenR = crate::BitReader;
#[doc = "Field `PCEN` writer - GPIO port C clock enable"]
pub type PcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDEN` reader - GPIO port D clock enable"]
pub type PdenR = crate::BitReader;
#[doc = "Field `PDEN` writer - GPIO port D clock enable"]
pub type PdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEEN` reader - GPIO port E clock enable"]
pub type PeenR = crate::BitReader;
#[doc = "Field `PEEN` writer - GPIO port E clock enable"]
pub type PeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFEN` reader - GPIO port F clock enable"]
pub type PfenR = crate::BitReader;
#[doc = "Field `PFEN` writer - GPIO port F clock enable"]
pub type PfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGEN` reader - GPIO port G clock enable"]
pub type PgenR = crate::BitReader;
#[doc = "Field `PGEN` writer - GPIO port G clock enable"]
pub type PgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHEN` reader - GPIO port H clock enable"]
pub type PhenR = crate::BitReader;
#[doc = "Field `PHEN` writer - GPIO port H clock enable"]
pub type PhenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIEN` reader - GPIO port I clock enable"]
pub type PienR = crate::BitReader;
#[doc = "Field `PIEN` writer - GPIO port I clock enable"]
pub type PienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub type CrcenR = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPSRAMEN` reader - BKPSRAM clock enable"]
pub type BkpsramenR = crate::BitReader;
#[doc = "Field `BKPSRAMEN` writer - BKPSRAM clock enable"]
pub type BkpsramenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCMSRAMEN` reader - TCMSRAM clock enable"]
pub type TcmsramenR = crate::BitReader;
#[doc = "Field `TCMSRAMEN` writer - TCMSRAM clock enable"]
pub type TcmsramenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA0EN` reader - DMA0 clock enable"]
pub type Dma0enR = crate::BitReader;
#[doc = "Field `DMA0EN` writer - DMA0 clock enable"]
pub type Dma0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1EN` reader - DMA1 clock enable"]
pub type Dma1enR = crate::BitReader;
#[doc = "Field `DMA1EN` writer - DMA1 clock enable"]
pub type Dma1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPAEN` reader - IPA clock enable"]
pub type IpaenR = crate::BitReader;
#[doc = "Field `IPAEN` writer - IPA clock enable"]
pub type IpaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENETEN` reader - Ethernet clock enable"]
pub type EnetenR = crate::BitReader;
#[doc = "Field `ENETEN` writer - Ethernet clock enable"]
pub type EnetenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENETTXEN` reader - Ethernet TX clock enable"]
pub type EnettxenR = crate::BitReader;
#[doc = "Field `ENETTXEN` writer - Ethernet TX clock enable"]
pub type EnettxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENETRXEN` reader - Ethernet RX clock enable"]
pub type EnetrxenR = crate::BitReader;
#[doc = "Field `ENETRXEN` writer - Ethernet RX clock enable"]
pub type EnetrxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENETPTPEN` reader - Ethernet PTP clock enable"]
pub type EnetptpenR = crate::BitReader;
#[doc = "Field `ENETPTPEN` writer - Ethernet PTP clock enable"]
pub type EnetptpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBHSEN` reader - USBHS clock enable"]
pub type UsbhsenR = crate::BitReader;
#[doc = "Field `USBHSEN` writer - USBHS clock enable"]
pub type UsbhsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBHSULPIEN` reader - USBHS ULPI clock enable"]
pub type UsbhsulpienR = crate::BitReader;
#[doc = "Field `USBHSULPIEN` writer - USBHS ULPI clock enable"]
pub type UsbhsulpienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO port A clock enable"]
    #[inline(always)]
    pub fn paen(&self) -> PaenR {
        PaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO port B clock enable"]
    #[inline(always)]
    pub fn pben(&self) -> PbenR {
        PbenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO port C clock enable"]
    #[inline(always)]
    pub fn pcen(&self) -> PcenR {
        PcenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO port D clock enable"]
    #[inline(always)]
    pub fn pden(&self) -> PdenR {
        PdenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO port E clock enable"]
    #[inline(always)]
    pub fn peen(&self) -> PeenR {
        PeenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO port F clock enable"]
    #[inline(always)]
    pub fn pfen(&self) -> PfenR {
        PfenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO port G clock enable"]
    #[inline(always)]
    pub fn pgen(&self) -> PgenR {
        PgenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO port H clock enable"]
    #[inline(always)]
    pub fn phen(&self) -> PhenR {
        PhenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO port I clock enable"]
    #[inline(always)]
    pub fn pien(&self) -> PienR {
        PienR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 18 - BKPSRAM clock enable"]
    #[inline(always)]
    pub fn bkpsramen(&self) -> BkpsramenR {
        BkpsramenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - TCMSRAM clock enable"]
    #[inline(always)]
    pub fn tcmsramen(&self) -> TcmsramenR {
        TcmsramenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA0 clock enable"]
    #[inline(always)]
    pub fn dma0en(&self) -> Dma0enR {
        Dma0enR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> Dma1enR {
        Dma1enR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - IPA clock enable"]
    #[inline(always)]
    pub fn ipaen(&self) -> IpaenR {
        IpaenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Ethernet clock enable"]
    #[inline(always)]
    pub fn eneten(&self) -> EnetenR {
        EnetenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Ethernet TX clock enable"]
    #[inline(always)]
    pub fn enettxen(&self) -> EnettxenR {
        EnettxenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Ethernet RX clock enable"]
    #[inline(always)]
    pub fn enetrxen(&self) -> EnetrxenR {
        EnetrxenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Ethernet PTP clock enable"]
    #[inline(always)]
    pub fn enetptpen(&self) -> EnetptpenR {
        EnetptpenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - USBHS clock enable"]
    #[inline(always)]
    pub fn usbhsen(&self) -> UsbhsenR {
        UsbhsenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - USBHS ULPI clock enable"]
    #[inline(always)]
    pub fn usbhsulpien(&self) -> UsbhsulpienR {
        UsbhsulpienR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn paen(&mut self) -> PaenW<Ahb1enSpec> {
        PaenW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pben(&mut self) -> PbenW<Ahb1enSpec> {
        PbenW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO port C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcen(&mut self) -> PcenW<Ahb1enSpec> {
        PcenW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO port D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pden(&mut self) -> PdenW<Ahb1enSpec> {
        PdenW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO port E clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn peen(&mut self) -> PeenW<Ahb1enSpec> {
        PeenW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO port F clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pfen(&mut self) -> PfenW<Ahb1enSpec> {
        PfenW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO port G clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pgen(&mut self) -> PgenW<Ahb1enSpec> {
        PgenW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO port H clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn phen(&mut self) -> PhenW<Ahb1enSpec> {
        PhenW::new(self, 7)
    }
    #[doc = "Bit 8 - GPIO port I clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pien(&mut self) -> PienW<Ahb1enSpec> {
        PienW::new(self, 8)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CrcenW<Ahb1enSpec> {
        CrcenW::new(self, 12)
    }
    #[doc = "Bit 18 - BKPSRAM clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkpsramen(&mut self) -> BkpsramenW<Ahb1enSpec> {
        BkpsramenW::new(self, 18)
    }
    #[doc = "Bit 20 - TCMSRAM clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcmsramen(&mut self) -> TcmsramenW<Ahb1enSpec> {
        TcmsramenW::new(self, 20)
    }
    #[doc = "Bit 21 - DMA0 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma0en(&mut self) -> Dma0enW<Ahb1enSpec> {
        Dma0enW::new(self, 21)
    }
    #[doc = "Bit 22 - DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> Dma1enW<Ahb1enSpec> {
        Dma1enW::new(self, 22)
    }
    #[doc = "Bit 23 - IPA clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ipaen(&mut self) -> IpaenW<Ahb1enSpec> {
        IpaenW::new(self, 23)
    }
    #[doc = "Bit 25 - Ethernet clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn eneten(&mut self) -> EnetenW<Ahb1enSpec> {
        EnetenW::new(self, 25)
    }
    #[doc = "Bit 26 - Ethernet TX clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn enettxen(&mut self) -> EnettxenW<Ahb1enSpec> {
        EnettxenW::new(self, 26)
    }
    #[doc = "Bit 27 - Ethernet RX clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn enetrxen(&mut self) -> EnetrxenW<Ahb1enSpec> {
        EnetrxenW::new(self, 27)
    }
    #[doc = "Bit 28 - Ethernet PTP clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn enetptpen(&mut self) -> EnetptpenW<Ahb1enSpec> {
        EnetptpenW::new(self, 28)
    }
    #[doc = "Bit 29 - USBHS clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbhsen(&mut self) -> UsbhsenW<Ahb1enSpec> {
        UsbhsenW::new(self, 29)
    }
    #[doc = "Bit 30 - USBHS ULPI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbhsulpien(&mut self) -> UsbhsulpienW<Ahb1enSpec> {
        UsbhsulpienW::new(self, 30)
    }
}
#[doc = "AHB1 enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb1enSpec;
impl crate::RegisterSpec for Ahb1enSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1en::R`](R) reader structure"]
impl crate::Readable for Ahb1enSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb1en::W`](W) writer structure"]
impl crate::Writable for Ahb1enSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB1EN to value 0x0010_0000"]
impl crate::Resettable for Ahb1enSpec {
    const RESET_VALUE: u32 = 0x0010_0000;
}
