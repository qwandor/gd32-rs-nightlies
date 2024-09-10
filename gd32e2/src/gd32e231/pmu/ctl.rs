#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "LDO Low Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ldolp {
    #[doc = "0: LDO operates normally during Deepsleep mode"]
    Normal = 0,
    #[doc = "1: LDO in low-power mode during Deepsleep mode"]
    LowPower = 1,
}
impl From<Ldolp> for bool {
    #[inline(always)]
    fn from(variant: Ldolp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDOLP` reader - LDO Low Power Mode"]
pub type LdolpR = crate::BitReader<Ldolp>;
impl LdolpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ldolp {
        match self.bits {
            false => Ldolp::Normal,
            true => Ldolp::LowPower,
        }
    }
    #[doc = "LDO operates normally during Deepsleep mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Ldolp::Normal
    }
    #[doc = "LDO in low-power mode during Deepsleep mode"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == Ldolp::LowPower
    }
}
#[doc = "Field `LDOLP` writer - LDO Low Power Mode"]
pub type LdolpW<'a, REG> = crate::BitWriter<'a, REG, Ldolp>;
impl<'a, REG> LdolpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LDO operates normally during Deepsleep mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Ldolp::Normal)
    }
    #[doc = "LDO in low-power mode during Deepsleep mode"]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut crate::W<REG> {
        self.variant(Ldolp::LowPower)
    }
}
#[doc = "Standby Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stbmod {
    #[doc = "0: Enter Deep-sleep mode when the CPU enters deepsleep"]
    DeepSleep = 0,
    #[doc = "1: Enter Standby mode when the CPU enters deepsleep"]
    Standby = 1,
}
impl From<Stbmod> for bool {
    #[inline(always)]
    fn from(variant: Stbmod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STBMOD` reader - Standby Mode"]
pub type StbmodR = crate::BitReader<Stbmod>;
impl StbmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stbmod {
        match self.bits {
            false => Stbmod::DeepSleep,
            true => Stbmod::Standby,
        }
    }
    #[doc = "Enter Deep-sleep mode when the CPU enters deepsleep"]
    #[inline(always)]
    pub fn is_deep_sleep(&self) -> bool {
        *self == Stbmod::DeepSleep
    }
    #[doc = "Enter Standby mode when the CPU enters deepsleep"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == Stbmod::Standby
    }
}
#[doc = "Field `STBMOD` writer - Standby Mode"]
pub type StbmodW<'a, REG> = crate::BitWriter<'a, REG, Stbmod>;
impl<'a, REG> StbmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enter Deep-sleep mode when the CPU enters deepsleep"]
    #[inline(always)]
    pub fn deep_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(Stbmod::DeepSleep)
    }
    #[doc = "Enter Standby mode when the CPU enters deepsleep"]
    #[inline(always)]
    pub fn standby(self) -> &'a mut crate::W<REG> {
        self.variant(Stbmod::Standby)
    }
}
#[doc = "Wakeup Flag Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wurstw {
    #[doc = "1: Clear the wakeup flag"]
    Clear = 1,
}
impl From<Wurstw> for bool {
    #[inline(always)]
    fn from(variant: Wurstw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WURST` reader - Wakeup Flag Reset"]
pub type WurstR = crate::BitReader<Wurstw>;
impl WurstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wurstw> {
        match self.bits {
            true => Some(Wurstw::Clear),
            _ => None,
        }
    }
    #[doc = "Clear the wakeup flag"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Wurstw::Clear
    }
}
#[doc = "Field `WURST` writer - Wakeup Flag Reset"]
pub type WurstW<'a, REG> = crate::BitWriter<'a, REG, Wurstw>;
impl<'a, REG> WurstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the wakeup flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Wurstw::Clear)
    }
}
#[doc = "Standby Flag Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stbrstw {
    #[doc = "1: Clear the standby flag"]
    Clear = 1,
}
impl From<Stbrstw> for bool {
    #[inline(always)]
    fn from(variant: Stbrstw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STBRST` reader - Standby Flag Reset"]
pub type StbrstR = crate::BitReader<Stbrstw>;
impl StbrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stbrstw> {
        match self.bits {
            true => Some(Stbrstw::Clear),
            _ => None,
        }
    }
    #[doc = "Clear the standby flag"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Stbrstw::Clear
    }
}
#[doc = "Field `STBRST` writer - Standby Flag Reset"]
pub type StbrstW<'a, REG> = crate::BitWriter<'a, REG, Stbrstw>;
impl<'a, REG> StbrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the standby flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Stbrstw::Clear)
    }
}
#[doc = "Low Voltage Detector Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvden {
    #[doc = "0: Low voltage detector disabled"]
    Disabled = 0,
    #[doc = "1: Low voltage detector enabled"]
    Enabled = 1,
}
impl From<Lvden> for bool {
    #[inline(always)]
    fn from(variant: Lvden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVDEN` reader - Low Voltage Detector Enable"]
pub type LvdenR = crate::BitReader<Lvden>;
impl LvdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvden {
        match self.bits {
            false => Lvden::Disabled,
            true => Lvden::Enabled,
        }
    }
    #[doc = "Low voltage detector disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lvden::Disabled
    }
    #[doc = "Low voltage detector enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lvden::Enabled
    }
}
#[doc = "Field `LVDEN` writer - Low Voltage Detector Enable"]
pub type LvdenW<'a, REG> = crate::BitWriter<'a, REG, Lvden>;
impl<'a, REG> LvdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low voltage detector disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lvden::Disabled)
    }
    #[doc = "Low voltage detector enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lvden::Enabled)
    }
}
#[doc = "Low Voltage Detector Threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lvdt {
    #[doc = "0: 2.2 V"]
    V2_2 = 0,
    #[doc = "1: 2.3 V"]
    V2_3 = 1,
    #[doc = "2: 2.4 V"]
    V2_4 = 2,
    #[doc = "3: 2.5 V"]
    V2_5 = 3,
    #[doc = "4: 2.6 V"]
    V2_6 = 4,
    #[doc = "5: 2.7 V"]
    V2_7 = 5,
    #[doc = "6: 2.8 V"]
    V2_8 = 6,
    #[doc = "7: 2.9 V"]
    V2_9 = 7,
}
impl From<Lvdt> for u8 {
    #[inline(always)]
    fn from(variant: Lvdt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lvdt {
    type Ux = u8;
}
#[doc = "Field `LVDT` reader - Low Voltage Detector Threshold"]
pub type LvdtR = crate::FieldReader<Lvdt>;
impl LvdtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvdt {
        match self.bits {
            0 => Lvdt::V2_2,
            1 => Lvdt::V2_3,
            2 => Lvdt::V2_4,
            3 => Lvdt::V2_5,
            4 => Lvdt::V2_6,
            5 => Lvdt::V2_7,
            6 => Lvdt::V2_8,
            7 => Lvdt::V2_9,
            _ => unreachable!(),
        }
    }
    #[doc = "2.2 V"]
    #[inline(always)]
    pub fn is_v2_2(&self) -> bool {
        *self == Lvdt::V2_2
    }
    #[doc = "2.3 V"]
    #[inline(always)]
    pub fn is_v2_3(&self) -> bool {
        *self == Lvdt::V2_3
    }
    #[doc = "2.4 V"]
    #[inline(always)]
    pub fn is_v2_4(&self) -> bool {
        *self == Lvdt::V2_4
    }
    #[doc = "2.5 V"]
    #[inline(always)]
    pub fn is_v2_5(&self) -> bool {
        *self == Lvdt::V2_5
    }
    #[doc = "2.6 V"]
    #[inline(always)]
    pub fn is_v2_6(&self) -> bool {
        *self == Lvdt::V2_6
    }
    #[doc = "2.7 V"]
    #[inline(always)]
    pub fn is_v2_7(&self) -> bool {
        *self == Lvdt::V2_7
    }
    #[doc = "2.8 V"]
    #[inline(always)]
    pub fn is_v2_8(&self) -> bool {
        *self == Lvdt::V2_8
    }
    #[doc = "2.9 V"]
    #[inline(always)]
    pub fn is_v2_9(&self) -> bool {
        *self == Lvdt::V2_9
    }
}
#[doc = "Field `LVDT` writer - Low Voltage Detector Threshold"]
pub type LvdtW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Lvdt>;
impl<'a, REG> LvdtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.2 V"]
    #[inline(always)]
    pub fn v2_2(self) -> &'a mut crate::W<REG> {
        self.variant(Lvdt::V2_2)
    }
    #[doc = "2.3 V"]
    #[inline(always)]
    pub fn v2_3(self) -> &'a mut crate::W<REG> {
        self.variant(Lvdt::V2_3)
    }
    #[doc = "2.4 V"]
    #[inline(always)]
    pub fn v2_4(self) -> &'a mut crate::W<REG> {
        self.variant(Lvdt::V2_4)
    }
    #[doc = "2.5 V"]
    #[inline(always)]
    pub fn v2_5(self) -> &'a mut crate::W<REG> {
        self.variant(Lvdt::V2_5)
    }
    #[doc = "2.6 V"]
    #[inline(always)]
    pub fn v2_6(self) -> &'a mut crate::W<REG> {
        self.variant(Lvdt::V2_6)
    }
    #[doc = "2.7 V"]
    #[inline(always)]
    pub fn v2_7(self) -> &'a mut crate::W<REG> {
        self.variant(Lvdt::V2_7)
    }
    #[doc = "2.8 V"]
    #[inline(always)]
    pub fn v2_8(self) -> &'a mut crate::W<REG> {
        self.variant(Lvdt::V2_8)
    }
    #[doc = "2.9 V"]
    #[inline(always)]
    pub fn v2_9(self) -> &'a mut crate::W<REG> {
        self.variant(Lvdt::V2_9)
    }
}
#[doc = "Backup Domain Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bkpwen {
    #[doc = "0: Access to backup domain registers disabled"]
    Disabled = 0,
    #[doc = "1: Access to backup domain registers enabled"]
    Enabled = 1,
}
impl From<Bkpwen> for bool {
    #[inline(always)]
    fn from(variant: Bkpwen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKPWEN` reader - Backup Domain Write Enable"]
pub type BkpwenR = crate::BitReader<Bkpwen>;
impl BkpwenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bkpwen {
        match self.bits {
            false => Bkpwen::Disabled,
            true => Bkpwen::Enabled,
        }
    }
    #[doc = "Access to backup domain registers disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Bkpwen::Disabled
    }
    #[doc = "Access to backup domain registers enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Bkpwen::Enabled
    }
}
#[doc = "Field `BKPWEN` writer - Backup Domain Write Enable"]
pub type BkpwenW<'a, REG> = crate::BitWriter<'a, REG, Bkpwen>;
impl<'a, REG> BkpwenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Access to backup domain registers disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Bkpwen::Disabled)
    }
    #[doc = "Access to backup domain registers enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Bkpwen::Enabled)
    }
}
#[doc = "Field `LDOVS` reader - LDO output voltage select"]
pub type LdovsR = crate::FieldReader;
#[doc = "Field `LDOVS` writer - LDO output voltage select"]
pub type LdovsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - LDO Low Power Mode"]
    #[inline(always)]
    pub fn ldolp(&self) -> LdolpR {
        LdolpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Standby Mode"]
    #[inline(always)]
    pub fn stbmod(&self) -> StbmodR {
        StbmodR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup Flag Reset"]
    #[inline(always)]
    pub fn wurst(&self) -> WurstR {
        WurstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Standby Flag Reset"]
    #[inline(always)]
    pub fn stbrst(&self) -> StbrstR {
        StbrstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Low Voltage Detector Enable"]
    #[inline(always)]
    pub fn lvden(&self) -> LvdenR {
        LvdenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Low Voltage Detector Threshold"]
    #[inline(always)]
    pub fn lvdt(&self) -> LvdtR {
        LvdtR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Backup Domain Write Enable"]
    #[inline(always)]
    pub fn bkpwen(&self) -> BkpwenR {
        BkpwenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 14:15 - LDO output voltage select"]
    #[inline(always)]
    pub fn ldovs(&self) -> LdovsR {
        LdovsR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LDO Low Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ldolp(&mut self) -> LdolpW<CtlSpec> {
        LdolpW::new(self, 0)
    }
    #[doc = "Bit 1 - Standby Mode"]
    #[inline(always)]
    #[must_use]
    pub fn stbmod(&mut self) -> StbmodW<CtlSpec> {
        StbmodW::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup Flag Reset"]
    #[inline(always)]
    #[must_use]
    pub fn wurst(&mut self) -> WurstW<CtlSpec> {
        WurstW::new(self, 2)
    }
    #[doc = "Bit 3 - Standby Flag Reset"]
    #[inline(always)]
    #[must_use]
    pub fn stbrst(&mut self) -> StbrstW<CtlSpec> {
        StbrstW::new(self, 3)
    }
    #[doc = "Bit 4 - Low Voltage Detector Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvden(&mut self) -> LvdenW<CtlSpec> {
        LvdenW::new(self, 4)
    }
    #[doc = "Bits 5:7 - Low Voltage Detector Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn lvdt(&mut self) -> LvdtW<CtlSpec> {
        LvdtW::new(self, 5)
    }
    #[doc = "Bit 8 - Backup Domain Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkpwen(&mut self) -> BkpwenW<CtlSpec> {
        BkpwenW::new(self, 8)
    }
    #[doc = "Bits 14:15 - LDO output voltage select"]
    #[inline(always)]
    #[must_use]
    pub fn ldovs(&mut self) -> LdovsW<CtlSpec> {
        LdovsW::new(self, 14)
    }
}
#[doc = "power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0x4000"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0x4000;
}
