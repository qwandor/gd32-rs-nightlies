#[doc = "Register `CTL2` reader"]
pub type R = crate::R<Ctl2Spec>;
#[doc = "Register `CTL2` writer"]
pub type W = crate::W<Ctl2Spec>;
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errie {
    #[doc = "0: Error interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated when FERR=1 or ORERR=1 or NERR=1 in the STAT register"]
    Enabled = 1,
}
impl From<Errie> for bool {
    #[inline(always)]
    fn from(variant: Errie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ErrieR = crate::BitReader<Errie>;
impl ErrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errie {
        match self.bits {
            false => Errie::Disabled,
            true => Errie::Enabled,
        }
    }
    #[doc = "Error interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Errie::Disabled
    }
    #[doc = "An interrupt is generated when FERR=1 or ORERR=1 or NERR=1 in the STAT register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Errie::Enabled
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG, Errie>;
impl<'a, REG> ErrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Errie::Disabled)
    }
    #[doc = "An interrupt is generated when FERR=1 or ORERR=1 or NERR=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Errie::Enabled)
    }
}
#[doc = "IrDA mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iren {
    #[doc = "0: IrDA disabled"]
    Disabled = 0,
    #[doc = "1: IrDA enabled"]
    Enabled = 1,
}
impl From<Iren> for bool {
    #[inline(always)]
    fn from(variant: Iren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREN` reader - IrDA mode enable"]
pub type IrenR = crate::BitReader<Iren>;
impl IrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iren {
        match self.bits {
            false => Iren::Disabled,
            true => Iren::Enabled,
        }
    }
    #[doc = "IrDA disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Iren::Disabled
    }
    #[doc = "IrDA enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Iren::Enabled
    }
}
#[doc = "Field `IREN` writer - IrDA mode enable"]
pub type IrenW<'a, REG> = crate::BitWriter<'a, REG, Iren>;
impl<'a, REG> IrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IrDA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Iren::Disabled)
    }
    #[doc = "IrDA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Iren::Enabled)
    }
}
#[doc = "IrDA low-power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irlp {
    #[doc = "0: Normal mode"]
    Normal = 0,
    #[doc = "1: Low-power mode"]
    LowPower = 1,
}
impl From<Irlp> for bool {
    #[inline(always)]
    fn from(variant: Irlp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRLP` reader - IrDA low-power"]
pub type IrlpR = crate::BitReader<Irlp>;
impl IrlpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irlp {
        match self.bits {
            false => Irlp::Normal,
            true => Irlp::LowPower,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Irlp::Normal
    }
    #[doc = "Low-power mode"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == Irlp::LowPower
    }
}
#[doc = "Field `IRLP` writer - IrDA low-power"]
pub type IrlpW<'a, REG> = crate::BitWriter<'a, REG, Irlp>;
impl<'a, REG> IrlpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Irlp::Normal)
    }
    #[doc = "Low-power mode"]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut crate::W<REG> {
        self.variant(Irlp::LowPower)
    }
}
#[doc = "Half-duplex selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hden {
    #[doc = "0: Half duplex mode is not selected"]
    NotSelected = 0,
    #[doc = "1: Half duplex mode is selected"]
    Selected = 1,
}
impl From<Hden> for bool {
    #[inline(always)]
    fn from(variant: Hden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDEN` reader - Half-duplex selection"]
pub type HdenR = crate::BitReader<Hden>;
impl HdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hden {
        match self.bits {
            false => Hden::NotSelected,
            true => Hden::Selected,
        }
    }
    #[doc = "Half duplex mode is not selected"]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == Hden::NotSelected
    }
    #[doc = "Half duplex mode is selected"]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == Hden::Selected
    }
}
#[doc = "Field `HDEN` writer - Half-duplex selection"]
pub type HdenW<'a, REG> = crate::BitWriter<'a, REG, Hden>;
impl<'a, REG> HdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Half duplex mode is not selected"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(Hden::NotSelected)
    }
    #[doc = "Half duplex mode is selected"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(Hden::Selected)
    }
}
#[doc = "NKEN enable in Smartcard mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nken {
    #[doc = "0: NACK transmission in case of parity error is disabled"]
    Disabled = 0,
    #[doc = "1: NACK transmission during parity error is enabled"]
    Enabled = 1,
}
impl From<Nken> for bool {
    #[inline(always)]
    fn from(variant: Nken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NKEN` reader - NKEN enable in Smartcard mode"]
pub type NkenR = crate::BitReader<Nken>;
impl NkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nken {
        match self.bits {
            false => Nken::Disabled,
            true => Nken::Enabled,
        }
    }
    #[doc = "NACK transmission in case of parity error is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Nken::Disabled
    }
    #[doc = "NACK transmission during parity error is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Nken::Enabled
    }
}
#[doc = "Field `NKEN` writer - NKEN enable in Smartcard mode"]
pub type NkenW<'a, REG> = crate::BitWriter<'a, REG, Nken>;
impl<'a, REG> NkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NACK transmission in case of parity error is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Nken::Disabled)
    }
    #[doc = "NACK transmission during parity error is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Nken::Enabled)
    }
}
#[doc = "Smartcard mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scen {
    #[doc = "0: Smartcard Mode disabled"]
    Disabled = 0,
    #[doc = "1: Smartcard Mode enabled"]
    Enabled = 1,
}
impl From<Scen> for bool {
    #[inline(always)]
    fn from(variant: Scen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCEN` reader - Smartcard mode enable"]
pub type ScenR = crate::BitReader<Scen>;
impl ScenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scen {
        match self.bits {
            false => Scen::Disabled,
            true => Scen::Enabled,
        }
    }
    #[doc = "Smartcard Mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Scen::Disabled
    }
    #[doc = "Smartcard Mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Scen::Enabled
    }
}
#[doc = "Field `SCEN` writer - Smartcard mode enable"]
pub type ScenW<'a, REG> = crate::BitWriter<'a, REG, Scen>;
impl<'a, REG> ScenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Smartcard Mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Scen::Disabled)
    }
    #[doc = "Smartcard Mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Scen::Enabled)
    }
}
#[doc = "DMA enable for reception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Denr {
    #[doc = "0: DMA mode is disabled for reception"]
    Disabled = 0,
    #[doc = "1: DMA mode is enabled for reception"]
    Enabled = 1,
}
impl From<Denr> for bool {
    #[inline(always)]
    fn from(variant: Denr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DENR` reader - DMA enable for reception"]
pub type DenrR = crate::BitReader<Denr>;
impl DenrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Denr {
        match self.bits {
            false => Denr::Disabled,
            true => Denr::Enabled,
        }
    }
    #[doc = "DMA mode is disabled for reception"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Denr::Disabled
    }
    #[doc = "DMA mode is enabled for reception"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Denr::Enabled
    }
}
#[doc = "Field `DENR` writer - DMA enable for reception"]
pub type DenrW<'a, REG> = crate::BitWriter<'a, REG, Denr>;
impl<'a, REG> DenrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode is disabled for reception"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Denr::Disabled)
    }
    #[doc = "DMA mode is enabled for reception"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Denr::Enabled)
    }
}
#[doc = "DMA enable transmitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dent {
    #[doc = "0: DMA mode is disabled for transmission"]
    Disabled = 0,
    #[doc = "1: DMA mode is enabled for transmission"]
    Enabled = 1,
}
impl From<Dent> for bool {
    #[inline(always)]
    fn from(variant: Dent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DENT` reader - DMA enable transmitter"]
pub type DentR = crate::BitReader<Dent>;
impl DentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dent {
        match self.bits {
            false => Dent::Disabled,
            true => Dent::Enabled,
        }
    }
    #[doc = "DMA mode is disabled for transmission"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dent::Disabled
    }
    #[doc = "DMA mode is enabled for transmission"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dent::Enabled
    }
}
#[doc = "Field `DENT` writer - DMA enable transmitter"]
pub type DentW<'a, REG> = crate::BitWriter<'a, REG, Dent>;
impl<'a, REG> DentW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode is disabled for transmission"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dent::Disabled)
    }
    #[doc = "DMA mode is enabled for transmission"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dent::Enabled)
    }
}
#[doc = "RTS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtsen {
    #[doc = "0: RTS hardware flow control disabled"]
    Disabled = 0,
    #[doc = "1: RTS output enabled, data is only requested when there is space in the receive buffer"]
    Enabled = 1,
}
impl From<Rtsen> for bool {
    #[inline(always)]
    fn from(variant: Rtsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSEN` reader - RTS enable"]
pub type RtsenR = crate::BitReader<Rtsen>;
impl RtsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtsen {
        match self.bits {
            false => Rtsen::Disabled,
            true => Rtsen::Enabled,
        }
    }
    #[doc = "RTS hardware flow control disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rtsen::Disabled
    }
    #[doc = "RTS output enabled, data is only requested when there is space in the receive buffer"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rtsen::Enabled
    }
}
#[doc = "Field `RTSEN` writer - RTS enable"]
pub type RtsenW<'a, REG> = crate::BitWriter<'a, REG, Rtsen>;
impl<'a, REG> RtsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTS hardware flow control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsen::Disabled)
    }
    #[doc = "RTS output enabled, data is only requested when there is space in the receive buffer"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsen::Enabled)
    }
}
#[doc = "CTS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsen {
    #[doc = "0: CTS hardware flow control disabled"]
    Disabled = 0,
    #[doc = "1: CTS mode enabled, data is only transmitted when the CTS input is asserted"]
    Enabled = 1,
}
impl From<Ctsen> for bool {
    #[inline(always)]
    fn from(variant: Ctsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSEN` reader - CTS enable"]
pub type CtsenR = crate::BitReader<Ctsen>;
impl CtsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsen {
        match self.bits {
            false => Ctsen::Disabled,
            true => Ctsen::Enabled,
        }
    }
    #[doc = "CTS hardware flow control disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ctsen::Disabled
    }
    #[doc = "CTS mode enabled, data is only transmitted when the CTS input is asserted"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ctsen::Enabled
    }
}
#[doc = "Field `CTSEN` writer - CTS enable"]
pub type CtsenW<'a, REG> = crate::BitWriter<'a, REG, Ctsen>;
impl<'a, REG> CtsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CTS hardware flow control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsen::Disabled)
    }
    #[doc = "CTS mode enabled, data is only transmitted when the CTS input is asserted"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsen::Enabled)
    }
}
#[doc = "CTS interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsie {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated whenever CTS=1 in the STAT register"]
    Enabled = 1,
}
impl From<Ctsie> for bool {
    #[inline(always)]
    fn from(variant: Ctsie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSIE` reader - CTS interrupt enable"]
pub type CtsieR = crate::BitReader<Ctsie>;
impl CtsieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsie {
        match self.bits {
            false => Ctsie::Disabled,
            true => Ctsie::Enabled,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ctsie::Disabled
    }
    #[doc = "An interrupt is generated whenever CTS=1 in the STAT register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ctsie::Enabled
    }
}
#[doc = "Field `CTSIE` writer - CTS interrupt enable"]
pub type CtsieW<'a, REG> = crate::BitWriter<'a, REG, Ctsie>;
impl<'a, REG> CtsieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsie::Disabled)
    }
    #[doc = "An interrupt is generated whenever CTS=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsie::Enabled)
    }
}
#[doc = "One sample bit method\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Osb {
    #[doc = "0: Three sample bit method"]
    Sample3 = 0,
    #[doc = "1: One sample bit method"]
    Sample1 = 1,
}
impl From<Osb> for bool {
    #[inline(always)]
    fn from(variant: Osb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSB` reader - One sample bit method"]
pub type OsbR = crate::BitReader<Osb>;
impl OsbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Osb {
        match self.bits {
            false => Osb::Sample3,
            true => Osb::Sample1,
        }
    }
    #[doc = "Three sample bit method"]
    #[inline(always)]
    pub fn is_sample3(&self) -> bool {
        *self == Osb::Sample3
    }
    #[doc = "One sample bit method"]
    #[inline(always)]
    pub fn is_sample1(&self) -> bool {
        *self == Osb::Sample1
    }
}
#[doc = "Field `OSB` writer - One sample bit method"]
pub type OsbW<'a, REG> = crate::BitWriter<'a, REG, Osb>;
impl<'a, REG> OsbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Three sample bit method"]
    #[inline(always)]
    pub fn sample3(self) -> &'a mut crate::W<REG> {
        self.variant(Osb::Sample3)
    }
    #[doc = "One sample bit method"]
    #[inline(always)]
    pub fn sample1(self) -> &'a mut crate::W<REG> {
        self.variant(Osb::Sample1)
    }
}
#[doc = "Overrun Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovrd {
    #[doc = "0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data"]
    Enabled = 0,
    #[doc = "1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDATA register"]
    Disabled = 1,
}
impl From<Ovrd> for bool {
    #[inline(always)]
    fn from(variant: Ovrd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRD` reader - Overrun Disable"]
pub type OvrdR = crate::BitReader<Ovrd>;
impl OvrdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovrd {
        match self.bits {
            false => Ovrd::Enabled,
            true => Ovrd::Disabled,
        }
    }
    #[doc = "Overrun Error Flag, ORE, is set when received data is not read before receiving new data"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ovrd::Enabled
    }
    #[doc = "Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDATA register"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ovrd::Disabled
    }
}
#[doc = "Field `OVRD` writer - Overrun Disable"]
pub type OvrdW<'a, REG> = crate::BitWriter<'a, REG, Ovrd>;
impl<'a, REG> OvrdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overrun Error Flag, ORE, is set when received data is not read before receiving new data"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrd::Enabled)
    }
    #[doc = "Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDATA register"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrd::Disabled)
    }
}
#[doc = "Disable DMA on reception error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ddre {
    #[doc = "0: DMA is not disabled in case of reception error"]
    NotDisabled = 0,
    #[doc = "1: DMA is disabled following a reception error"]
    Disabled = 1,
}
impl From<Ddre> for bool {
    #[inline(always)]
    fn from(variant: Ddre) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDRE` reader - Disable DMA on reception error"]
pub type DdreR = crate::BitReader<Ddre>;
impl DdreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ddre {
        match self.bits {
            false => Ddre::NotDisabled,
            true => Ddre::Disabled,
        }
    }
    #[doc = "DMA is not disabled in case of reception error"]
    #[inline(always)]
    pub fn is_not_disabled(&self) -> bool {
        *self == Ddre::NotDisabled
    }
    #[doc = "DMA is disabled following a reception error"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ddre::Disabled
    }
}
#[doc = "Field `DDRE` writer - Disable DMA on reception error"]
pub type DdreW<'a, REG> = crate::BitWriter<'a, REG, Ddre>;
impl<'a, REG> DdreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not disabled in case of reception error"]
    #[inline(always)]
    pub fn not_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ddre::NotDisabled)
    }
    #[doc = "DMA is disabled following a reception error"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ddre::Disabled)
    }
}
#[doc = "Driver enable mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dem {
    #[doc = "0: DE function is disabled"]
    Disabled = 0,
    #[doc = "1: The DE signal is output on the RTS pin"]
    Enabled = 1,
}
impl From<Dem> for bool {
    #[inline(always)]
    fn from(variant: Dem) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEM` reader - Driver enable mode"]
pub type DemR = crate::BitReader<Dem>;
impl DemR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dem {
        match self.bits {
            false => Dem::Disabled,
            true => Dem::Enabled,
        }
    }
    #[doc = "DE function is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dem::Disabled
    }
    #[doc = "The DE signal is output on the RTS pin"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dem::Enabled
    }
}
#[doc = "Field `DEM` writer - Driver enable mode"]
pub type DemW<'a, REG> = crate::BitWriter<'a, REG, Dem>;
impl<'a, REG> DemW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DE function is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dem::Disabled)
    }
    #[doc = "The DE signal is output on the RTS pin"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dem::Enabled)
    }
}
#[doc = "Driver enable polarity mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dep {
    #[doc = "0: DE signal is active high"]
    High = 0,
    #[doc = "1: DE signal is active low"]
    Low = 1,
}
impl From<Dep> for bool {
    #[inline(always)]
    fn from(variant: Dep) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEP` reader - Driver enable polarity mode"]
pub type DepR = crate::BitReader<Dep>;
impl DepR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dep {
        match self.bits {
            false => Dep::High,
            true => Dep::Low,
        }
    }
    #[doc = "DE signal is active high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Dep::High
    }
    #[doc = "DE signal is active low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Dep::Low
    }
}
#[doc = "Field `DEP` writer - Driver enable polarity mode"]
pub type DepW<'a, REG> = crate::BitWriter<'a, REG, Dep>;
impl<'a, REG> DepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DE signal is active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Dep::High)
    }
    #[doc = "DE signal is active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Dep::Low)
    }
}
#[doc = "Field `SCRTNUM` reader - Smartcard auto-retry number"]
pub type ScrtnumR = crate::FieldReader;
#[doc = "Field `SCRTNUM` writer - Smartcard auto-retry number"]
pub type ScrtnumW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
#[doc = "Wakeup mode from Deep-sleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wum {
    #[doc = "0: WUF active on address match"]
    Address = 0,
    #[doc = "2: WUF active on start bit detection"]
    Start = 2,
    #[doc = "3: WUF active on RBNE"]
    Rxne = 3,
}
impl From<Wum> for u8 {
    #[inline(always)]
    fn from(variant: Wum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wum {
    type Ux = u8;
}
#[doc = "Field `WUM` reader - Wakeup mode from Deep-sleep mode"]
pub type WumR = crate::FieldReader<Wum>;
impl WumR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wum> {
        match self.bits {
            0 => Some(Wum::Address),
            2 => Some(Wum::Start),
            3 => Some(Wum::Rxne),
            _ => None,
        }
    }
    #[doc = "WUF active on address match"]
    #[inline(always)]
    pub fn is_address(&self) -> bool {
        *self == Wum::Address
    }
    #[doc = "WUF active on start bit detection"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Wum::Start
    }
    #[doc = "WUF active on RBNE"]
    #[inline(always)]
    pub fn is_rxne(&self) -> bool {
        *self == Wum::Rxne
    }
}
#[doc = "Field `WUM` writer - Wakeup mode from Deep-sleep mode"]
pub type WumW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wum>;
impl<'a, REG> WumW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "WUF active on address match"]
    #[inline(always)]
    pub fn address(self) -> &'a mut crate::W<REG> {
        self.variant(Wum::Address)
    }
    #[doc = "WUF active on start bit detection"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Wum::Start)
    }
    #[doc = "WUF active on RBNE"]
    #[inline(always)]
    pub fn rxne(self) -> &'a mut crate::W<REG> {
        self.variant(Wum::Rxne)
    }
}
#[doc = "Wakeup from Deep-sleep mode interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuie {
    #[doc = "0: Wake-up from deep-sleep mode interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Wake-up from deep-sleep mode interrupt is generated whenever WUF=1 in the STAT register"]
    Enabled = 1,
}
impl From<Wuie> for bool {
    #[inline(always)]
    fn from(variant: Wuie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUIE` reader - Wakeup from Deep-sleep mode interrupt enable"]
pub type WuieR = crate::BitReader<Wuie>;
impl WuieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuie {
        match self.bits {
            false => Wuie::Disabled,
            true => Wuie::Enabled,
        }
    }
    #[doc = "Wake-up from deep-sleep mode interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Wuie::Disabled
    }
    #[doc = "Wake-up from deep-sleep mode interrupt is generated whenever WUF=1 in the STAT register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Wuie::Enabled
    }
}
#[doc = "Field `WUIE` writer - Wakeup from Deep-sleep mode interrupt enable"]
pub type WuieW<'a, REG> = crate::BitWriter<'a, REG, Wuie>;
impl<'a, REG> WuieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up from deep-sleep mode interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wuie::Disabled)
    }
    #[doc = "Wake-up from deep-sleep mode interrupt is generated whenever WUF=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wuie::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    pub fn iren(&self) -> IrenR {
        IrenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    pub fn irlp(&self) -> IrlpR {
        IrlpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn hden(&self) -> HdenR {
        HdenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NKEN enable in Smartcard mode"]
    #[inline(always)]
    pub fn nken(&self) -> NkenR {
        NkenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    pub fn scen(&self) -> ScenR {
        ScenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA enable for reception"]
    #[inline(always)]
    pub fn denr(&self) -> DenrR {
        DenrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    pub fn dent(&self) -> DentR {
        DentR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtsen(&self) -> RtsenR {
        RtsenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctsen(&self) -> CtsenR {
        CtsenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn ctsie(&self) -> CtsieR {
        CtsieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - One sample bit method"]
    #[inline(always)]
    pub fn osb(&self) -> OsbR {
        OsbR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Overrun Disable"]
    #[inline(always)]
    pub fn ovrd(&self) -> OvrdR {
        OvrdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Disable DMA on reception error"]
    #[inline(always)]
    pub fn ddre(&self) -> DdreR {
        DdreR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Driver enable mode"]
    #[inline(always)]
    pub fn dem(&self) -> DemR {
        DemR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Driver enable polarity mode"]
    #[inline(always)]
    pub fn dep(&self) -> DepR {
        DepR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Smartcard auto-retry number"]
    #[inline(always)]
    pub fn scrtnum(&self) -> ScrtnumR {
        ScrtnumR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:21 - Wakeup mode from Deep-sleep mode"]
    #[inline(always)]
    pub fn wum(&self) -> WumR {
        WumR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Wakeup from Deep-sleep mode interrupt enable"]
    #[inline(always)]
    pub fn wuie(&self) -> WuieR {
        WuieR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ErrieW<Ctl2Spec> {
        ErrieW::new(self, 0)
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IrenW<Ctl2Spec> {
        IrenW::new(self, 1)
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    #[must_use]
    pub fn irlp(&mut self) -> IrlpW<Ctl2Spec> {
        IrlpW::new(self, 2)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    #[must_use]
    pub fn hden(&mut self) -> HdenW<Ctl2Spec> {
        HdenW::new(self, 3)
    }
    #[doc = "Bit 4 - NKEN enable in Smartcard mode"]
    #[inline(always)]
    #[must_use]
    pub fn nken(&mut self) -> NkenW<Ctl2Spec> {
        NkenW::new(self, 4)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn scen(&mut self) -> ScenW<Ctl2Spec> {
        ScenW::new(self, 5)
    }
    #[doc = "Bit 6 - DMA enable for reception"]
    #[inline(always)]
    #[must_use]
    pub fn denr(&mut self) -> DenrW<Ctl2Spec> {
        DenrW::new(self, 6)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn dent(&mut self) -> DentW<Ctl2Spec> {
        DentW::new(self, 7)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtsen(&mut self) -> RtsenW<Ctl2Spec> {
        RtsenW::new(self, 8)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsen(&mut self) -> CtsenW<Ctl2Spec> {
        CtsenW::new(self, 9)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsie(&mut self) -> CtsieW<Ctl2Spec> {
        CtsieW::new(self, 10)
    }
    #[doc = "Bit 11 - One sample bit method"]
    #[inline(always)]
    #[must_use]
    pub fn osb(&mut self) -> OsbW<Ctl2Spec> {
        OsbW::new(self, 11)
    }
    #[doc = "Bit 12 - Overrun Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ovrd(&mut self) -> OvrdW<Ctl2Spec> {
        OvrdW::new(self, 12)
    }
    #[doc = "Bit 13 - Disable DMA on reception error"]
    #[inline(always)]
    #[must_use]
    pub fn ddre(&mut self) -> DdreW<Ctl2Spec> {
        DdreW::new(self, 13)
    }
    #[doc = "Bit 14 - Driver enable mode"]
    #[inline(always)]
    #[must_use]
    pub fn dem(&mut self) -> DemW<Ctl2Spec> {
        DemW::new(self, 14)
    }
    #[doc = "Bit 15 - Driver enable polarity mode"]
    #[inline(always)]
    #[must_use]
    pub fn dep(&mut self) -> DepW<Ctl2Spec> {
        DepW::new(self, 15)
    }
    #[doc = "Bits 17:19 - Smartcard auto-retry number"]
    #[inline(always)]
    #[must_use]
    pub fn scrtnum(&mut self) -> ScrtnumW<Ctl2Spec> {
        ScrtnumW::new(self, 17)
    }
    #[doc = "Bits 20:21 - Wakeup mode from Deep-sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn wum(&mut self) -> WumW<Ctl2Spec> {
        WumW::new(self, 20)
    }
    #[doc = "Bit 22 - Wakeup from Deep-sleep mode interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wuie(&mut self) -> WuieW<Ctl2Spec> {
        WuieW::new(self, 22)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl2Spec;
impl crate::RegisterSpec for Ctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl2::R`](R) reader structure"]
impl crate::Readable for Ctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl2::W`](W) writer structure"]
impl crate::Writable for Ctl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL2 to value 0"]
impl crate::Resettable for Ctl2Spec {
    const RESET_VALUE: u32 = 0;
}
