#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Sleep mode hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlpHold {
    #[doc = "0: No effect"]
    Disabled = 0,
    #[doc = "1: In sleep mode the AHB clock is on"]
    Enabled = 1,
}
impl From<SlpHold> for bool {
    #[inline(always)]
    fn from(variant: SlpHold) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLP_HOLD` reader - Sleep mode hold register"]
pub type SlpHoldR = crate::BitReader<SlpHold>;
impl SlpHoldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SlpHold {
        match self.bits {
            false => SlpHold::Disabled,
            true => SlpHold::Enabled,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SlpHold::Disabled
    }
    #[doc = "In sleep mode the AHB clock is on"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SlpHold::Enabled
    }
}
#[doc = "Field `SLP_HOLD` writer - Sleep mode hold register"]
pub type SlpHoldW<'a, REG> = crate::BitWriter<'a, REG, SlpHold>;
impl<'a, REG> SlpHoldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SlpHold::Disabled)
    }
    #[doc = "In sleep mode the AHB clock is on"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SlpHold::Enabled)
    }
}
#[doc = "Deep-sleep mode hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DslpHold {
    #[doc = "0: No effect"]
    Disabled = 0,
    #[doc = "1: In deep-sleep mode the AHB clock and system clock are provided by IRC8M"]
    Enabled = 1,
}
impl From<DslpHold> for bool {
    #[inline(always)]
    fn from(variant: DslpHold) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSLP_HOLD` reader - Deep-sleep mode hold register"]
pub type DslpHoldR = crate::BitReader<DslpHold>;
impl DslpHoldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DslpHold {
        match self.bits {
            false => DslpHold::Disabled,
            true => DslpHold::Enabled,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DslpHold::Disabled
    }
    #[doc = "In deep-sleep mode the AHB clock and system clock are provided by IRC8M"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DslpHold::Enabled
    }
}
#[doc = "Field `DSLP_HOLD` writer - Deep-sleep mode hold register"]
pub type DslpHoldW<'a, REG> = crate::BitWriter<'a, REG, DslpHold>;
impl<'a, REG> DslpHoldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DslpHold::Disabled)
    }
    #[doc = "In deep-sleep mode the AHB clock and system clock are provided by IRC8M"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DslpHold::Enabled)
    }
}
#[doc = "Standby mode hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StbHold {
    #[doc = "0: No effect"]
    Disabled = 0,
    #[doc = "1: In standby mode the AHB clock and system clock are provided by IRC8M"]
    Enabled = 1,
}
impl From<StbHold> for bool {
    #[inline(always)]
    fn from(variant: StbHold) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STB_HOLD` reader - Standby mode hold register"]
pub type StbHoldR = crate::BitReader<StbHold>;
impl StbHoldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StbHold {
        match self.bits {
            false => StbHold::Disabled,
            true => StbHold::Enabled,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == StbHold::Disabled
    }
    #[doc = "In standby mode the AHB clock and system clock are provided by IRC8M"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == StbHold::Enabled
    }
}
#[doc = "Field `STB_HOLD` writer - Standby mode hold register"]
pub type StbHoldW<'a, REG> = crate::BitWriter<'a, REG, StbHold>;
impl<'a, REG> StbHoldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(StbHold::Disabled)
    }
    #[doc = "In standby mode the AHB clock and system clock are provided by IRC8M"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(StbHold::Enabled)
    }
}
#[doc = "FWDGT hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FwdgtHold {
    #[doc = "0: Continue running the watchdog timer as usual"]
    Continue = 0,
    #[doc = "1: Hold the watchdog timer for debug when the core is halted"]
    Stop = 1,
}
impl From<FwdgtHold> for bool {
    #[inline(always)]
    fn from(variant: FwdgtHold) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWDGT_HOLD` reader - FWDGT hold register"]
pub type FwdgtHoldR = crate::BitReader<FwdgtHold>;
impl FwdgtHoldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FwdgtHold {
        match self.bits {
            false => FwdgtHold::Continue,
            true => FwdgtHold::Stop,
        }
    }
    #[doc = "Continue running the watchdog timer as usual"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == FwdgtHold::Continue
    }
    #[doc = "Hold the watchdog timer for debug when the core is halted"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == FwdgtHold::Stop
    }
}
#[doc = "Field `FWDGT_HOLD` writer - FWDGT hold register"]
pub type FwdgtHoldW<'a, REG> = crate::BitWriter<'a, REG, FwdgtHold>;
impl<'a, REG> FwdgtHoldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continue running the watchdog timer as usual"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(FwdgtHold::Continue)
    }
    #[doc = "Hold the watchdog timer for debug when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(FwdgtHold::Stop)
    }
}
#[doc = "Field `WWDGT_HOLD` reader - WWDGT hold register"]
pub use FwdgtHoldR as WwdgtHoldR;
#[doc = "Field `WWDGT_HOLD` writer - WWDGT hold register"]
pub use FwdgtHoldW as WwdgtHoldW;
#[doc = "Timer 0 hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0Hold {
    #[doc = "0: Continue running the timer as usual"]
    Continue = 0,
    #[doc = "1: Hold the timer counter for debug when the core is halted"]
    Stop = 1,
}
impl From<Timer0Hold> for bool {
    #[inline(always)]
    fn from(variant: Timer0Hold) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER0_HOLD` reader - Timer 0 hold register"]
pub type Timer0HoldR = crate::BitReader<Timer0Hold>;
impl Timer0HoldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0Hold {
        match self.bits {
            false => Timer0Hold::Continue,
            true => Timer0Hold::Stop,
        }
    }
    #[doc = "Continue running the timer as usual"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == Timer0Hold::Continue
    }
    #[doc = "Hold the timer counter for debug when the core is halted"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Timer0Hold::Stop
    }
}
#[doc = "Field `TIMER0_HOLD` writer - Timer 0 hold register"]
pub type Timer0HoldW<'a, REG> = crate::BitWriter<'a, REG, Timer0Hold>;
impl<'a, REG> Timer0HoldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continue running the timer as usual"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0Hold::Continue)
    }
    #[doc = "Hold the timer counter for debug when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0Hold::Stop)
    }
}
#[doc = "Field `TIMER1_HOLD` reader - Timer 1 hold register"]
pub use Timer0HoldR as Timer1HoldR;
#[doc = "Field `TIMER2_HOLD` reader - Timer 2 hold register"]
pub use Timer0HoldR as Timer2HoldR;
#[doc = "Field `TIMER1_HOLD` writer - Timer 1 hold register"]
pub use Timer0HoldW as Timer1HoldW;
#[doc = "Field `TIMER2_HOLD` writer - Timer 2 hold register"]
pub use Timer0HoldW as Timer2HoldW;
#[doc = "CAN 0 hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Can0Hold {
    #[doc = "0: Continue running CAN as usual"]
    Continue = 0,
    #[doc = "1: Hold the CAN for debug when the core is halted"]
    Stop = 1,
}
impl From<Can0Hold> for bool {
    #[inline(always)]
    fn from(variant: Can0Hold) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAN0_HOLD` reader - CAN 0 hold register"]
pub type Can0HoldR = crate::BitReader<Can0Hold>;
impl Can0HoldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Can0Hold {
        match self.bits {
            false => Can0Hold::Continue,
            true => Can0Hold::Stop,
        }
    }
    #[doc = "Continue running CAN as usual"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == Can0Hold::Continue
    }
    #[doc = "Hold the CAN for debug when the core is halted"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Can0Hold::Stop
    }
}
#[doc = "Field `CAN0_HOLD` writer - CAN 0 hold register"]
pub type Can0HoldW<'a, REG> = crate::BitWriter<'a, REG, Can0Hold>;
impl<'a, REG> Can0HoldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continue running CAN as usual"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(Can0Hold::Continue)
    }
    #[doc = "Hold the CAN for debug when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Can0Hold::Stop)
    }
}
#[doc = "I2C0 hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c0Hold {
    #[doc = "0: Continue running I2C as usual"]
    Continue = 0,
    #[doc = "1: Hold the I2C timeout for debug when the core is halted"]
    Stop = 1,
}
impl From<I2c0Hold> for bool {
    #[inline(always)]
    fn from(variant: I2c0Hold) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C0_HOLD` reader - I2C0 hold register"]
pub type I2c0HoldR = crate::BitReader<I2c0Hold>;
impl I2c0HoldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c0Hold {
        match self.bits {
            false => I2c0Hold::Continue,
            true => I2c0Hold::Stop,
        }
    }
    #[doc = "Continue running I2C as usual"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == I2c0Hold::Continue
    }
    #[doc = "Hold the I2C timeout for debug when the core is halted"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == I2c0Hold::Stop
    }
}
#[doc = "Field `I2C0_HOLD` writer - I2C0 hold register"]
pub type I2c0HoldW<'a, REG> = crate::BitWriter<'a, REG, I2c0Hold>;
impl<'a, REG> I2c0HoldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continue running I2C as usual"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(I2c0Hold::Continue)
    }
    #[doc = "Hold the I2C timeout for debug when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(I2c0Hold::Stop)
    }
}
#[doc = "Field `CAN1_HOLD` reader - CAN1 hold register"]
pub use Can0HoldR as Can1HoldR;
#[doc = "Field `CAN1_HOLD` writer - CAN1 hold register"]
pub use Can0HoldW as Can1HoldW;
#[doc = "Field `I2C1_HOLD` reader - I2C1 hold register"]
pub use I2c0HoldR as I2c1HoldR;
#[doc = "Field `I2C2_HOLD` reader - I2C2 hold register"]
pub use I2c0HoldR as I2c2HoldR;
#[doc = "Field `I2C1_HOLD` writer - I2C1 hold register"]
pub use I2c0HoldW as I2c1HoldW;
#[doc = "Field `I2C2_HOLD` writer - I2C2 hold register"]
pub use I2c0HoldW as I2c2HoldW;
#[doc = "Field `TIMER5_HOLD` reader - Timer 5 hold register"]
pub use Timer0HoldR as Timer5HoldR;
#[doc = "Field `TIMER13_HOLD` reader - Timer 13 hold register"]
pub use Timer0HoldR as Timer13HoldR;
#[doc = "Field `TIMER5_HOLD` writer - Timer 5 hold register"]
pub use Timer0HoldW as Timer5HoldW;
#[doc = "Field `TIMER13_HOLD` writer - Timer 13 hold register"]
pub use Timer0HoldW as Timer13HoldW;
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
    #[doc = "Bit 8 - FWDGT hold register"]
    #[inline(always)]
    pub fn fwdgt_hold(&self) -> FwdgtHoldR {
        FwdgtHoldR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WWDGT hold register"]
    #[inline(always)]
    pub fn wwdgt_hold(&self) -> WwdgtHoldR {
        WwdgtHoldR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Timer 0 hold register"]
    #[inline(always)]
    pub fn timer0_hold(&self) -> Timer0HoldR {
        Timer0HoldR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer 1 hold register"]
    #[inline(always)]
    pub fn timer1_hold(&self) -> Timer1HoldR {
        Timer1HoldR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timer 2 hold register"]
    #[inline(always)]
    pub fn timer2_hold(&self) -> Timer2HoldR {
        Timer2HoldR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - CAN 0 hold register"]
    #[inline(always)]
    pub fn can0_hold(&self) -> Can0HoldR {
        Can0HoldR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C0 hold register"]
    #[inline(always)]
    pub fn i2c0_hold(&self) -> I2c0HoldR {
        I2c0HoldR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - I2C1 hold register"]
    #[inline(always)]
    pub fn i2c1_hold(&self) -> I2c1HoldR {
        I2c1HoldR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - I2C2 hold register"]
    #[inline(always)]
    pub fn i2c2_hold(&self) -> I2c2HoldR {
        I2c2HoldR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer 5 hold register"]
    #[inline(always)]
    pub fn timer5_hold(&self) -> Timer5HoldR {
        Timer5HoldR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - CAN1 hold register"]
    #[inline(always)]
    pub fn can1_hold(&self) -> Can1HoldR {
        Can1HoldR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 27 - Timer 13 hold register"]
    #[inline(always)]
    pub fn timer13_hold(&self) -> Timer13HoldR {
        Timer13HoldR::new(((self.bits >> 27) & 1) != 0)
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
    #[doc = "Bit 8 - FWDGT hold register"]
    #[inline(always)]
    #[must_use]
    pub fn fwdgt_hold(&mut self) -> FwdgtHoldW<Ctl0Spec> {
        FwdgtHoldW::new(self, 8)
    }
    #[doc = "Bit 9 - WWDGT hold register"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgt_hold(&mut self) -> WwdgtHoldW<Ctl0Spec> {
        WwdgtHoldW::new(self, 9)
    }
    #[doc = "Bit 10 - Timer 0 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_hold(&mut self) -> Timer0HoldW<Ctl0Spec> {
        Timer0HoldW::new(self, 10)
    }
    #[doc = "Bit 11 - Timer 1 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_hold(&mut self) -> Timer1HoldW<Ctl0Spec> {
        Timer1HoldW::new(self, 11)
    }
    #[doc = "Bit 12 - Timer 2 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_hold(&mut self) -> Timer2HoldW<Ctl0Spec> {
        Timer2HoldW::new(self, 12)
    }
    #[doc = "Bit 14 - CAN 0 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn can0_hold(&mut self) -> Can0HoldW<Ctl0Spec> {
        Can0HoldW::new(self, 14)
    }
    #[doc = "Bit 15 - I2C0 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_hold(&mut self) -> I2c0HoldW<Ctl0Spec> {
        I2c0HoldW::new(self, 15)
    }
    #[doc = "Bit 16 - I2C1 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_hold(&mut self) -> I2c1HoldW<Ctl0Spec> {
        I2c1HoldW::new(self, 16)
    }
    #[doc = "Bit 17 - I2C2 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_hold(&mut self) -> I2c2HoldW<Ctl0Spec> {
        I2c2HoldW::new(self, 17)
    }
    #[doc = "Bit 19 - Timer 5 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn timer5_hold(&mut self) -> Timer5HoldW<Ctl0Spec> {
        Timer5HoldW::new(self, 19)
    }
    #[doc = "Bit 21 - CAN1 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn can1_hold(&mut self) -> Can1HoldW<Ctl0Spec> {
        Can1HoldW::new(self, 21)
    }
    #[doc = "Bit 27 - Timer 13 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn timer13_hold(&mut self) -> Timer13HoldW<Ctl0Spec> {
        Timer13HoldW::new(self, 27)
    }
}
#[doc = "Debug Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
