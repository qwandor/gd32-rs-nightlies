#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Field `ADCON` reader - ADC on"]
pub type AdconR = crate::BitReader;
#[doc = "Field `ADCON` writer - ADC on"]
pub type AdconW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTN` reader - Continuous mode"]
pub type CtnR = crate::BitReader;
#[doc = "Field `CTN` writer - Continuous mode"]
pub type CtnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLB` reader - ADC calibration"]
pub type ClbR = crate::BitReader;
#[doc = "Field `CLB` writer - ADC calibration"]
pub type ClbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTCLB` reader - Reset calibration"]
pub type RstclbR = crate::BitReader;
#[doc = "Field `RSTCLB` writer - Reset calibration"]
pub type RstclbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA` reader - DMA request enable"]
pub type DmaR = crate::BitReader;
#[doc = "Field `DMA` writer - DMA request enable"]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDM` reader - DMA disable mode"]
pub type DdmR = crate::BitReader;
#[doc = "Field `DDM` writer - DMA disable mode"]
pub type DdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOCM` reader - End of conversion mode"]
pub type EocmR = crate::BitReader;
#[doc = "Field `EOCM` writer - End of conversion mode"]
pub type EocmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAL` reader - Data alignment"]
pub type DalR = crate::BitReader;
#[doc = "Field `DAL` writer - Data alignment"]
pub type DalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETSIC` reader - External trigger select for inserted channel"]
pub type EtsicR = crate::FieldReader;
#[doc = "Field `ETSIC` writer - External trigger select for inserted channel"]
pub type EtsicW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ETMIC` reader - External trigger mode for inserted channel"]
pub type EtmicR = crate::FieldReader;
#[doc = "Field `ETMIC` writer - External trigger mode for inserted channel"]
pub type EtmicW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWICST` reader - Software start on inserted channel"]
pub type SwicstR = crate::BitReader;
#[doc = "Field `SWICST` writer - Software start on inserted channel"]
pub type SwicstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETSRC` reader - External trigger select for regular channel"]
pub type EtsrcR = crate::FieldReader;
#[doc = "Field `ETSRC` writer - External trigger select for regular channel"]
pub type EtsrcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ETMRC` reader - External trigger mode for regular channel"]
pub type EtmrcR = crate::FieldReader;
#[doc = "Field `ETMRC` writer - External trigger mode for regular channel"]
pub type EtmrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWRCST` reader - Software start on regular channel"]
pub type SwrcstR = crate::BitReader;
#[doc = "Field `SWRCST` writer - Software start on regular channel"]
pub type SwrcstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC on"]
    #[inline(always)]
    pub fn adcon(&self) -> AdconR {
        AdconR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Continuous mode"]
    #[inline(always)]
    pub fn ctn(&self) -> CtnR {
        CtnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC calibration"]
    #[inline(always)]
    pub fn clb(&self) -> ClbR {
        ClbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    pub fn rstclb(&self) -> RstclbR {
        RstclbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA request enable"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA disable mode"]
    #[inline(always)]
    pub fn ddm(&self) -> DdmR {
        DdmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - End of conversion mode"]
    #[inline(always)]
    pub fn eocm(&self) -> EocmR {
        EocmR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn dal(&self) -> DalR {
        DalR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:19 - External trigger select for inserted channel"]
    #[inline(always)]
    pub fn etsic(&self) -> EtsicR {
        EtsicR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - External trigger mode for inserted channel"]
    #[inline(always)]
    pub fn etmic(&self) -> EtmicR {
        EtmicR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Software start on inserted channel"]
    #[inline(always)]
    pub fn swicst(&self) -> SwicstR {
        SwicstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:27 - External trigger select for regular channel"]
    #[inline(always)]
    pub fn etsrc(&self) -> EtsrcR {
        EtsrcR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - External trigger mode for regular channel"]
    #[inline(always)]
    pub fn etmrc(&self) -> EtmrcR {
        EtmrcR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Software start on regular channel"]
    #[inline(always)]
    pub fn swrcst(&self) -> SwrcstR {
        SwrcstR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC on"]
    #[inline(always)]
    #[must_use]
    pub fn adcon(&mut self) -> AdconW<Ctl1Spec> {
        AdconW::new(self, 0)
    }
    #[doc = "Bit 1 - Continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn ctn(&mut self) -> CtnW<Ctl1Spec> {
        CtnW::new(self, 1)
    }
    #[doc = "Bit 2 - ADC calibration"]
    #[inline(always)]
    #[must_use]
    pub fn clb(&mut self) -> ClbW<Ctl1Spec> {
        ClbW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    #[must_use]
    pub fn rstclb(&mut self) -> RstclbW<Ctl1Spec> {
        RstclbW::new(self, 3)
    }
    #[doc = "Bit 8 - DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DmaW<Ctl1Spec> {
        DmaW::new(self, 8)
    }
    #[doc = "Bit 9 - DMA disable mode"]
    #[inline(always)]
    #[must_use]
    pub fn ddm(&mut self) -> DdmW<Ctl1Spec> {
        DdmW::new(self, 9)
    }
    #[doc = "Bit 10 - End of conversion mode"]
    #[inline(always)]
    #[must_use]
    pub fn eocm(&mut self) -> EocmW<Ctl1Spec> {
        EocmW::new(self, 10)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    #[must_use]
    pub fn dal(&mut self) -> DalW<Ctl1Spec> {
        DalW::new(self, 11)
    }
    #[doc = "Bits 16:19 - External trigger select for inserted channel"]
    #[inline(always)]
    #[must_use]
    pub fn etsic(&mut self) -> EtsicW<Ctl1Spec> {
        EtsicW::new(self, 16)
    }
    #[doc = "Bits 20:21 - External trigger mode for inserted channel"]
    #[inline(always)]
    #[must_use]
    pub fn etmic(&mut self) -> EtmicW<Ctl1Spec> {
        EtmicW::new(self, 20)
    }
    #[doc = "Bit 22 - Software start on inserted channel"]
    #[inline(always)]
    #[must_use]
    pub fn swicst(&mut self) -> SwicstW<Ctl1Spec> {
        SwicstW::new(self, 22)
    }
    #[doc = "Bits 24:27 - External trigger select for regular channel"]
    #[inline(always)]
    #[must_use]
    pub fn etsrc(&mut self) -> EtsrcW<Ctl1Spec> {
        EtsrcW::new(self, 24)
    }
    #[doc = "Bits 28:29 - External trigger mode for regular channel"]
    #[inline(always)]
    #[must_use]
    pub fn etmrc(&mut self) -> EtmrcW<Ctl1Spec> {
        EtmrcW::new(self, 28)
    }
    #[doc = "Bit 30 - Software start on regular channel"]
    #[inline(always)]
    #[must_use]
    pub fn swrcst(&mut self) -> SwrcstW<Ctl1Spec> {
        SwrcstW::new(self, 30)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {
    const RESET_VALUE: u32 = 0;
}
