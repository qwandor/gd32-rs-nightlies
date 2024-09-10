#[doc = "Register `CS` reader"]
pub type R = crate::R<CsSpec>;
#[doc = "Register `CS` writer"]
pub type W = crate::W<CsSpec>;
#[doc = "Wakeup flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wufr {
    #[doc = "0: No wakeup event occurred"]
    NoWakeupEvent = 0,
    #[doc = "1: A wakeup event was received from the WKUP pin or from the RTC wakeup event (RTC Tamper event, RTC TimeStamp event or RTC alarm)"]
    WakeupEvent = 1,
}
impl From<Wufr> for bool {
    #[inline(always)]
    fn from(variant: Wufr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF` reader - Wakeup flag"]
pub type WufR = crate::BitReader<Wufr>;
impl WufR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wufr {
        match self.bits {
            false => Wufr::NoWakeupEvent,
            true => Wufr::WakeupEvent,
        }
    }
    #[doc = "No wakeup event occurred"]
    #[inline(always)]
    pub fn is_no_wakeup_event(&self) -> bool {
        *self == Wufr::NoWakeupEvent
    }
    #[doc = "A wakeup event was received from the WKUP pin or from the RTC wakeup event (RTC Tamper event, RTC TimeStamp event or RTC alarm)"]
    #[inline(always)]
    pub fn is_wakeup_event(&self) -> bool {
        *self == Wufr::WakeupEvent
    }
}
#[doc = "Standby flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stbfr {
    #[doc = "0: Device has not been in Standby mode"]
    NoStandbyEvent = 0,
    #[doc = "1: Device has been in Standby mode"]
    StandbyEvent = 1,
}
impl From<Stbfr> for bool {
    #[inline(always)]
    fn from(variant: Stbfr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STBF` reader - Standby flag"]
pub type StbfR = crate::BitReader<Stbfr>;
impl StbfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stbfr {
        match self.bits {
            false => Stbfr::NoStandbyEvent,
            true => Stbfr::StandbyEvent,
        }
    }
    #[doc = "Device has not been in Standby mode"]
    #[inline(always)]
    pub fn is_no_standby_event(&self) -> bool {
        *self == Stbfr::NoStandbyEvent
    }
    #[doc = "Device has been in Standby mode"]
    #[inline(always)]
    pub fn is_standby_event(&self) -> bool {
        *self == Stbfr::StandbyEvent
    }
}
#[doc = "Low Voltage Detector Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvdfr {
    #[doc = "0: VDD is higher than the LVD threshold"]
    AboveThreshold = 0,
    #[doc = "1: VDD is lower than or equal to the LVD threshold"]
    BelowThreshold = 1,
}
impl From<Lvdfr> for bool {
    #[inline(always)]
    fn from(variant: Lvdfr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVDF` reader - Low Voltage Detector Status Flag"]
pub type LvdfR = crate::BitReader<Lvdfr>;
impl LvdfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvdfr {
        match self.bits {
            false => Lvdfr::AboveThreshold,
            true => Lvdfr::BelowThreshold,
        }
    }
    #[doc = "VDD is higher than the LVD threshold"]
    #[inline(always)]
    pub fn is_above_threshold(&self) -> bool {
        *self == Lvdfr::AboveThreshold
    }
    #[doc = "VDD is lower than or equal to the LVD threshold"]
    #[inline(always)]
    pub fn is_below_threshold(&self) -> bool {
        *self == Lvdfr::BelowThreshold
    }
}
#[doc = "WKUP pin0 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupen0 {
    #[doc = "0: WKUP pin 0 is used for general purpose I/Os. An event on the WKUP pin 0 does not wakeup the device from Standby mode"]
    Disabled = 0,
    #[doc = "1: WKUP pin 0 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 0 wakes-up the system from Standby mode)"]
    Enabled = 1,
}
impl From<Wupen0> for bool {
    #[inline(always)]
    fn from(variant: Wupen0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPEN0` reader - WKUP pin0 Enable"]
pub type Wupen0R = crate::BitReader<Wupen0>;
impl Wupen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupen0 {
        match self.bits {
            false => Wupen0::Disabled,
            true => Wupen0::Enabled,
        }
    }
    #[doc = "WKUP pin 0 is used for general purpose I/Os. An event on the WKUP pin 0 does not wakeup the device from Standby mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Wupen0::Disabled
    }
    #[doc = "WKUP pin 0 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 0 wakes-up the system from Standby mode)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Wupen0::Enabled
    }
}
#[doc = "Field `WUPEN0` writer - WKUP pin0 Enable"]
pub type Wupen0W<'a, REG> = crate::BitWriter<'a, REG, Wupen0>;
impl<'a, REG> Wupen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WKUP pin 0 is used for general purpose I/Os. An event on the WKUP pin 0 does not wakeup the device from Standby mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wupen0::Disabled)
    }
    #[doc = "WKUP pin 0 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 0 wakes-up the system from Standby mode)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wupen0::Enabled)
    }
}
#[doc = "Field `WUPEN5` reader - WKUP pin5 Enable"]
pub type Wupen5R = crate::BitReader;
#[doc = "Field `WUPEN5` writer - WKUP pin5 Enable"]
pub type Wupen5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUPEN6` reader - WKUP pin6 Enable"]
pub type Wupen6R = crate::BitReader;
#[doc = "Field `WUPEN6` writer - WKUP pin6 Enable"]
pub type Wupen6W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wakeup flag"]
    #[inline(always)]
    pub fn wuf(&self) -> WufR {
        WufR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Standby flag"]
    #[inline(always)]
    pub fn stbf(&self) -> StbfR {
        StbfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low Voltage Detector Status Flag"]
    #[inline(always)]
    pub fn lvdf(&self) -> LvdfR {
        LvdfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - WKUP pin0 Enable"]
    #[inline(always)]
    pub fn wupen0(&self) -> Wupen0R {
        Wupen0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 13 - WKUP pin5 Enable"]
    #[inline(always)]
    pub fn wupen5(&self) -> Wupen5R {
        Wupen5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - WKUP pin6 Enable"]
    #[inline(always)]
    pub fn wupen6(&self) -> Wupen6R {
        Wupen6R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - WKUP pin0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wupen0(&mut self) -> Wupen0W<CsSpec> {
        Wupen0W::new(self, 8)
    }
    #[doc = "Bit 13 - WKUP pin5 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wupen5(&mut self) -> Wupen5W<CsSpec> {
        Wupen5W::new(self, 13)
    }
    #[doc = "Bit 14 - WKUP pin6 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wupen6(&mut self) -> Wupen6W<CsSpec> {
        Wupen6W::new(self, 14)
    }
}
#[doc = "power control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsSpec;
impl crate::RegisterSpec for CsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cs::R`](R) reader structure"]
impl crate::Readable for CsSpec {}
#[doc = "`write(|w| ..)` method takes [`cs::W`](W) writer structure"]
impl crate::Writable for CsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CS to value 0"]
impl crate::Resettable for CsSpec {
    const RESET_VALUE: u32 = 0;
}
