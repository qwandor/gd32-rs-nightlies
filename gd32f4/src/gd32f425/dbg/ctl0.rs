#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Field `SLP_HOLD` reader - Sleep mode hold register"]
pub type SlpHoldR = crate::BitReader;
#[doc = "Field `SLP_HOLD` writer - Sleep mode hold register"]
pub type SlpHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSLP_HOLD` reader - Deep-sleep mode hold register"]
pub type DslpHoldR = crate::BitReader;
#[doc = "Field `DSLP_HOLD` writer - Deep-sleep mode hold register"]
pub type DslpHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STB_HOLD` reader - Standby mode hold register"]
pub type StbHoldR = crate::BitReader;
#[doc = "Field `STB_HOLD` writer - Standby mode hold register"]
pub type StbHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_IOEN` reader - Trace pin allocation enable"]
pub type TraceIoenR = crate::BitReader;
#[doc = "Field `TRACE_IOEN` writer - Trace pin allocation enable"]
pub type TraceIoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_MODE` reader - Trace pin allocation mode"]
pub type TraceModeR = crate::FieldReader;
#[doc = "Field `TRACE_MODE` writer - Trace pin allocation mode"]
pub type TraceModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Sleep mode hold register"]
    #[inline(always)]
    pub fn slp_hold(&self) -> SlpHoldR {
        SlpHoldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Deep-sleep mode hold register"]
    #[inline(always)]
    pub fn dslp_hold(&self) -> DslpHoldR {
        DslpHoldR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Standby mode hold register"]
    #[inline(always)]
    pub fn stb_hold(&self) -> StbHoldR {
        StbHoldR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Trace pin allocation enable"]
    #[inline(always)]
    pub fn trace_ioen(&self) -> TraceIoenR {
        TraceIoenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Trace pin allocation mode"]
    #[inline(always)]
    pub fn trace_mode(&self) -> TraceModeR {
        TraceModeR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Sleep mode hold register"]
    #[inline(always)]
    #[must_use]
    pub fn slp_hold(&mut self) -> SlpHoldW<Ctl0Spec> {
        SlpHoldW::new(self, 0)
    }
    #[doc = "Bit 1 - Deep-sleep mode hold register"]
    #[inline(always)]
    #[must_use]
    pub fn dslp_hold(&mut self) -> DslpHoldW<Ctl0Spec> {
        DslpHoldW::new(self, 1)
    }
    #[doc = "Bit 2 - Standby mode hold register"]
    #[inline(always)]
    #[must_use]
    pub fn stb_hold(&mut self) -> StbHoldW<Ctl0Spec> {
        StbHoldW::new(self, 2)
    }
    #[doc = "Bit 5 - Trace pin allocation enable"]
    #[inline(always)]
    #[must_use]
    pub fn trace_ioen(&mut self) -> TraceIoenW<Ctl0Spec> {
        TraceIoenW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Trace pin allocation mode"]
    #[inline(always)]
    #[must_use]
    pub fn trace_mode(&mut self) -> TraceModeW<Ctl0Spec> {
        TraceModeW::new(self, 6)
    }
}
#[doc = "Control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
