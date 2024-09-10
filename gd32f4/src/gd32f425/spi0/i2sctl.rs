#[doc = "Register `I2SCTL` reader"]
pub type R = crate::R<I2sctlSpec>;
#[doc = "Register `I2SCTL` writer"]
pub type W = crate::W<I2sctlSpec>;
#[doc = "Field `CHLEN` reader - Channel length (number of bits per audio channel)"]
pub type ChlenR = crate::BitReader;
#[doc = "Field `CHLEN` writer - Channel length (number of bits per audio channel)"]
pub type ChlenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTLEN` reader - Data length"]
pub type DtlenR = crate::FieldReader;
#[doc = "Field `DTLEN` writer - Data length"]
pub type DtlenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CKPL` reader - Idle state clock polarity"]
pub type CkplR = crate::BitReader;
#[doc = "Field `CKPL` writer - Idle state clock polarity"]
pub type CkplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SSTD` reader - I2S standard selection"]
pub type I2sstdR = crate::FieldReader;
#[doc = "Field `I2SSTD` writer - I2S standard selection"]
pub type I2sstdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCMSMOD` reader - PCM frame synchronization mode"]
pub type PcmsmodR = crate::BitReader;
#[doc = "Field `PCMSMOD` writer - PCM frame synchronization mode"]
pub type PcmsmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SOPMOD` reader - I2S operation mode"]
pub type I2sopmodR = crate::FieldReader;
#[doc = "Field `I2SOPMOD` writer - I2S operation mode"]
pub type I2sopmodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2SEN` reader - I2S Enable"]
pub type I2senR = crate::BitReader;
#[doc = "Field `I2SEN` writer - I2S Enable"]
pub type I2senW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SSEL` reader - I2S mode selection"]
pub type I2sselR = crate::BitReader;
#[doc = "Field `I2SSEL` writer - I2S mode selection"]
pub type I2sselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel length (number of bits per audio channel)"]
    #[inline(always)]
    pub fn chlen(&self) -> ChlenR {
        ChlenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Data length"]
    #[inline(always)]
    pub fn dtlen(&self) -> DtlenR {
        DtlenR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Idle state clock polarity"]
    #[inline(always)]
    pub fn ckpl(&self) -> CkplR {
        CkplR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline(always)]
    pub fn i2sstd(&self) -> I2sstdR {
        I2sstdR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - PCM frame synchronization mode"]
    #[inline(always)]
    pub fn pcmsmod(&self) -> PcmsmodR {
        PcmsmodR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - I2S operation mode"]
    #[inline(always)]
    pub fn i2sopmod(&self) -> I2sopmodR {
        I2sopmodR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - I2S Enable"]
    #[inline(always)]
    pub fn i2sen(&self) -> I2senR {
        I2senR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2S mode selection"]
    #[inline(always)]
    pub fn i2ssel(&self) -> I2sselR {
        I2sselR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel length (number of bits per audio channel)"]
    #[inline(always)]
    #[must_use]
    pub fn chlen(&mut self) -> ChlenW<I2sctlSpec> {
        ChlenW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Data length"]
    #[inline(always)]
    #[must_use]
    pub fn dtlen(&mut self) -> DtlenW<I2sctlSpec> {
        DtlenW::new(self, 1)
    }
    #[doc = "Bit 3 - Idle state clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ckpl(&mut self) -> CkplW<I2sctlSpec> {
        CkplW::new(self, 3)
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2sstd(&mut self) -> I2sstdW<I2sctlSpec> {
        I2sstdW::new(self, 4)
    }
    #[doc = "Bit 7 - PCM frame synchronization mode"]
    #[inline(always)]
    #[must_use]
    pub fn pcmsmod(&mut self) -> PcmsmodW<I2sctlSpec> {
        PcmsmodW::new(self, 7)
    }
    #[doc = "Bits 8:9 - I2S operation mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2sopmod(&mut self) -> I2sopmodW<I2sctlSpec> {
        I2sopmodW::new(self, 8)
    }
    #[doc = "Bit 10 - I2S Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2sen(&mut self) -> I2senW<I2sctlSpec> {
        I2senW::new(self, 10)
    }
    #[doc = "Bit 11 - I2S mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2ssel(&mut self) -> I2sselW<I2sctlSpec> {
        I2sselW::new(self, 11)
    }
}
#[doc = "I2S control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sctlSpec;
impl crate::RegisterSpec for I2sctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2sctl::R`](R) reader structure"]
impl crate::Readable for I2sctlSpec {}
#[doc = "`write(|w| ..)` method takes [`i2sctl::W`](W) writer structure"]
impl crate::Writable for I2sctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2SCTL to value 0"]
impl crate::Resettable for I2sctlSpec {
    const RESET_VALUE: u32 = 0;
}
