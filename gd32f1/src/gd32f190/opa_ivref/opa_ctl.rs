#[doc = "Register `OPA_CTL` reader"]
pub type R = crate::R<OpaCtlSpec>;
#[doc = "Register `OPA_CTL` writer"]
pub type W = crate::W<OpaCtlSpec>;
#[doc = "OPA0 power down\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opa0pd {
    #[doc = "0: OPA enabled"]
    Enabled = 0,
    #[doc = "1: OPA disabled"]
    Disabled = 1,
}
impl From<Opa0pd> for bool {
    #[inline(always)]
    fn from(variant: Opa0pd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPA0PD` reader - OPA0 power down"]
pub type Opa0pdR = crate::BitReader<Opa0pd>;
impl Opa0pdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Opa0pd {
        match self.bits {
            false => Opa0pd::Enabled,
            true => Opa0pd::Disabled,
        }
    }
    #[doc = "OPA enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Opa0pd::Enabled
    }
    #[doc = "OPA disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Opa0pd::Disabled
    }
}
#[doc = "Field `OPA0PD` writer - OPA0 power down"]
pub type Opa0pdW<'a, REG> = crate::BitWriter<'a, REG, Opa0pd>;
impl<'a, REG> Opa0pdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OPA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Opa0pd::Enabled)
    }
    #[doc = "OPA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Opa0pd::Disabled)
    }
}
#[doc = "T3 switch enable for OPA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T3opa0 {
    #[doc = "0: Switch opened"]
    Opened = 0,
    #[doc = "1: Switch closed"]
    Closed = 1,
}
impl From<T3opa0> for bool {
    #[inline(always)]
    fn from(variant: T3opa0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `T3OPA0` reader - T3 switch enable for OPA0"]
pub type T3opa0R = crate::BitReader<T3opa0>;
impl T3opa0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> T3opa0 {
        match self.bits {
            false => T3opa0::Opened,
            true => T3opa0::Closed,
        }
    }
    #[doc = "Switch opened"]
    #[inline(always)]
    pub fn is_opened(&self) -> bool {
        *self == T3opa0::Opened
    }
    #[doc = "Switch closed"]
    #[inline(always)]
    pub fn is_closed(&self) -> bool {
        *self == T3opa0::Closed
    }
}
#[doc = "Field `T3OPA0` writer - T3 switch enable for OPA0"]
pub type T3opa0W<'a, REG> = crate::BitWriter<'a, REG, T3opa0>;
impl<'a, REG> T3opa0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Switch opened"]
    #[inline(always)]
    pub fn opened(self) -> &'a mut crate::W<REG> {
        self.variant(T3opa0::Opened)
    }
    #[doc = "Switch closed"]
    #[inline(always)]
    pub fn closed(self) -> &'a mut crate::W<REG> {
        self.variant(T3opa0::Closed)
    }
}
#[doc = "Field `S1OPA0` reader - S1 switch enable for OPA0"]
pub use T3opa0R as S1opa0R;
#[doc = "Field `S2OPA0` reader - S2 switch enable for OPA0"]
pub use T3opa0R as S2opa0R;
#[doc = "Field `S3OPA0` reader - S3 switch enable for OPA0"]
pub use T3opa0R as S3opa0R;
#[doc = "Field `S1OPA0` writer - S1 switch enable for OPA0"]
pub use T3opa0W as S1opa0W;
#[doc = "Field `S2OPA0` writer - S2 switch enable for OPA0"]
pub use T3opa0W as S2opa0W;
#[doc = "Field `S3OPA0` writer - S3 switch enable for OPA0"]
pub use T3opa0W as S3opa0W;
#[doc = "OPA0 offset calibration for P diff\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opa0calL {
    #[doc = "0: Offset calibration for P diff disabled"]
    Disabled = 0,
    #[doc = "1: Offset calibration for P diff enabled"]
    Enabled = 1,
}
impl From<Opa0calL> for bool {
    #[inline(always)]
    fn from(variant: Opa0calL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPA0CAL_L` reader - OPA0 offset calibration for P diff"]
pub type Opa0calLR = crate::BitReader<Opa0calL>;
impl Opa0calLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Opa0calL {
        match self.bits {
            false => Opa0calL::Disabled,
            true => Opa0calL::Enabled,
        }
    }
    #[doc = "Offset calibration for P diff disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Opa0calL::Disabled
    }
    #[doc = "Offset calibration for P diff enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Opa0calL::Enabled
    }
}
#[doc = "Field `OPA0CAL_L` writer - OPA0 offset calibration for P diff"]
pub type Opa0calLW<'a, REG> = crate::BitWriter<'a, REG, Opa0calL>;
impl<'a, REG> Opa0calLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Offset calibration for P diff disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Opa0calL::Disabled)
    }
    #[doc = "Offset calibration for P diff enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Opa0calL::Enabled)
    }
}
#[doc = "OPA0 offset calibration for N diff\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opa0calH {
    #[doc = "0: Offset calibration for N diff disabled"]
    Disabled = 0,
    #[doc = "1: Offset calibration for N diff enabled"]
    Enabled = 1,
}
impl From<Opa0calH> for bool {
    #[inline(always)]
    fn from(variant: Opa0calH) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPA0CAL_H` reader - OPA0 offset calibration for N diff"]
pub type Opa0calHR = crate::BitReader<Opa0calH>;
impl Opa0calHR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Opa0calH {
        match self.bits {
            false => Opa0calH::Disabled,
            true => Opa0calH::Enabled,
        }
    }
    #[doc = "Offset calibration for N diff disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Opa0calH::Disabled
    }
    #[doc = "Offset calibration for N diff enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Opa0calH::Enabled
    }
}
#[doc = "Field `OPA0CAL_H` writer - OPA0 offset calibration for N diff"]
pub type Opa0calHW<'a, REG> = crate::BitWriter<'a, REG, Opa0calH>;
impl<'a, REG> Opa0calHW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Offset calibration for N diff disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Opa0calH::Disabled)
    }
    #[doc = "Offset calibration for N diff enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Opa0calH::Enabled)
    }
}
#[doc = "OPA0 low power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opa0lpm {
    #[doc = "0: Low power mode disabled"]
    Disabled = 0,
    #[doc = "1: Low power mode enabled"]
    Enabled = 1,
}
impl From<Opa0lpm> for bool {
    #[inline(always)]
    fn from(variant: Opa0lpm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPA0LPM` reader - OPA0 low power mode"]
pub type Opa0lpmR = crate::BitReader<Opa0lpm>;
impl Opa0lpmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Opa0lpm {
        match self.bits {
            false => Opa0lpm::Disabled,
            true => Opa0lpm::Enabled,
        }
    }
    #[doc = "Low power mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Opa0lpm::Disabled
    }
    #[doc = "Low power mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Opa0lpm::Enabled
    }
}
#[doc = "Field `OPA0LPM` writer - OPA0 low power mode"]
pub type Opa0lpmW<'a, REG> = crate::BitWriter<'a, REG, Opa0lpm>;
impl<'a, REG> Opa0lpmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low power mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Opa0lpm::Disabled)
    }
    #[doc = "Low power mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Opa0lpm::Enabled)
    }
}
#[doc = "Field `OPA1CAL_H` reader - OPA1 offset calibration for N diff"]
pub use Opa0calHR as Opa1calHR;
#[doc = "Field `OPA2CAL_H` reader - OPA2 offset calibration for N diff"]
pub use Opa0calHR as Opa2calHR;
#[doc = "Field `OPA1CAL_H` writer - OPA1 offset calibration for N diff"]
pub use Opa0calHW as Opa1calHW;
#[doc = "Field `OPA2CAL_H` writer - OPA2 offset calibration for N diff"]
pub use Opa0calHW as Opa2calHW;
#[doc = "Field `OPA1CAL_L` reader - OPA1 offset calibration for P diff"]
pub use Opa0calLR as Opa1calLR;
#[doc = "Field `OPA2CAL_L` reader - OPA2 offset calibration for P diff"]
pub use Opa0calLR as Opa2calLR;
#[doc = "Field `OPA1CAL_L` writer - OPA1 offset calibration for P diff"]
pub use Opa0calLW as Opa1calLW;
#[doc = "Field `OPA2CAL_L` writer - OPA2 offset calibration for P diff"]
pub use Opa0calLW as Opa2calLW;
#[doc = "Field `OPA1LPM` reader - OPA1 low power mode"]
pub use Opa0lpmR as Opa1lpmR;
#[doc = "Field `OPA2LPM` reader - OPA2 low power mode"]
pub use Opa0lpmR as Opa2lpmR;
#[doc = "Field `OPA1LPM` writer - OPA1 low power mode"]
pub use Opa0lpmW as Opa1lpmW;
#[doc = "Field `OPA2LPM` writer - OPA2 low power mode"]
pub use Opa0lpmW as Opa2lpmW;
#[doc = "Field `OPA1PD` reader - OPA1 power down"]
pub use Opa0pdR as Opa1pdR;
#[doc = "Field `OPA2PD` reader - OPA2 power down"]
pub use Opa0pdR as Opa2pdR;
#[doc = "Field `OPA1PD` writer - OPA1 power down"]
pub use Opa0pdW as Opa1pdW;
#[doc = "Field `OPA2PD` writer - OPA2 power down"]
pub use Opa0pdW as Opa2pdW;
#[doc = "Field `T3OPA1` reader - T3 switch enable for OPA1"]
pub use T3opa0R as T3opa1R;
#[doc = "Field `S1OPA1` reader - S1 switch enable for OPA1"]
pub use T3opa0R as S1opa1R;
#[doc = "Field `S2OPA1` reader - S2 switch enable for OPA1"]
pub use T3opa0R as S2opa1R;
#[doc = "Field `S3OPA1` reader - S3 switch enable for OPA1"]
pub use T3opa0R as S3opa1R;
#[doc = "Field `T3OPA2` reader - T3 switch enable for OPA2"]
pub use T3opa0R as T3opa2R;
#[doc = "Field `S1OPA2` reader - S1 switch enable for OPA2"]
pub use T3opa0R as S1opa2R;
#[doc = "Field `S2OPA2` reader - S2 switch enable for OPA2"]
pub use T3opa0R as S2opa2R;
#[doc = "Field `S3OPA2` reader - S3 switch enable for OPA2"]
pub use T3opa0R as S3opa2R;
#[doc = "Field `S4OPA1` reader - S4 switch enable for OPA1"]
pub use T3opa0R as S4opa1R;
#[doc = "Field `T3OPA1` writer - T3 switch enable for OPA1"]
pub use T3opa0W as T3opa1W;
#[doc = "Field `S1OPA1` writer - S1 switch enable for OPA1"]
pub use T3opa0W as S1opa1W;
#[doc = "Field `S2OPA1` writer - S2 switch enable for OPA1"]
pub use T3opa0W as S2opa1W;
#[doc = "Field `S3OPA1` writer - S3 switch enable for OPA1"]
pub use T3opa0W as S3opa1W;
#[doc = "Field `T3OPA2` writer - T3 switch enable for OPA2"]
pub use T3opa0W as T3opa2W;
#[doc = "Field `S1OPA2` writer - S1 switch enable for OPA2"]
pub use T3opa0W as S1opa2W;
#[doc = "Field `S2OPA2` writer - S2 switch enable for OPA2"]
pub use T3opa0W as S2opa2W;
#[doc = "Field `S3OPA2` writer - S3 switch enable for OPA2"]
pub use T3opa0W as S3opa2W;
#[doc = "Field `S4OPA1` writer - S4 switch enable for OPA1"]
pub use T3opa0W as S4opa1W;
#[doc = "Power supply range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpaRange {
    #[doc = "0: Low range, &lt; 3.3 V"]
    Low = 0,
    #[doc = "1: High range, > 3.3 V"]
    High = 1,
}
impl From<OpaRange> for bool {
    #[inline(always)]
    fn from(variant: OpaRange) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPA_RANGE` reader - Power supply range"]
pub type OpaRangeR = crate::BitReader<OpaRange>;
impl OpaRangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OpaRange {
        match self.bits {
            false => OpaRange::Low,
            true => OpaRange::High,
        }
    }
    #[doc = "Low range, &lt; 3.3 V"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OpaRange::Low
    }
    #[doc = "High range, > 3.3 V"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OpaRange::High
    }
}
#[doc = "Field `OPA_RANGE` writer - Power supply range"]
pub type OpaRangeW<'a, REG> = crate::BitWriter<'a, REG, OpaRange>;
impl<'a, REG> OpaRangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low range, &lt; 3.3 V"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(OpaRange::Low)
    }
    #[doc = "High range, > 3.3 V"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(OpaRange::High)
    }
}
#[doc = "OPA0 calibration output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opa0calout {
    #[doc = "0: The offset is not trimmed"]
    NotTrimmed = 0,
    #[doc = "1: The offset is trimmed, in calibration mode"]
    Trimmed = 1,
}
impl From<Opa0calout> for bool {
    #[inline(always)]
    fn from(variant: Opa0calout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPA0CALOUT` reader - OPA0 calibration output"]
pub type Opa0caloutR = crate::BitReader<Opa0calout>;
impl Opa0caloutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Opa0calout {
        match self.bits {
            false => Opa0calout::NotTrimmed,
            true => Opa0calout::Trimmed,
        }
    }
    #[doc = "The offset is not trimmed"]
    #[inline(always)]
    pub fn is_not_trimmed(&self) -> bool {
        *self == Opa0calout::NotTrimmed
    }
    #[doc = "The offset is trimmed, in calibration mode"]
    #[inline(always)]
    pub fn is_trimmed(&self) -> bool {
        *self == Opa0calout::Trimmed
    }
}
#[doc = "Field `OPA0CALOUT` writer - OPA0 calibration output"]
pub type Opa0caloutW<'a, REG> = crate::BitWriter<'a, REG, Opa0calout>;
impl<'a, REG> Opa0caloutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The offset is not trimmed"]
    #[inline(always)]
    pub fn not_trimmed(self) -> &'a mut crate::W<REG> {
        self.variant(Opa0calout::NotTrimmed)
    }
    #[doc = "The offset is trimmed, in calibration mode"]
    #[inline(always)]
    pub fn trimmed(self) -> &'a mut crate::W<REG> {
        self.variant(Opa0calout::Trimmed)
    }
}
#[doc = "Field `OPA1CALOUT` reader - OPA1 calibration output"]
pub use Opa0caloutR as Opa1caloutR;
#[doc = "Field `OPA2CALOUT` reader - OPA2 calibration output"]
pub use Opa0caloutR as Opa2caloutR;
#[doc = "Field `OPA1CALOUT` writer - OPA1 calibration output"]
pub use Opa0caloutW as Opa1caloutW;
#[doc = "Field `OPA2CALOUT` writer - OPA2 calibration output"]
pub use Opa0caloutW as Opa2caloutW;
impl R {
    #[doc = "Bit 0 - OPA0 power down"]
    #[inline(always)]
    pub fn opa0pd(&self) -> Opa0pdR {
        Opa0pdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - T3 switch enable for OPA0"]
    #[inline(always)]
    pub fn t3opa0(&self) -> T3opa0R {
        T3opa0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - S1 switch enable for OPA0"]
    #[inline(always)]
    pub fn s1opa0(&self) -> S1opa0R {
        S1opa0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - S2 switch enable for OPA0"]
    #[inline(always)]
    pub fn s2opa0(&self) -> S2opa0R {
        S2opa0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - S3 switch enable for OPA0"]
    #[inline(always)]
    pub fn s3opa0(&self) -> S3opa0R {
        S3opa0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OPA0 offset calibration for P diff"]
    #[inline(always)]
    pub fn opa0cal_l(&self) -> Opa0calLR {
        Opa0calLR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OPA0 offset calibration for N diff"]
    #[inline(always)]
    pub fn opa0cal_h(&self) -> Opa0calHR {
        Opa0calHR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - OPA0 low power mode"]
    #[inline(always)]
    pub fn opa0lpm(&self) -> Opa0lpmR {
        Opa0lpmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OPA1 power down"]
    #[inline(always)]
    pub fn opa1pd(&self) -> Opa1pdR {
        Opa1pdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - T3 switch enable for OPA1"]
    #[inline(always)]
    pub fn t3opa1(&self) -> T3opa1R {
        T3opa1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - S1 switch enable for OPA1"]
    #[inline(always)]
    pub fn s1opa1(&self) -> S1opa1R {
        S1opa1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - S2 switch enable for OPA1"]
    #[inline(always)]
    pub fn s2opa1(&self) -> S2opa1R {
        S2opa1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - S3 switch enable for OPA1"]
    #[inline(always)]
    pub fn s3opa1(&self) -> S3opa1R {
        S3opa1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - OPA1 offset calibration for P diff"]
    #[inline(always)]
    pub fn opa1cal_l(&self) -> Opa1calLR {
        Opa1calLR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - OPA1 offset calibration for N diff"]
    #[inline(always)]
    pub fn opa1cal_h(&self) -> Opa1calHR {
        Opa1calHR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - OPA1 low power mode"]
    #[inline(always)]
    pub fn opa1lpm(&self) -> Opa1lpmR {
        Opa1lpmR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - OPA2 power down"]
    #[inline(always)]
    pub fn opa2pd(&self) -> Opa2pdR {
        Opa2pdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - T3 switch enable for OPA2"]
    #[inline(always)]
    pub fn t3opa2(&self) -> T3opa2R {
        T3opa2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - S1 switch enable for OPA2"]
    #[inline(always)]
    pub fn s1opa2(&self) -> S1opa2R {
        S1opa2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - S2 switch enable for OPA2"]
    #[inline(always)]
    pub fn s2opa2(&self) -> S2opa2R {
        S2opa2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - S3 switch enable for OPA2"]
    #[inline(always)]
    pub fn s3opa2(&self) -> S3opa2R {
        S3opa2R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OPA2 offset calibration for P diff"]
    #[inline(always)]
    pub fn opa2cal_l(&self) -> Opa2calLR {
        Opa2calLR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OPA2 offset calibration for N diff"]
    #[inline(always)]
    pub fn opa2cal_h(&self) -> Opa2calHR {
        Opa2calHR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - OPA2 low power mode"]
    #[inline(always)]
    pub fn opa2lpm(&self) -> Opa2lpmR {
        Opa2lpmR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 27 - S4 switch enable for OPA1"]
    #[inline(always)]
    pub fn s4opa1(&self) -> S4opa1R {
        S4opa1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power supply range"]
    #[inline(always)]
    pub fn opa_range(&self) -> OpaRangeR {
        OpaRangeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - OPA0 calibration output"]
    #[inline(always)]
    pub fn opa0calout(&self) -> Opa0caloutR {
        Opa0caloutR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - OPA1 calibration output"]
    #[inline(always)]
    pub fn opa1calout(&self) -> Opa1caloutR {
        Opa1caloutR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - OPA2 calibration output"]
    #[inline(always)]
    pub fn opa2calout(&self) -> Opa2caloutR {
        Opa2caloutR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OPA0 power down"]
    #[inline(always)]
    #[must_use]
    pub fn opa0pd(&mut self) -> Opa0pdW<OpaCtlSpec> {
        Opa0pdW::new(self, 0)
    }
    #[doc = "Bit 1 - T3 switch enable for OPA0"]
    #[inline(always)]
    #[must_use]
    pub fn t3opa0(&mut self) -> T3opa0W<OpaCtlSpec> {
        T3opa0W::new(self, 1)
    }
    #[doc = "Bit 2 - S1 switch enable for OPA0"]
    #[inline(always)]
    #[must_use]
    pub fn s1opa0(&mut self) -> S1opa0W<OpaCtlSpec> {
        S1opa0W::new(self, 2)
    }
    #[doc = "Bit 3 - S2 switch enable for OPA0"]
    #[inline(always)]
    #[must_use]
    pub fn s2opa0(&mut self) -> S2opa0W<OpaCtlSpec> {
        S2opa0W::new(self, 3)
    }
    #[doc = "Bit 4 - S3 switch enable for OPA0"]
    #[inline(always)]
    #[must_use]
    pub fn s3opa0(&mut self) -> S3opa0W<OpaCtlSpec> {
        S3opa0W::new(self, 4)
    }
    #[doc = "Bit 5 - OPA0 offset calibration for P diff"]
    #[inline(always)]
    #[must_use]
    pub fn opa0cal_l(&mut self) -> Opa0calLW<OpaCtlSpec> {
        Opa0calLW::new(self, 5)
    }
    #[doc = "Bit 6 - OPA0 offset calibration for N diff"]
    #[inline(always)]
    #[must_use]
    pub fn opa0cal_h(&mut self) -> Opa0calHW<OpaCtlSpec> {
        Opa0calHW::new(self, 6)
    }
    #[doc = "Bit 7 - OPA0 low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn opa0lpm(&mut self) -> Opa0lpmW<OpaCtlSpec> {
        Opa0lpmW::new(self, 7)
    }
    #[doc = "Bit 8 - OPA1 power down"]
    #[inline(always)]
    #[must_use]
    pub fn opa1pd(&mut self) -> Opa1pdW<OpaCtlSpec> {
        Opa1pdW::new(self, 8)
    }
    #[doc = "Bit 9 - T3 switch enable for OPA1"]
    #[inline(always)]
    #[must_use]
    pub fn t3opa1(&mut self) -> T3opa1W<OpaCtlSpec> {
        T3opa1W::new(self, 9)
    }
    #[doc = "Bit 10 - S1 switch enable for OPA1"]
    #[inline(always)]
    #[must_use]
    pub fn s1opa1(&mut self) -> S1opa1W<OpaCtlSpec> {
        S1opa1W::new(self, 10)
    }
    #[doc = "Bit 11 - S2 switch enable for OPA1"]
    #[inline(always)]
    #[must_use]
    pub fn s2opa1(&mut self) -> S2opa1W<OpaCtlSpec> {
        S2opa1W::new(self, 11)
    }
    #[doc = "Bit 12 - S3 switch enable for OPA1"]
    #[inline(always)]
    #[must_use]
    pub fn s3opa1(&mut self) -> S3opa1W<OpaCtlSpec> {
        S3opa1W::new(self, 12)
    }
    #[doc = "Bit 13 - OPA1 offset calibration for P diff"]
    #[inline(always)]
    #[must_use]
    pub fn opa1cal_l(&mut self) -> Opa1calLW<OpaCtlSpec> {
        Opa1calLW::new(self, 13)
    }
    #[doc = "Bit 14 - OPA1 offset calibration for N diff"]
    #[inline(always)]
    #[must_use]
    pub fn opa1cal_h(&mut self) -> Opa1calHW<OpaCtlSpec> {
        Opa1calHW::new(self, 14)
    }
    #[doc = "Bit 15 - OPA1 low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn opa1lpm(&mut self) -> Opa1lpmW<OpaCtlSpec> {
        Opa1lpmW::new(self, 15)
    }
    #[doc = "Bit 16 - OPA2 power down"]
    #[inline(always)]
    #[must_use]
    pub fn opa2pd(&mut self) -> Opa2pdW<OpaCtlSpec> {
        Opa2pdW::new(self, 16)
    }
    #[doc = "Bit 17 - T3 switch enable for OPA2"]
    #[inline(always)]
    #[must_use]
    pub fn t3opa2(&mut self) -> T3opa2W<OpaCtlSpec> {
        T3opa2W::new(self, 17)
    }
    #[doc = "Bit 18 - S1 switch enable for OPA2"]
    #[inline(always)]
    #[must_use]
    pub fn s1opa2(&mut self) -> S1opa2W<OpaCtlSpec> {
        S1opa2W::new(self, 18)
    }
    #[doc = "Bit 19 - S2 switch enable for OPA2"]
    #[inline(always)]
    #[must_use]
    pub fn s2opa2(&mut self) -> S2opa2W<OpaCtlSpec> {
        S2opa2W::new(self, 19)
    }
    #[doc = "Bit 20 - S3 switch enable for OPA2"]
    #[inline(always)]
    #[must_use]
    pub fn s3opa2(&mut self) -> S3opa2W<OpaCtlSpec> {
        S3opa2W::new(self, 20)
    }
    #[doc = "Bit 21 - OPA2 offset calibration for P diff"]
    #[inline(always)]
    #[must_use]
    pub fn opa2cal_l(&mut self) -> Opa2calLW<OpaCtlSpec> {
        Opa2calLW::new(self, 21)
    }
    #[doc = "Bit 22 - OPA2 offset calibration for N diff"]
    #[inline(always)]
    #[must_use]
    pub fn opa2cal_h(&mut self) -> Opa2calHW<OpaCtlSpec> {
        Opa2calHW::new(self, 22)
    }
    #[doc = "Bit 23 - OPA2 low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn opa2lpm(&mut self) -> Opa2lpmW<OpaCtlSpec> {
        Opa2lpmW::new(self, 23)
    }
    #[doc = "Bit 27 - S4 switch enable for OPA1"]
    #[inline(always)]
    #[must_use]
    pub fn s4opa1(&mut self) -> S4opa1W<OpaCtlSpec> {
        S4opa1W::new(self, 27)
    }
    #[doc = "Bit 28 - Power supply range"]
    #[inline(always)]
    #[must_use]
    pub fn opa_range(&mut self) -> OpaRangeW<OpaCtlSpec> {
        OpaRangeW::new(self, 28)
    }
    #[doc = "Bit 29 - OPA0 calibration output"]
    #[inline(always)]
    #[must_use]
    pub fn opa0calout(&mut self) -> Opa0caloutW<OpaCtlSpec> {
        Opa0caloutW::new(self, 29)
    }
    #[doc = "Bit 30 - OPA1 calibration output"]
    #[inline(always)]
    #[must_use]
    pub fn opa1calout(&mut self) -> Opa1caloutW<OpaCtlSpec> {
        Opa1caloutW::new(self, 30)
    }
    #[doc = "Bit 31 - OPA2 calibration output"]
    #[inline(always)]
    #[must_use]
    pub fn opa2calout(&mut self) -> Opa2caloutW<OpaCtlSpec> {
        Opa2caloutW::new(self, 31)
    }
}
#[doc = "OPA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OpaCtlSpec;
impl crate::RegisterSpec for OpaCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opa_ctl::R`](R) reader structure"]
impl crate::Readable for OpaCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`opa_ctl::W`](W) writer structure"]
impl crate::Writable for OpaCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPA_CTL to value 0x0001_0101"]
impl crate::Resettable for OpaCtlSpec {
    const RESET_VALUE: u32 = 0x0001_0101;
}
