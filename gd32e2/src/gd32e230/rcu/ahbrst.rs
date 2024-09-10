#[doc = "Register `AHBRST` reader"]
pub type R = crate::R<AhbrstSpec>;
#[doc = "Register `AHBRST` writer"]
pub type W = crate::W<AhbrstSpec>;
#[doc = "GPIO port A reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Parst {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<Parst> for bool {
    #[inline(always)]
    fn from(variant: Parst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARST` reader - GPIO port A reset"]
pub type ParstR = crate::BitReader<Parst>;
impl ParstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Parst> {
        match self.bits {
            true => Some(Parst::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Parst::Reset
    }
}
#[doc = "Field `PARST` writer - GPIO port A reset"]
pub type ParstW<'a, REG> = crate::BitWriter<'a, REG, Parst>;
impl<'a, REG> ParstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Parst::Reset)
    }
}
#[doc = "Field `PBRST` reader - GPIO port B reset"]
pub use ParstR as PbrstR;
#[doc = "Field `PCRST` reader - GPIO port C reset"]
pub use ParstR as PcrstR;
#[doc = "Field `PFRST` reader - GPIO port F reset"]
pub use ParstR as PfrstR;
#[doc = "Field `PBRST` writer - GPIO port B reset"]
pub use ParstW as PbrstW;
#[doc = "Field `PCRST` writer - GPIO port C reset"]
pub use ParstW as PcrstW;
#[doc = "Field `PFRST` writer - GPIO port F reset"]
pub use ParstW as PfrstW;
impl R {
    #[doc = "Bit 17 - GPIO port A reset"]
    #[inline(always)]
    pub fn parst(&self) -> ParstR {
        ParstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO port B reset"]
    #[inline(always)]
    pub fn pbrst(&self) -> PbrstR {
        PbrstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO port C reset"]
    #[inline(always)]
    pub fn pcrst(&self) -> PcrstR {
        PcrstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO port F reset"]
    #[inline(always)]
    pub fn pfrst(&self) -> PfrstR {
        PfrstR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - GPIO port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn parst(&mut self) -> ParstW<AhbrstSpec> {
        ParstW::new(self, 17)
    }
    #[doc = "Bit 18 - GPIO port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst(&mut self) -> PbrstW<AhbrstSpec> {
        PbrstW::new(self, 18)
    }
    #[doc = "Bit 19 - GPIO port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst(&mut self) -> PcrstW<AhbrstSpec> {
        PcrstW::new(self, 19)
    }
    #[doc = "Bit 22 - GPIO port F reset"]
    #[inline(always)]
    #[must_use]
    pub fn pfrst(&mut self) -> PfrstW<AhbrstSpec> {
        PfrstW::new(self, 22)
    }
}
#[doc = "AHB reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbrstSpec;
impl crate::RegisterSpec for AhbrstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrst::R`](R) reader structure"]
impl crate::Readable for AhbrstSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbrst::W`](W) writer structure"]
impl crate::Writable for AhbrstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBRST to value 0"]
impl crate::Resettable for AhbrstSpec {
    const RESET_VALUE: u32 = 0;
}
