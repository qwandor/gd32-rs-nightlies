#[doc = "Register `CFG1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "Field `PLLSAIRDIV` reader - The divider factor from PLLSAIR clock"]
pub type PllsairdivR = crate::FieldReader;
#[doc = "Field `PLLSAIRDIV` writer - The divider factor from PLLSAIR clock"]
pub type PllsairdivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMERSEL` reader - TIMER clock selection"]
pub type TimerselR = crate::BitReader;
#[doc = "Field `TIMERSEL` writer - TIMER clock selection"]
pub type TimerselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:17 - The divider factor from PLLSAIR clock"]
    #[inline(always)]
    pub fn pllsairdiv(&self) -> PllsairdivR {
        PllsairdivR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - TIMER clock selection"]
    #[inline(always)]
    pub fn timersel(&self) -> TimerselR {
        TimerselR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - The divider factor from PLLSAIR clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllsairdiv(&mut self) -> PllsairdivW<Cfg1Spec> {
        PllsairdivW::new(self, 16)
    }
    #[doc = "Bit 24 - TIMER clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn timersel(&mut self) -> TimerselW<Cfg1Spec> {
        TimerselW::new(self, 24)
    }
}
#[doc = "Clock Configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg1Spec;
impl crate::RegisterSpec for Cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for Cfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for Cfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for Cfg1Spec {
    const RESET_VALUE: u32 = 0;
}
