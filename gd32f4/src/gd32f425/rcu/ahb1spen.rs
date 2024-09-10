#[doc = "Register `AHB1SPEN` reader"]
pub type R = crate::R<Ahb1spenSpec>;
#[doc = "Register `AHB1SPEN` writer"]
pub type W = crate::W<Ahb1spenSpec>;
#[doc = "Field `PASPEN` reader - GPIO port A clock enable when sleep mode"]
pub type PaspenR = crate::BitReader;
#[doc = "Field `PASPEN` writer - GPIO port A clock enable when sleep mode"]
pub type PaspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBSPEN` reader - GPIO port B clock enable when sleep mode"]
pub type PbspenR = crate::BitReader;
#[doc = "Field `PBSPEN` writer - GPIO port B clock enable when sleep mode"]
pub type PbspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSPEN` reader - GPIO port C clock enable when sleep mode"]
pub type PcspenR = crate::BitReader;
#[doc = "Field `PCSPEN` writer - GPIO port C clock enable when sleep mode"]
pub type PcspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDSPEN` reader - GPIO port D clock enable when sleep mode"]
pub type PdspenR = crate::BitReader;
#[doc = "Field `PDSPEN` writer - GPIO port D clock enable when sleep mode"]
pub type PdspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PESPEN` reader - GPIO port E clock enable when sleep mode"]
pub type PespenR = crate::BitReader;
#[doc = "Field `PESPEN` writer - GPIO port E clock enable when sleep mode"]
pub type PespenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFSPEN` reader - GPIO port F clock enable when sleep mode"]
pub type PfspenR = crate::BitReader;
#[doc = "Field `PFSPEN` writer - GPIO port F clock enable when sleep mode"]
pub type PfspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGSPEN` reader - GPIO port G clock enable when sleep mode"]
pub type PgspenR = crate::BitReader;
#[doc = "Field `PGSPEN` writer - GPIO port G clock enable when sleep mode"]
pub type PgspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHSPEN` reader - GPIO port H clock enable when sleep mode"]
pub type PhspenR = crate::BitReader;
#[doc = "Field `PHSPEN` writer - GPIO port H clock enable when sleep mode"]
pub type PhspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PISPEN` reader - GPIO port I clock enable when sleep mode"]
pub type PispenR = crate::BitReader;
#[doc = "Field `PISPEN` writer - GPIO port I clock enable when sleep mode"]
pub type PispenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCSPEN` reader - CRC clock enable when sleep mode"]
pub type CrcspenR = crate::BitReader;
#[doc = "Field `CRCSPEN` writer - CRC clock enable when sleep mode"]
pub type CrcspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMCSPEN` reader - FMC clock enable when sleep mode"]
pub type FmcspenR = crate::BitReader;
#[doc = "Field `FMCSPEN` writer - FMC clock enable when sleep mode"]
pub type FmcspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM0SPEN` reader - SRAM0 clock enable when sleep mode"]
pub type Sram0spenR = crate::BitReader;
#[doc = "Field `SRAM0SPEN` writer - SRAM0 clock enable when sleep mode"]
pub type Sram0spenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM1SPEN` reader - SRAM1 clock enable when sleep mode"]
pub type Sram1spenR = crate::BitReader;
#[doc = "Field `SRAM1SPEN` writer - SRAM1 clock enable when sleep mode"]
pub type Sram1spenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPSRAMSPEN` reader - BKPSRAM clock enable when sleep mode"]
pub type BkpsramspenR = crate::BitReader;
#[doc = "Field `BKPSRAMSPEN` writer - BKPSRAM clock enable when sleep mode"]
pub type BkpsramspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2SPEN` reader - SRAM2 clock enable when sleep mode"]
pub type Sram2spenR = crate::BitReader;
#[doc = "Field `SRAM2SPEN` writer - SRAM2 clock enable when sleep mode"]
pub type Sram2spenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA0SPEN` reader - DMA0 clock enable when sleep mode"]
pub type Dma0spenR = crate::BitReader;
#[doc = "Field `DMA0SPEN` writer - DMA0 clock enable when sleep mode"]
pub type Dma0spenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1SPEN` reader - DMA1 clock enable when sleep mode"]
pub type Dma1spenR = crate::BitReader;
#[doc = "Field `DMA1SPEN` writer - DMA1 clock enable when sleep mode"]
pub type Dma1spenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPASPEN` reader - IPA clock enable when sleep mode"]
pub type IpaspenR = crate::BitReader;
#[doc = "Field `IPASPEN` writer - IPA clock enable when sleep mode"]
pub type IpaspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENETSPEN` reader - Ethernet clock enable when sleep mode"]
pub type EnetspenR = crate::BitReader;
#[doc = "Field `ENETSPEN` writer - Ethernet clock enable when sleep mode"]
pub type EnetspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENETTXSPEN` reader - Ethernet TX clock enable when sleep mode"]
pub type EnettxspenR = crate::BitReader;
#[doc = "Field `ENETTXSPEN` writer - Ethernet TX clock enable when sleep mode"]
pub type EnettxspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENETRXSPEN` reader - Ethernet RX clock enable when sleep mode"]
pub type EnetrxspenR = crate::BitReader;
#[doc = "Field `ENETRXSPEN` writer - Ethernet RX clock enable when sleep mode"]
pub type EnetrxspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENETPTPSPEN` reader - Ethernet PTP clock enable when sleep mode"]
pub type EnetptpspenR = crate::BitReader;
#[doc = "Field `ENETPTPSPEN` writer - Ethernet PTP clock enable when sleep mode"]
pub type EnetptpspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBHSSPEN` reader - USBHS clock enable when sleep mode"]
pub type UsbhsspenR = crate::BitReader;
#[doc = "Field `USBHSSPEN` writer - USBHS clock enable when sleep mode"]
pub type UsbhsspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBHSULPISPEN` reader - USBHS ULPI clock enable when sleep mode"]
pub type UsbhsulpispenR = crate::BitReader;
#[doc = "Field `USBHSULPISPEN` writer - USBHS ULPI clock enable when sleep mode"]
pub type UsbhsulpispenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO port A clock enable when sleep mode"]
    #[inline(always)]
    pub fn paspen(&self) -> PaspenR {
        PaspenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO port B clock enable when sleep mode"]
    #[inline(always)]
    pub fn pbspen(&self) -> PbspenR {
        PbspenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO port C clock enable when sleep mode"]
    #[inline(always)]
    pub fn pcspen(&self) -> PcspenR {
        PcspenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO port D clock enable when sleep mode"]
    #[inline(always)]
    pub fn pdspen(&self) -> PdspenR {
        PdspenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO port E clock enable when sleep mode"]
    #[inline(always)]
    pub fn pespen(&self) -> PespenR {
        PespenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO port F clock enable when sleep mode"]
    #[inline(always)]
    pub fn pfspen(&self) -> PfspenR {
        PfspenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO port G clock enable when sleep mode"]
    #[inline(always)]
    pub fn pgspen(&self) -> PgspenR {
        PgspenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO port H clock enable when sleep mode"]
    #[inline(always)]
    pub fn phspen(&self) -> PhspenR {
        PhspenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO port I clock enable when sleep mode"]
    #[inline(always)]
    pub fn pispen(&self) -> PispenR {
        PispenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable when sleep mode"]
    #[inline(always)]
    pub fn crcspen(&self) -> CrcspenR {
        CrcspenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - FMC clock enable when sleep mode"]
    #[inline(always)]
    pub fn fmcspen(&self) -> FmcspenR {
        FmcspenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SRAM0 clock enable when sleep mode"]
    #[inline(always)]
    pub fn sram0spen(&self) -> Sram0spenR {
        Sram0spenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SRAM1 clock enable when sleep mode"]
    #[inline(always)]
    pub fn sram1spen(&self) -> Sram1spenR {
        Sram1spenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - BKPSRAM clock enable when sleep mode"]
    #[inline(always)]
    pub fn bkpsramspen(&self) -> BkpsramspenR {
        BkpsramspenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SRAM2 clock enable when sleep mode"]
    #[inline(always)]
    pub fn sram2spen(&self) -> Sram2spenR {
        Sram2spenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA0 clock enable when sleep mode"]
    #[inline(always)]
    pub fn dma0spen(&self) -> Dma0spenR {
        Dma0spenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA1 clock enable when sleep mode"]
    #[inline(always)]
    pub fn dma1spen(&self) -> Dma1spenR {
        Dma1spenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - IPA clock enable when sleep mode"]
    #[inline(always)]
    pub fn ipaspen(&self) -> IpaspenR {
        IpaspenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Ethernet clock enable when sleep mode"]
    #[inline(always)]
    pub fn enetspen(&self) -> EnetspenR {
        EnetspenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Ethernet TX clock enable when sleep mode"]
    #[inline(always)]
    pub fn enettxspen(&self) -> EnettxspenR {
        EnettxspenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Ethernet RX clock enable when sleep mode"]
    #[inline(always)]
    pub fn enetrxspen(&self) -> EnetrxspenR {
        EnetrxspenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Ethernet PTP clock enable when sleep mode"]
    #[inline(always)]
    pub fn enetptpspen(&self) -> EnetptpspenR {
        EnetptpspenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - USBHS clock enable when sleep mode"]
    #[inline(always)]
    pub fn usbhsspen(&self) -> UsbhsspenR {
        UsbhsspenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - USBHS ULPI clock enable when sleep mode"]
    #[inline(always)]
    pub fn usbhsulpispen(&self) -> UsbhsulpispenR {
        UsbhsulpispenR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO port A clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn paspen(&mut self) -> PaspenW<Ahb1spenSpec> {
        PaspenW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO port B clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn pbspen(&mut self) -> PbspenW<Ahb1spenSpec> {
        PbspenW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO port C clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn pcspen(&mut self) -> PcspenW<Ahb1spenSpec> {
        PcspenW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO port D clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn pdspen(&mut self) -> PdspenW<Ahb1spenSpec> {
        PdspenW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO port E clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn pespen(&mut self) -> PespenW<Ahb1spenSpec> {
        PespenW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO port F clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn pfspen(&mut self) -> PfspenW<Ahb1spenSpec> {
        PfspenW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO port G clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn pgspen(&mut self) -> PgspenW<Ahb1spenSpec> {
        PgspenW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO port H clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn phspen(&mut self) -> PhspenW<Ahb1spenSpec> {
        PhspenW::new(self, 7)
    }
    #[doc = "Bit 8 - GPIO port I clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn pispen(&mut self) -> PispenW<Ahb1spenSpec> {
        PispenW::new(self, 8)
    }
    #[doc = "Bit 12 - CRC clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn crcspen(&mut self) -> CrcspenW<Ahb1spenSpec> {
        CrcspenW::new(self, 12)
    }
    #[doc = "Bit 15 - FMC clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmcspen(&mut self) -> FmcspenW<Ahb1spenSpec> {
        FmcspenW::new(self, 15)
    }
    #[doc = "Bit 16 - SRAM0 clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram0spen(&mut self) -> Sram0spenW<Ahb1spenSpec> {
        Sram0spenW::new(self, 16)
    }
    #[doc = "Bit 17 - SRAM1 clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram1spen(&mut self) -> Sram1spenW<Ahb1spenSpec> {
        Sram1spenW::new(self, 17)
    }
    #[doc = "Bit 18 - BKPSRAM clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn bkpsramspen(&mut self) -> BkpsramspenW<Ahb1spenSpec> {
        BkpsramspenW::new(self, 18)
    }
    #[doc = "Bit 19 - SRAM2 clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram2spen(&mut self) -> Sram2spenW<Ahb1spenSpec> {
        Sram2spenW::new(self, 19)
    }
    #[doc = "Bit 21 - DMA0 clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma0spen(&mut self) -> Dma0spenW<Ahb1spenSpec> {
        Dma0spenW::new(self, 21)
    }
    #[doc = "Bit 22 - DMA1 clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma1spen(&mut self) -> Dma1spenW<Ahb1spenSpec> {
        Dma1spenW::new(self, 22)
    }
    #[doc = "Bit 23 - IPA clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn ipaspen(&mut self) -> IpaspenW<Ahb1spenSpec> {
        IpaspenW::new(self, 23)
    }
    #[doc = "Bit 25 - Ethernet clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn enetspen(&mut self) -> EnetspenW<Ahb1spenSpec> {
        EnetspenW::new(self, 25)
    }
    #[doc = "Bit 26 - Ethernet TX clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn enettxspen(&mut self) -> EnettxspenW<Ahb1spenSpec> {
        EnettxspenW::new(self, 26)
    }
    #[doc = "Bit 27 - Ethernet RX clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn enetrxspen(&mut self) -> EnetrxspenW<Ahb1spenSpec> {
        EnetrxspenW::new(self, 27)
    }
    #[doc = "Bit 28 - Ethernet PTP clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn enetptpspen(&mut self) -> EnetptpspenW<Ahb1spenSpec> {
        EnetptpspenW::new(self, 28)
    }
    #[doc = "Bit 29 - USBHS clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usbhsspen(&mut self) -> UsbhsspenW<Ahb1spenSpec> {
        UsbhsspenW::new(self, 29)
    }
    #[doc = "Bit 30 - USBHS ULPI clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usbhsulpispen(&mut self) -> UsbhsulpispenW<Ahb1spenSpec> {
        UsbhsulpispenW::new(self, 30)
    }
}
#[doc = "AHB1 sleep mode enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1spen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1spen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb1spenSpec;
impl crate::RegisterSpec for Ahb1spenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1spen::R`](R) reader structure"]
impl crate::Readable for Ahb1spenSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb1spen::W`](W) writer structure"]
impl crate::Writable for Ahb1spenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB1SPEN to value 0x7eef_97ff"]
impl crate::Resettable for Ahb1spenSpec {
    const RESET_VALUE: u32 = 0x7eef_97ff;
}
