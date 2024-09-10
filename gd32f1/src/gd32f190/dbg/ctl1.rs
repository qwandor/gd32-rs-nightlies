#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "RTC hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcHold {
    #[doc = "0: Continue running the RTC as usual"]
    Continue = 0,
    #[doc = "1: Hold the RTC for debug when the core is halted"]
    Stop = 1,
}
impl From<RtcHold> for bool {
    #[inline(always)]
    fn from(variant: RtcHold) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_HOLD` reader - RTC hold register"]
pub type RtcHoldR = crate::BitReader<RtcHold>;
impl RtcHoldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtcHold {
        match self.bits {
            false => RtcHold::Continue,
            true => RtcHold::Stop,
        }
    }
    #[doc = "Continue running the RTC as usual"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == RtcHold::Continue
    }
    #[doc = "Hold the RTC for debug when the core is halted"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == RtcHold::Stop
    }
}
#[doc = "Field `RTC_HOLD` writer - RTC hold register"]
pub type RtcHoldW<'a, REG> = crate::BitWriter<'a, REG, RtcHold>;
impl<'a, REG> RtcHoldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continue running the RTC as usual"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(RtcHold::Continue)
    }
    #[doc = "Hold the RTC for debug when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(RtcHold::Stop)
    }
}
#[doc = "Timer 14 hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer14Hold {
    #[doc = "0: Continue running the timer as usual"]
    Continue = 0,
    #[doc = "1: Hold the timer counter for debug when the core is halted"]
    Stop = 1,
}
impl From<Timer14Hold> for bool {
    #[inline(always)]
    fn from(variant: Timer14Hold) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER14_HOLD` reader - Timer 14 hold register"]
pub type Timer14HoldR = crate::BitReader<Timer14Hold>;
impl Timer14HoldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer14Hold {
        match self.bits {
            false => Timer14Hold::Continue,
            true => Timer14Hold::Stop,
        }
    }
    #[doc = "Continue running the timer as usual"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == Timer14Hold::Continue
    }
    #[doc = "Hold the timer counter for debug when the core is halted"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Timer14Hold::Stop
    }
}
#[doc = "Field `TIMER14_HOLD` writer - Timer 14 hold register"]
pub type Timer14HoldW<'a, REG> = crate::BitWriter<'a, REG, Timer14Hold>;
impl<'a, REG> Timer14HoldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continue running the timer as usual"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(Timer14Hold::Continue)
    }
    #[doc = "Hold the timer counter for debug when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Timer14Hold::Stop)
    }
}
#[doc = "Field `TIMER15_HOLD` reader - Timer 15 hold register"]
pub use Timer14HoldR as Timer15HoldR;
#[doc = "Field `TIMER16_HOLD` reader - Timer 16 hold register"]
pub use Timer14HoldR as Timer16HoldR;
#[doc = "Field `TIMER15_HOLD` writer - Timer 15 hold register"]
pub use Timer14HoldW as Timer15HoldW;
#[doc = "Field `TIMER16_HOLD` writer - Timer 16 hold register"]
pub use Timer14HoldW as Timer16HoldW;
impl R {
    #[doc = "Bit 10 - RTC hold register"]
    #[inline(always)]
    pub fn rtc_hold(&self) -> RtcHoldR {
        RtcHoldR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer 14 hold register"]
    #[inline(always)]
    pub fn timer14_hold(&self) -> Timer14HoldR {
        Timer14HoldR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer 15 hold register"]
    #[inline(always)]
    pub fn timer15_hold(&self) -> Timer15HoldR {
        Timer15HoldR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer 16 hold register"]
    #[inline(always)]
    pub fn timer16_hold(&self) -> Timer16HoldR {
        Timer16HoldR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - RTC hold register"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_hold(&mut self) -> RtcHoldW<Ctl1Spec> {
        RtcHoldW::new(self, 10)
    }
    #[doc = "Bit 16 - Timer 14 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn timer14_hold(&mut self) -> Timer14HoldW<Ctl1Spec> {
        Timer14HoldW::new(self, 16)
    }
    #[doc = "Bit 17 - Timer 15 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn timer15_hold(&mut self) -> Timer15HoldW<Ctl1Spec> {
        Timer15HoldW::new(self, 17)
    }
    #[doc = "Bit 18 - Timer 16 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn timer16_hold(&mut self) -> Timer16HoldW<Ctl1Spec> {
        Timer16HoldW::new(self, 18)
    }
}
#[doc = "Debug Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
