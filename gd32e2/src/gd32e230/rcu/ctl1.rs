#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "IRC28M Internal 28M RC oscillator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc28men {
    #[doc = "0: Clock Off"]
    Off = 0,
    #[doc = "1: Clock On"]
    On = 1,
}
impl From<Irc28men> for bool {
    #[inline(always)]
    fn from(variant: Irc28men) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC28MEN` reader - IRC28M Internal 28M RC oscillator Enable"]
pub type Irc28menR = crate::BitReader<Irc28men>;
impl Irc28menR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irc28men {
        match self.bits {
            false => Irc28men::Off,
            true => Irc28men::On,
        }
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Irc28men::Off
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Irc28men::On
    }
}
#[doc = "Field `IRC28MEN` writer - IRC28M Internal 28M RC oscillator Enable"]
pub type Irc28menW<'a, REG> = crate::BitWriter<'a, REG, Irc28men>;
impl<'a, REG> Irc28menW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Irc28men::Off)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Irc28men::On)
    }
}
#[doc = "IRC28M Internal 28M RC Oscillator stabilization Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc28mstbr {
    #[doc = "0: IRC14M is not stable"]
    NotReady = 0,
    #[doc = "1: IRC14M is stable"]
    Ready = 1,
}
impl From<Irc28mstbr> for bool {
    #[inline(always)]
    fn from(variant: Irc28mstbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC28MSTB` reader - IRC28M Internal 28M RC Oscillator stabilization Flag"]
pub type Irc28mstbR = crate::BitReader<Irc28mstbr>;
impl Irc28mstbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irc28mstbr {
        match self.bits {
            false => Irc28mstbr::NotReady,
            true => Irc28mstbr::Ready,
        }
    }
    #[doc = "IRC14M is not stable"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Irc28mstbr::NotReady
    }
    #[doc = "IRC14M is stable"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Irc28mstbr::Ready
    }
}
#[doc = "Field `IRC28MADJ` reader - Internal 28M RC Oscillator clock trim adjust value"]
pub type Irc28madjR = crate::FieldReader;
#[doc = "Field `IRC28MADJ` writer - Internal 28M RC Oscillator clock trim adjust value"]
pub type Irc28madjW<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `IRC28MCALIB` reader - Internal 28M RC Oscillator calibration value register"]
pub type Irc28mcalibR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - IRC28M Internal 28M RC oscillator Enable"]
    #[inline(always)]
    pub fn irc28men(&self) -> Irc28menR {
        Irc28menR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRC28M Internal 28M RC Oscillator stabilization Flag"]
    #[inline(always)]
    pub fn irc28mstb(&self) -> Irc28mstbR {
        Irc28mstbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:7 - Internal 28M RC Oscillator clock trim adjust value"]
    #[inline(always)]
    pub fn irc28madj(&self) -> Irc28madjR {
        Irc28madjR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - Internal 28M RC Oscillator calibration value register"]
    #[inline(always)]
    pub fn irc28mcalib(&self) -> Irc28mcalibR {
        Irc28mcalibR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - IRC28M Internal 28M RC oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn irc28men(&mut self) -> Irc28menW<Ctl1Spec> {
        Irc28menW::new(self, 0)
    }
    #[doc = "Bits 3:7 - Internal 28M RC Oscillator clock trim adjust value"]
    #[inline(always)]
    #[must_use]
    pub fn irc28madj(&mut self) -> Irc28madjW<Ctl1Spec> {
        Irc28madjW::new(self, 3)
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
#[doc = "`reset()` method sets CTL1 to value 0x80"]
impl crate::Resettable for Ctl1Spec {
    const RESET_VALUE: u32 = 0x80;
}
