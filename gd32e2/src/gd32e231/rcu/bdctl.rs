#[doc = "Register `BDCTL` reader"]
pub type R = crate::R<BdctlSpec>;
#[doc = "Register `BDCTL` writer"]
pub type W = crate::W<BdctlSpec>;
#[doc = "LXTAL enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lxtalen {
    #[doc = "0: LXTAL oscillator Off"]
    Off = 0,
    #[doc = "1: LXTAL oscillator On"]
    On = 1,
}
impl From<Lxtalen> for bool {
    #[inline(always)]
    fn from(variant: Lxtalen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LXTALEN` reader - LXTAL enable"]
pub type LxtalenR = crate::BitReader<Lxtalen>;
impl LxtalenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lxtalen {
        match self.bits {
            false => Lxtalen::Off,
            true => Lxtalen::On,
        }
    }
    #[doc = "LXTAL oscillator Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Lxtalen::Off
    }
    #[doc = "LXTAL oscillator On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Lxtalen::On
    }
}
#[doc = "Field `LXTALEN` writer - LXTAL enable"]
pub type LxtalenW<'a, REG> = crate::BitWriter<'a, REG, Lxtalen>;
impl<'a, REG> LxtalenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LXTAL oscillator Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Lxtalen::Off)
    }
    #[doc = "LXTAL oscillator On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Lxtalen::On)
    }
}
#[doc = "External low-speed oscillator stabilization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lxtalstbr {
    #[doc = "0: LXTAL oscillator not ready"]
    NotReady = 0,
    #[doc = "1: LXTAL oscillator ready"]
    Ready = 1,
}
impl From<Lxtalstbr> for bool {
    #[inline(always)]
    fn from(variant: Lxtalstbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LXTALSTB` reader - External low-speed oscillator stabilization"]
pub type LxtalstbR = crate::BitReader<Lxtalstbr>;
impl LxtalstbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lxtalstbr {
        match self.bits {
            false => Lxtalstbr::NotReady,
            true => Lxtalstbr::Ready,
        }
    }
    #[doc = "LXTAL oscillator not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Lxtalstbr::NotReady
    }
    #[doc = "LXTAL oscillator ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Lxtalstbr::Ready
    }
}
#[doc = "LXTAL bypass mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lxtalbps {
    #[doc = "0: LXTAL crystal oscillator not bypassed"]
    NotBypassed = 0,
    #[doc = "1: LXTAL crystal oscillator bypassed with external clock"]
    Bypassed = 1,
}
impl From<Lxtalbps> for bool {
    #[inline(always)]
    fn from(variant: Lxtalbps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LXTALBPS` reader - LXTAL bypass mode enable"]
pub type LxtalbpsR = crate::BitReader<Lxtalbps>;
impl LxtalbpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lxtalbps {
        match self.bits {
            false => Lxtalbps::NotBypassed,
            true => Lxtalbps::Bypassed,
        }
    }
    #[doc = "LXTAL crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == Lxtalbps::NotBypassed
    }
    #[doc = "LXTAL crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == Lxtalbps::Bypassed
    }
}
#[doc = "Field `LXTALBPS` writer - LXTAL bypass mode enable"]
pub type LxtalbpsW<'a, REG> = crate::BitWriter<'a, REG, Lxtalbps>;
impl<'a, REG> LxtalbpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LXTAL crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(Lxtalbps::NotBypassed)
    }
    #[doc = "LXTAL crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(Lxtalbps::Bypassed)
    }
}
#[doc = "LXTAL drive capability\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lxtaldri {
    #[doc = "0: Low driving capability"]
    Low = 0,
    #[doc = "1: Medium low driving capability"]
    MediumLow = 1,
    #[doc = "2: Medium high driving capability"]
    MediumHigh = 2,
    #[doc = "3: High driving capability (reset value)"]
    High = 3,
}
impl From<Lxtaldri> for u8 {
    #[inline(always)]
    fn from(variant: Lxtaldri) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lxtaldri {
    type Ux = u8;
}
#[doc = "Field `LXTALDRI` reader - LXTAL drive capability"]
pub type LxtaldriR = crate::FieldReader<Lxtaldri>;
impl LxtaldriR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lxtaldri {
        match self.bits {
            0 => Lxtaldri::Low,
            1 => Lxtaldri::MediumLow,
            2 => Lxtaldri::MediumHigh,
            3 => Lxtaldri::High,
            _ => unreachable!(),
        }
    }
    #[doc = "Low driving capability"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Lxtaldri::Low
    }
    #[doc = "Medium low driving capability"]
    #[inline(always)]
    pub fn is_medium_low(&self) -> bool {
        *self == Lxtaldri::MediumLow
    }
    #[doc = "Medium high driving capability"]
    #[inline(always)]
    pub fn is_medium_high(&self) -> bool {
        *self == Lxtaldri::MediumHigh
    }
    #[doc = "High driving capability (reset value)"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Lxtaldri::High
    }
}
#[doc = "Field `LXTALDRI` writer - LXTAL drive capability"]
pub type LxtaldriW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Lxtaldri>;
impl<'a, REG> LxtaldriW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low driving capability"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Lxtaldri::Low)
    }
    #[doc = "Medium low driving capability"]
    #[inline(always)]
    pub fn medium_low(self) -> &'a mut crate::W<REG> {
        self.variant(Lxtaldri::MediumLow)
    }
    #[doc = "Medium high driving capability"]
    #[inline(always)]
    pub fn medium_high(self) -> &'a mut crate::W<REG> {
        self.variant(Lxtaldri::MediumHigh)
    }
    #[doc = "High driving capability (reset value)"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Lxtaldri::High)
    }
}
#[doc = "RTC clock entry selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtcsrc {
    #[doc = "0: No clock"]
    NoClock = 0,
    #[doc = "1: LXTAL oscillator clock used as RTC clock"]
    Lxtal = 1,
    #[doc = "2: IRC40K oscillator clock used as RTC clock"]
    Irc40k = 2,
    #[doc = "3: HXTAL oscillator / 32 used as RTC clock"]
    Hxtal = 3,
}
impl From<Rtcsrc> for u8 {
    #[inline(always)]
    fn from(variant: Rtcsrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtcsrc {
    type Ux = u8;
}
#[doc = "Field `RTCSRC` reader - RTC clock entry selection"]
pub type RtcsrcR = crate::FieldReader<Rtcsrc>;
impl RtcsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcsrc {
        match self.bits {
            0 => Rtcsrc::NoClock,
            1 => Rtcsrc::Lxtal,
            2 => Rtcsrc::Irc40k,
            3 => Rtcsrc::Hxtal,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == Rtcsrc::NoClock
    }
    #[doc = "LXTAL oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn is_lxtal(&self) -> bool {
        *self == Rtcsrc::Lxtal
    }
    #[doc = "IRC40K oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn is_irc40k(&self) -> bool {
        *self == Rtcsrc::Irc40k
    }
    #[doc = "HXTAL oscillator / 32 used as RTC clock"]
    #[inline(always)]
    pub fn is_hxtal(&self) -> bool {
        *self == Rtcsrc::Hxtal
    }
}
#[doc = "Field `RTCSRC` writer - RTC clock entry selection"]
pub type RtcsrcW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Rtcsrc>;
impl<'a, REG> RtcsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcsrc::NoClock)
    }
    #[doc = "LXTAL oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn lxtal(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcsrc::Lxtal)
    }
    #[doc = "IRC40K oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn irc40k(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcsrc::Irc40k)
    }
    #[doc = "HXTAL oscillator / 32 used as RTC clock"]
    #[inline(always)]
    pub fn hxtal(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcsrc::Hxtal)
    }
}
#[doc = "RTC clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcen {
    #[doc = "0: RTC clock disabled"]
    Disabled = 0,
    #[doc = "1: RTC clock enabled"]
    Enabled = 1,
}
impl From<Rtcen> for bool {
    #[inline(always)]
    fn from(variant: Rtcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEN` reader - RTC clock enable"]
pub type RtcenR = crate::BitReader<Rtcen>;
impl RtcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcen {
        match self.bits {
            false => Rtcen::Disabled,
            true => Rtcen::Enabled,
        }
    }
    #[doc = "RTC clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rtcen::Disabled
    }
    #[doc = "RTC clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rtcen::Enabled
    }
}
#[doc = "Field `RTCEN` writer - RTC clock enable"]
pub type RtcenW<'a, REG> = crate::BitWriter<'a, REG, Rtcen>;
impl<'a, REG> RtcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcen::Disabled)
    }
    #[doc = "RTC clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcen::Enabled)
    }
}
#[doc = "Backup domain reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bkprst {
    #[doc = "0: Reset not activated"]
    NoReset = 0,
    #[doc = "1: Reset the entire RTC domain"]
    Reset = 1,
}
impl From<Bkprst> for bool {
    #[inline(always)]
    fn from(variant: Bkprst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKPRST` reader - Backup domain reset"]
pub type BkprstR = crate::BitReader<Bkprst>;
impl BkprstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bkprst {
        match self.bits {
            false => Bkprst::NoReset,
            true => Bkprst::Reset,
        }
    }
    #[doc = "Reset not activated"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == Bkprst::NoReset
    }
    #[doc = "Reset the entire RTC domain"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Bkprst::Reset
    }
}
#[doc = "Field `BKPRST` writer - Backup domain reset"]
pub type BkprstW<'a, REG> = crate::BitWriter<'a, REG, Bkprst>;
impl<'a, REG> BkprstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset not activated"]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Bkprst::NoReset)
    }
    #[doc = "Reset the entire RTC domain"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Bkprst::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - LXTAL enable"]
    #[inline(always)]
    pub fn lxtalen(&self) -> LxtalenR {
        LxtalenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External low-speed oscillator stabilization"]
    #[inline(always)]
    pub fn lxtalstb(&self) -> LxtalstbR {
        LxtalstbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LXTAL bypass mode enable"]
    #[inline(always)]
    pub fn lxtalbps(&self) -> LxtalbpsR {
        LxtalbpsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - LXTAL drive capability"]
    #[inline(always)]
    pub fn lxtaldri(&self) -> LxtaldriR {
        LxtaldriR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 8:9 - RTC clock entry selection"]
    #[inline(always)]
    pub fn rtcsrc(&self) -> RtcsrcR {
        RtcsrcR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RtcenR {
        RtcenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Backup domain reset"]
    #[inline(always)]
    pub fn bkprst(&self) -> BkprstR {
        BkprstR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LXTAL enable"]
    #[inline(always)]
    #[must_use]
    pub fn lxtalen(&mut self) -> LxtalenW<BdctlSpec> {
        LxtalenW::new(self, 0)
    }
    #[doc = "Bit 2 - LXTAL bypass mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn lxtalbps(&mut self) -> LxtalbpsW<BdctlSpec> {
        LxtalbpsW::new(self, 2)
    }
    #[doc = "Bits 3:4 - LXTAL drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn lxtaldri(&mut self) -> LxtaldriW<BdctlSpec> {
        LxtaldriW::new(self, 3)
    }
    #[doc = "Bits 8:9 - RTC clock entry selection"]
    #[inline(always)]
    #[must_use]
    pub fn rtcsrc(&mut self) -> RtcsrcW<BdctlSpec> {
        RtcsrcW::new(self, 8)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RtcenW<BdctlSpec> {
        RtcenW::new(self, 15)
    }
    #[doc = "Bit 16 - Backup domain reset"]
    #[inline(always)]
    #[must_use]
    pub fn bkprst(&mut self) -> BkprstW<BdctlSpec> {
        BkprstW::new(self, 16)
    }
}
#[doc = "Backup domain control register (RCU_BDCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BdctlSpec;
impl crate::RegisterSpec for BdctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdctl::R`](R) reader structure"]
impl crate::Readable for BdctlSpec {}
#[doc = "`write(|w| ..)` method takes [`bdctl::W`](W) writer structure"]
impl crate::Writable for BdctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BDCTL to value 0x18"]
impl crate::Resettable for BdctlSpec {
    const RESET_VALUE: u32 = 0x18;
}
