#[doc = "Register `RSTSCK` reader"]
pub type R = crate::R<RstsckSpec>;
#[doc = "Register `RSTSCK` writer"]
pub type W = crate::W<RstsckSpec>;
#[doc = "IRC40K enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc40ken {
    #[doc = "0: IRC40K oscillator disabled"]
    Off = 0,
    #[doc = "1: IRC40K oscillator enabled"]
    On = 1,
}
impl From<Irc40ken> for bool {
    #[inline(always)]
    fn from(variant: Irc40ken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC40KEN` reader - IRC40K enable"]
pub type Irc40kenR = crate::BitReader<Irc40ken>;
impl Irc40kenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irc40ken {
        match self.bits {
            false => Irc40ken::Off,
            true => Irc40ken::On,
        }
    }
    #[doc = "IRC40K oscillator disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Irc40ken::Off
    }
    #[doc = "IRC40K oscillator enabled"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Irc40ken::On
    }
}
#[doc = "Field `IRC40KEN` writer - IRC40K enable"]
pub type Irc40kenW<'a, REG> = crate::BitWriter<'a, REG, Irc40ken>;
impl<'a, REG> Irc40kenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IRC40K oscillator disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Irc40ken::Off)
    }
    #[doc = "IRC40K oscillator enabled"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Irc40ken::On)
    }
}
#[doc = "IRC40K stabilization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc40kstbr {
    #[doc = "0: IRC40K oscillator is not stable"]
    NotReady = 0,
    #[doc = "1: IRC40K oscillator is stable"]
    Ready = 1,
}
impl From<Irc40kstbr> for bool {
    #[inline(always)]
    fn from(variant: Irc40kstbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC40KSTB` reader - IRC40K stabilization"]
pub type Irc40kstbR = crate::BitReader<Irc40kstbr>;
impl Irc40kstbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irc40kstbr {
        match self.bits {
            false => Irc40kstbr::NotReady,
            true => Irc40kstbr::Ready,
        }
    }
    #[doc = "IRC40K oscillator is not stable"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Irc40kstbr::NotReady
    }
    #[doc = "IRC40K oscillator is stable"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Irc40kstbr::Ready
    }
}
#[doc = "Reset flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rstfcw {
    #[doc = "1: Clears reset flags"]
    Clear = 1,
}
impl From<Rstfcw> for bool {
    #[inline(always)]
    fn from(variant: Rstfcw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTFC` reader - Reset flag clear"]
pub type RstfcR = crate::BitReader<Rstfcw>;
impl RstfcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rstfcw> {
        match self.bits {
            true => Some(Rstfcw::Clear),
            _ => None,
        }
    }
    #[doc = "Clears reset flags"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Rstfcw::Clear
    }
}
#[doc = "Field `RSTFC` writer - Reset flag clear"]
pub type RstfcW<'a, REG> = crate::BitWriter<'a, REG, Rstfcw>;
impl<'a, REG> RstfcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears reset flags"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfcw::Clear)
    }
}
#[doc = "External PIN reset flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eprstfr {
    #[doc = "0: No reset has occured"]
    NoReset = 0,
    #[doc = "1: A reset has occured"]
    Reset = 1,
}
impl From<Eprstfr> for bool {
    #[inline(always)]
    fn from(variant: Eprstfr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPRSTF` reader - External PIN reset flag"]
pub type EprstfR = crate::BitReader<Eprstfr>;
impl EprstfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eprstfr {
        match self.bits {
            false => Eprstfr::NoReset,
            true => Eprstfr::Reset,
        }
    }
    #[doc = "No reset has occured"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == Eprstfr::NoReset
    }
    #[doc = "A reset has occured"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Eprstfr::Reset
    }
}
#[doc = "Field `PORRSTF` reader - Power reset flag"]
pub use EprstfR as PorrstfR;
#[doc = "Field `SWRSTF` reader - Software reset flag"]
pub use EprstfR as SwrstfR;
#[doc = "Field `FWDGTRSTF` reader - Free Watchdog timer reset flag"]
pub use EprstfR as FwdgtrstfR;
#[doc = "Field `WWDGTRSTF` reader - Window watchdog timer reset flag"]
pub use EprstfR as WwdgtrstfR;
#[doc = "Field `LPRSTF` reader - Low-power reset flag"]
pub use EprstfR as LprstfR;
impl R {
    #[doc = "Bit 0 - IRC40K enable"]
    #[inline(always)]
    pub fn irc40ken(&self) -> Irc40kenR {
        Irc40kenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRC40K stabilization"]
    #[inline(always)]
    pub fn irc40kstb(&self) -> Irc40kstbR {
        Irc40kstbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 24 - Reset flag clear"]
    #[inline(always)]
    pub fn rstfc(&self) -> RstfcR {
        RstfcR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - External PIN reset flag"]
    #[inline(always)]
    pub fn eprstf(&self) -> EprstfR {
        EprstfR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Power reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PorrstfR {
        PorrstfR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn swrstf(&self) -> SwrstfR {
        SwrstfR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Free Watchdog timer reset flag"]
    #[inline(always)]
    pub fn fwdgtrstf(&self) -> FwdgtrstfR {
        FwdgtrstfR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog timer reset flag"]
    #[inline(always)]
    pub fn wwdgtrstf(&self) -> WwdgtrstfR {
        WwdgtrstfR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lprstf(&self) -> LprstfR {
        LprstfR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRC40K enable"]
    #[inline(always)]
    #[must_use]
    pub fn irc40ken(&mut self) -> Irc40kenW<RstsckSpec> {
        Irc40kenW::new(self, 0)
    }
    #[doc = "Bit 24 - Reset flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rstfc(&mut self) -> RstfcW<RstsckSpec> {
        RstfcW::new(self, 24)
    }
}
#[doc = "Reset source /clock register (RCU_RSTSCK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstsck::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstsck::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstsckSpec;
impl crate::RegisterSpec for RstsckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstsck::R`](R) reader structure"]
impl crate::Readable for RstsckSpec {}
#[doc = "`write(|w| ..)` method takes [`rstsck::W`](W) writer structure"]
impl crate::Writable for RstsckSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSTSCK to value 0x0c00_0000"]
impl crate::Resettable for RstsckSpec {
    const RESET_VALUE: u32 = 0x0c00_0000;
}
