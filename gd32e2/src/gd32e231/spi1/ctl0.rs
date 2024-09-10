#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Field `CKPH` reader - Clock Phase Selection"]
pub type CkphR = crate::BitReader;
#[doc = "Field `CKPH` writer - Clock Phase Selection"]
pub type CkphW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKPL` reader - Clock Polarity Selection"]
pub type CkplR = crate::BitReader;
#[doc = "Field `CKPL` writer - Clock Polarity Selection"]
pub type CkplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTMOD` reader - Master Mode Enable"]
pub type MstmodR = crate::BitReader;
#[doc = "Field `MSTMOD` writer - Master Mode Enable"]
pub type MstmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSC` reader - Master Clock Prescaler Selection"]
pub type PscR = crate::FieldReader;
#[doc = "Field `PSC` writer - Master Clock Prescaler Selection"]
pub type PscW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPIEN` reader - SPI enable"]
pub type SpienR = crate::BitReader;
#[doc = "Field `SPIEN` writer - SPI enable"]
pub type SpienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LF` reader - LSB First Mode"]
pub type LfR = crate::BitReader;
#[doc = "Field `LF` writer - LSB First Mode"]
pub type LfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWNSS` reader - NSS Pin Selection In NSS Software Mode"]
pub type SwnssR = crate::BitReader;
#[doc = "Field `SWNSS` writer - NSS Pin Selection In NSS Software Mode"]
pub type SwnssW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWNSSEN` reader - NSS Software Mode Selection"]
pub type SwnssenR = crate::BitReader;
#[doc = "Field `SWNSSEN` writer - NSS Software Mode Selection"]
pub type SwnssenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RO` reader - Receive only"]
pub type RoR = crate::BitReader;
#[doc = "Field `RO` writer - Receive only"]
pub type RoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCL` reader - CRC length"]
pub type CrclR = crate::BitReader;
#[doc = "Field `CRCL` writer - CRC length"]
pub type CrclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCNT` reader - CRC transfer next"]
pub type CrcntR = crate::BitReader;
#[doc = "Field `CRCNT` writer - CRC transfer next"]
pub type CrcntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - Hardware CRC calculation enable"]
pub type CrcenR = crate::BitReader;
#[doc = "Field `CRCEN` writer - Hardware CRC calculation enable"]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDOEN` reader - Bidirectional Transmit output enable"]
pub type BdoenR = crate::BitReader;
#[doc = "Field `BDOEN` writer - Bidirectional Transmit output enable"]
pub type BdoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDEN` reader - Bidirectional enable"]
pub type BdenR = crate::BitReader;
#[doc = "Field `BDEN` writer - Bidirectional enable"]
pub type BdenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock Phase Selection"]
    #[inline(always)]
    pub fn ckph(&self) -> CkphR {
        CkphR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Polarity Selection"]
    #[inline(always)]
    pub fn ckpl(&self) -> CkplR {
        CkplR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master Mode Enable"]
    #[inline(always)]
    pub fn mstmod(&self) -> MstmodR {
        MstmodR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Master Clock Prescaler Selection"]
    #[inline(always)]
    pub fn psc(&self) -> PscR {
        PscR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    pub fn spien(&self) -> SpienR {
        SpienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LSB First Mode"]
    #[inline(always)]
    pub fn lf(&self) -> LfR {
        LfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NSS Pin Selection In NSS Software Mode"]
    #[inline(always)]
    pub fn swnss(&self) -> SwnssR {
        SwnssR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NSS Software Mode Selection"]
    #[inline(always)]
    pub fn swnssen(&self) -> SwnssenR {
        SwnssenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline(always)]
    pub fn ro(&self) -> RoR {
        RoR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CRC length"]
    #[inline(always)]
    pub fn crcl(&self) -> CrclR {
        CrclR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC transfer next"]
    #[inline(always)]
    pub fn crcnt(&self) -> CrcntR {
        CrcntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bidirectional Transmit output enable"]
    #[inline(always)]
    pub fn bdoen(&self) -> BdoenR {
        BdoenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Bidirectional enable"]
    #[inline(always)]
    pub fn bden(&self) -> BdenR {
        BdenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Phase Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckph(&mut self) -> CkphW<Ctl0Spec> {
        CkphW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock Polarity Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckpl(&mut self) -> CkplW<Ctl0Spec> {
        CkplW::new(self, 1)
    }
    #[doc = "Bit 2 - Master Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mstmod(&mut self) -> MstmodW<Ctl0Spec> {
        MstmodW::new(self, 2)
    }
    #[doc = "Bits 3:5 - Master Clock Prescaler Selection"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PscW<Ctl0Spec> {
        PscW::new(self, 3)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SpienW<Ctl0Spec> {
        SpienW::new(self, 6)
    }
    #[doc = "Bit 7 - LSB First Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lf(&mut self) -> LfW<Ctl0Spec> {
        LfW::new(self, 7)
    }
    #[doc = "Bit 8 - NSS Pin Selection In NSS Software Mode"]
    #[inline(always)]
    #[must_use]
    pub fn swnss(&mut self) -> SwnssW<Ctl0Spec> {
        SwnssW::new(self, 8)
    }
    #[doc = "Bit 9 - NSS Software Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn swnssen(&mut self) -> SwnssenW<Ctl0Spec> {
        SwnssenW::new(self, 9)
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline(always)]
    #[must_use]
    pub fn ro(&mut self) -> RoW<Ctl0Spec> {
        RoW::new(self, 10)
    }
    #[doc = "Bit 11 - CRC length"]
    #[inline(always)]
    #[must_use]
    pub fn crcl(&mut self) -> CrclW<Ctl0Spec> {
        CrclW::new(self, 11)
    }
    #[doc = "Bit 12 - CRC transfer next"]
    #[inline(always)]
    #[must_use]
    pub fn crcnt(&mut self) -> CrcntW<Ctl0Spec> {
        CrcntW::new(self, 12)
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CrcenW<Ctl0Spec> {
        CrcenW::new(self, 13)
    }
    #[doc = "Bit 14 - Bidirectional Transmit output enable"]
    #[inline(always)]
    #[must_use]
    pub fn bdoen(&mut self) -> BdoenW<Ctl0Spec> {
        BdoenW::new(self, 14)
    }
    #[doc = "Bit 15 - Bidirectional enable"]
    #[inline(always)]
    #[must_use]
    pub fn bden(&mut self) -> BdenW<Ctl0Spec> {
        BdenW::new(self, 15)
    }
}
#[doc = "control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for Ctl0Spec {
    const RESET_VALUE: u32 = 0;
}
