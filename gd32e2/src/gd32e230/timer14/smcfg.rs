#[doc = "Register `SMCFG` reader"]
pub type R = crate::R<SmcfgSpec>;
#[doc = "Register `SMCFG` writer"]
pub type W = crate::W<SmcfgSpec>;
#[doc = "Slave mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Smc {
    #[doc = "0: Slave mode disabled - if CEN=1 then the prescaler is clocked directly by the internal clock."]
    Disabled = 0,
    #[doc = "4: Restart Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
    RestartMode = 4,
    #[doc = "5: Pause Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
    PauseMode = 5,
    #[doc = "6: Event Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
    EventMode = 6,
    #[doc = "7: External Clock Mode 0 - Rising edges of the selected trigger (TRGI) clock the counter."]
    ExternalClockMode = 7,
}
impl From<Smc> for u8 {
    #[inline(always)]
    fn from(variant: Smc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Smc {
    type Ux = u8;
}
#[doc = "Field `SMC` reader - Slave mode selection"]
pub type SmcR = crate::FieldReader<Smc>;
impl SmcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Smc> {
        match self.bits {
            0 => Some(Smc::Disabled),
            4 => Some(Smc::RestartMode),
            5 => Some(Smc::PauseMode),
            6 => Some(Smc::EventMode),
            7 => Some(Smc::ExternalClockMode),
            _ => None,
        }
    }
    #[doc = "Slave mode disabled - if CEN=1 then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Smc::Disabled
    }
    #[doc = "Restart Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
    #[inline(always)]
    pub fn is_restart_mode(&self) -> bool {
        *self == Smc::RestartMode
    }
    #[doc = "Pause Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
    #[inline(always)]
    pub fn is_pause_mode(&self) -> bool {
        *self == Smc::PauseMode
    }
    #[doc = "Event Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
    #[inline(always)]
    pub fn is_event_mode(&self) -> bool {
        *self == Smc::EventMode
    }
    #[doc = "External Clock Mode 0 - Rising edges of the selected trigger (TRGI) clock the counter."]
    #[inline(always)]
    pub fn is_external_clock_mode(&self) -> bool {
        *self == Smc::ExternalClockMode
    }
}
#[doc = "Field `SMC` writer - Slave mode selection"]
pub type SmcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Smc>;
impl<'a, REG> SmcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slave mode disabled - if CEN=1 then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Smc::Disabled)
    }
    #[doc = "Restart Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
    #[inline(always)]
    pub fn restart_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Smc::RestartMode)
    }
    #[doc = "Pause Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
    #[inline(always)]
    pub fn pause_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Smc::PauseMode)
    }
    #[doc = "Event Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
    #[inline(always)]
    pub fn event_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Smc::EventMode)
    }
    #[doc = "External Clock Mode 0 - Rising edges of the selected trigger (TRGI) clock the counter."]
    #[inline(always)]
    pub fn external_clock_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Smc::ExternalClockMode)
    }
}
#[doc = "Trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgs {
    #[doc = "0: Internal Trigger 0 (ITI0)"]
    Iti0 = 0,
    #[doc = "1: Internal Trigger 1 (ITI1)"]
    Iti1 = 1,
    #[doc = "4: CI0 Edge Detector (CI0F_ED)"]
    Ci0fEd = 4,
    #[doc = "5: Filtered Timer Input 0 (CI0FE0)"]
    Ci0fe0 = 5,
    #[doc = "6: Filtered Timer Input 1 (CI1FE1)"]
    Ci1fe1 = 6,
}
impl From<Trgs> for u8 {
    #[inline(always)]
    fn from(variant: Trgs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgs {
    type Ux = u8;
}
#[doc = "Field `TRGS` reader - Trigger selection"]
pub type TrgsR = crate::FieldReader<Trgs>;
impl TrgsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trgs> {
        match self.bits {
            0 => Some(Trgs::Iti0),
            1 => Some(Trgs::Iti1),
            4 => Some(Trgs::Ci0fEd),
            5 => Some(Trgs::Ci0fe0),
            6 => Some(Trgs::Ci1fe1),
            _ => None,
        }
    }
    #[doc = "Internal Trigger 0 (ITI0)"]
    #[inline(always)]
    pub fn is_iti0(&self) -> bool {
        *self == Trgs::Iti0
    }
    #[doc = "Internal Trigger 1 (ITI1)"]
    #[inline(always)]
    pub fn is_iti1(&self) -> bool {
        *self == Trgs::Iti1
    }
    #[doc = "CI0 Edge Detector (CI0F_ED)"]
    #[inline(always)]
    pub fn is_ci0f_ed(&self) -> bool {
        *self == Trgs::Ci0fEd
    }
    #[doc = "Filtered Timer Input 0 (CI0FE0)"]
    #[inline(always)]
    pub fn is_ci0fe0(&self) -> bool {
        *self == Trgs::Ci0fe0
    }
    #[doc = "Filtered Timer Input 1 (CI1FE1)"]
    #[inline(always)]
    pub fn is_ci1fe1(&self) -> bool {
        *self == Trgs::Ci1fe1
    }
}
#[doc = "Field `TRGS` writer - Trigger selection"]
pub type TrgsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Trgs>;
impl<'a, REG> TrgsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal Trigger 0 (ITI0)"]
    #[inline(always)]
    pub fn iti0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgs::Iti0)
    }
    #[doc = "Internal Trigger 1 (ITI1)"]
    #[inline(always)]
    pub fn iti1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgs::Iti1)
    }
    #[doc = "CI0 Edge Detector (CI0F_ED)"]
    #[inline(always)]
    pub fn ci0f_ed(self) -> &'a mut crate::W<REG> {
        self.variant(Trgs::Ci0fEd)
    }
    #[doc = "Filtered Timer Input 0 (CI0FE0)"]
    #[inline(always)]
    pub fn ci0fe0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgs::Ci0fe0)
    }
    #[doc = "Filtered Timer Input 1 (CI1FE1)"]
    #[inline(always)]
    pub fn ci1fe1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgs::Ci1fe1)
    }
}
#[doc = "Master/Slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msm {
    #[doc = "0: No action"]
    NoSync = 0,
    #[doc = "1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    Sync = 1,
}
impl From<Msm> for bool {
    #[inline(always)]
    fn from(variant: Msm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSM` reader - Master/Slave mode"]
pub type MsmR = crate::BitReader<Msm>;
impl MsmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msm {
        match self.bits {
            false => Msm::NoSync,
            true => Msm::Sync,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_no_sync(&self) -> bool {
        *self == Msm::NoSync
    }
    #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == Msm::Sync
    }
}
#[doc = "Field `MSM` writer - Master/Slave mode"]
pub type MsmW<'a, REG> = crate::BitWriter<'a, REG, Msm>;
impl<'a, REG> MsmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn no_sync(self) -> &'a mut crate::W<REG> {
        self.variant(Msm::NoSync)
    }
    #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    #[inline(always)]
    pub fn sync(self) -> &'a mut crate::W<REG> {
        self.variant(Msm::Sync)
    }
}
impl R {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn smc(&self) -> SmcR {
        SmcR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn trgs(&self) -> TrgsR {
        TrgsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msm(&self) -> MsmR {
        MsmR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn smc(&mut self) -> SmcW<SmcfgSpec> {
        SmcW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn trgs(&mut self) -> TrgsW<SmcfgSpec> {
        TrgsW::new(self, 4)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn msm(&mut self) -> MsmW<SmcfgSpec> {
        MsmW::new(self, 7)
    }
}
#[doc = "slave mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmcfgSpec;
impl crate::RegisterSpec for SmcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smcfg::R`](R) reader structure"]
impl crate::Readable for SmcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`smcfg::W`](W) writer structure"]
impl crate::Writable for SmcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMCFG to value 0"]
impl crate::Resettable for SmcfgSpec {
    const RESET_VALUE: u32 = 0;
}
