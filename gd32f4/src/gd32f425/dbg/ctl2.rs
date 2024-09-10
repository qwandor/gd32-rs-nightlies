#[doc = "Register `CTL2` reader"]
pub type R = crate::R<Ctl2Spec>;
#[doc = "Register `CTL2` writer"]
pub type W = crate::W<Ctl2Spec>;
#[doc = "Field `TIMER0_HOLD` reader - TIMER 0 hold register"]
pub type Timer0HoldR = crate::BitReader;
#[doc = "Field `TIMER0_HOLD` writer - TIMER 0 hold register"]
pub type Timer0HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER7_HOLD` reader - TIMER 7 hold register"]
pub type Timer7HoldR = crate::BitReader;
#[doc = "Field `TIMER7_HOLD` writer - TIMER 7 hold register"]
pub type Timer7HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER8_HOLD` reader - TIMER 8 hold register"]
pub type Timer8HoldR = crate::BitReader;
#[doc = "Field `TIMER8_HOLD` writer - TIMER 8 hold register"]
pub type Timer8HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER9_HOLD` reader - TIMER 9 hold register"]
pub type Timer9HoldR = crate::BitReader;
#[doc = "Field `TIMER9_HOLD` writer - TIMER 9 hold register"]
pub type Timer9HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER10_HOLD` reader - TIMER 10 hold register"]
pub type Timer10HoldR = crate::BitReader;
#[doc = "Field `TIMER10_HOLD` writer - TIMER 10 hold register"]
pub type Timer10HoldW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIMER 0 hold register"]
    #[inline(always)]
    pub fn timer0_hold(&self) -> Timer0HoldR {
        Timer0HoldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIMER 7 hold register"]
    #[inline(always)]
    pub fn timer7_hold(&self) -> Timer7HoldR {
        Timer7HoldR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - TIMER 8 hold register"]
    #[inline(always)]
    pub fn timer8_hold(&self) -> Timer8HoldR {
        Timer8HoldR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIMER 9 hold register"]
    #[inline(always)]
    pub fn timer9_hold(&self) -> Timer9HoldR {
        Timer9HoldR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIMER 10 hold register"]
    #[inline(always)]
    pub fn timer10_hold(&self) -> Timer10HoldR {
        Timer10HoldR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIMER 0 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_hold(&mut self) -> Timer0HoldW<Ctl2Spec> {
        Timer0HoldW::new(self, 0)
    }
    #[doc = "Bit 1 - TIMER 7 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn timer7_hold(&mut self) -> Timer7HoldW<Ctl2Spec> {
        Timer7HoldW::new(self, 1)
    }
    #[doc = "Bit 16 - TIMER 8 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn timer8_hold(&mut self) -> Timer8HoldW<Ctl2Spec> {
        Timer8HoldW::new(self, 16)
    }
    #[doc = "Bit 17 - TIMER 9 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn timer9_hold(&mut self) -> Timer9HoldW<Ctl2Spec> {
        Timer9HoldW::new(self, 17)
    }
    #[doc = "Bit 18 - TIMER 10 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn timer10_hold(&mut self) -> Timer10HoldW<Ctl2Spec> {
        Timer10HoldW::new(self, 18)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl2Spec;
impl crate::RegisterSpec for Ctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl2::R`](R) reader structure"]
impl crate::Readable for Ctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl2::W`](W) writer structure"]
impl crate::Writable for Ctl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL2 to value 0"]
impl crate::Resettable for Ctl2Spec {
    const RESET_VALUE: u32 = 0;
}
