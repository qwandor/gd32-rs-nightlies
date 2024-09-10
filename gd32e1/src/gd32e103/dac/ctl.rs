#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "DAC0 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Den0 {
    #[doc = "0: DAC channel disabled"]
    Disabled = 0,
    #[doc = "1: DAC channel enabled"]
    Enabled = 1,
}
impl From<Den0> for bool {
    #[inline(always)]
    fn from(variant: Den0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEN0` reader - DAC0 enable"]
pub type Den0R = crate::BitReader<Den0>;
impl Den0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Den0 {
        match self.bits {
            false => Den0::Disabled,
            true => Den0::Enabled,
        }
    }
    #[doc = "DAC channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Den0::Disabled
    }
    #[doc = "DAC channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Den0::Enabled
    }
}
#[doc = "Field `DEN0` writer - DAC0 enable"]
pub type Den0W<'a, REG> = crate::BitWriter<'a, REG, Den0>;
impl<'a, REG> Den0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Den0::Disabled)
    }
    #[doc = "DAC channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Den0::Enabled)
    }
}
#[doc = "DAC0 output buffer turn off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dboff0 {
    #[doc = "0: DAC X output buffer enabled"]
    Enabled = 0,
    #[doc = "1: DAC X output buffer disabled"]
    Disabled = 1,
}
impl From<Dboff0> for bool {
    #[inline(always)]
    fn from(variant: Dboff0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBOFF0` reader - DAC0 output buffer turn off"]
pub type Dboff0R = crate::BitReader<Dboff0>;
impl Dboff0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dboff0 {
        match self.bits {
            false => Dboff0::Enabled,
            true => Dboff0::Disabled,
        }
    }
    #[doc = "DAC X output buffer enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dboff0::Enabled
    }
    #[doc = "DAC X output buffer disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dboff0::Disabled
    }
}
#[doc = "Field `DBOFF0` writer - DAC0 output buffer turn off"]
pub type Dboff0W<'a, REG> = crate::BitWriter<'a, REG, Dboff0>;
impl<'a, REG> Dboff0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC X output buffer enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dboff0::Enabled)
    }
    #[doc = "DAC X output buffer disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dboff0::Disabled)
    }
}
#[doc = "DAC0 trigger enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dten0 {
    #[doc = "0: DAC trigger disabled"]
    Disabled = 0,
    #[doc = "1: DAC trigger enabled"]
    Enabled = 1,
}
impl From<Dten0> for bool {
    #[inline(always)]
    fn from(variant: Dten0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTEN0` reader - DAC0 trigger enable"]
pub type Dten0R = crate::BitReader<Dten0>;
impl Dten0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dten0 {
        match self.bits {
            false => Dten0::Disabled,
            true => Dten0::Enabled,
        }
    }
    #[doc = "DAC trigger disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dten0::Disabled
    }
    #[doc = "DAC trigger enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dten0::Enabled
    }
}
#[doc = "Field `DTEN0` writer - DAC0 trigger enable"]
pub type Dten0W<'a, REG> = crate::BitWriter<'a, REG, Dten0>;
impl<'a, REG> Dten0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC trigger disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dten0::Disabled)
    }
    #[doc = "DAC trigger enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dten0::Enabled)
    }
}
#[doc = "DAC0 trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dtsel0 {
    #[doc = "0: Timer 5 TRGO event"]
    Timer5Trgo = 0,
    #[doc = "1: Timer 2 TRGO event"]
    Timer2Trgo = 1,
    #[doc = "3: Timer 14 TRGO event"]
    Timer14Trgo = 3,
    #[doc = "4: Timer 1 TRGO event"]
    Timer1Trgo = 4,
    #[doc = "6: External line9"]
    External9 = 6,
    #[doc = "7: Software trigger"]
    Software = 7,
}
impl From<Dtsel0> for u8 {
    #[inline(always)]
    fn from(variant: Dtsel0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dtsel0 {
    type Ux = u8;
}
#[doc = "Field `DTSEL0` reader - DAC0 trigger selection"]
pub type Dtsel0R = crate::FieldReader<Dtsel0>;
impl Dtsel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dtsel0> {
        match self.bits {
            0 => Some(Dtsel0::Timer5Trgo),
            1 => Some(Dtsel0::Timer2Trgo),
            3 => Some(Dtsel0::Timer14Trgo),
            4 => Some(Dtsel0::Timer1Trgo),
            6 => Some(Dtsel0::External9),
            7 => Some(Dtsel0::Software),
            _ => None,
        }
    }
    #[doc = "Timer 5 TRGO event"]
    #[inline(always)]
    pub fn is_timer5_trgo(&self) -> bool {
        *self == Dtsel0::Timer5Trgo
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn is_timer2_trgo(&self) -> bool {
        *self == Dtsel0::Timer2Trgo
    }
    #[doc = "Timer 14 TRGO event"]
    #[inline(always)]
    pub fn is_timer14_trgo(&self) -> bool {
        *self == Dtsel0::Timer14Trgo
    }
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn is_timer1_trgo(&self) -> bool {
        *self == Dtsel0::Timer1Trgo
    }
    #[doc = "External line9"]
    #[inline(always)]
    pub fn is_external9(&self) -> bool {
        *self == Dtsel0::External9
    }
    #[doc = "Software trigger"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == Dtsel0::Software
    }
}
#[doc = "Field `DTSEL0` writer - DAC0 trigger selection"]
pub type Dtsel0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Dtsel0>;
impl<'a, REG> Dtsel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer 5 TRGO event"]
    #[inline(always)]
    pub fn timer5_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(Dtsel0::Timer5Trgo)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn timer2_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(Dtsel0::Timer2Trgo)
    }
    #[doc = "Timer 14 TRGO event"]
    #[inline(always)]
    pub fn timer14_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(Dtsel0::Timer14Trgo)
    }
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn timer1_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(Dtsel0::Timer1Trgo)
    }
    #[doc = "External line9"]
    #[inline(always)]
    pub fn external9(self) -> &'a mut crate::W<REG> {
        self.variant(Dtsel0::External9)
    }
    #[doc = "Software trigger"]
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(Dtsel0::Software)
    }
}
#[doc = "DAC0 noise wave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dwm0 {
    #[doc = "0: Wave disabled"]
    WaveDisabled = 0,
    #[doc = "1: LFSR noise mode"]
    Lfsr = 1,
    #[doc = "2: Triangle noise mode"]
    Triangle = 2,
}
impl From<Dwm0> for u8 {
    #[inline(always)]
    fn from(variant: Dwm0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dwm0 {
    type Ux = u8;
}
#[doc = "Field `DWM0` reader - DAC0 noise wave mode"]
pub type Dwm0R = crate::FieldReader<Dwm0>;
impl Dwm0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dwm0> {
        match self.bits {
            0 => Some(Dwm0::WaveDisabled),
            1 => Some(Dwm0::Lfsr),
            2 => Some(Dwm0::Triangle),
            _ => None,
        }
    }
    #[doc = "Wave disabled"]
    #[inline(always)]
    pub fn is_wave_disabled(&self) -> bool {
        *self == Dwm0::WaveDisabled
    }
    #[doc = "LFSR noise mode"]
    #[inline(always)]
    pub fn is_lfsr(&self) -> bool {
        *self == Dwm0::Lfsr
    }
    #[doc = "Triangle noise mode"]
    #[inline(always)]
    pub fn is_triangle(&self) -> bool {
        *self == Dwm0::Triangle
    }
}
#[doc = "Field `DWM0` writer - DAC0 noise wave mode"]
pub type Dwm0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dwm0>;
impl<'a, REG> Dwm0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Wave disabled"]
    #[inline(always)]
    pub fn wave_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dwm0::WaveDisabled)
    }
    #[doc = "LFSR noise mode"]
    #[inline(always)]
    pub fn lfsr(self) -> &'a mut crate::W<REG> {
        self.variant(Dwm0::Lfsr)
    }
    #[doc = "Triangle noise mode"]
    #[inline(always)]
    pub fn triangle(self) -> &'a mut crate::W<REG> {
        self.variant(Dwm0::Triangle)
    }
}
#[doc = "DAC0 noise wave bit width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dwbw0 {
    #[doc = "0: The bit width of the wave signal is 1"]
    BitWidth1 = 0,
    #[doc = "1: The bit width of the wave signal is 2"]
    BitWidth2 = 1,
    #[doc = "2: The bit width of the wave signal is 3"]
    BitWidth3 = 2,
    #[doc = "3: The bit width of the wave signal is 4"]
    BitWidth4 = 3,
    #[doc = "4: The bit width of the wave signal is 5"]
    BitWidth5 = 4,
    #[doc = "5: The bit width of the wave signal is 6"]
    BitWidth6 = 5,
    #[doc = "6: The bit width of the wave signal is 7"]
    BitWidth7 = 6,
    #[doc = "7: The bit width of the wave signal is 8"]
    BitWidth8 = 7,
    #[doc = "8: The bit width of the wave signal is 9"]
    BitWidth9 = 8,
    #[doc = "9: The bit width of the wave signal is 10"]
    BitWidth10 = 9,
    #[doc = "10: The bit width of the wave signal is 11"]
    BitWidth11 = 10,
    #[doc = "11: The bit width of the wave signal is 12"]
    BitWidth12 = 11,
}
impl From<Dwbw0> for u8 {
    #[inline(always)]
    fn from(variant: Dwbw0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dwbw0 {
    type Ux = u8;
}
#[doc = "Field `DWBW0` reader - DAC0 noise wave bit width"]
pub type Dwbw0R = crate::FieldReader<Dwbw0>;
impl Dwbw0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dwbw0> {
        match self.bits {
            0 => Some(Dwbw0::BitWidth1),
            1 => Some(Dwbw0::BitWidth2),
            2 => Some(Dwbw0::BitWidth3),
            3 => Some(Dwbw0::BitWidth4),
            4 => Some(Dwbw0::BitWidth5),
            5 => Some(Dwbw0::BitWidth6),
            6 => Some(Dwbw0::BitWidth7),
            7 => Some(Dwbw0::BitWidth8),
            8 => Some(Dwbw0::BitWidth9),
            9 => Some(Dwbw0::BitWidth10),
            10 => Some(Dwbw0::BitWidth11),
            11 => Some(Dwbw0::BitWidth12),
            _ => None,
        }
    }
    #[doc = "The bit width of the wave signal is 1"]
    #[inline(always)]
    pub fn is_bit_width1(&self) -> bool {
        *self == Dwbw0::BitWidth1
    }
    #[doc = "The bit width of the wave signal is 2"]
    #[inline(always)]
    pub fn is_bit_width2(&self) -> bool {
        *self == Dwbw0::BitWidth2
    }
    #[doc = "The bit width of the wave signal is 3"]
    #[inline(always)]
    pub fn is_bit_width3(&self) -> bool {
        *self == Dwbw0::BitWidth3
    }
    #[doc = "The bit width of the wave signal is 4"]
    #[inline(always)]
    pub fn is_bit_width4(&self) -> bool {
        *self == Dwbw0::BitWidth4
    }
    #[doc = "The bit width of the wave signal is 5"]
    #[inline(always)]
    pub fn is_bit_width5(&self) -> bool {
        *self == Dwbw0::BitWidth5
    }
    #[doc = "The bit width of the wave signal is 6"]
    #[inline(always)]
    pub fn is_bit_width6(&self) -> bool {
        *self == Dwbw0::BitWidth6
    }
    #[doc = "The bit width of the wave signal is 7"]
    #[inline(always)]
    pub fn is_bit_width7(&self) -> bool {
        *self == Dwbw0::BitWidth7
    }
    #[doc = "The bit width of the wave signal is 8"]
    #[inline(always)]
    pub fn is_bit_width8(&self) -> bool {
        *self == Dwbw0::BitWidth8
    }
    #[doc = "The bit width of the wave signal is 9"]
    #[inline(always)]
    pub fn is_bit_width9(&self) -> bool {
        *self == Dwbw0::BitWidth9
    }
    #[doc = "The bit width of the wave signal is 10"]
    #[inline(always)]
    pub fn is_bit_width10(&self) -> bool {
        *self == Dwbw0::BitWidth10
    }
    #[doc = "The bit width of the wave signal is 11"]
    #[inline(always)]
    pub fn is_bit_width11(&self) -> bool {
        *self == Dwbw0::BitWidth11
    }
    #[doc = "The bit width of the wave signal is 12"]
    #[inline(always)]
    pub fn is_bit_width12(&self) -> bool {
        *self == Dwbw0::BitWidth12
    }
}
#[doc = "Field `DWBW0` writer - DAC0 noise wave bit width"]
pub type Dwbw0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Dwbw0>;
impl<'a, REG> Dwbw0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The bit width of the wave signal is 1"]
    #[inline(always)]
    pub fn bit_width1(self) -> &'a mut crate::W<REG> {
        self.variant(Dwbw0::BitWidth1)
    }
    #[doc = "The bit width of the wave signal is 2"]
    #[inline(always)]
    pub fn bit_width2(self) -> &'a mut crate::W<REG> {
        self.variant(Dwbw0::BitWidth2)
    }
    #[doc = "The bit width of the wave signal is 3"]
    #[inline(always)]
    pub fn bit_width3(self) -> &'a mut crate::W<REG> {
        self.variant(Dwbw0::BitWidth3)
    }
    #[doc = "The bit width of the wave signal is 4"]
    #[inline(always)]
    pub fn bit_width4(self) -> &'a mut crate::W<REG> {
        self.variant(Dwbw0::BitWidth4)
    }
    #[doc = "The bit width of the wave signal is 5"]
    #[inline(always)]
    pub fn bit_width5(self) -> &'a mut crate::W<REG> {
        self.variant(Dwbw0::BitWidth5)
    }
    #[doc = "The bit width of the wave signal is 6"]
    #[inline(always)]
    pub fn bit_width6(self) -> &'a mut crate::W<REG> {
        self.variant(Dwbw0::BitWidth6)
    }
    #[doc = "The bit width of the wave signal is 7"]
    #[inline(always)]
    pub fn bit_width7(self) -> &'a mut crate::W<REG> {
        self.variant(Dwbw0::BitWidth7)
    }
    #[doc = "The bit width of the wave signal is 8"]
    #[inline(always)]
    pub fn bit_width8(self) -> &'a mut crate::W<REG> {
        self.variant(Dwbw0::BitWidth8)
    }
    #[doc = "The bit width of the wave signal is 9"]
    #[inline(always)]
    pub fn bit_width9(self) -> &'a mut crate::W<REG> {
        self.variant(Dwbw0::BitWidth9)
    }
    #[doc = "The bit width of the wave signal is 10"]
    #[inline(always)]
    pub fn bit_width10(self) -> &'a mut crate::W<REG> {
        self.variant(Dwbw0::BitWidth10)
    }
    #[doc = "The bit width of the wave signal is 11"]
    #[inline(always)]
    pub fn bit_width11(self) -> &'a mut crate::W<REG> {
        self.variant(Dwbw0::BitWidth11)
    }
    #[doc = "The bit width of the wave signal is 12"]
    #[inline(always)]
    pub fn bit_width12(self) -> &'a mut crate::W<REG> {
        self.variant(Dwbw0::BitWidth12)
    }
}
#[doc = "DAC0 DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ddmaen0 {
    #[doc = "0: DAC DMA mode disabled"]
    Disabled = 0,
    #[doc = "1: DAC DMA mode enabled"]
    Enabled = 1,
}
impl From<Ddmaen0> for bool {
    #[inline(always)]
    fn from(variant: Ddmaen0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDMAEN0` reader - DAC0 DMA enable"]
pub type Ddmaen0R = crate::BitReader<Ddmaen0>;
impl Ddmaen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ddmaen0 {
        match self.bits {
            false => Ddmaen0::Disabled,
            true => Ddmaen0::Enabled,
        }
    }
    #[doc = "DAC DMA mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ddmaen0::Disabled
    }
    #[doc = "DAC DMA mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ddmaen0::Enabled
    }
}
#[doc = "Field `DDMAEN0` writer - DAC0 DMA enable"]
pub type Ddmaen0W<'a, REG> = crate::BitWriter<'a, REG, Ddmaen0>;
impl<'a, REG> Ddmaen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC DMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ddmaen0::Disabled)
    }
    #[doc = "DAC DMA mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ddmaen0::Enabled)
    }
}
#[doc = "Field `DBOFF1` reader - DAC1 output buffer turn off"]
pub use Dboff0R as Dboff1R;
#[doc = "Field `DBOFF1` writer - DAC1 output buffer turn off"]
pub use Dboff0W as Dboff1W;
#[doc = "Field `DDMAEN1` reader - DAC1 DMA enable"]
pub use Ddmaen0R as Ddmaen1R;
#[doc = "Field `DDMAEN1` writer - DAC1 DMA enable"]
pub use Ddmaen0W as Ddmaen1W;
#[doc = "Field `DEN1` reader - DAC1 enable"]
pub use Den0R as Den1R;
#[doc = "Field `DEN1` writer - DAC1 enable"]
pub use Den0W as Den1W;
#[doc = "Field `DTEN1` reader - DAC1 trigger enable"]
pub use Dten0R as Dten1R;
#[doc = "Field `DTEN1` writer - DAC1 trigger enable"]
pub use Dten0W as Dten1W;
#[doc = "Field `DTSEL1` reader - DAC1 trigger selection"]
pub use Dtsel0R as Dtsel1R;
#[doc = "Field `DTSEL1` writer - DAC1 trigger selection"]
pub use Dtsel0W as Dtsel1W;
#[doc = "Field `DWBW1` reader - DAC1 noise wave bit width"]
pub use Dwbw0R as Dwbw1R;
#[doc = "Field `DWBW1` writer - DAC1 noise wave bit width"]
pub use Dwbw0W as Dwbw1W;
#[doc = "Field `DWM1` reader - DAC1 noise wave mode"]
pub use Dwm0R as Dwm1R;
#[doc = "Field `DWM1` writer - DAC1 noise wave mode"]
pub use Dwm0W as Dwm1W;
impl R {
    #[doc = "Bit 0 - DAC0 enable"]
    #[inline(always)]
    pub fn den0(&self) -> Den0R {
        Den0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC0 output buffer turn off"]
    #[inline(always)]
    pub fn dboff0(&self) -> Dboff0R {
        Dboff0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAC0 trigger enable"]
    #[inline(always)]
    pub fn dten0(&self) -> Dten0R {
        Dten0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - DAC0 trigger selection"]
    #[inline(always)]
    pub fn dtsel0(&self) -> Dtsel0R {
        Dtsel0R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - DAC0 noise wave mode"]
    #[inline(always)]
    pub fn dwm0(&self) -> Dwm0R {
        Dwm0R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - DAC0 noise wave bit width"]
    #[inline(always)]
    pub fn dwbw0(&self) -> Dwbw0R {
        Dwbw0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DAC0 DMA enable"]
    #[inline(always)]
    pub fn ddmaen0(&self) -> Ddmaen0R {
        Ddmaen0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - DAC1 enable"]
    #[inline(always)]
    pub fn den1(&self) -> Den1R {
        Den1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DAC1 output buffer turn off"]
    #[inline(always)]
    pub fn dboff1(&self) -> Dboff1R {
        Dboff1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DAC1 trigger enable"]
    #[inline(always)]
    pub fn dten1(&self) -> Dten1R {
        Dten1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - DAC1 trigger selection"]
    #[inline(always)]
    pub fn dtsel1(&self) -> Dtsel1R {
        Dtsel1R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - DAC1 noise wave mode"]
    #[inline(always)]
    pub fn dwm1(&self) -> Dwm1R {
        Dwm1R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - DAC1 noise wave bit width"]
    #[inline(always)]
    pub fn dwbw1(&self) -> Dwbw1R {
        Dwbw1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - DAC1 DMA enable"]
    #[inline(always)]
    pub fn ddmaen1(&self) -> Ddmaen1R {
        Ddmaen1R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn den0(&mut self) -> Den0W<CtlSpec> {
        Den0W::new(self, 0)
    }
    #[doc = "Bit 1 - DAC0 output buffer turn off"]
    #[inline(always)]
    #[must_use]
    pub fn dboff0(&mut self) -> Dboff0W<CtlSpec> {
        Dboff0W::new(self, 1)
    }
    #[doc = "Bit 2 - DAC0 trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn dten0(&mut self) -> Dten0W<CtlSpec> {
        Dten0W::new(self, 2)
    }
    #[doc = "Bits 3:5 - DAC0 trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn dtsel0(&mut self) -> Dtsel0W<CtlSpec> {
        Dtsel0W::new(self, 3)
    }
    #[doc = "Bits 6:7 - DAC0 noise wave mode"]
    #[inline(always)]
    #[must_use]
    pub fn dwm0(&mut self) -> Dwm0W<CtlSpec> {
        Dwm0W::new(self, 6)
    }
    #[doc = "Bits 8:11 - DAC0 noise wave bit width"]
    #[inline(always)]
    #[must_use]
    pub fn dwbw0(&mut self) -> Dwbw0W<CtlSpec> {
        Dwbw0W::new(self, 8)
    }
    #[doc = "Bit 12 - DAC0 DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddmaen0(&mut self) -> Ddmaen0W<CtlSpec> {
        Ddmaen0W::new(self, 12)
    }
    #[doc = "Bit 16 - DAC1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn den1(&mut self) -> Den1W<CtlSpec> {
        Den1W::new(self, 16)
    }
    #[doc = "Bit 17 - DAC1 output buffer turn off"]
    #[inline(always)]
    #[must_use]
    pub fn dboff1(&mut self) -> Dboff1W<CtlSpec> {
        Dboff1W::new(self, 17)
    }
    #[doc = "Bit 18 - DAC1 trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn dten1(&mut self) -> Dten1W<CtlSpec> {
        Dten1W::new(self, 18)
    }
    #[doc = "Bits 19:21 - DAC1 trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn dtsel1(&mut self) -> Dtsel1W<CtlSpec> {
        Dtsel1W::new(self, 19)
    }
    #[doc = "Bits 22:23 - DAC1 noise wave mode"]
    #[inline(always)]
    #[must_use]
    pub fn dwm1(&mut self) -> Dwm1W<CtlSpec> {
        Dwm1W::new(self, 22)
    }
    #[doc = "Bits 24:27 - DAC1 noise wave bit width"]
    #[inline(always)]
    #[must_use]
    pub fn dwbw1(&mut self) -> Dwbw1W<CtlSpec> {
        Dwbw1W::new(self, 24)
    }
    #[doc = "Bit 28 - DAC1 DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddmaen1(&mut self) -> Ddmaen1W<CtlSpec> {
        Ddmaen1W::new(self, 28)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
