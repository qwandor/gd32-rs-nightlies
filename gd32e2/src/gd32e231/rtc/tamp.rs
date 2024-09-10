#[doc = "Register `TAMP` reader"]
pub type R = crate::R<TampSpec>;
#[doc = "Register `TAMP` writer"]
pub type W = crate::W<TampSpec>;
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
#[doc = "Field `FREQ` reader - Tamper sampling frequency"]
pub type FreqR = crate::FieldReader;
#[doc = "Field `FREQ` writer - Tamper sampling frequency"]
pub type FreqW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FLT` reader - RTC_TAMPx filter count"]
pub type FltR = crate::FieldReader;
#[doc = "Field `FLT` writer - RTC_TAMPx filter count"]
pub type FltW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRCH` reader - RTC_TAMPx precharge duration"]
pub type PrchR = crate::FieldReader;
#[doc = "Field `PRCH` writer - RTC_TAMPx precharge duration"]
pub type PrchW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DISPU` reader - RTC_TAMPx pull-up disable"]
pub type DispuR = crate::BitReader;
#[doc = "Field `DISPU` writer - RTC_TAMPx pull-up disable"]
pub type DispuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC14VAL` reader - PC14 value"]
pub type Pc14valR = crate::BitReader;
#[doc = "Field `PC14VAL` writer - PC14 value"]
pub type Pc14valW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC14MDE` reader - PC14 mode"]
pub type Pc14mdeR = crate::BitReader;
#[doc = "Field `PC14MDE` writer - PC14 mode"]
pub type Pc14mdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC15VAL` reader - PC15 value"]
pub type Pc15valR = crate::BitReader;
#[doc = "Field `PC15VAL` writer - PC15 value"]
pub type Pc15valW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC15MDE` reader - PC15 mode"]
pub type Pc15mdeR = crate::BitReader;
#[doc = "Field `PC15MDE` writer - PC15 mode"]
pub type Pc15mdeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
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
    #[doc = "Bits 8:10 - Tamper sampling frequency"]
    #[inline(always)]
    pub fn freq(&self) -> FreqR {
        FreqR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - RTC_TAMPx filter count"]
    #[inline(always)]
    pub fn flt(&self) -> FltR {
        FltR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - RTC_TAMPx precharge duration"]
    #[inline(always)]
    pub fn prch(&self) -> PrchR {
        PrchR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC_TAMPx pull-up disable"]
    #[inline(always)]
    pub fn dispu(&self) -> DispuR {
        DispuR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - PC14 value"]
    #[inline(always)]
    pub fn pc14val(&self) -> Pc14valR {
        Pc14valR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PC14 mode"]
    #[inline(always)]
    pub fn pc14mde(&self) -> Pc14mdeR {
        Pc14mdeR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PC15 value"]
    #[inline(always)]
    pub fn pc15val(&self) -> Pc15valR {
        Pc15valR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PC15 mode"]
    #[inline(always)]
    pub fn pc15mde(&self) -> Pc15mdeR {
        Pc15mdeR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
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
    #[doc = "Bits 8:10 - Tamper sampling frequency"]
    #[inline(always)]
    #[must_use]
    pub fn freq(&mut self) -> FreqW<TampSpec> {
        FreqW::new(self, 8)
    }
    #[doc = "Bits 11:12 - RTC_TAMPx filter count"]
    #[inline(always)]
    #[must_use]
    pub fn flt(&mut self) -> FltW<TampSpec> {
        FltW::new(self, 11)
    }
    #[doc = "Bits 13:14 - RTC_TAMPx precharge duration"]
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
    #[doc = "Bit 20 - PC14 value"]
    #[inline(always)]
    #[must_use]
    pub fn pc14val(&mut self) -> Pc14valW<TampSpec> {
        Pc14valW::new(self, 20)
    }
    #[doc = "Bit 21 - PC14 mode"]
    #[inline(always)]
    #[must_use]
    pub fn pc14mde(&mut self) -> Pc14mdeW<TampSpec> {
        Pc14mdeW::new(self, 21)
    }
    #[doc = "Bit 22 - PC15 value"]
    #[inline(always)]
    #[must_use]
    pub fn pc15val(&mut self) -> Pc15valW<TampSpec> {
        Pc15valW::new(self, 22)
    }
    #[doc = "Bit 23 - PC15 mode"]
    #[inline(always)]
    #[must_use]
    pub fn pc15mde(&mut self) -> Pc15mdeW<TampSpec> {
        Pc15mdeW::new(self, 23)
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
