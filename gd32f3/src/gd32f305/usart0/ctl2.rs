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
#[doc = "Smartcard NACK enable\n\nValue on reset: 0"]
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
#[doc = "Field `NKEN` reader - Smartcard NACK enable"]
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
#[doc = "Field `NKEN` writer - Smartcard NACK enable"]
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
#[doc = "DMA request enable for reception\n\nValue on reset: 0"]
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
#[doc = "Field `DENR` reader - DMA request enable for reception"]
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
#[doc = "Field `DENR` writer - DMA request enable for reception"]
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
#[doc = "DMA request enable for transmission\n\nValue on reset: 0"]
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
#[doc = "Field `DENT` reader - DMA request enable for transmission"]
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
#[doc = "Field `DENT` writer - DMA request enable for transmission"]
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
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    pub fn nken(&self) -> NkenR {
        NkenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    pub fn scen(&self) -> ScenR {
        ScenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA request enable for reception"]
    #[inline(always)]
    pub fn denr(&self) -> DenrR {
        DenrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA request enable for transmission"]
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
    #[doc = "Bit 4 - Smartcard NACK enable"]
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
    #[doc = "Bit 6 - DMA request enable for reception"]
    #[inline(always)]
    #[must_use]
    pub fn denr(&mut self) -> DenrW<Ctl2Spec> {
        DenrW::new(self, 6)
    }
    #[doc = "Bit 7 - DMA request enable for transmission"]
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
