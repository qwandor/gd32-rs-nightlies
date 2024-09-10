#[doc = "Register `TAMP` reader"]
pub type R = crate::R<TampSpec>;
#[doc = "Register `TAMP` writer"]
pub type W = crate::W<TampSpec>;
#[doc = "Field `TP0EN` reader - Tamper 0 detection enable"]
pub type Tp0enR = crate::BitReader;
#[doc = "Field `TP0EN` writer - Tamper 0 detection enable"]
pub type Tp0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TP0EG` reader - Tamper 0 event trigger edge"]
pub type Tp0egR = crate::BitReader;
#[doc = "Field `TP0EG` writer - Tamper 0 event trigger edge"]
pub type Tp0egW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPIE` reader - Tamper detection interrupt enable"]
pub type TpieR = crate::BitReader;
#[doc = "Field `TPIE` writer - Tamper detection interrupt enable"]
pub type TpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TP1EN` reader - Tamper 1 detection enable"]
pub type Tp1enR = crate::BitReader;
#[doc = "Field `TP1EN` writer - Tamper 1 detection enable"]
pub type Tp1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TP1EG` reader - Tamper 1 event trigger edge"]
pub type Tp1egR = crate::BitReader;
#[doc = "Field `TP1EG` writer - Tamper 1 event trigger edge"]
pub type Tp1egW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPTS` reader - Make tamper function used for timestamp function"]
pub type TptsR = crate::BitReader;
#[doc = "Field `TPTS` writer - Make tamper function used for timestamp function"]
pub type TptsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREQ` reader - Sampling frequency of tamper event detection"]
pub type FreqR = crate::FieldReader;
#[doc = "Field `FREQ` writer - Sampling frequency of tamper event detection"]
pub type FreqW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FLT` reader - RTC_TAMPx filter count setting"]
pub type FltR = crate::FieldReader;
#[doc = "Field `FLT` writer - RTC_TAMPx filter count setting"]
pub type FltW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRCH` reader - Pre-charge duration time of RTC_TAMPx"]
pub type PrchR = crate::FieldReader;
#[doc = "Field `PRCH` writer - Pre-charge duration time of RTC_TAMPx"]
pub type PrchW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DISPU` reader - RTC_TAMPx pull-up disable"]
pub type DispuR = crate::BitReader;
#[doc = "Field `DISPU` writer - RTC_TAMPx pull-up disable"]
pub type DispuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TP0SEL` reader - Tamper 0 function input mapping selection"]
pub type Tp0selR = crate::BitReader;
#[doc = "Field `TP0SEL` writer - Tamper 0 function input mapping selection"]
pub type Tp0selW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSSEL` reader - Timestamp input mapping selection"]
pub type TsselR = crate::BitReader;
#[doc = "Field `TSSEL` writer - Timestamp input mapping selection"]
pub type TsselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AOT` reader - RTC_ALARM Output Type"]
pub type AotR = crate::BitReader;
#[doc = "Field `AOT` writer - RTC_ALARM Output Type"]
pub type AotW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tamper 0 detection enable"]
    #[inline(always)]
    pub fn tp0en(&self) -> Tp0enR {
        Tp0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper 0 event trigger edge"]
    #[inline(always)]
    pub fn tp0eg(&self) -> Tp0egR {
        Tp0egR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper detection interrupt enable"]
    #[inline(always)]
    pub fn tpie(&self) -> TpieR {
        TpieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tamper 1 detection enable"]
    #[inline(always)]
    pub fn tp1en(&self) -> Tp1enR {
        Tp1enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tamper 1 event trigger edge"]
    #[inline(always)]
    pub fn tp1eg(&self) -> Tp1egR {
        Tp1egR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Make tamper function used for timestamp function"]
    #[inline(always)]
    pub fn tpts(&self) -> TptsR {
        TptsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Sampling frequency of tamper event detection"]
    #[inline(always)]
    pub fn freq(&self) -> FreqR {
        FreqR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - RTC_TAMPx filter count setting"]
    #[inline(always)]
    pub fn flt(&self) -> FltR {
        FltR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Pre-charge duration time of RTC_TAMPx"]
    #[inline(always)]
    pub fn prch(&self) -> PrchR {
        PrchR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC_TAMPx pull-up disable"]
    #[inline(always)]
    pub fn dispu(&self) -> DispuR {
        DispuR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Tamper 0 function input mapping selection"]
    #[inline(always)]
    pub fn tp0sel(&self) -> Tp0selR {
        Tp0selR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timestamp input mapping selection"]
    #[inline(always)]
    pub fn tssel(&self) -> TsselR {
        TsselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RTC_ALARM Output Type"]
    #[inline(always)]
    pub fn aot(&self) -> AotR {
        AotR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper 0 detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn tp0en(&mut self) -> Tp0enW<TampSpec> {
        Tp0enW::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper 0 event trigger edge"]
    #[inline(always)]
    #[must_use]
    pub fn tp0eg(&mut self) -> Tp0egW<TampSpec> {
        Tp0egW::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpie(&mut self) -> TpieW<TampSpec> {
        TpieW::new(self, 2)
    }
    #[doc = "Bit 3 - Tamper 1 detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn tp1en(&mut self) -> Tp1enW<TampSpec> {
        Tp1enW::new(self, 3)
    }
    #[doc = "Bit 4 - Tamper 1 event trigger edge"]
    #[inline(always)]
    #[must_use]
    pub fn tp1eg(&mut self) -> Tp1egW<TampSpec> {
        Tp1egW::new(self, 4)
    }
    #[doc = "Bit 7 - Make tamper function used for timestamp function"]
    #[inline(always)]
    #[must_use]
    pub fn tpts(&mut self) -> TptsW<TampSpec> {
        TptsW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Sampling frequency of tamper event detection"]
    #[inline(always)]
    #[must_use]
    pub fn freq(&mut self) -> FreqW<TampSpec> {
        FreqW::new(self, 8)
    }
    #[doc = "Bits 11:12 - RTC_TAMPx filter count setting"]
    #[inline(always)]
    #[must_use]
    pub fn flt(&mut self) -> FltW<TampSpec> {
        FltW::new(self, 11)
    }
    #[doc = "Bits 13:14 - Pre-charge duration time of RTC_TAMPx"]
    #[inline(always)]
    #[must_use]
    pub fn prch(&mut self) -> PrchW<TampSpec> {
        PrchW::new(self, 13)
    }
    #[doc = "Bit 15 - RTC_TAMPx pull-up disable"]
    #[inline(always)]
    #[must_use]
    pub fn dispu(&mut self) -> DispuW<TampSpec> {
        DispuW::new(self, 15)
    }
    #[doc = "Bit 16 - Tamper 0 function input mapping selection"]
    #[inline(always)]
    #[must_use]
    pub fn tp0sel(&mut self) -> Tp0selW<TampSpec> {
        Tp0selW::new(self, 16)
    }
    #[doc = "Bit 17 - Timestamp input mapping selection"]
    #[inline(always)]
    #[must_use]
    pub fn tssel(&mut self) -> TsselW<TampSpec> {
        TsselW::new(self, 17)
    }
    #[doc = "Bit 18 - RTC_ALARM Output Type"]
    #[inline(always)]
    #[must_use]
    pub fn aot(&mut self) -> AotW<TampSpec> {
        AotW::new(self, 18)
    }
}
#[doc = "tamper and alternate function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TampSpec;
impl crate::RegisterSpec for TampSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamp::R`](R) reader structure"]
impl crate::Readable for TampSpec {}
#[doc = "`write(|w| ..)` method takes [`tamp::W`](W) writer structure"]
impl crate::Writable for TampSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAMP to value 0"]
impl crate::Resettable for TampSpec {
    const RESET_VALUE: u32 = 0;
}
