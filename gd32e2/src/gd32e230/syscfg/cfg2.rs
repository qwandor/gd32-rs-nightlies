#[doc = "Register `CFG2` reader"]
pub type R = crate::R<Cfg2Spec>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<Cfg2Spec>;
#[doc = "Cortex-M4 LOCKUP output lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LockupLock {
    #[doc = "0: The Cortex-M3 LOCKUP output is disconnected from the break input"]
    Unlocked = 0,
    #[doc = "1: The Cortex-M3 LOCKUP output is connected to the break input"]
    Locked = 1,
}
impl From<LockupLock> for bool {
    #[inline(always)]
    fn from(variant: LockupLock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKUP_LOCK` reader - Cortex-M4 LOCKUP output lock"]
pub type LockupLockR = crate::BitReader<LockupLock>;
impl LockupLockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LockupLock {
        match self.bits {
            false => LockupLock::Unlocked,
            true => LockupLock::Locked,
        }
    }
    #[doc = "The Cortex-M3 LOCKUP output is disconnected from the break input"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LockupLock::Unlocked
    }
    #[doc = "The Cortex-M3 LOCKUP output is connected to the break input"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LockupLock::Locked
    }
}
#[doc = "Field `LOCKUP_LOCK` writer - Cortex-M4 LOCKUP output lock"]
pub type LockupLockW<'a, REG> = crate::BitWriter<'a, REG, LockupLock>;
impl<'a, REG> LockupLockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Cortex-M3 LOCKUP output is disconnected from the break input"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(LockupLock::Unlocked)
    }
    #[doc = "The Cortex-M3 LOCKUP output is connected to the break input"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LockupLock::Locked)
    }
}
#[doc = "SRAM parity check error lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramParityErrorLock {
    #[doc = "0: The SRAM parity check error is disconnected from the break input"]
    Unlocked = 0,
    #[doc = "1: The SRAM parity check error is connected to the break input"]
    Locked = 1,
}
impl From<SramParityErrorLock> for bool {
    #[inline(always)]
    fn from(variant: SramParityErrorLock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_PARITY_ERROR_LOCK` reader - SRAM parity check error lock"]
pub type SramParityErrorLockR = crate::BitReader<SramParityErrorLock>;
impl SramParityErrorLockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramParityErrorLock {
        match self.bits {
            false => SramParityErrorLock::Unlocked,
            true => SramParityErrorLock::Locked,
        }
    }
    #[doc = "The SRAM parity check error is disconnected from the break input"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == SramParityErrorLock::Unlocked
    }
    #[doc = "The SRAM parity check error is connected to the break input"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == SramParityErrorLock::Locked
    }
}
#[doc = "Field `SRAM_PARITY_ERROR_LOCK` writer - SRAM parity check error lock"]
pub type SramParityErrorLockW<'a, REG> = crate::BitWriter<'a, REG, SramParityErrorLock>;
impl<'a, REG> SramParityErrorLockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The SRAM parity check error is disconnected from the break input"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(SramParityErrorLock::Unlocked)
    }
    #[doc = "The SRAM parity check error is connected to the break input"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(SramParityErrorLock::Locked)
    }
}
#[doc = "LVD lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LvdLock {
    #[doc = "0: The LVD interrupt is disconnected from the break input"]
    Unlocked = 0,
    #[doc = "1: The LVD interrupt is connected to the break input"]
    Locked = 1,
}
impl From<LvdLock> for bool {
    #[inline(always)]
    fn from(variant: LvdLock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD_LOCK` reader - LVD lock"]
pub type LvdLockR = crate::BitReader<LvdLock>;
impl LvdLockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LvdLock {
        match self.bits {
            false => LvdLock::Unlocked,
            true => LvdLock::Locked,
        }
    }
    #[doc = "The LVD interrupt is disconnected from the break input"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LvdLock::Unlocked
    }
    #[doc = "The LVD interrupt is connected to the break input"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LvdLock::Locked
    }
}
#[doc = "Field `LVD_LOCK` writer - LVD lock"]
pub type LvdLockW<'a, REG> = crate::BitWriter<'a, REG, LvdLock>;
impl<'a, REG> LvdLockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The LVD interrupt is disconnected from the break input"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(LvdLock::Unlocked)
    }
    #[doc = "The LVD interrupt is connected to the break input"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LvdLock::Locked)
    }
}
#[doc = "SRAM parity check error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramPcefr {
    #[doc = "0: No SRAM parity check error detected"]
    NoError = 0,
    #[doc = "1: SRAM parity check error detected"]
    Error = 1,
}
impl From<SramPcefr> for bool {
    #[inline(always)]
    fn from(variant: SramPcefr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_PCEF` reader - SRAM parity check error flag"]
pub type SramPcefR = crate::BitReader<SramPcefr>;
impl SramPcefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramPcefr {
        match self.bits {
            false => SramPcefr::NoError,
            true => SramPcefr::Error,
        }
    }
    #[doc = "No SRAM parity check error detected"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == SramPcefr::NoError
    }
    #[doc = "SRAM parity check error detected"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == SramPcefr::Error
    }
}
#[doc = "SRAM parity check error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramPcefwWO {
    #[doc = "1: Clear error flag"]
    Clear = 1,
}
impl From<SramPcefwWO> for bool {
    #[inline(always)]
    fn from(variant: SramPcefwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_PCEF` writer - SRAM parity check error flag"]
pub type SramPcefW<'a, REG> = crate::BitWriter<'a, REG, SramPcefwWO>;
impl<'a, REG> SramPcefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear error flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SramPcefwWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Cortex-M4 LOCKUP output lock"]
    #[inline(always)]
    pub fn lockup_lock(&self) -> LockupLockR {
        LockupLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM parity check error lock"]
    #[inline(always)]
    pub fn sram_parity_error_lock(&self) -> SramParityErrorLockR {
        SramParityErrorLockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LVD lock"]
    #[inline(always)]
    pub fn lvd_lock(&self) -> LvdLockR {
        LvdLockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM parity check error flag"]
    #[inline(always)]
    pub fn sram_pcef(&self) -> SramPcefR {
        SramPcefR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cortex-M4 LOCKUP output lock"]
    #[inline(always)]
    #[must_use]
    pub fn lockup_lock(&mut self) -> LockupLockW<Cfg2Spec> {
        LockupLockW::new(self, 0)
    }
    #[doc = "Bit 1 - SRAM parity check error lock"]
    #[inline(always)]
    #[must_use]
    pub fn sram_parity_error_lock(&mut self) -> SramParityErrorLockW<Cfg2Spec> {
        SramParityErrorLockW::new(self, 1)
    }
    #[doc = "Bit 2 - LVD lock"]
    #[inline(always)]
    #[must_use]
    pub fn lvd_lock(&mut self) -> LvdLockW<Cfg2Spec> {
        LvdLockW::new(self, 2)
    }
    #[doc = "Bit 8 - SRAM parity check error flag"]
    #[inline(always)]
    #[must_use]
    pub fn sram_pcef(&mut self) -> SramPcefW<Cfg2Spec> {
        SramPcefW::new(self, 8)
    }
}
#[doc = "System configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg2Spec;
impl crate::RegisterSpec for Cfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg2::R`](R) reader structure"]
impl crate::Readable for Cfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg2::W`](W) writer structure"]
impl crate::Writable for Cfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG2 to value 0"]
impl crate::Resettable for Cfg2Spec {
    const RESET_VALUE: u32 = 0;
}
