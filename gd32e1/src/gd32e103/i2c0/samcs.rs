#[doc = "Register `SAMCS` reader"]
pub type R = crate::R<SamcsSpec>;
#[doc = "Register `SAMCS` writer"]
pub type W = crate::W<SamcsSpec>;
#[doc = "SAM_V interface enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Samen {
    #[doc = "0: SAM_V interface disabled"]
    Disabled = 0,
    #[doc = "1: SAM_V interface enabled"]
    Enabled = 1,
}
impl From<Samen> for bool {
    #[inline(always)]
    fn from(variant: Samen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMEN` reader - SAM_V interface enable"]
pub type SamenR = crate::BitReader<Samen>;
impl SamenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Samen {
        match self.bits {
            false => Samen::Disabled,
            true => Samen::Enabled,
        }
    }
    #[doc = "SAM_V interface disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Samen::Disabled
    }
    #[doc = "SAM_V interface enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Samen::Enabled
    }
}
#[doc = "Field `SAMEN` writer - SAM_V interface enable"]
pub type SamenW<'a, REG> = crate::BitWriter<'a, REG, Samen>;
impl<'a, REG> SamenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SAM_V interface disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Samen::Disabled)
    }
    #[doc = "SAM_V interface enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Samen::Enabled)
    }
}
#[doc = "SAM_V interface timeout detect enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stoen {
    #[doc = "0: SAM_V interface timeout detect disabled"]
    Disabled = 0,
    #[doc = "1: SAM_V interface timeout detect enabled"]
    Enabled = 1,
}
impl From<Stoen> for bool {
    #[inline(always)]
    fn from(variant: Stoen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOEN` reader - SAM_V interface timeout detect enable"]
pub type StoenR = crate::BitReader<Stoen>;
impl StoenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stoen {
        match self.bits {
            false => Stoen::Disabled,
            true => Stoen::Enabled,
        }
    }
    #[doc = "SAM_V interface timeout detect disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stoen::Disabled
    }
    #[doc = "SAM_V interface timeout detect enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stoen::Enabled
    }
}
#[doc = "Field `STOEN` writer - SAM_V interface timeout detect enable"]
pub type StoenW<'a, REG> = crate::BitWriter<'a, REG, Stoen>;
impl<'a, REG> StoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SAM_V interface timeout detect disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stoen::Disabled)
    }
    #[doc = "SAM_V interface timeout detect enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stoen::Enabled)
    }
}
#[doc = "Txframe fall interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tffie {
    #[doc = "0: Txframe fall interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Txframe fall interrupt enabled"]
    Enabled = 1,
}
impl From<Tffie> for bool {
    #[inline(always)]
    fn from(variant: Tffie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFFIE` reader - Txframe fall interrupt enable"]
pub type TffieR = crate::BitReader<Tffie>;
impl TffieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tffie {
        match self.bits {
            false => Tffie::Disabled,
            true => Tffie::Enabled,
        }
    }
    #[doc = "Txframe fall interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tffie::Disabled
    }
    #[doc = "Txframe fall interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tffie::Enabled
    }
}
#[doc = "Field `TFFIE` writer - Txframe fall interrupt enable"]
pub type TffieW<'a, REG> = crate::BitWriter<'a, REG, Tffie>;
impl<'a, REG> TffieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Txframe fall interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tffie::Disabled)
    }
    #[doc = "Txframe fall interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tffie::Enabled)
    }
}
#[doc = "Txframe rise interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfrie {
    #[doc = "0: Txframe rise interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Txframe rise interrupt enabled"]
    Enabled = 1,
}
impl From<Tfrie> for bool {
    #[inline(always)]
    fn from(variant: Tfrie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFRIE` reader - Txframe rise interrupt enable"]
pub type TfrieR = crate::BitReader<Tfrie>;
impl TfrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfrie {
        match self.bits {
            false => Tfrie::Disabled,
            true => Tfrie::Enabled,
        }
    }
    #[doc = "Txframe rise interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tfrie::Disabled
    }
    #[doc = "Txframe rise interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tfrie::Enabled
    }
}
#[doc = "Field `TFRIE` writer - Txframe rise interrupt enable"]
pub type TfrieW<'a, REG> = crate::BitWriter<'a, REG, Tfrie>;
impl<'a, REG> TfrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Txframe rise interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tfrie::Disabled)
    }
    #[doc = "Txframe rise interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tfrie::Enabled)
    }
}
#[doc = "Rxframe fall interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rffie {
    #[doc = "0: Rxframe fall interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Rxframe fall interrupt enabled"]
    Enabled = 1,
}
impl From<Rffie> for bool {
    #[inline(always)]
    fn from(variant: Rffie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFFIE` reader - Rxframe fall interrupt enable"]
pub type RffieR = crate::BitReader<Rffie>;
impl RffieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rffie {
        match self.bits {
            false => Rffie::Disabled,
            true => Rffie::Enabled,
        }
    }
    #[doc = "Rxframe fall interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rffie::Disabled
    }
    #[doc = "Rxframe fall interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rffie::Enabled
    }
}
#[doc = "Field `RFFIE` writer - Rxframe fall interrupt enable"]
pub type RffieW<'a, REG> = crate::BitWriter<'a, REG, Rffie>;
impl<'a, REG> RffieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rxframe fall interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rffie::Disabled)
    }
    #[doc = "Rxframe fall interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rffie::Enabled)
    }
}
#[doc = "Rxframe rise interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfrie {
    #[doc = "0: Rxframe rise interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Rxframe rise interrupt enabled"]
    Enabled = 1,
}
impl From<Rfrie> for bool {
    #[inline(always)]
    fn from(variant: Rfrie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFRIE` reader - Rxframe rise interrupt enable"]
pub type RfrieR = crate::BitReader<Rfrie>;
impl RfrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfrie {
        match self.bits {
            false => Rfrie::Disabled,
            true => Rfrie::Enabled,
        }
    }
    #[doc = "Rxframe rise interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rfrie::Disabled
    }
    #[doc = "Rxframe rise interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rfrie::Enabled
    }
}
#[doc = "Field `RFRIE` writer - Rxframe rise interrupt enable"]
pub type RfrieW<'a, REG> = crate::BitWriter<'a, REG, Rfrie>;
impl<'a, REG> RfrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rxframe rise interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rfrie::Disabled)
    }
    #[doc = "Rxframe rise interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rfrie::Enabled)
    }
}
#[doc = "Field `TXF` reader - Level of txframe signal"]
pub type TxfR = crate::BitReader;
#[doc = "Field `RXF` reader - Level of rxframe signal"]
pub type RxfR = crate::BitReader;
#[doc = "Field `TFF` reader - Txframe fall flag"]
pub type TffR = crate::BitReader;
#[doc = "Field `TFF` writer - Txframe fall flag"]
pub type TffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFR` reader - Txframe rise flag"]
pub type TfrR = crate::BitReader;
#[doc = "Field `TFR` writer - Txframe rise flag"]
pub type TfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFF` reader - Rxframe fall flag"]
pub type RffR = crate::BitReader;
#[doc = "Field `RFF` writer - Rxframe fall flag"]
pub type RffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFR` reader - Rxframe rise flag"]
pub type RfrR = crate::BitReader;
#[doc = "Field `RFR` writer - Rxframe rise flag"]
pub type RfrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SAM_V interface enable"]
    #[inline(always)]
    pub fn samen(&self) -> SamenR {
        SamenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SAM_V interface timeout detect enable"]
    #[inline(always)]
    pub fn stoen(&self) -> StoenR {
        StoenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Txframe fall interrupt enable"]
    #[inline(always)]
    pub fn tffie(&self) -> TffieR {
        TffieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Txframe rise interrupt enable"]
    #[inline(always)]
    pub fn tfrie(&self) -> TfrieR {
        TfrieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rxframe fall interrupt enable"]
    #[inline(always)]
    pub fn rffie(&self) -> RffieR {
        RffieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rxframe rise interrupt enable"]
    #[inline(always)]
    pub fn rfrie(&self) -> RfrieR {
        RfrieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Level of txframe signal"]
    #[inline(always)]
    pub fn txf(&self) -> TxfR {
        TxfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Level of rxframe signal"]
    #[inline(always)]
    pub fn rxf(&self) -> RxfR {
        RxfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Txframe fall flag"]
    #[inline(always)]
    pub fn tff(&self) -> TffR {
        TffR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Txframe rise flag"]
    #[inline(always)]
    pub fn tfr(&self) -> TfrR {
        TfrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rxframe fall flag"]
    #[inline(always)]
    pub fn rff(&self) -> RffR {
        RffR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rxframe rise flag"]
    #[inline(always)]
    pub fn rfr(&self) -> RfrR {
        RfrR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SAM_V interface enable"]
    #[inline(always)]
    #[must_use]
    pub fn samen(&mut self) -> SamenW<SamcsSpec> {
        SamenW::new(self, 0)
    }
    #[doc = "Bit 1 - SAM_V interface timeout detect enable"]
    #[inline(always)]
    #[must_use]
    pub fn stoen(&mut self) -> StoenW<SamcsSpec> {
        StoenW::new(self, 1)
    }
    #[doc = "Bit 4 - Txframe fall interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tffie(&mut self) -> TffieW<SamcsSpec> {
        TffieW::new(self, 4)
    }
    #[doc = "Bit 5 - Txframe rise interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfrie(&mut self) -> TfrieW<SamcsSpec> {
        TfrieW::new(self, 5)
    }
    #[doc = "Bit 6 - Rxframe fall interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rffie(&mut self) -> RffieW<SamcsSpec> {
        RffieW::new(self, 6)
    }
    #[doc = "Bit 7 - Rxframe rise interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfrie(&mut self) -> RfrieW<SamcsSpec> {
        RfrieW::new(self, 7)
    }
    #[doc = "Bit 12 - Txframe fall flag"]
    #[inline(always)]
    #[must_use]
    pub fn tff(&mut self) -> TffW<SamcsSpec> {
        TffW::new(self, 12)
    }
    #[doc = "Bit 13 - Txframe rise flag"]
    #[inline(always)]
    #[must_use]
    pub fn tfr(&mut self) -> TfrW<SamcsSpec> {
        TfrW::new(self, 13)
    }
    #[doc = "Bit 14 - Rxframe fall flag"]
    #[inline(always)]
    #[must_use]
    pub fn rff(&mut self) -> RffW<SamcsSpec> {
        RffW::new(self, 14)
    }
    #[doc = "Bit 15 - Rxframe rise flag"]
    #[inline(always)]
    #[must_use]
    pub fn rfr(&mut self) -> RfrW<SamcsSpec> {
        RfrW::new(self, 15)
    }
}
#[doc = "SAM control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`samcs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`samcs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SamcsSpec;
impl crate::RegisterSpec for SamcsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`samcs::R`](R) reader structure"]
impl crate::Readable for SamcsSpec {}
#[doc = "`write(|w| ..)` method takes [`samcs::W`](W) writer structure"]
impl crate::Writable for SamcsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAMCS to value 0"]
impl crate::Resettable for SamcsSpec {
    const RESET_VALUE: u32 = 0;
}
