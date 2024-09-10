#[doc = "Register `SWEVG` writer"]
pub type W = crate::W<SwevgSpec>;
#[doc = "Channel 0 capture or compare event generation"]
pub use crate::gd32f190::timer0::swevg::Ch0g;
#[doc = "Field `CH0G` writer - Channel 0 capture or compare event generation"]
pub use crate::gd32f190::timer0::swevg::Ch0gW;
#[doc = "Field `CH1G` writer - Channel 1 capture or compare event generation"]
pub use crate::gd32f190::timer0::swevg::Ch0gW as Ch1gW;
#[doc = "Channel commutation event generation"]
pub use crate::gd32f190::timer0::swevg::Cmtg;
#[doc = "Field `CMTG` writer - Channel commutation event generation"]
pub use crate::gd32f190::timer0::swevg::CmtgW;
#[doc = "Update generation"]
pub use crate::gd32f190::timer0::swevg::Upg;
#[doc = "Field `UPG` writer - Update generation"]
pub use crate::gd32f190::timer0::swevg::UpgW;
#[doc = "Trigger generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trgg {
    #[doc = "1: Generate a trigger event"]
    Trigger = 1,
}
impl From<Trgg> for bool {
    #[inline(always)]
    fn from(variant: Trgg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGG` writer - Trigger generation"]
pub type TrggW<'a, REG> = crate::BitWriter<'a, REG, Trgg>;
impl<'a, REG> TrggW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate a trigger event"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgg::Trigger)
    }
}
#[doc = "Break generation"]
pub use crate::gd32f190::timer0::swevg::Brkg;
#[doc = "Field `BRKG` writer - Break generation"]
pub use crate::gd32f190::timer0::swevg::BrkgW;
impl W {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    #[must_use]
    pub fn upg(&mut self) -> UpgW<SwevgSpec> {
        UpgW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 0 capture or compare event generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch0g(&mut self) -> Ch0gW<SwevgSpec> {
        Ch0gW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 1 capture or compare event generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch1g(&mut self) -> Ch1gW<SwevgSpec> {
        Ch1gW::new(self, 2)
    }
    #[doc = "Bit 5 - Channel commutation event generation"]
    #[inline(always)]
    #[must_use]
    pub fn cmtg(&mut self) -> CmtgW<SwevgSpec> {
        CmtgW::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger generation"]
    #[inline(always)]
    #[must_use]
    pub fn trgg(&mut self) -> TrggW<SwevgSpec> {
        TrggW::new(self, 6)
    }
    #[doc = "Bit 7 - Break generation"]
    #[inline(always)]
    #[must_use]
    pub fn brkg(&mut self) -> BrkgW<SwevgSpec> {
        BrkgW::new(self, 7)
    }
}
#[doc = "Software event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swevg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwevgSpec;
impl crate::RegisterSpec for SwevgSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`swevg::W`](W) writer structure"]
impl crate::Writable for SwevgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SWEVG to value 0"]
impl crate::Resettable for SwevgSpec {
    const RESET_VALUE: u16 = 0;
}
