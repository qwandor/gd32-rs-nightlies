#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `TSEG` reader - Time-stamp event active edge"]
pub type TsegR = crate::BitReader;
#[doc = "Field `TSEG` writer - Time-stamp event active edge"]
pub type TsegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFEN` reader - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
pub type RefenR = crate::BitReader;
#[doc = "Field `REFEN` writer - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
pub type RefenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BPSHAD` reader - Bypass the shadow registers"]
pub type BpshadR = crate::BitReader;
#[doc = "Field `BPSHAD` writer - Bypass the shadow registers"]
pub type BpshadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS` reader - Hour format"]
pub type CsR = crate::BitReader;
#[doc = "Field `CS` writer - Hour format"]
pub type CsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRM0EN` reader - Alarm A enable"]
pub type Alrm0enR = crate::BitReader;
#[doc = "Field `ALRM0EN` writer - Alarm A enable"]
pub type Alrm0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEN` reader - timestamp enable"]
pub type TsenR = crate::BitReader;
#[doc = "Field `TSEN` writer - timestamp enable"]
pub type TsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRM0IE` reader - Alarm A interrupt enable"]
pub type Alrm0ieR = crate::BitReader;
#[doc = "Field `ALRM0IE` writer - Alarm A interrupt enable"]
pub type Alrm0ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIE` reader - Time-stamp interrupt enable"]
pub type TsieR = crate::BitReader;
#[doc = "Field `TSIE` writer - Time-stamp interrupt enable"]
pub type TsieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A1H` writer - Add 1 hour (summer time change)"]
pub type A1hW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1H` writer - Subtract 1 hour (winter time change)"]
pub type S1hW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSM` reader - Backup"]
pub type DsmR = crate::BitReader;
#[doc = "Field `DSM` writer - Backup"]
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
    #[doc = "Bit 3 - Time-stamp event active edge"]
    #[inline(always)]
    pub fn tseg(&self) -> TsegR {
        TsegR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
    #[inline(always)]
    pub fn refen(&self) -> RefenR {
        RefenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bypass the shadow registers"]
    #[inline(always)]
    pub fn bpshad(&self) -> BpshadR {
        BpshadR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    pub fn cs(&self) -> CsR {
        CsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    pub fn alrm0en(&self) -> Alrm0enR {
        Alrm0enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - timestamp enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TsenR {
        TsenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    pub fn alrm0ie(&self) -> Alrm0ieR {
        Alrm0ieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Time-stamp interrupt enable"]
    #[inline(always)]
    pub fn tsie(&self) -> TsieR {
        TsieR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - Backup"]
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
    #[doc = "Bit 3 - Time-stamp event active edge"]
    #[inline(always)]
    #[must_use]
    pub fn tseg(&mut self) -> TsegW<CtlSpec> {
        TsegW::new(self, 3)
    }
    #[doc = "Bit 4 - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
    #[inline(always)]
    #[must_use]
    pub fn refen(&mut self) -> RefenW<CtlSpec> {
        RefenW::new(self, 4)
    }
    #[doc = "Bit 5 - Bypass the shadow registers"]
    #[inline(always)]
    #[must_use]
    pub fn bpshad(&mut self) -> BpshadW<CtlSpec> {
        BpshadW::new(self, 5)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CsW<CtlSpec> {
        CsW::new(self, 6)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    #[must_use]
    pub fn alrm0en(&mut self) -> Alrm0enW<CtlSpec> {
        Alrm0enW::new(self, 8)
    }
    #[doc = "Bit 11 - timestamp enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TsenW<CtlSpec> {
        TsenW::new(self, 11)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
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
    #[doc = "Bit 16 - Add 1 hour (summer time change)"]
    #[inline(always)]
    #[must_use]
    pub fn a1h(&mut self) -> A1hW<CtlSpec> {
        A1hW::new(self, 16)
    }
    #[doc = "Bit 17 - Subtract 1 hour (winter time change)"]
    #[inline(always)]
    #[must_use]
    pub fn s1h(&mut self) -> S1hW<CtlSpec> {
        S1hW::new(self, 17)
    }
    #[doc = "Bit 18 - Backup"]
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
