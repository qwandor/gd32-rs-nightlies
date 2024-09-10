#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Send break command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbkcmd {
    #[doc = "0: Do not transmit a break frame"]
    Disabled = 0,
    #[doc = "1: Do not transmit a break frame"]
    Enabled = 1,
}
impl From<Sbkcmd> for bool {
    #[inline(always)]
    fn from(variant: Sbkcmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBKCMD` reader - Send break command"]
pub type SbkcmdR = crate::BitReader<Sbkcmd>;
impl SbkcmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbkcmd {
        match self.bits {
            false => Sbkcmd::Disabled,
            true => Sbkcmd::Enabled,
        }
    }
    #[doc = "Do not transmit a break frame"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sbkcmd::Disabled
    }
    #[doc = "Do not transmit a break frame"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sbkcmd::Enabled
    }
}
#[doc = "Field `SBKCMD` writer - Send break command"]
pub type SbkcmdW<'a, REG> = crate::BitWriter<'a, REG, Sbkcmd>;
impl<'a, REG> SbkcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not transmit a break frame"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sbkcmd::Disabled)
    }
    #[doc = "Do not transmit a break frame"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sbkcmd::Enabled)
    }
}
#[doc = "Receiver wakeup from mute mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rwu {
    #[doc = "0: Receiver wakeup from mute mode"]
    Disabled = 0,
    #[doc = "1: Receiver in mute mode"]
    Enabled = 1,
}
impl From<Rwu> for bool {
    #[inline(always)]
    fn from(variant: Rwu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWU` reader - Receiver wakeup from mute mode"]
pub type RwuR = crate::BitReader<Rwu>;
impl RwuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rwu {
        match self.bits {
            false => Rwu::Disabled,
            true => Rwu::Enabled,
        }
    }
    #[doc = "Receiver wakeup from mute mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rwu::Disabled
    }
    #[doc = "Receiver in mute mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rwu::Enabled
    }
}
#[doc = "Field `RWU` writer - Receiver wakeup from mute mode"]
pub type RwuW<'a, REG> = crate::BitWriter<'a, REG, Rwu>;
impl<'a, REG> RwuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver wakeup from mute mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rwu::Disabled)
    }
    #[doc = "Receiver in mute mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rwu::Enabled)
    }
}
#[doc = "Receiver enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ren {
    #[doc = "0: Receiver is disabled"]
    Disabled = 0,
    #[doc = "1: Receiver is enabled"]
    Enabled = 1,
}
impl From<Ren> for bool {
    #[inline(always)]
    fn from(variant: Ren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REN` reader - Receiver enable"]
pub type RenR = crate::BitReader<Ren>;
impl RenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ren {
        match self.bits {
            false => Ren::Disabled,
            true => Ren::Enabled,
        }
    }
    #[doc = "Receiver is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ren::Disabled
    }
    #[doc = "Receiver is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ren::Enabled
    }
}
#[doc = "Field `REN` writer - Receiver enable"]
pub type RenW<'a, REG> = crate::BitWriter<'a, REG, Ren>;
impl<'a, REG> RenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ren::Disabled)
    }
    #[doc = "Receiver is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ren::Enabled)
    }
}
#[doc = "Transmitter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ten {
    #[doc = "0: Transmitter is disabled"]
    Disabled = 0,
    #[doc = "1: Transmitter is enabled"]
    Enabled = 1,
}
impl From<Ten> for bool {
    #[inline(always)]
    fn from(variant: Ten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEN` reader - Transmitter enable"]
pub type TenR = crate::BitReader<Ten>;
impl TenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ten {
        match self.bits {
            false => Ten::Disabled,
            true => Ten::Enabled,
        }
    }
    #[doc = "Transmitter is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ten::Disabled
    }
    #[doc = "Transmitter is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ten::Enabled
    }
}
#[doc = "Field `TEN` writer - Transmitter enable"]
pub type TenW<'a, REG> = crate::BitWriter<'a, REG, Ten>;
impl<'a, REG> TenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmitter is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ten::Disabled)
    }
    #[doc = "Transmitter is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ten::Enabled)
    }
}
#[doc = "IDLE line detected interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idleie {
    #[doc = "0: Idle line detected interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is generated whenever IDLEF=1 in the STAT register"]
    Enabled = 1,
}
impl From<Idleie> for bool {
    #[inline(always)]
    fn from(variant: Idleie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLEIE` reader - IDLE line detected interrupt enable"]
pub type IdleieR = crate::BitReader<Idleie>;
impl IdleieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idleie {
        match self.bits {
            false => Idleie::Disabled,
            true => Idleie::Enabled,
        }
    }
    #[doc = "Idle line detected interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Idleie::Disabled
    }
    #[doc = "Interrupt is generated whenever IDLEF=1 in the STAT register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Idleie::Enabled
    }
}
#[doc = "Field `IDLEIE` writer - IDLE line detected interrupt enable"]
pub type IdleieW<'a, REG> = crate::BitWriter<'a, REG, Idleie>;
impl<'a, REG> IdleieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Idle line detected interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Idleie::Disabled)
    }
    #[doc = "Interrupt is generated whenever IDLEF=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Idleie::Enabled)
    }
}
#[doc = "Read data buffer not empty interrupt and overrun error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rbneie {
    #[doc = "0: Read data buffer not empty and overrrun error interrupts are disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is generated whenever ORERR=1 or RBNE=1 in the STAT register"]
    Enabled = 1,
}
impl From<Rbneie> for bool {
    #[inline(always)]
    fn from(variant: Rbneie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBNEIE` reader - Read data buffer not empty interrupt and overrun error interrupt enable"]
pub type RbneieR = crate::BitReader<Rbneie>;
impl RbneieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rbneie {
        match self.bits {
            false => Rbneie::Disabled,
            true => Rbneie::Enabled,
        }
    }
    #[doc = "Read data buffer not empty and overrrun error interrupts are disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rbneie::Disabled
    }
    #[doc = "Interrupt is generated whenever ORERR=1 or RBNE=1 in the STAT register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rbneie::Enabled
    }
}
#[doc = "Field `RBNEIE` writer - Read data buffer not empty interrupt and overrun error interrupt enable"]
pub type RbneieW<'a, REG> = crate::BitWriter<'a, REG, Rbneie>;
impl<'a, REG> RbneieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read data buffer not empty and overrrun error interrupts are disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rbneie::Disabled)
    }
    #[doc = "Interrupt is generated whenever ORERR=1 or RBNE=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rbneie::Enabled)
    }
}
#[doc = "Transmission complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcie {
    #[doc = "0: Transmission complete interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is generated whenever TC=1 in the STAT register"]
    Enabled = 1,
}
impl From<Tcie> for bool {
    #[inline(always)]
    fn from(variant: Tcie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIE` reader - Transmission complete interrupt enable"]
pub type TcieR = crate::BitReader<Tcie>;
impl TcieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcie {
        match self.bits {
            false => Tcie::Disabled,
            true => Tcie::Enabled,
        }
    }
    #[doc = "Transmission complete interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tcie::Disabled
    }
    #[doc = "Interrupt is generated whenever TC=1 in the STAT register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tcie::Enabled
    }
}
#[doc = "Field `TCIE` writer - Transmission complete interrupt enable"]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG, Tcie>;
impl<'a, REG> TcieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmission complete interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tcie::Disabled)
    }
    #[doc = "Interrupt is generated whenever TC=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tcie::Enabled)
    }
}
#[doc = "Transmitter buffer empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbeie {
    #[doc = "0: Transmission register empty interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is generated whenever TBE=1 in the STAT register"]
    Enabled = 1,
}
impl From<Tbeie> for bool {
    #[inline(always)]
    fn from(variant: Tbeie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBEIE` reader - Transmitter buffer empty interrupt enable"]
pub type TbeieR = crate::BitReader<Tbeie>;
impl TbeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbeie {
        match self.bits {
            false => Tbeie::Disabled,
            true => Tbeie::Enabled,
        }
    }
    #[doc = "Transmission register empty interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tbeie::Disabled
    }
    #[doc = "Interrupt is generated whenever TBE=1 in the STAT register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tbeie::Enabled
    }
}
#[doc = "Field `TBEIE` writer - Transmitter buffer empty interrupt enable"]
pub type TbeieW<'a, REG> = crate::BitWriter<'a, REG, Tbeie>;
impl<'a, REG> TbeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmission register empty interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tbeie::Disabled)
    }
    #[doc = "Interrupt is generated whenever TBE=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tbeie::Enabled)
    }
}
#[doc = "Parity error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perrie {
    #[doc = "0: Parity error interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is generated whenever PERR=1 in the STAT register"]
    Enabled = 1,
}
impl From<Perrie> for bool {
    #[inline(always)]
    fn from(variant: Perrie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERRIE` reader - Parity error interrupt enable"]
pub type PerrieR = crate::BitReader<Perrie>;
impl PerrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perrie {
        match self.bits {
            false => Perrie::Disabled,
            true => Perrie::Enabled,
        }
    }
    #[doc = "Parity error interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Perrie::Disabled
    }
    #[doc = "Interrupt is generated whenever PERR=1 in the STAT register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Perrie::Enabled
    }
}
#[doc = "Field `PERRIE` writer - Parity error interrupt enable"]
pub type PerrieW<'a, REG> = crate::BitWriter<'a, REG, Perrie>;
impl<'a, REG> PerrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Parity error interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Perrie::Disabled)
    }
    #[doc = "Interrupt is generated whenever PERR=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Perrie::Enabled)
    }
}
#[doc = "Parity mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pm {
    #[doc = "0: Even parity"]
    Even = 0,
    #[doc = "1: Odd parity"]
    Odd = 1,
}
impl From<Pm> for bool {
    #[inline(always)]
    fn from(variant: Pm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PM` reader - Parity mode"]
pub type PmR = crate::BitReader<Pm>;
impl PmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pm {
        match self.bits {
            false => Pm::Even,
            true => Pm::Odd,
        }
    }
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == Pm::Even
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == Pm::Odd
    }
}
#[doc = "Field `PM` writer - Parity mode"]
pub type PmW<'a, REG> = crate::BitWriter<'a, REG, Pm>;
impl<'a, REG> PmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(Pm::Even)
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(Pm::Odd)
    }
}
#[doc = "Parity check function enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcen {
    #[doc = "0: Parity control disabled"]
    Disabled = 0,
    #[doc = "1: Parity control enabled"]
    Enabled = 1,
}
impl From<Pcen> for bool {
    #[inline(always)]
    fn from(variant: Pcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCEN` reader - Parity check function enable"]
pub type PcenR = crate::BitReader<Pcen>;
impl PcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcen {
        match self.bits {
            false => Pcen::Disabled,
            true => Pcen::Enabled,
        }
    }
    #[doc = "Parity control disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pcen::Disabled
    }
    #[doc = "Parity control enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pcen::Enabled
    }
}
#[doc = "Field `PCEN` writer - Parity check function enable"]
pub type PcenW<'a, REG> = crate::BitWriter<'a, REG, Pcen>;
impl<'a, REG> PcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Parity control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pcen::Disabled)
    }
    #[doc = "Parity control enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pcen::Enabled)
    }
}
#[doc = "Wakeup method in mute mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wm {
    #[doc = "0: Idle line"]
    Idle = 0,
    #[doc = "1: Address mask"]
    Address = 1,
}
impl From<Wm> for bool {
    #[inline(always)]
    fn from(variant: Wm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WM` reader - Wakeup method in mute mode"]
pub type WmR = crate::BitReader<Wm>;
impl WmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wm {
        match self.bits {
            false => Wm::Idle,
            true => Wm::Address,
        }
    }
    #[doc = "Idle line"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Wm::Idle
    }
    #[doc = "Address mask"]
    #[inline(always)]
    pub fn is_address(&self) -> bool {
        *self == Wm::Address
    }
}
#[doc = "Field `WM` writer - Wakeup method in mute mode"]
pub type WmW<'a, REG> = crate::BitWriter<'a, REG, Wm>;
impl<'a, REG> WmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Idle line"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(Wm::Idle)
    }
    #[doc = "Address mask"]
    #[inline(always)]
    pub fn address(self) -> &'a mut crate::W<REG> {
        self.variant(Wm::Address)
    }
}
#[doc = "Word length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wl {
    #[doc = "0: 8 data bits"]
    Bit8 = 0,
    #[doc = "1: 9 data bits"]
    Bit9 = 1,
}
impl From<Wl> for bool {
    #[inline(always)]
    fn from(variant: Wl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WL` reader - Word length"]
pub type WlR = crate::BitReader<Wl>;
impl WlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wl {
        match self.bits {
            false => Wl::Bit8,
            true => Wl::Bit9,
        }
    }
    #[doc = "8 data bits"]
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == Wl::Bit8
    }
    #[doc = "9 data bits"]
    #[inline(always)]
    pub fn is_bit9(&self) -> bool {
        *self == Wl::Bit9
    }
}
#[doc = "Field `WL` writer - Word length"]
pub type WlW<'a, REG> = crate::BitWriter<'a, REG, Wl>;
impl<'a, REG> WlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "8 data bits"]
    #[inline(always)]
    pub fn bit8(self) -> &'a mut crate::W<REG> {
        self.variant(Wl::Bit8)
    }
    #[doc = "9 data bits"]
    #[inline(always)]
    pub fn bit9(self) -> &'a mut crate::W<REG> {
        self.variant(Wl::Bit9)
    }
}
#[doc = "USART enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uen {
    #[doc = "0: USART prescaler and outputs disabled"]
    Disabled = 0,
    #[doc = "1: USART prescaler and outputs enabled"]
    Enabled = 1,
}
impl From<Uen> for bool {
    #[inline(always)]
    fn from(variant: Uen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UEN` reader - USART enable"]
pub type UenR = crate::BitReader<Uen>;
impl UenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uen {
        match self.bits {
            false => Uen::Disabled,
            true => Uen::Enabled,
        }
    }
    #[doc = "USART prescaler and outputs disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Uen::Disabled
    }
    #[doc = "USART prescaler and outputs enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Uen::Enabled
    }
}
#[doc = "Field `UEN` writer - USART enable"]
pub type UenW<'a, REG> = crate::BitWriter<'a, REG, Uen>;
impl<'a, REG> UenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART prescaler and outputs disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Uen::Disabled)
    }
    #[doc = "USART prescaler and outputs enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Uen::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Send break command"]
    #[inline(always)]
    pub fn sbkcmd(&self) -> SbkcmdR {
        SbkcmdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver wakeup from mute mode"]
    #[inline(always)]
    pub fn rwu(&self) -> RwuR {
        RwuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn ren(&self) -> RenR {
        RenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn ten(&self) -> TenR {
        TenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE line detected interrupt enable"]
    #[inline(always)]
    pub fn idleie(&self) -> IdleieR {
        IdleieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read data buffer not empty interrupt and overrun error interrupt enable"]
    #[inline(always)]
    pub fn rbneie(&self) -> RbneieR {
        RbneieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmitter buffer empty interrupt enable"]
    #[inline(always)]
    pub fn tbeie(&self) -> TbeieR {
        TbeieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity error interrupt enable"]
    #[inline(always)]
    pub fn perrie(&self) -> PerrieR {
        PerrieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity mode"]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity check function enable"]
    #[inline(always)]
    pub fn pcen(&self) -> PcenR {
        PcenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wakeup method in mute mode"]
    #[inline(always)]
    pub fn wm(&self) -> WmR {
        WmR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    pub fn wl(&self) -> WlR {
        WlR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    pub fn uen(&self) -> UenR {
        UenR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Send break command"]
    #[inline(always)]
    #[must_use]
    pub fn sbkcmd(&mut self) -> SbkcmdW<Ctl0Spec> {
        SbkcmdW::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver wakeup from mute mode"]
    #[inline(always)]
    #[must_use]
    pub fn rwu(&mut self) -> RwuW<Ctl0Spec> {
        RwuW::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    #[must_use]
    pub fn ren(&mut self) -> RenW<Ctl0Spec> {
        RenW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TenW<Ctl0Spec> {
        TenW::new(self, 3)
    }
    #[doc = "Bit 4 - IDLE line detected interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn idleie(&mut self) -> IdleieW<Ctl0Spec> {
        IdleieW::new(self, 4)
    }
    #[doc = "Bit 5 - Read data buffer not empty interrupt and overrun error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rbneie(&mut self) -> RbneieW<Ctl0Spec> {
        RbneieW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TcieW<Ctl0Spec> {
        TcieW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmitter buffer empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbeie(&mut self) -> TbeieW<Ctl0Spec> {
        TbeieW::new(self, 7)
    }
    #[doc = "Bit 8 - Parity error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn perrie(&mut self) -> PerrieW<Ctl0Spec> {
        PerrieW::new(self, 8)
    }
    #[doc = "Bit 9 - Parity mode"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PmW<Ctl0Spec> {
        PmW::new(self, 9)
    }
    #[doc = "Bit 10 - Parity check function enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcen(&mut self) -> PcenW<Ctl0Spec> {
        PcenW::new(self, 10)
    }
    #[doc = "Bit 11 - Wakeup method in mute mode"]
    #[inline(always)]
    #[must_use]
    pub fn wm(&mut self) -> WmW<Ctl0Spec> {
        WmW::new(self, 11)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    #[must_use]
    pub fn wl(&mut self) -> WlW<Ctl0Spec> {
        WlW::new(self, 12)
    }
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    #[must_use]
    pub fn uen(&mut self) -> UenW<Ctl0Spec> {
        UenW::new(self, 13)
    }
}
#[doc = "Control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for Ctl0Spec {
    const RESET_VALUE: u32 = 0;
}
