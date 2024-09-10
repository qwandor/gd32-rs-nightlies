#[doc = "Register `AHB1EN` reader"]
pub type R = crate::R<Ahb1enSpec>;
#[doc = "Register `AHB1EN` writer"]
pub type W = crate::W<Ahb1enSpec>;
#[doc = "Field `DMA0EN` reader - DMA0 clock enable"]
pub type Dma0enR = crate::BitReader;
#[doc = "Field `DMA0EN` writer - DMA0 clock enable"]
pub type Dma0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1EN` reader - DMA1 clock enable"]
pub type Dma1enR = crate::BitReader;
#[doc = "Field `DMA1EN` writer - DMA1 clock enable"]
pub type Dma1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMSPEN` reader - SRAM interface clock enable when sleep mode"]
pub type SramspenR = crate::BitReader;
#[doc = "Field `SRAMSPEN` writer - SRAM interface clock enable when sleep mode"]
pub type SramspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMCSPEN` reader - FMC clock enable when sleep mode"]
pub type FmcspenR = crate::BitReader;
#[doc = "Field `FMCSPEN` writer - FMC clock enable when sleep mode"]
pub type FmcspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub type CrcenR = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXMCEN` reader - EXMC clock enable"]
pub type ExmcenR = crate::BitReader;
#[doc = "Field `EXMCEN` writer - EXMC clock enable"]
pub type ExmcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOEN` reader - SDIO clock enable"]
pub type SdioenR = crate::BitReader;
#[doc = "Field `SDIOEN` writer - SDIO clock enable"]
pub type SdioenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBFSEN` reader - USBFS clock enable"]
pub type UsbfsenR = crate::BitReader;
#[doc = "Field `USBFSEN` writer - USBFS clock enable"]
pub type UsbfsenW<'a, REG> = crate::BitWriter<'a, REG>;
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
impl R {
    #[doc = "Bit 0 - DMA0 clock enable"]
    #[inline(always)]
    pub fn dma0en(&self) -> Dma0enR {
        Dma0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> Dma1enR {
        Dma1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM interface clock enable when sleep mode"]
    #[inline(always)]
    pub fn sramspen(&self) -> SramspenR {
        SramspenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FMC clock enable when sleep mode"]
    #[inline(always)]
    pub fn fmcspen(&self) -> FmcspenR {
        FmcspenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - EXMC clock enable"]
    #[inline(always)]
    pub fn exmcen(&self) -> ExmcenR {
        ExmcenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - SDIO clock enable"]
    #[inline(always)]
    pub fn sdioen(&self) -> SdioenR {
        SdioenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - USBFS clock enable"]
    #[inline(always)]
    pub fn usbfsen(&self) -> UsbfsenR {
        UsbfsenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Ethernet clock enable"]
    #[inline(always)]
    pub fn eneten(&self) -> EnetenR {
        EnetenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Ethernet TX clock enable"]
    #[inline(always)]
    pub fn enettxen(&self) -> EnettxenR {
        EnettxenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Ethernet RX clock enable"]
    #[inline(always)]
    pub fn enetrxen(&self) -> EnetrxenR {
        EnetrxenR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA0 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma0en(&mut self) -> Dma0enW<Ahb1enSpec> {
        Dma0enW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> Dma1enW<Ahb1enSpec> {
        Dma1enW::new(self, 1)
    }
    #[doc = "Bit 2 - SRAM interface clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sramspen(&mut self) -> SramspenW<Ahb1enSpec> {
        SramspenW::new(self, 2)
    }
    #[doc = "Bit 4 - FMC clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmcspen(&mut self) -> FmcspenW<Ahb1enSpec> {
        FmcspenW::new(self, 4)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CrcenW<Ahb1enSpec> {
        CrcenW::new(self, 6)
    }
    #[doc = "Bit 8 - EXMC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn exmcen(&mut self) -> ExmcenW<Ahb1enSpec> {
        ExmcenW::new(self, 8)
    }
    #[doc = "Bit 10 - SDIO clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdioen(&mut self) -> SdioenW<Ahb1enSpec> {
        SdioenW::new(self, 10)
    }
    #[doc = "Bit 12 - USBFS clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbfsen(&mut self) -> UsbfsenW<Ahb1enSpec> {
        UsbfsenW::new(self, 12)
    }
    #[doc = "Bit 14 - Ethernet clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn eneten(&mut self) -> EnetenW<Ahb1enSpec> {
        EnetenW::new(self, 14)
    }
    #[doc = "Bit 15 - Ethernet TX clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn enettxen(&mut self) -> EnettxenW<Ahb1enSpec> {
        EnettxenW::new(self, 15)
    }
    #[doc = "Bit 16 - Ethernet RX clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn enetrxen(&mut self) -> EnetrxenW<Ahb1enSpec> {
        EnetrxenW::new(self, 16)
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
#[doc = "`reset()` method sets AHB1EN to value 0x14"]
impl crate::Resettable for Ahb1enSpec {
    const RESET_VALUE: u32 = 0x14;
}
