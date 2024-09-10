#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "IRC14M Internal 14M RC oscillator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc14men {
    #[doc = "0: Clock Off"]
    Off = 0,
    #[doc = "1: Clock On"]
    On = 1,
}
impl From<Irc14men> for bool {
    #[inline(always)]
    fn from(variant: Irc14men) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC14MEN` reader - IRC14M Internal 14M RC oscillator Enable"]
pub type Irc14menR = crate::BitReader<Irc14men>;
impl Irc14menR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irc14men {
        match self.bits {
            false => Irc14men::Off,
            true => Irc14men::On,
        }
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Irc14men::Off
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Irc14men::On
    }
}
#[doc = "Field `IRC14MEN` writer - IRC14M Internal 14M RC oscillator Enable"]
pub type Irc14menW<'a, REG> = crate::BitWriter<'a, REG, Irc14men>;
impl<'a, REG> Irc14menW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Irc14men::Off)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Irc14men::On)
    }
}
#[doc = "IRC14M Internal 14M RC Oscillator stabilization Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc14mstbr {
    #[doc = "0: IRC14M is not stable"]
    NotReady = 0,
    #[doc = "1: IRC14M is stable"]
    Ready = 1,
}
impl From<Irc14mstbr> for bool {
    #[inline(always)]
    fn from(variant: Irc14mstbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC14MSTB` reader - IRC14M Internal 14M RC Oscillator stabilization Flag"]
pub type Irc14mstbR = crate::BitReader<Irc14mstbr>;
impl Irc14mstbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irc14mstbr {
        match self.bits {
            false => Irc14mstbr::NotReady,
            true => Irc14mstbr::Ready,
        }
    }
    #[doc = "IRC14M is not stable"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Irc14mstbr::NotReady
    }
    #[doc = "IRC14M is stable"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Irc14mstbr::Ready
    }
}
#[doc = "Field `IRC14MADJ` reader - Internal 14M RC Oscillator clock trim adjust value"]
pub type Irc14madjR = crate::FieldReader;
#[doc = "Field `IRC14MADJ` writer - Internal 14M RC Oscillator clock trim adjust value"]
pub type Irc14madjW<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `IRC14MCALIB` reader - Internal 14M RC Oscillator calibration value register"]
pub type Irc14mcalibR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - IRC14M Internal 14M RC oscillator Enable"]
    #[inline(always)]
    pub fn irc14men(&self) -> Irc14menR {
        Irc14menR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRC14M Internal 14M RC Oscillator stabilization Flag"]
    #[inline(always)]
    pub fn irc14mstb(&self) -> Irc14mstbR {
        Irc14mstbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:7 - Internal 14M RC Oscillator clock trim adjust value"]
    #[inline(always)]
    pub fn irc14madj(&self) -> Irc14madjR {
        Irc14madjR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - Internal 14M RC Oscillator calibration value register"]
    #[inline(always)]
    pub fn irc14mcalib(&self) -> Irc14mcalibR {
        Irc14mcalibR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - IRC14M Internal 14M RC oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn irc14men(&mut self) -> Irc14menW<Ctl1Spec> {
        Irc14menW::new(self, 0)
    }
    #[doc = "Bits 3:7 - Internal 14M RC Oscillator clock trim adjust value"]
    #[inline(always)]
    #[must_use]
    pub fn irc14madj(&mut self) -> Irc14madjW<Ctl1Spec> {
        Irc14madjW::new(self, 3)
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
