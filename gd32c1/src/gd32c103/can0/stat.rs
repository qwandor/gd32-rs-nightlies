#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Initial working state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iws {
    #[doc = "0: CAN is not in initial working mode"]
    Normal = 0,
    #[doc = "1: CAN is in initial working mode"]
    Initial = 1,
}
impl From<Iws> for bool {
    #[inline(always)]
    fn from(variant: Iws) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWS` reader - Initial working state"]
pub type IwsR = crate::BitReader<Iws>;
impl IwsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iws {
        match self.bits {
            false => Iws::Normal,
            true => Iws::Initial,
        }
    }
    #[doc = "CAN is not in initial working mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Iws::Normal
    }
    #[doc = "CAN is in initial working mode"]
    #[inline(always)]
    pub fn is_initial(&self) -> bool {
        *self == Iws::Initial
    }
}
#[doc = "Sleep working state\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slpws {
    #[doc = "0: CAN is not in sleep working mode"]
    Normal = 0,
    #[doc = "1: CAN is in sleep working mode"]
    Sleeping = 1,
}
impl From<Slpws> for bool {
    #[inline(always)]
    fn from(variant: Slpws) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLPWS` reader - Sleep working state"]
pub type SlpwsR = crate::BitReader<Slpws>;
impl SlpwsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slpws {
        match self.bits {
            false => Slpws::Normal,
            true => Slpws::Sleeping,
        }
    }
    #[doc = "CAN is not in sleep working mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Slpws::Normal
    }
    #[doc = "CAN is in sleep working mode"]
    #[inline(always)]
    pub fn is_sleeping(&self) -> bool {
        *self == Slpws::Sleeping
    }
}
#[doc = "Error interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errifr {
    #[doc = "0: There was no error"]
    NoError = 0,
    #[doc = "1: An error was detected"]
    Error = 1,
}
impl From<Errifr> for bool {
    #[inline(always)]
    fn from(variant: Errifr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIF` reader - Error interrupt flag"]
pub type ErrifR = crate::BitReader<Errifr>;
impl ErrifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errifr {
        match self.bits {
            false => Errifr::NoError,
            true => Errifr::Error,
        }
    }
    #[doc = "There was no error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Errifr::NoError
    }
    #[doc = "An error was detected"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Errifr::Error
    }
}
#[doc = "Error interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrifwWO {
    #[doc = "1: Clears flag"]
    Clear = 1,
}
impl From<ErrifwWO> for bool {
    #[inline(always)]
    fn from(variant: ErrifwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIF` writer - Error interrupt flag"]
pub type ErrifW<'a, REG> = crate::BitWriter<'a, REG, ErrifwWO>;
impl<'a, REG> ErrifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ErrifwWO::Clear)
    }
}
#[doc = "Status change interrupt flag of wakeup from sleep working mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuifr {
    #[doc = "0: No wakeup event"]
    NoWakeup = 0,
    #[doc = "1: Wakeup event"]
    Wakeup = 1,
}
impl From<Wuifr> for bool {
    #[inline(always)]
    fn from(variant: Wuifr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUIF` reader - Status change interrupt flag of wakeup from sleep working mode"]
pub type WuifR = crate::BitReader<Wuifr>;
impl WuifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuifr {
        match self.bits {
            false => Wuifr::NoWakeup,
            true => Wuifr::Wakeup,
        }
    }
    #[doc = "No wakeup event"]
    #[inline(always)]
    pub fn is_no_wakeup(&self) -> bool {
        *self == Wuifr::NoWakeup
    }
    #[doc = "Wakeup event"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == Wuifr::Wakeup
    }
}
#[doc = "Status change interrupt flag of wakeup from sleep working mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WuifwWO {
    #[doc = "1: Clears flag"]
    Clear = 1,
}
impl From<WuifwWO> for bool {
    #[inline(always)]
    fn from(variant: WuifwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUIF` writer - Status change interrupt flag of wakeup from sleep working mode"]
pub type WuifW<'a, REG> = crate::BitWriter<'a, REG, WuifwWO>;
impl<'a, REG> WuifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(WuifwWO::Clear)
    }
}
#[doc = "Status change interrupt flag of sleep working mode entering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slpifr {
    #[doc = "0: CAN is not entering sleep working mode"]
    Awake = 0,
    #[doc = "1: CAN is entering sleep working mode"]
    Sleeping = 1,
}
impl From<Slpifr> for bool {
    #[inline(always)]
    fn from(variant: Slpifr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLPIF` reader - Status change interrupt flag of sleep working mode entering"]
pub type SlpifR = crate::BitReader<Slpifr>;
impl SlpifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slpifr {
        match self.bits {
            false => Slpifr::Awake,
            true => Slpifr::Sleeping,
        }
    }
    #[doc = "CAN is not entering sleep working mode"]
    #[inline(always)]
    pub fn is_awake(&self) -> bool {
        *self == Slpifr::Awake
    }
    #[doc = "CAN is entering sleep working mode"]
    #[inline(always)]
    pub fn is_sleeping(&self) -> bool {
        *self == Slpifr::Sleeping
    }
}
#[doc = "Status change interrupt flag of sleep working mode entering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlpifwWO {
    #[doc = "1: Clears flag"]
    Clear = 1,
}
impl From<SlpifwWO> for bool {
    #[inline(always)]
    fn from(variant: SlpifwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLPIF` writer - Status change interrupt flag of sleep working mode entering"]
pub type SlpifW<'a, REG> = crate::BitWriter<'a, REG, SlpifwWO>;
impl<'a, REG> SlpifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SlpifwWO::Clear)
    }
}
#[doc = "Transmitting state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ts {
    #[doc = "0: CAN is not working in transmitting state"]
    NotWorking = 0,
    #[doc = "1: CAN is working in transmitting state"]
    Working = 1,
}
impl From<Ts> for bool {
    #[inline(always)]
    fn from(variant: Ts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS` reader - Transmitting state"]
pub type TsR = crate::BitReader<Ts>;
impl TsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ts {
        match self.bits {
            false => Ts::NotWorking,
            true => Ts::Working,
        }
    }
    #[doc = "CAN is not working in transmitting state"]
    #[inline(always)]
    pub fn is_not_working(&self) -> bool {
        *self == Ts::NotWorking
    }
    #[doc = "CAN is working in transmitting state"]
    #[inline(always)]
    pub fn is_working(&self) -> bool {
        *self == Ts::Working
    }
}
#[doc = "Receiving state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rs {
    #[doc = "0: CAN is not working in receiving state"]
    NotWorking = 0,
    #[doc = "1: CAN is working in receiving state"]
    Working = 1,
}
impl From<Rs> for bool {
    #[inline(always)]
    fn from(variant: Rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RS` reader - Receiving state"]
pub type RsR = crate::BitReader<Rs>;
impl RsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rs {
        match self.bits {
            false => Rs::NotWorking,
            true => Rs::Working,
        }
    }
    #[doc = "CAN is not working in receiving state"]
    #[inline(always)]
    pub fn is_not_working(&self) -> bool {
        *self == Rs::NotWorking
    }
    #[doc = "CAN is working in receiving state"]
    #[inline(always)]
    pub fn is_working(&self) -> bool {
        *self == Rs::Working
    }
}
#[doc = "Field `LASTRX` reader - Last sample value of RX pin"]
pub type LastrxR = crate::BitReader;
#[doc = "Field `RXL` reader - RX level"]
pub type RxlR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Initial working state"]
    #[inline(always)]
    pub fn iws(&self) -> IwsR {
        IwsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sleep working state"]
    #[inline(always)]
    pub fn slpws(&self) -> SlpwsR {
        SlpwsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error interrupt flag"]
    #[inline(always)]
    pub fn errif(&self) -> ErrifR {
        ErrifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status change interrupt flag of wakeup from sleep working mode"]
    #[inline(always)]
    pub fn wuif(&self) -> WuifR {
        WuifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Status change interrupt flag of sleep working mode entering"]
    #[inline(always)]
    pub fn slpif(&self) -> SlpifR {
        SlpifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmitting state"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receiving state"]
    #[inline(always)]
    pub fn rs(&self) -> RsR {
        RsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Last sample value of RX pin"]
    #[inline(always)]
    pub fn lastrx(&self) -> LastrxR {
        LastrxR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RX level"]
    #[inline(always)]
    pub fn rxl(&self) -> RxlR {
        RxlR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn errif(&mut self) -> ErrifW<StatSpec> {
        ErrifW::new(self, 2)
    }
    #[doc = "Bit 3 - Status change interrupt flag of wakeup from sleep working mode"]
    #[inline(always)]
    #[must_use]
    pub fn wuif(&mut self) -> WuifW<StatSpec> {
        WuifW::new(self, 3)
    }
    #[doc = "Bit 4 - Status change interrupt flag of sleep working mode entering"]
    #[inline(always)]
    #[must_use]
    pub fn slpif(&mut self) -> SlpifW<StatSpec> {
        SlpifW::new(self, 4)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0x0c02"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0x0c02;
}
