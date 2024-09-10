#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `TSEG` reader - Valid event edge of time-stamp"]
pub type TsegR = crate::BitReader;
#[doc = "Field `TSEG` writer - Valid event edge of time-stamp"]
pub type TsegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFEN` reader - Reference clock detection function enable"]
pub type RefenR = crate::BitReader;
#[doc = "Field `REFEN` writer - Reference clock detection function enable"]
pub type RefenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BPSHAD` reader - Shadow registers bypass control"]
pub type BpshadR = crate::BitReader;
#[doc = "Field `BPSHAD` writer - Shadow registers bypass control"]
pub type BpshadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS` reader - Clock System"]
pub type CsR = crate::BitReader;
#[doc = "Field `CS` writer - Clock System"]
pub type CsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRM0EN` reader - Alarm-0 function enable"]
pub type Alrm0enR = crate::BitReader;
#[doc = "Field `ALRM0EN` writer - Alarm-0 function enable"]
pub type Alrm0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEN` reader - time-stamp function enable"]
pub type TsenR = crate::BitReader;
#[doc = "Field `TSEN` writer - time-stamp function enable"]
pub type TsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRM0IE` reader - RTC alarm-0 interrupt enable"]
pub type Alrm0ieR = crate::BitReader;
#[doc = "Field `ALRM0IE` writer - RTC alarm-0 interrupt enable"]
pub type Alrm0ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIE` reader - Time-stamp interrupt enable"]
pub type TsieR = crate::BitReader;
#[doc = "Field `TSIE` writer - Time-stamp interrupt enable"]
pub type TsieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A1H` writer - Add 1 hour(summer time change"]
pub type A1hW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1H` writer - Subtract 1 hour(winter time change)"]
pub type S1hW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSM` reader - Daylight saving mark"]
pub type DsmR = crate::BitReader;
#[doc = "Field `DSM` writer - Daylight saving mark"]
pub type DsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COS` reader - Calibration output selection"]
pub type CosR = crate::BitReader;
#[doc = "Field `COS` writer - Calibration output selection"]
pub type CosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPOL` reader - Output polarity"]
pub type OpolR = crate::BitReader;
#[doc = "Field `OPOL` writer - Output polarity"]
pub type OpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OS` reader - Output selection"]
pub type OsR = crate::FieldReader;
#[doc = "Field `OS` writer - Output selection"]
pub type OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COEN` reader - Calibration output enable"]
pub type CoenR = crate::BitReader;
#[doc = "Field `COEN` writer - Calibration output enable"]
pub type CoenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Valid event edge of time-stamp"]
    #[inline(always)]
    pub fn tseg(&self) -> TsegR {
        TsegR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reference clock detection function enable"]
    #[inline(always)]
    pub fn refen(&self) -> RefenR {
        RefenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Shadow registers bypass control"]
    #[inline(always)]
    pub fn bpshad(&self) -> BpshadR {
        BpshadR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clock System"]
    #[inline(always)]
    pub fn cs(&self) -> CsR {
        CsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm-0 function enable"]
    #[inline(always)]
    pub fn alrm0en(&self) -> Alrm0enR {
        Alrm0enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - time-stamp function enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TsenR {
        TsenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RTC alarm-0 interrupt enable"]
    #[inline(always)]
    pub fn alrm0ie(&self) -> Alrm0ieR {
        Alrm0ieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Time-stamp interrupt enable"]
    #[inline(always)]
    pub fn tsie(&self) -> TsieR {
        TsieR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - Daylight saving mark"]
    #[inline(always)]
    pub fn dsm(&self) -> DsmR {
        DsmR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Calibration output selection"]
    #[inline(always)]
    pub fn cos(&self) -> CosR {
        CosR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output polarity"]
    #[inline(always)]
    pub fn opol(&self) -> OpolR {
        OpolR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Output selection"]
    #[inline(always)]
    pub fn os(&self) -> OsR {
        OsR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Calibration output enable"]
    #[inline(always)]
    pub fn coen(&self) -> CoenR {
        CoenR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Valid event edge of time-stamp"]
    #[inline(always)]
    #[must_use]
    pub fn tseg(&mut self) -> TsegW<CtlSpec> {
        TsegW::new(self, 3)
    }
    #[doc = "Bit 4 - Reference clock detection function enable"]
    #[inline(always)]
    #[must_use]
    pub fn refen(&mut self) -> RefenW<CtlSpec> {
        RefenW::new(self, 4)
    }
    #[doc = "Bit 5 - Shadow registers bypass control"]
    #[inline(always)]
    #[must_use]
    pub fn bpshad(&mut self) -> BpshadW<CtlSpec> {
        BpshadW::new(self, 5)
    }
    #[doc = "Bit 6 - Clock System"]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CsW<CtlSpec> {
        CsW::new(self, 6)
    }
    #[doc = "Bit 8 - Alarm-0 function enable"]
    #[inline(always)]
    #[must_use]
    pub fn alrm0en(&mut self) -> Alrm0enW<CtlSpec> {
        Alrm0enW::new(self, 8)
    }
    #[doc = "Bit 11 - time-stamp function enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TsenW<CtlSpec> {
        TsenW::new(self, 11)
    }
    #[doc = "Bit 12 - RTC alarm-0 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn alrm0ie(&mut self) -> Alrm0ieW<CtlSpec> {
        Alrm0ieW::new(self, 12)
    }
    #[doc = "Bit 15 - Time-stamp interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsie(&mut self) -> TsieW<CtlSpec> {
        TsieW::new(self, 15)
    }
    #[doc = "Bit 16 - Add 1 hour(summer time change"]
    #[inline(always)]
    #[must_use]
    pub fn a1h(&mut self) -> A1hW<CtlSpec> {
        A1hW::new(self, 16)
    }
    #[doc = "Bit 17 - Subtract 1 hour(winter time change)"]
    #[inline(always)]
    #[must_use]
    pub fn s1h(&mut self) -> S1hW<CtlSpec> {
        S1hW::new(self, 17)
    }
    #[doc = "Bit 18 - Daylight saving mark"]
    #[inline(always)]
    #[must_use]
    pub fn dsm(&mut self) -> DsmW<CtlSpec> {
        DsmW::new(self, 18)
    }
    #[doc = "Bit 19 - Calibration output selection"]
    #[inline(always)]
    #[must_use]
    pub fn cos(&mut self) -> CosW<CtlSpec> {
        CosW::new(self, 19)
    }
    #[doc = "Bit 20 - Output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn opol(&mut self) -> OpolW<CtlSpec> {
        OpolW::new(self, 20)
    }
    #[doc = "Bits 21:22 - Output selection"]
    #[inline(always)]
    #[must_use]
    pub fn os(&mut self) -> OsW<CtlSpec> {
        OsW::new(self, 21)
    }
    #[doc = "Bit 23 - Calibration output enable"]
    #[inline(always)]
    #[must_use]
    pub fn coen(&mut self) -> CoenW<CtlSpec> {
        CoenW::new(self, 23)
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
