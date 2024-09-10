#[doc = "Register `INT` reader"]
pub type R = crate::R<IntSpec>;
#[doc = "Register `INT` writer"]
pub type W = crate::W<IntSpec>;
#[doc = "IRC40K stabilization interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc40kstbifr {
    #[doc = "0: No interrupt generated"]
    NotInterrupted = 0,
    #[doc = "1: IRC40K stabilisation interrupt generated"]
    Interrupted = 1,
}
impl From<Irc40kstbifr> for bool {
    #[inline(always)]
    fn from(variant: Irc40kstbifr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC40KSTBIF` reader - IRC40K stabilization interrupt flag"]
pub type Irc40kstbifR = crate::BitReader<Irc40kstbifr>;
impl Irc40kstbifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irc40kstbifr {
        match self.bits {
            false => Irc40kstbifr::NotInterrupted,
            true => Irc40kstbifr::Interrupted,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == Irc40kstbifr::NotInterrupted
    }
    #[doc = "IRC40K stabilisation interrupt generated"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == Irc40kstbifr::Interrupted
    }
}
#[doc = "LXTAL stabilization interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lxtalstbifr {
    #[doc = "0: No interrupt generated"]
    NotInterrupted = 0,
    #[doc = "1: LXTAL stabilisation interrupt generated"]
    Interrupted = 1,
}
impl From<Lxtalstbifr> for bool {
    #[inline(always)]
    fn from(variant: Lxtalstbifr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LXTALSTBIF` reader - LXTAL stabilization interrupt flag"]
pub type LxtalstbifR = crate::BitReader<Lxtalstbifr>;
impl LxtalstbifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lxtalstbifr {
        match self.bits {
            false => Lxtalstbifr::NotInterrupted,
            true => Lxtalstbifr::Interrupted,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == Lxtalstbifr::NotInterrupted
    }
    #[doc = "LXTAL stabilisation interrupt generated"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == Lxtalstbifr::Interrupted
    }
}
#[doc = "IRC8M stabilization interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc8mstbifr {
    #[doc = "0: No interrupt generated"]
    NotInterrupted = 0,
    #[doc = "1: IRC8M stabilisation interrupt generated"]
    Interrupted = 1,
}
impl From<Irc8mstbifr> for bool {
    #[inline(always)]
    fn from(variant: Irc8mstbifr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC8MSTBIF` reader - IRC8M stabilization interrupt flag"]
pub type Irc8mstbifR = crate::BitReader<Irc8mstbifr>;
impl Irc8mstbifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irc8mstbifr {
        match self.bits {
            false => Irc8mstbifr::NotInterrupted,
            true => Irc8mstbifr::Interrupted,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == Irc8mstbifr::NotInterrupted
    }
    #[doc = "IRC8M stabilisation interrupt generated"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == Irc8mstbifr::Interrupted
    }
}
#[doc = "HXTAL stabilization interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hxtalstbifr {
    #[doc = "0: No interrupt generated"]
    NotInterrupted = 0,
    #[doc = "1: HXTAL stabilisation interrupt generated"]
    Interrupted = 1,
}
impl From<Hxtalstbifr> for bool {
    #[inline(always)]
    fn from(variant: Hxtalstbifr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTALSTBIF` reader - HXTAL stabilization interrupt flag"]
pub type HxtalstbifR = crate::BitReader<Hxtalstbifr>;
impl HxtalstbifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hxtalstbifr {
        match self.bits {
            false => Hxtalstbifr::NotInterrupted,
            true => Hxtalstbifr::Interrupted,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == Hxtalstbifr::NotInterrupted
    }
    #[doc = "HXTAL stabilisation interrupt generated"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == Hxtalstbifr::Interrupted
    }
}
#[doc = "PLL stabilization interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllstbifr {
    #[doc = "0: No interrupt generated"]
    NotInterrupted = 0,
    #[doc = "1: PLL stabilisation interrupt generated"]
    Interrupted = 1,
}
impl From<Pllstbifr> for bool {
    #[inline(always)]
    fn from(variant: Pllstbifr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSTBIF` reader - PLL stabilization interrupt flag"]
pub type PllstbifR = crate::BitReader<Pllstbifr>;
impl PllstbifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllstbifr {
        match self.bits {
            false => Pllstbifr::NotInterrupted,
            true => Pllstbifr::Interrupted,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == Pllstbifr::NotInterrupted
    }
    #[doc = "PLL stabilisation interrupt generated"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == Pllstbifr::Interrupted
    }
}
#[doc = "Field `PLL1STBIF` reader - PLL1 stabilization interrupt flag"]
pub use PllstbifR as Pll1stbifR;
#[doc = "Field `PLL2STBIF` reader - PLL2 stabilization interrupt flag"]
pub use PllstbifR as Pll2stbifR;
#[doc = "HXTAL Clock Stuck Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ckmifr {
    #[doc = "0: Clock operating normally"]
    NotInterrupted = 0,
    #[doc = "1: HXTAL clock stuck"]
    Interrupted = 1,
}
impl From<Ckmifr> for bool {
    #[inline(always)]
    fn from(variant: Ckmifr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKMIF` reader - HXTAL Clock Stuck Interrupt Flag"]
pub type CkmifR = crate::BitReader<Ckmifr>;
impl CkmifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckmifr {
        match self.bits {
            false => Ckmifr::NotInterrupted,
            true => Ckmifr::Interrupted,
        }
    }
    #[doc = "Clock operating normally"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == Ckmifr::NotInterrupted
    }
    #[doc = "HXTAL clock stuck"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == Ckmifr::Interrupted
    }
}
#[doc = "IRC40K Stabilization interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc40kstbie {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<Irc40kstbie> for bool {
    #[inline(always)]
    fn from(variant: Irc40kstbie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC40KSTBIE` reader - IRC40K Stabilization interrupt enable"]
pub type Irc40kstbieR = crate::BitReader<Irc40kstbie>;
impl Irc40kstbieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irc40kstbie {
        match self.bits {
            false => Irc40kstbie::Disabled,
            true => Irc40kstbie::Enabled,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Irc40kstbie::Disabled
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Irc40kstbie::Enabled
    }
}
#[doc = "Field `IRC40KSTBIE` writer - IRC40K Stabilization interrupt enable"]
pub type Irc40kstbieW<'a, REG> = crate::BitWriter<'a, REG, Irc40kstbie>;
impl<'a, REG> Irc40kstbieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Irc40kstbie::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Irc40kstbie::Enabled)
    }
}
#[doc = "LXTAL Stabilization Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lxtalstbie {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<Lxtalstbie> for bool {
    #[inline(always)]
    fn from(variant: Lxtalstbie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LXTALSTBIE` reader - LXTAL Stabilization Interrupt Enable"]
pub type LxtalstbieR = crate::BitReader<Lxtalstbie>;
impl LxtalstbieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lxtalstbie {
        match self.bits {
            false => Lxtalstbie::Disabled,
            true => Lxtalstbie::Enabled,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lxtalstbie::Disabled
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lxtalstbie::Enabled
    }
}
#[doc = "Field `LXTALSTBIE` writer - LXTAL Stabilization Interrupt Enable"]
pub type LxtalstbieW<'a, REG> = crate::BitWriter<'a, REG, Lxtalstbie>;
impl<'a, REG> LxtalstbieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lxtalstbie::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lxtalstbie::Enabled)
    }
}
#[doc = "IRC8M Stabilization Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc8mstbie {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<Irc8mstbie> for bool {
    #[inline(always)]
    fn from(variant: Irc8mstbie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC8MSTBIE` reader - IRC8M Stabilization Interrupt Enable"]
pub type Irc8mstbieR = crate::BitReader<Irc8mstbie>;
impl Irc8mstbieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irc8mstbie {
        match self.bits {
            false => Irc8mstbie::Disabled,
            true => Irc8mstbie::Enabled,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Irc8mstbie::Disabled
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Irc8mstbie::Enabled
    }
}
#[doc = "Field `IRC8MSTBIE` writer - IRC8M Stabilization Interrupt Enable"]
pub type Irc8mstbieW<'a, REG> = crate::BitWriter<'a, REG, Irc8mstbie>;
impl<'a, REG> Irc8mstbieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Irc8mstbie::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Irc8mstbie::Enabled)
    }
}
#[doc = "HXTAL Stabilization Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hxtalstbie {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<Hxtalstbie> for bool {
    #[inline(always)]
    fn from(variant: Hxtalstbie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTALSTBIE` reader - HXTAL Stabilization Interrupt Enable"]
pub type HxtalstbieR = crate::BitReader<Hxtalstbie>;
impl HxtalstbieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hxtalstbie {
        match self.bits {
            false => Hxtalstbie::Disabled,
            true => Hxtalstbie::Enabled,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Hxtalstbie::Disabled
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Hxtalstbie::Enabled
    }
}
#[doc = "Field `HXTALSTBIE` writer - HXTAL Stabilization Interrupt Enable"]
pub type HxtalstbieW<'a, REG> = crate::BitWriter<'a, REG, Hxtalstbie>;
impl<'a, REG> HxtalstbieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalstbie::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalstbie::Enabled)
    }
}
#[doc = "PLL Stabilization Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllstbie {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<Pllstbie> for bool {
    #[inline(always)]
    fn from(variant: Pllstbie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSTBIE` reader - PLL Stabilization Interrupt Enable"]
pub type PllstbieR = crate::BitReader<Pllstbie>;
impl PllstbieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllstbie {
        match self.bits {
            false => Pllstbie::Disabled,
            true => Pllstbie::Enabled,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pllstbie::Disabled
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pllstbie::Enabled
    }
}
#[doc = "Field `PLLSTBIE` writer - PLL Stabilization Interrupt Enable"]
pub type PllstbieW<'a, REG> = crate::BitWriter<'a, REG, Pllstbie>;
impl<'a, REG> PllstbieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pllstbie::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pllstbie::Enabled)
    }
}
#[doc = "Field `PLL1STBIE` reader - PLL1 Stabilization Interrupt Enable"]
pub use PllstbieR as Pll1stbieR;
#[doc = "Field `PLL2STBIE` reader - PLL2 Stabilization Interrupt Enable"]
pub use PllstbieR as Pll2stbieR;
#[doc = "Field `PLL1STBIE` writer - PLL1 Stabilization Interrupt Enable"]
pub use PllstbieW as Pll1stbieW;
#[doc = "Field `PLL2STBIE` writer - PLL2 Stabilization Interrupt Enable"]
pub use PllstbieW as Pll2stbieW;
#[doc = "IRC40K Stabilization Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc40kstbicw {
    #[doc = "1: Clear IRC40KSTBIF flag"]
    Clear = 1,
}
impl From<Irc40kstbicw> for bool {
    #[inline(always)]
    fn from(variant: Irc40kstbicw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC40KSTBIC` writer - IRC40K Stabilization Interrupt Clear"]
pub type Irc40kstbicW<'a, REG> = crate::BitWriter<'a, REG, Irc40kstbicw>;
impl<'a, REG> Irc40kstbicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear IRC40KSTBIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Irc40kstbicw::Clear)
    }
}
#[doc = "LXTAL Stabilization Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lxtalstbicw {
    #[doc = "1: Clear LXTALSTBIF flag"]
    Clear = 1,
}
impl From<Lxtalstbicw> for bool {
    #[inline(always)]
    fn from(variant: Lxtalstbicw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LXTALSTBIC` writer - LXTAL Stabilization Interrupt Clear"]
pub type LxtalstbicW<'a, REG> = crate::BitWriter<'a, REG, Lxtalstbicw>;
impl<'a, REG> LxtalstbicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear LXTALSTBIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Lxtalstbicw::Clear)
    }
}
#[doc = "IRC8M Stabilization Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc8mstbicw {
    #[doc = "1: Clear IRC8MSTBIF flag"]
    Clear = 1,
}
impl From<Irc8mstbicw> for bool {
    #[inline(always)]
    fn from(variant: Irc8mstbicw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC8MSTBIC` writer - IRC8M Stabilization Interrupt Clear"]
pub type Irc8mstbicW<'a, REG> = crate::BitWriter<'a, REG, Irc8mstbicw>;
impl<'a, REG> Irc8mstbicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear IRC8MSTBIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Irc8mstbicw::Clear)
    }
}
#[doc = "HXTAL Stabilization Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hxtalstbicw {
    #[doc = "1: Clear HXTALSTBIF flag"]
    Clear = 1,
}
impl From<Hxtalstbicw> for bool {
    #[inline(always)]
    fn from(variant: Hxtalstbicw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTALSTBIC` writer - HXTAL Stabilization Interrupt Clear"]
pub type HxtalstbicW<'a, REG> = crate::BitWriter<'a, REG, Hxtalstbicw>;
impl<'a, REG> HxtalstbicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear HXTALSTBIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalstbicw::Clear)
    }
}
#[doc = "PLL stabilization Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllstbicw {
    #[doc = "1: Clear PLLSTBIF flag"]
    Clear = 1,
}
impl From<Pllstbicw> for bool {
    #[inline(always)]
    fn from(variant: Pllstbicw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSTBIC` writer - PLL stabilization Interrupt Clear"]
pub type PllstbicW<'a, REG> = crate::BitWriter<'a, REG, Pllstbicw>;
impl<'a, REG> PllstbicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear PLLSTBIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pllstbicw::Clear)
    }
}
#[doc = "Field `PLL1STBIC` writer - PLL1 stabilization Interrupt Clear"]
pub use PllstbicW as Pll1stbicW;
#[doc = "Field `PLL2STBIC` writer - PLL2 stabilization Interrupt Clear"]
pub use PllstbicW as Pll2stbicW;
#[doc = "HXTAL Clock Stuck Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ckmicw {
    #[doc = "1: Clear CKMIF flag"]
    Clear = 1,
}
impl From<Ckmicw> for bool {
    #[inline(always)]
    fn from(variant: Ckmicw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKMIC` writer - HXTAL Clock Stuck Interrupt Clear"]
pub type CkmicW<'a, REG> = crate::BitWriter<'a, REG, Ckmicw>;
impl<'a, REG> CkmicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear CKMIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ckmicw::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - IRC40K stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc40kstbif(&self) -> Irc40kstbifR {
        Irc40kstbifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LXTAL stabilization interrupt flag"]
    #[inline(always)]
    pub fn lxtalstbif(&self) -> LxtalstbifR {
        LxtalstbifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IRC8M stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc8mstbif(&self) -> Irc8mstbifR {
        Irc8mstbifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HXTAL stabilization interrupt flag"]
    #[inline(always)]
    pub fn hxtalstbif(&self) -> HxtalstbifR {
        HxtalstbifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL stabilization interrupt flag"]
    #[inline(always)]
    pub fn pllstbif(&self) -> PllstbifR {
        PllstbifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL1 stabilization interrupt flag"]
    #[inline(always)]
    pub fn pll1stbif(&self) -> Pll1stbifR {
        Pll1stbifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL2 stabilization interrupt flag"]
    #[inline(always)]
    pub fn pll2stbif(&self) -> Pll2stbifR {
        Pll2stbifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HXTAL Clock Stuck Interrupt Flag"]
    #[inline(always)]
    pub fn ckmif(&self) -> CkmifR {
        CkmifR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IRC40K Stabilization interrupt enable"]
    #[inline(always)]
    pub fn irc40kstbie(&self) -> Irc40kstbieR {
        Irc40kstbieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn lxtalstbie(&self) -> LxtalstbieR {
        LxtalstbieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IRC8M Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn irc8mstbie(&self) -> Irc8mstbieR {
        Irc8mstbieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn hxtalstbie(&self) -> HxtalstbieR {
        HxtalstbieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PLL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pllstbie(&self) -> PllstbieR {
        PllstbieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PLL1 Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pll1stbie(&self) -> Pll1stbieR {
        Pll1stbieR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PLL2 Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pll2stbie(&self) -> Pll2stbieR {
        Pll2stbieR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - IRC40K Stabilization interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn irc40kstbie(&mut self) -> Irc40kstbieW<IntSpec> {
        Irc40kstbieW::new(self, 8)
    }
    #[doc = "Bit 9 - LXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lxtalstbie(&mut self) -> LxtalstbieW<IntSpec> {
        LxtalstbieW::new(self, 9)
    }
    #[doc = "Bit 10 - IRC8M Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn irc8mstbie(&mut self) -> Irc8mstbieW<IntSpec> {
        Irc8mstbieW::new(self, 10)
    }
    #[doc = "Bit 11 - HXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hxtalstbie(&mut self) -> HxtalstbieW<IntSpec> {
        HxtalstbieW::new(self, 11)
    }
    #[doc = "Bit 12 - PLL Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllstbie(&mut self) -> PllstbieW<IntSpec> {
        PllstbieW::new(self, 12)
    }
    #[doc = "Bit 13 - PLL1 Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll1stbie(&mut self) -> Pll1stbieW<IntSpec> {
        Pll1stbieW::new(self, 13)
    }
    #[doc = "Bit 14 - PLL2 Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll2stbie(&mut self) -> Pll2stbieW<IntSpec> {
        Pll2stbieW::new(self, 14)
    }
    #[doc = "Bit 16 - IRC40K Stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn irc40kstbic(&mut self) -> Irc40kstbicW<IntSpec> {
        Irc40kstbicW::new(self, 16)
    }
    #[doc = "Bit 17 - LXTAL Stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lxtalstbic(&mut self) -> LxtalstbicW<IntSpec> {
        LxtalstbicW::new(self, 17)
    }
    #[doc = "Bit 18 - IRC8M Stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn irc8mstbic(&mut self) -> Irc8mstbicW<IntSpec> {
        Irc8mstbicW::new(self, 18)
    }
    #[doc = "Bit 19 - HXTAL Stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hxtalstbic(&mut self) -> HxtalstbicW<IntSpec> {
        HxtalstbicW::new(self, 19)
    }
    #[doc = "Bit 20 - PLL stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pllstbic(&mut self) -> PllstbicW<IntSpec> {
        PllstbicW::new(self, 20)
    }
    #[doc = "Bit 21 - PLL1 stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pll1stbic(&mut self) -> Pll1stbicW<IntSpec> {
        Pll1stbicW::new(self, 21)
    }
    #[doc = "Bit 22 - PLL2 stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pll2stbic(&mut self) -> Pll2stbicW<IntSpec> {
        Pll2stbicW::new(self, 22)
    }
    #[doc = "Bit 23 - HXTAL Clock Stuck Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ckmic(&mut self) -> CkmicW<IntSpec> {
        CkmicW::new(self, 23)
    }
}
#[doc = "Clock interrupt register (RCU_INT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntSpec;
impl crate::RegisterSpec for IntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int::R`](R) reader structure"]
impl crate::Readable for IntSpec {}
#[doc = "`write(|w| ..)` method takes [`int::W`](W) writer structure"]
impl crate::Writable for IntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT to value 0"]
impl crate::Resettable for IntSpec {
    const RESET_VALUE: u32 = 0;
}
