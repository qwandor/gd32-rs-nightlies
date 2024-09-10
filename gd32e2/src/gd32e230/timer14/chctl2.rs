#[doc = "Register `CHCTL2` reader"]
pub type R = crate::R<Chctl2Spec>;
#[doc = "Register `CHCTL2` writer"]
pub type W = crate::W<Chctl2Spec>;
#[doc = "Capture/Compare 0 output enable"]
pub use crate::gd32e230::timer0::chctl2::Ch0en;
#[doc = "Field `CH0EN` reader - Capture/Compare 0 output enable"]
pub use crate::gd32e230::timer0::chctl2::Ch0enR;
#[doc = "Field `CH0NEN` reader - Capture/Compare 0 complementary output enable"]
pub use crate::gd32e230::timer0::chctl2::Ch0enR as Ch0nenR;
#[doc = "Field `CH1EN` reader - Capture/Compare 1 output enable"]
pub use crate::gd32e230::timer0::chctl2::Ch0enR as Ch1enR;
#[doc = "Field `CH0EN` writer - Capture/Compare 0 output enable"]
pub use crate::gd32e230::timer0::chctl2::Ch0enW;
#[doc = "Field `CH0NEN` writer - Capture/Compare 0 complementary output enable"]
pub use crate::gd32e230::timer0::chctl2::Ch0enW as Ch0nenW;
#[doc = "Field `CH1EN` writer - Capture/Compare 1 output enable"]
pub use crate::gd32e230::timer0::chctl2::Ch0enW as Ch1enW;
#[doc = "Capture/Compare 0 output Polarity"]
pub use crate::gd32e230::timer0::chctl2::Ch0p;
#[doc = "Field `CH0P` reader - Capture/Compare 0 output Polarity"]
pub use crate::gd32e230::timer0::chctl2::Ch0pR;
#[doc = "Field `CH0NP` reader - Capture/Compare 0 output Polarity"]
pub use crate::gd32e230::timer0::chctl2::Ch0pR as Ch0npR;
#[doc = "Field `CH1P` reader - Capture/Compare 1 output Polarity"]
pub use crate::gd32e230::timer0::chctl2::Ch0pR as Ch1pR;
#[doc = "Field `CH1NP` reader - Capture/Compare 1 output Polarity"]
pub use crate::gd32e230::timer0::chctl2::Ch0pR as Ch1npR;
#[doc = "Field `CH0P` writer - Capture/Compare 0 output Polarity"]
pub use crate::gd32e230::timer0::chctl2::Ch0pW;
#[doc = "Field `CH0NP` writer - Capture/Compare 0 output Polarity"]
pub use crate::gd32e230::timer0::chctl2::Ch0pW as Ch0npW;
#[doc = "Field `CH1P` writer - Capture/Compare 1 output Polarity"]
pub use crate::gd32e230::timer0::chctl2::Ch0pW as Ch1pW;
#[doc = "Field `CH1NP` writer - Capture/Compare 1 output Polarity"]
pub use crate::gd32e230::timer0::chctl2::Ch0pW as Ch1npW;
impl R {
    #[doc = "Bit 0 - Capture/Compare 0 output enable"]
    #[inline(always)]
    pub fn ch0en(&self) -> Ch0enR {
        Ch0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 0 output Polarity"]
    #[inline(always)]
    pub fn ch0p(&self) -> Ch0pR {
        Ch0pR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 0 complementary output enable"]
    #[inline(always)]
    pub fn ch0nen(&self) -> Ch0nenR {
        Ch0nenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 0 output Polarity"]
    #[inline(always)]
    pub fn ch0np(&self) -> Ch0npR {
        Ch0npR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 1 output enable"]
    #[inline(always)]
    pub fn ch1en(&self) -> Ch1enR {
        Ch1enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn ch1p(&self) -> Ch1pR {
        Ch1pR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn ch1np(&self) -> Ch1npR {
        Ch1npR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/Compare 0 output enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0en(&mut self) -> Ch0enW<Chctl2Spec> {
        Ch0enW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/Compare 0 output Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch0p(&mut self) -> Ch0pW<Chctl2Spec> {
        Ch0pW::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/Compare 0 complementary output enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0nen(&mut self) -> Ch0nenW<Chctl2Spec> {
        Ch0nenW::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/Compare 0 output Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch0np(&mut self) -> Ch0npW<Chctl2Spec> {
        Ch0npW::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/Compare 1 output enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1en(&mut self) -> Ch1enW<Chctl2Spec> {
        Ch1enW::new(self, 4)
    }
    #[doc = "Bit 5 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch1p(&mut self) -> Ch1pW<Chctl2Spec> {
        Ch1pW::new(self, 5)
    }
    #[doc = "Bit 7 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch1np(&mut self) -> Ch1npW<Chctl2Spec> {
        Ch1npW::new(self, 7)
    }
}
#[doc = "capture/compare enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chctl2Spec;
impl crate::RegisterSpec for Chctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctl2::R`](R) reader structure"]
impl crate::Readable for Chctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`chctl2::W`](W) writer structure"]
impl crate::Writable for Chctl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHCTL2 to value 0"]
impl crate::Resettable for Chctl2Spec {
    const RESET_VALUE: u32 = 0;
}
