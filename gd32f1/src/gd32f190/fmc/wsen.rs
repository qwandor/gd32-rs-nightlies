#[doc = "Register `WSEN` reader"]
pub type R = crate::R<WsenSpec>;
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
#[doc = "Field `BPEN` reader - FMC bit program enable register"]
pub type BpenR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - FMC wait state enable register"]
    #[inline(always)]
    pub fn wsen(&self) -> WsenR {
        WsenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FMC bit program enable register"]
    #[inline(always)]
    pub fn bpen(&self) -> BpenR {
        BpenR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Flash wait state control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wsen::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WsenSpec;
impl crate::RegisterSpec for WsenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wsen::R`](R) reader structure"]
impl crate::Readable for WsenSpec {}
#[doc = "`reset()` method sets WSEN to value 0"]
impl crate::Resettable for WsenSpec {
    const RESET_VALUE: u32 = 0;
}
