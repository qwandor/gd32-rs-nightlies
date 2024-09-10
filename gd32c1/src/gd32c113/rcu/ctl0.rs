#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Internal 8MHz RC oscillator Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc8men {
    #[doc = "0: Clock Off"]
    Off = 0,
    #[doc = "1: Clock On"]
    On = 1,
}
impl From<Irc8men> for bool {
    #[inline(always)]
    fn from(variant: Irc8men) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC8MEN` reader - Internal 8MHz RC oscillator Enable"]
pub type Irc8menR = crate::BitReader<Irc8men>;
impl Irc8menR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irc8men {
        match self.bits {
            false => Irc8men::Off,
            true => Irc8men::On,
        }
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Irc8men::Off
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Irc8men::On
    }
}
#[doc = "Field `IRC8MEN` writer - Internal 8MHz RC oscillator Enable"]
pub type Irc8menW<'a, REG> = crate::BitWriter<'a, REG, Irc8men>;
impl<'a, REG> Irc8menW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Irc8men::Off)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Irc8men::On)
    }
}
#[doc = "IRC8M Internal 8MHz RC Oscillator stabilization Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc8mstbr {
    #[doc = "0: IRC8M is not stable"]
    NotReady = 0,
    #[doc = "1: IRC8M is stable"]
    Ready = 1,
}
impl From<Irc8mstbr> for bool {
    #[inline(always)]
    fn from(variant: Irc8mstbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC8MSTB` reader - IRC8M Internal 8MHz RC Oscillator stabilization Flag"]
pub type Irc8mstbR = crate::BitReader<Irc8mstbr>;
impl Irc8mstbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irc8mstbr {
        match self.bits {
            false => Irc8mstbr::NotReady,
            true => Irc8mstbr::Ready,
        }
    }
    #[doc = "IRC8M is not stable"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Irc8mstbr::NotReady
    }
    #[doc = "IRC8M is stable"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Irc8mstbr::Ready
    }
}
#[doc = "Field `IRC8MADJ` reader - Internal 8MHz RC Oscillator clock trim adjust value"]
pub type Irc8madjR = crate::FieldReader;
#[doc = "Field `IRC8MADJ` writer - Internal 8MHz RC Oscillator clock trim adjust value"]
pub type Irc8madjW<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `IRC8MCALIB` reader - Internal 8MHz RC Oscillator calibration value register"]
pub type Irc8mcalibR = crate::FieldReader;
#[doc = "Field `HXTALEN` reader - External High Speed oscillator Enable"]
pub use Irc8menR as HxtalenR;
#[doc = "Field `HXTALEN` writer - External High Speed oscillator Enable"]
pub use Irc8menW as HxtalenW;
#[doc = "External crystal oscillator (HXTAL) clock stabilization flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hxtalstbr {
    #[doc = "0: HXTAL is not stable"]
    NotReady = 0,
    #[doc = "1: HXTAL is stable"]
    Ready = 1,
}
impl From<Hxtalstbr> for bool {
    #[inline(always)]
    fn from(variant: Hxtalstbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTALSTB` reader - External crystal oscillator (HXTAL) clock stabilization flag"]
pub type HxtalstbR = crate::BitReader<Hxtalstbr>;
impl HxtalstbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hxtalstbr {
        match self.bits {
            false => Hxtalstbr::NotReady,
            true => Hxtalstbr::Ready,
        }
    }
    #[doc = "HXTAL is not stable"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Hxtalstbr::NotReady
    }
    #[doc = "HXTAL is stable"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Hxtalstbr::Ready
    }
}
#[doc = "External crystal oscillator (HXTAL) clock bypass mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hxtalbps {
    #[doc = "0: HXTAL crystal oscillator not bypassed"]
    NotBypassed = 0,
    #[doc = "1: HXTAL crystal oscillator bypassed with external clock"]
    Bypassed = 1,
}
impl From<Hxtalbps> for bool {
    #[inline(always)]
    fn from(variant: Hxtalbps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTALBPS` reader - External crystal oscillator (HXTAL) clock bypass mode enable"]
pub type HxtalbpsR = crate::BitReader<Hxtalbps>;
impl HxtalbpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hxtalbps {
        match self.bits {
            false => Hxtalbps::NotBypassed,
            true => Hxtalbps::Bypassed,
        }
    }
    #[doc = "HXTAL crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == Hxtalbps::NotBypassed
    }
    #[doc = "HXTAL crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == Hxtalbps::Bypassed
    }
}
#[doc = "Field `HXTALBPS` writer - External crystal oscillator (HXTAL) clock bypass mode enable"]
pub type HxtalbpsW<'a, REG> = crate::BitWriter<'a, REG, Hxtalbps>;
impl<'a, REG> HxtalbpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HXTAL crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalbps::NotBypassed)
    }
    #[doc = "HXTAL crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalbps::Bypassed)
    }
}
#[doc = "HXTAL Clock Monitor Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ckmen {
    #[doc = "0: Clock monitor disabled"]
    Off = 0,
    #[doc = "1: Clock monitor enabled"]
    On = 1,
}
impl From<Ckmen> for bool {
    #[inline(always)]
    fn from(variant: Ckmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKMEN` reader - HXTAL Clock Monitor Enable"]
pub type CkmenR = crate::BitReader<Ckmen>;
impl CkmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckmen {
        match self.bits {
            false => Ckmen::Off,
            true => Ckmen::On,
        }
    }
    #[doc = "Clock monitor disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Ckmen::Off
    }
    #[doc = "Clock monitor enabled"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Ckmen::On
    }
}
#[doc = "Field `CKMEN` writer - HXTAL Clock Monitor Enable"]
pub type CkmenW<'a, REG> = crate::BitWriter<'a, REG, Ckmen>;
impl<'a, REG> CkmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock monitor disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Ckmen::Off)
    }
    #[doc = "Clock monitor enabled"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Ckmen::On)
    }
}
#[doc = "PLL enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllen {
    #[doc = "0: Clock Off"]
    Off = 0,
    #[doc = "1: Clock On"]
    On = 1,
}
impl From<Pllen> for bool {
    #[inline(always)]
    fn from(variant: Pllen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLEN` reader - PLL enable"]
pub type PllenR = crate::BitReader<Pllen>;
impl PllenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllen {
        match self.bits {
            false => Pllen::Off,
            true => Pllen::On,
        }
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pllen::Off
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pllen::On
    }
}
#[doc = "Field `PLLEN` writer - PLL enable"]
pub type PllenW<'a, REG> = crate::BitWriter<'a, REG, Pllen>;
impl<'a, REG> PllenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pllen::Off)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pllen::On)
    }
}
#[doc = "PLL Clock Stabilization Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllstbr {
    #[doc = "0: PLL is not stable"]
    NotReady = 0,
    #[doc = "1: PLL is stable"]
    Ready = 1,
}
impl From<Pllstbr> for bool {
    #[inline(always)]
    fn from(variant: Pllstbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSTB` reader - PLL Clock Stabilization Flag"]
pub type PllstbR = crate::BitReader<Pllstbr>;
impl PllstbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllstbr {
        match self.bits {
            false => Pllstbr::NotReady,
            true => Pllstbr::Ready,
        }
    }
    #[doc = "PLL is not stable"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Pllstbr::NotReady
    }
    #[doc = "PLL is stable"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Pllstbr::Ready
    }
}
#[doc = "Field `PLL1EN` reader - PLL1 enable"]
pub use PllenR as Pll1enR;
#[doc = "Field `PLL2EN` reader - PLL2 enable"]
pub use PllenR as Pll2enR;
#[doc = "Field `PLL1EN` writer - PLL1 enable"]
pub use PllenW as Pll1enW;
#[doc = "Field `PLL2EN` writer - PLL2 enable"]
pub use PllenW as Pll2enW;
#[doc = "Field `PLL1STB` reader - PLL1 Clock Stabilization Flag"]
pub use PllstbR as Pll1stbR;
#[doc = "Field `PLL2STB` reader - PLL2 Clock Stabilization Flag"]
pub use PllstbR as Pll2stbR;
impl R {
    #[doc = "Bit 0 - Internal 8MHz RC oscillator Enable"]
    #[inline(always)]
    pub fn irc8men(&self) -> Irc8menR {
        Irc8menR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRC8M Internal 8MHz RC Oscillator stabilization Flag"]
    #[inline(always)]
    pub fn irc8mstb(&self) -> Irc8mstbR {
        Irc8mstbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:7 - Internal 8MHz RC Oscillator clock trim adjust value"]
    #[inline(always)]
    pub fn irc8madj(&self) -> Irc8madjR {
        Irc8madjR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - Internal 8MHz RC Oscillator calibration value register"]
    #[inline(always)]
    pub fn irc8mcalib(&self) -> Irc8mcalibR {
        Irc8mcalibR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - External High Speed oscillator Enable"]
    #[inline(always)]
    pub fn hxtalen(&self) -> HxtalenR {
        HxtalenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - External crystal oscillator (HXTAL) clock stabilization flag"]
    #[inline(always)]
    pub fn hxtalstb(&self) -> HxtalstbR {
        HxtalstbR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - External crystal oscillator (HXTAL) clock bypass mode enable"]
    #[inline(always)]
    pub fn hxtalbps(&self) -> HxtalbpsR {
        HxtalbpsR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HXTAL Clock Monitor Enable"]
    #[inline(always)]
    pub fn ckmen(&self) -> CkmenR {
        CkmenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    pub fn pllen(&self) -> PllenR {
        PllenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PLL Clock Stabilization Flag"]
    #[inline(always)]
    pub fn pllstb(&self) -> PllstbR {
        PllstbR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PLL1 enable"]
    #[inline(always)]
    pub fn pll1en(&self) -> Pll1enR {
        Pll1enR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PLL1 Clock Stabilization Flag"]
    #[inline(always)]
    pub fn pll1stb(&self) -> Pll1stbR {
        Pll1stbR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PLL2 enable"]
    #[inline(always)]
    pub fn pll2en(&self) -> Pll2enR {
        Pll2enR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PLL2 Clock Stabilization Flag"]
    #[inline(always)]
    pub fn pll2stb(&self) -> Pll2stbR {
        Pll2stbR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal 8MHz RC oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn irc8men(&mut self) -> Irc8menW<Ctl0Spec> {
        Irc8menW::new(self, 0)
    }
    #[doc = "Bits 3:7 - Internal 8MHz RC Oscillator clock trim adjust value"]
    #[inline(always)]
    #[must_use]
    pub fn irc8madj(&mut self) -> Irc8madjW<Ctl0Spec> {
        Irc8madjW::new(self, 3)
    }
    #[doc = "Bit 16 - External High Speed oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hxtalen(&mut self) -> HxtalenW<Ctl0Spec> {
        HxtalenW::new(self, 16)
    }
    #[doc = "Bit 18 - External crystal oscillator (HXTAL) clock bypass mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn hxtalbps(&mut self) -> HxtalbpsW<Ctl0Spec> {
        HxtalbpsW::new(self, 18)
    }
    #[doc = "Bit 19 - HXTAL Clock Monitor Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ckmen(&mut self) -> CkmenW<Ctl0Spec> {
        CkmenW::new(self, 19)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllen(&mut self) -> PllenW<Ctl0Spec> {
        PllenW::new(self, 24)
    }
    #[doc = "Bit 26 - PLL1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll1en(&mut self) -> Pll1enW<Ctl0Spec> {
        Pll1enW::new(self, 26)
    }
    #[doc = "Bit 28 - PLL2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll2en(&mut self) -> Pll2enW<Ctl0Spec> {
        Pll2enW::new(self, 28)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0x83"]
impl crate::Resettable for Ctl0Spec {
    const RESET_VALUE: u32 = 0x83;
}
