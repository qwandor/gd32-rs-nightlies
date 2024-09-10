#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Field `TIMER1_HOLD` reader - TIMER 1 hold register"]
pub type Timer1HoldR = crate::BitReader;
#[doc = "Field `TIMER1_HOLD` writer - TIMER 1 hold register"]
pub type Timer1HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_HOLD` reader - TIMER 2 hold register"]
pub type Timer2HoldR = crate::BitReader;
#[doc = "Field `TIMER2_HOLD` writer - TIMER 2 hold register"]
pub type Timer2HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER3_HOLD` reader - TIMER 3 hold register"]
pub type Timer3HoldR = crate::BitReader;
#[doc = "Field `TIMER3_HOLD` writer - TIMER 3 hold register"]
pub type Timer3HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER4_HOLD` reader - TIMER 4 hold register"]
pub type Timer4HoldR = crate::BitReader;
#[doc = "Field `TIMER4_HOLD` writer - TIMER 4 hold register"]
pub type Timer4HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER5_HOLD` reader - TIMER 5 hold register"]
pub type Timer5HoldR = crate::BitReader;
#[doc = "Field `TIMER5_HOLD` writer - TIMER 5 hold register"]
pub type Timer5HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER6_HOLD` reader - TIMER 6 hold register"]
pub type Timer6HoldR = crate::BitReader;
#[doc = "Field `TIMER6_HOLD` writer - TIMER 6 hold register"]
pub type Timer6HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER11_HOLD` reader - TIMER 11 hold register"]
pub type Timer11HoldR = crate::BitReader;
#[doc = "Field `TIMER11_HOLD` writer - TIMER 11 hold register"]
pub type Timer11HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER12_HOLD` reader - TIMER 12 hold register"]
pub type Timer12HoldR = crate::BitReader;
#[doc = "Field `TIMER12_HOLD` writer - TIMER 12 hold register"]
pub type Timer12HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER13_HOLD` reader - TIMER 13 hold register"]
pub type Timer13HoldR = crate::BitReader;
#[doc = "Field `TIMER13_HOLD` writer - TIMER 13 hold register"]
pub type Timer13HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_HOLD` reader - RTC hold register"]
pub type RtcHoldR = crate::BitReader;
#[doc = "Field `RTC_HOLD` writer - RTC hold register"]
pub type RtcHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGT_HOLD` reader - WWDGT hold register"]
pub type WwdgtHoldR = crate::BitReader;
#[doc = "Field `WWDGT_HOLD` writer - WWDGT hold register"]
pub type WwdgtHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FWDGT_HOLD` reader - FWDGT hold register"]
pub type FwdgtHoldR = crate::BitReader;
#[doc = "Field `FWDGT_HOLD` writer - FWDGT hold register"]
pub type FwdgtHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0_HOLD` reader - I2C0 hold register"]
pub type I2c0HoldR = crate::BitReader;
#[doc = "Field `I2C0_HOLD` writer - I2C0 hold register"]
pub type I2c0HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_HOLD` reader - I2C1 hold register"]
pub type I2c1HoldR = crate::BitReader;
#[doc = "Field `I2C1_HOLD` writer - I2C1 hold register"]
pub type I2c1HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_HOLD` reader - I2C2 hold register"]
pub type I2c2HoldR = crate::BitReader;
#[doc = "Field `I2C2_HOLD` writer - I2C2 hold register"]
pub type I2c2HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN0_HOLD` reader - CAN0 hold register"]
pub type Can0HoldR = crate::BitReader;
#[doc = "Field `CAN0_HOLD` writer - CAN0 hold register"]
pub type Can0HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN1_HOLD` reader - CAN1 hold register"]
pub type Can1HoldR = crate::BitReader;
#[doc = "Field `CAN1_HOLD` writer - CAN1 hold register"]
pub type Can1HoldW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIMER 1 hold register"]
    #[inline(always)]
    pub fn timer1_hold(&self) -> Timer1HoldR {
        Timer1HoldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIMER 2 hold register"]
    #[inline(always)]
    pub fn timer2_hold(&self) -> Timer2HoldR {
        Timer2HoldR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIMER 3 hold register"]
    #[inline(always)]
    pub fn timer3_hold(&self) -> Timer3HoldR {
        Timer3HoldR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIMER 4 hold register"]
    #[inline(always)]
    pub fn timer4_hold(&self) -> Timer4HoldR {
        Timer4HoldR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIMER 5 hold register"]
    #[inline(always)]
    pub fn timer5_hold(&self) -> Timer5HoldR {
        Timer5HoldR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIMER 6 hold register"]
    #[inline(always)]
    pub fn timer6_hold(&self) -> Timer6HoldR {
        Timer6HoldR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIMER 11 hold register"]
    #[inline(always)]
    pub fn timer11_hold(&self) -> Timer11HoldR {
        Timer11HoldR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIMER 12 hold register"]
    #[inline(always)]
    pub fn timer12_hold(&self) -> Timer12HoldR {
        Timer12HoldR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIMER 13 hold register"]
    #[inline(always)]
    pub fn timer13_hold(&self) -> Timer13HoldR {
        Timer13HoldR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC hold register"]
    #[inline(always)]
    pub fn rtc_hold(&self) -> RtcHoldR {
        RtcHoldR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDGT hold register"]
    #[inline(always)]
    pub fn wwdgt_hold(&self) -> WwdgtHoldR {
        WwdgtHoldR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FWDGT hold register"]
    #[inline(always)]
    pub fn fwdgt_hold(&self) -> FwdgtHoldR {
        FwdgtHoldR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C0 hold register"]
    #[inline(always)]
    pub fn i2c0_hold(&self) -> I2c0HoldR {
        I2c0HoldR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C1 hold register"]
    #[inline(always)]
    pub fn i2c1_hold(&self) -> I2c1HoldR {
        I2c1HoldR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C2 hold register"]
    #[inline(always)]
    pub fn i2c2_hold(&self) -> I2c2HoldR {
        I2c2HoldR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN0 hold register"]
    #[inline(always)]
    pub fn can0_hold(&self) -> Can0HoldR {
        Can0HoldR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CAN1 hold register"]
    #[inline(always)]
    pub fn can1_hold(&self) -> Can1HoldR {
        Can1HoldR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIMER 1 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_hold(&mut self) -> Timer1HoldW<Ctl1Spec> {
        Timer1HoldW::new(self, 0)
    }
    #[doc = "Bit 1 - TIMER 2 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_hold(&mut self) -> Timer2HoldW<Ctl1Spec> {
        Timer2HoldW::new(self, 1)
    }
    #[doc = "Bit 2 - TIMER 3 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn timer3_hold(&mut self) -> Timer3HoldW<Ctl1Spec> {
        Timer3HoldW::new(self, 2)
    }
    #[doc = "Bit 3 - TIMER 4 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn timer4_hold(&mut self) -> Timer4HoldW<Ctl1Spec> {
        Timer4HoldW::new(self, 3)
    }
    #[doc = "Bit 4 - TIMER 5 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn timer5_hold(&mut self) -> Timer5HoldW<Ctl1Spec> {
        Timer5HoldW::new(self, 4)
    }
    #[doc = "Bit 5 - TIMER 6 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn timer6_hold(&mut self) -> Timer6HoldW<Ctl1Spec> {
        Timer6HoldW::new(self, 5)
    }
    #[doc = "Bit 6 - TIMER 11 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn timer11_hold(&mut self) -> Timer11HoldW<Ctl1Spec> {
        Timer11HoldW::new(self, 6)
    }
    #[doc = "Bit 7 - TIMER 12 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn timer12_hold(&mut self) -> Timer12HoldW<Ctl1Spec> {
        Timer12HoldW::new(self, 7)
    }
    #[doc = "Bit 8 - TIMER 13 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn timer13_hold(&mut self) -> Timer13HoldW<Ctl1Spec> {
        Timer13HoldW::new(self, 8)
    }
    #[doc = "Bit 10 - RTC hold register"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_hold(&mut self) -> RtcHoldW<Ctl1Spec> {
        RtcHoldW::new(self, 10)
    }
    #[doc = "Bit 11 - WWDGT hold register"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgt_hold(&mut self) -> WwdgtHoldW<Ctl1Spec> {
        WwdgtHoldW::new(self, 11)
    }
    #[doc = "Bit 12 - FWDGT hold register"]
    #[inline(always)]
    #[must_use]
    pub fn fwdgt_hold(&mut self) -> FwdgtHoldW<Ctl1Spec> {
        FwdgtHoldW::new(self, 12)
    }
    #[doc = "Bit 21 - I2C0 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_hold(&mut self) -> I2c0HoldW<Ctl1Spec> {
        I2c0HoldW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C1 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_hold(&mut self) -> I2c1HoldW<Ctl1Spec> {
        I2c1HoldW::new(self, 22)
    }
    #[doc = "Bit 23 - I2C2 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_hold(&mut self) -> I2c2HoldW<Ctl1Spec> {
        I2c2HoldW::new(self, 23)
    }
    #[doc = "Bit 25 - CAN0 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn can0_hold(&mut self) -> Can0HoldW<Ctl1Spec> {
        Can0HoldW::new(self, 25)
    }
    #[doc = "Bit 26 - CAN1 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn can1_hold(&mut self) -> Can1HoldW<Ctl1Spec> {
        Can1HoldW::new(self, 26)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
