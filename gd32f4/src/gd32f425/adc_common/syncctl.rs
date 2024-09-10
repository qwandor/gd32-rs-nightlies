#[doc = "Register `SYNCCTL` reader"]
pub type R = crate::R<SyncctlSpec>;
#[doc = "Register `SYNCCTL` writer"]
pub type W = crate::W<SyncctlSpec>;
#[doc = "Field `SYNCM` reader - ADC sync mode"]
pub type SyncmR = crate::FieldReader;
#[doc = "Field `SYNCM` writer - ADC sync mode"]
pub type SyncmW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SYNCDLY` reader - ADC sync delay"]
pub type SyncdlyR = crate::FieldReader;
#[doc = "Field `SYNCDLY` writer - ADC sync delay"]
pub type SyncdlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SYNCDDM` reader - ADC sync DMA disable mode"]
pub type SyncddmR = crate::BitReader;
#[doc = "Field `SYNCDDM` writer - ADC sync DMA disable mode"]
pub type SyncddmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCDMA` reader - ADC sync DMA mode selection"]
pub type SyncdmaR = crate::FieldReader;
#[doc = "Field `SYNCDMA` writer - ADC sync DMA mode selection"]
pub type SyncdmaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADCCK` reader - ADC clock"]
pub type AdcckR = crate::FieldReader;
#[doc = "Field `ADCCK` writer - ADC clock"]
pub type AdcckW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VBATEN` reader - Channel 18 (1/4 voltate of external battery) enable of ADC0"]
pub type VbatenR = crate::BitReader;
#[doc = "Field `VBATEN` writer - Channel 18 (1/4 voltate of external battery) enable of ADC0"]
pub type VbatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSVREN` reader - Channel 16 (temperature sensor) and 17 (internal reference voltage) enable of ADC0"]
pub type TsvrenR = crate::BitReader;
#[doc = "Field `TSVREN` writer - Channel 16 (temperature sensor) and 17 (internal reference voltage) enable of ADC0"]
pub type TsvrenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - ADC sync mode"]
    #[inline(always)]
    pub fn syncm(&self) -> SyncmR {
        SyncmR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - ADC sync delay"]
    #[inline(always)]
    pub fn syncdly(&self) -> SyncdlyR {
        SyncdlyR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - ADC sync DMA disable mode"]
    #[inline(always)]
    pub fn syncddm(&self) -> SyncddmR {
        SyncddmR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - ADC sync DMA mode selection"]
    #[inline(always)]
    pub fn syncdma(&self) -> SyncdmaR {
        SyncdmaR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18 - ADC clock"]
    #[inline(always)]
    pub fn adcck(&self) -> AdcckR {
        AdcckR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 22 - Channel 18 (1/4 voltate of external battery) enable of ADC0"]
    #[inline(always)]
    pub fn vbaten(&self) -> VbatenR {
        VbatenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel 16 (temperature sensor) and 17 (internal reference voltage) enable of ADC0"]
    #[inline(always)]
    pub fn tsvren(&self) -> TsvrenR {
        TsvrenR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADC sync mode"]
    #[inline(always)]
    #[must_use]
    pub fn syncm(&mut self) -> SyncmW<SyncctlSpec> {
        SyncmW::new(self, 0)
    }
    #[doc = "Bits 8:11 - ADC sync delay"]
    #[inline(always)]
    #[must_use]
    pub fn syncdly(&mut self) -> SyncdlyW<SyncctlSpec> {
        SyncdlyW::new(self, 8)
    }
    #[doc = "Bit 13 - ADC sync DMA disable mode"]
    #[inline(always)]
    #[must_use]
    pub fn syncddm(&mut self) -> SyncddmW<SyncctlSpec> {
        SyncddmW::new(self, 13)
    }
    #[doc = "Bits 14:15 - ADC sync DMA mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn syncdma(&mut self) -> SyncdmaW<SyncctlSpec> {
        SyncdmaW::new(self, 14)
    }
    #[doc = "Bits 16:18 - ADC clock"]
    #[inline(always)]
    #[must_use]
    pub fn adcck(&mut self) -> AdcckW<SyncctlSpec> {
        AdcckW::new(self, 16)
    }
    #[doc = "Bit 22 - Channel 18 (1/4 voltate of external battery) enable of ADC0"]
    #[inline(always)]
    #[must_use]
    pub fn vbaten(&mut self) -> VbatenW<SyncctlSpec> {
        VbatenW::new(self, 22)
    }
    #[doc = "Bit 23 - Channel 16 (temperature sensor) and 17 (internal reference voltage) enable of ADC0"]
    #[inline(always)]
    #[must_use]
    pub fn tsvren(&mut self) -> TsvrenW<SyncctlSpec> {
        TsvrenW::new(self, 23)
    }
}
#[doc = "sync control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syncctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncctlSpec;
impl crate::RegisterSpec for SyncctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncctl::R`](R) reader structure"]
impl crate::Readable for SyncctlSpec {}
#[doc = "`write(|w| ..)` method takes [`syncctl::W`](W) writer structure"]
impl crate::Writable for SyncctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYNCCTL to value 0"]
impl crate::Resettable for SyncctlSpec {
    const RESET_VALUE: u32 = 0;
}
