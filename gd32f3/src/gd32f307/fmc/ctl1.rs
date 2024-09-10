#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Main flash program for bank0 command bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pg {
    #[doc = "1: Flash programming activated"]
    Program = 1,
}
impl From<Pg> for bool {
    #[inline(always)]
    fn from(variant: Pg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PG` reader - Main flash program for bank0 command bit"]
pub type PgR = crate::BitReader<Pg>;
impl PgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pg> {
        match self.bits {
            true => Some(Pg::Program),
            _ => None,
        }
    }
    #[doc = "Flash programming activated"]
    #[inline(always)]
    pub fn is_program(&self) -> bool {
        *self == Pg::Program
    }
}
#[doc = "Field `PG` writer - Main flash program for bank0 command bit"]
pub type PgW<'a, REG> = crate::BitWriter<'a, REG, Pg>;
impl<'a, REG> PgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash programming activated"]
    #[inline(always)]
    pub fn program(self) -> &'a mut crate::W<REG> {
        self.variant(Pg::Program)
    }
}
#[doc = "Main flash page erase for bank0 command bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Per {
    #[doc = "1: Erase activated for selected page"]
    PageErase = 1,
}
impl From<Per> for bool {
    #[inline(always)]
    fn from(variant: Per) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PER` reader - Main flash page erase for bank0 command bit"]
pub type PerR = crate::BitReader<Per>;
impl PerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Per> {
        match self.bits {
            true => Some(Per::PageErase),
            _ => None,
        }
    }
    #[doc = "Erase activated for selected page"]
    #[inline(always)]
    pub fn is_page_erase(&self) -> bool {
        *self == Per::PageErase
    }
}
#[doc = "Field `PER` writer - Main flash page erase for bank0 command bit"]
pub type PerW<'a, REG> = crate::BitWriter<'a, REG, Per>;
impl<'a, REG> PerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Erase activated for selected page"]
    #[inline(always)]
    pub fn page_erase(self) -> &'a mut crate::W<REG> {
        self.variant(Per::PageErase)
    }
}
#[doc = "Main flash mass erase for bank0 command bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mer {
    #[doc = "1: Erase activated for all user sectors"]
    MassErase = 1,
}
impl From<Mer> for bool {
    #[inline(always)]
    fn from(variant: Mer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MER` reader - Main flash mass erase for bank0 command bit"]
pub type MerR = crate::BitReader<Mer>;
impl MerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mer> {
        match self.bits {
            true => Some(Mer::MassErase),
            _ => None,
        }
    }
    #[doc = "Erase activated for all user sectors"]
    #[inline(always)]
    pub fn is_mass_erase(&self) -> bool {
        *self == Mer::MassErase
    }
}
#[doc = "Field `MER` writer - Main flash mass erase for bank0 command bit"]
pub type MerW<'a, REG> = crate::BitWriter<'a, REG, Mer>;
impl<'a, REG> MerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Erase activated for all user sectors"]
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut crate::W<REG> {
        self.variant(Mer::MassErase)
    }
}
#[doc = "Send erase command to FMC bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Startw {
    #[doc = "1: Trigger an erase operation"]
    Start = 1,
}
impl From<Startw> for bool {
    #[inline(always)]
    fn from(variant: Startw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Send erase command to FMC bit"]
pub type StartR = crate::BitReader<Startw>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Startw> {
        match self.bits {
            true => Some(Startw::Start),
            _ => None,
        }
    }
    #[doc = "Trigger an erase operation"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Startw::Start
    }
}
#[doc = "Field `START` writer - Send erase command to FMC bit"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG, Startw>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger an erase operation"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Startw::Start)
    }
}
#[doc = "FMC_CTL0 lock bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lkr {
    #[doc = "0: CTL register is unlocked"]
    Unlocked = 0,
    #[doc = "1: CTL register is locked"]
    Locked = 1,
}
impl From<Lkr> for bool {
    #[inline(always)]
    fn from(variant: Lkr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LK` reader - FMC_CTL0 lock bit"]
pub type LkR = crate::BitReader<Lkr>;
impl LkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lkr {
        match self.bits {
            false => Lkr::Unlocked,
            true => Lkr::Locked,
        }
    }
    #[doc = "CTL register is unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Lkr::Unlocked
    }
    #[doc = "CTL register is locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lkr::Locked
    }
}
#[doc = "FMC_CTL0 lock bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LkwWO {
    #[doc = "1: Lock CTL register"]
    Lock = 1,
}
impl From<LkwWO> for bool {
    #[inline(always)]
    fn from(variant: LkwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LK` writer - FMC_CTL0 lock bit"]
pub type LkW<'a, REG> = crate::BitWriter<'a, REG, LkwWO>;
impl<'a, REG> LkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lock CTL register"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(LkwWO::Lock)
    }
}
#[doc = "Error interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errie {
    #[doc = "0: Error interrupt generation disabled"]
    Disabled = 0,
    #[doc = "1: Error interrupt generation enabled"]
    Enabled = 1,
}
impl From<Errie> for bool {
    #[inline(always)]
    fn from(variant: Errie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable bit"]
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
    #[doc = "Error interrupt generation disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Errie::Disabled
    }
    #[doc = "Error interrupt generation enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Errie::Enabled
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable bit"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG, Errie>;
impl<'a, REG> ErrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error interrupt generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Errie::Disabled)
    }
    #[doc = "Error interrupt generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Errie::Enabled)
    }
}
#[doc = "End of operation interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endie {
    #[doc = "0: End of operation interrupt disabled"]
    Disabled = 0,
    #[doc = "1: End of operation interrupt enabled"]
    Enabled = 1,
}
impl From<Endie> for bool {
    #[inline(always)]
    fn from(variant: Endie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDIE` reader - End of operation interrupt enable bit"]
pub type EndieR = crate::BitReader<Endie>;
impl EndieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endie {
        match self.bits {
            false => Endie::Disabled,
            true => Endie::Enabled,
        }
    }
    #[doc = "End of operation interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endie::Disabled
    }
    #[doc = "End of operation interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endie::Enabled
    }
}
#[doc = "Field `ENDIE` writer - End of operation interrupt enable bit"]
pub type EndieW<'a, REG> = crate::BitWriter<'a, REG, Endie>;
impl<'a, REG> EndieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "End of operation interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Endie::Disabled)
    }
    #[doc = "End of operation interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Endie::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Main flash program for bank0 command bit"]
    #[inline(always)]
    pub fn pg(&self) -> PgR {
        PgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Main flash page erase for bank0 command bit"]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Main flash mass erase for bank0 command bit"]
    #[inline(always)]
    pub fn mer(&self) -> MerR {
        MerR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Send erase command to FMC bit"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FMC_CTL0 lock bit"]
    #[inline(always)]
    pub fn lk(&self) -> LkR {
        LkR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Error interrupt enable bit"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - End of operation interrupt enable bit"]
    #[inline(always)]
    pub fn endie(&self) -> EndieR {
        EndieR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Main flash program for bank0 command bit"]
    #[inline(always)]
    #[must_use]
    pub fn pg(&mut self) -> PgW<Ctl1Spec> {
        PgW::new(self, 0)
    }
    #[doc = "Bit 1 - Main flash page erase for bank0 command bit"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PerW<Ctl1Spec> {
        PerW::new(self, 1)
    }
    #[doc = "Bit 2 - Main flash mass erase for bank0 command bit"]
    #[inline(always)]
    #[must_use]
    pub fn mer(&mut self) -> MerW<Ctl1Spec> {
        MerW::new(self, 2)
    }
    #[doc = "Bit 6 - Send erase command to FMC bit"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<Ctl1Spec> {
        StartW::new(self, 6)
    }
    #[doc = "Bit 7 - FMC_CTL0 lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn lk(&mut self) -> LkW<Ctl1Spec> {
        LkW::new(self, 7)
    }
    #[doc = "Bit 10 - Error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ErrieW<Ctl1Spec> {
        ErrieW::new(self, 10)
    }
    #[doc = "Bit 12 - End of operation interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn endie(&mut self) -> EndieW<Ctl1Spec> {
        EndieW::new(self, 12)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0x80"]
impl crate::Resettable for Ctl1Spec {
    const RESET_VALUE: u32 = 0x80;
}
