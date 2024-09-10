#[doc = "Register `WSEN` reader"]
pub type R = crate::R<WsenSpec>;
#[doc = "Register `WSEN` writer"]
pub type W = crate::W<WsenSpec>;
#[doc = "FMC wait state enable register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wsen {
    #[doc = "0: No wait state added"]
    NoWaitState = 0,
    #[doc = "1: Wait state added"]
    WaitState = 1,
}
impl From<Wsen> for bool {
    #[inline(always)]
    fn from(variant: Wsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WSEN` reader - FMC wait state enable register"]
pub type WsenR = crate::BitReader<Wsen>;
impl WsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wsen {
        match self.bits {
            false => Wsen::NoWaitState,
            true => Wsen::WaitState,
        }
    }
    #[doc = "No wait state added"]
    #[inline(always)]
    pub fn is_no_wait_state(&self) -> bool {
        *self == Wsen::NoWaitState
    }
    #[doc = "Wait state added"]
    #[inline(always)]
    pub fn is_wait_state(&self) -> bool {
        *self == Wsen::WaitState
    }
}
#[doc = "Field `WSEN` writer - FMC wait state enable register"]
pub type WsenW<'a, REG> = crate::BitWriter<'a, REG, Wsen>;
impl<'a, REG> WsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No wait state added"]
    #[inline(always)]
    pub fn no_wait_state(self) -> &'a mut crate::W<REG> {
        self.variant(Wsen::NoWaitState)
    }
    #[doc = "Wait state added"]
    #[inline(always)]
    pub fn wait_state(self) -> &'a mut crate::W<REG> {
        self.variant(Wsen::WaitState)
    }
}
impl R {
    #[doc = "Bit 0 - FMC wait state enable register"]
    #[inline(always)]
    pub fn wsen(&self) -> WsenR {
        WsenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FMC wait state enable register"]
    #[inline(always)]
    #[must_use]
    pub fn wsen(&mut self) -> WsenW<WsenSpec> {
        WsenW::new(self, 0)
    }
}
#[doc = "Wait state enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wsen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wsen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WsenSpec;
impl crate::RegisterSpec for WsenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wsen::R`](R) reader structure"]
impl crate::Readable for WsenSpec {}
#[doc = "`write(|w| ..)` method takes [`wsen::W`](W) writer structure"]
impl crate::Writable for WsenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WSEN to value 0"]
impl crate::Resettable for WsenSpec {
    const RESET_VALUE: u32 = 0;
}
