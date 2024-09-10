#[doc = "Register `CS` reader"]
pub type R = crate::R<CsSpec>;
#[doc = "Register `CS` writer"]
pub type W = crate::W<CsSpec>;
#[doc = "Comparator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpen {
    #[doc = "0: Comparator disabled"]
    Disabled = 0,
    #[doc = "1: Comparator enabled"]
    Enabled = 1,
}
impl From<Cmpen> for bool {
    #[inline(always)]
    fn from(variant: Cmpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPEN` reader - Comparator enable"]
pub type CmpenR = crate::BitReader<Cmpen>;
impl CmpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpen {
        match self.bits {
            false => Cmpen::Disabled,
            true => Cmpen::Enabled,
        }
    }
    #[doc = "Comparator disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cmpen::Disabled
    }
    #[doc = "Comparator enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cmpen::Enabled
    }
}
#[doc = "Field `CMPEN` writer - Comparator enable"]
pub type CmpenW<'a, REG> = crate::BitWriter<'a, REG, Cmpen>;
impl<'a, REG> CmpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpen::Disabled)
    }
    #[doc = "Comparator enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpen::Enabled)
    }
}
#[doc = "Comparator switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpsw {
    #[doc = "0: Switch open"]
    Open = 0,
    #[doc = "1: Switch closed"]
    Closed = 1,
}
impl From<Cmpsw> for bool {
    #[inline(always)]
    fn from(variant: Cmpsw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSW` reader - Comparator switch"]
pub type CmpswR = crate::BitReader<Cmpsw>;
impl CmpswR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpsw {
        match self.bits {
            false => Cmpsw::Open,
            true => Cmpsw::Closed,
        }
    }
    #[doc = "Switch open"]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == Cmpsw::Open
    }
    #[doc = "Switch closed"]
    #[inline(always)]
    pub fn is_closed(&self) -> bool {
        *self == Cmpsw::Closed
    }
}
#[doc = "Field `CMPSW` writer - Comparator switch"]
pub type CmpswW<'a, REG> = crate::BitWriter<'a, REG, Cmpsw>;
impl<'a, REG> CmpswW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Switch open"]
    #[inline(always)]
    pub fn open(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpsw::Open)
    }
    #[doc = "Switch closed"]
    #[inline(always)]
    pub fn closed(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpsw::Closed)
    }
}
#[doc = "Comparator mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmpm {
    #[doc = "0: High speed / full power"]
    HighSpeed = 0,
    #[doc = "1: Medium speed / medium power"]
    MediumSpeed = 1,
    #[doc = "2: Low speed / low power"]
    LowSpeed = 2,
    #[doc = "3: Very-low speed / ultra-low power"]
    VeryLowSpeed = 3,
}
impl From<Cmpm> for u8 {
    #[inline(always)]
    fn from(variant: Cmpm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmpm {
    type Ux = u8;
}
#[doc = "Field `CMPM` reader - Comparator mode"]
pub type CmpmR = crate::FieldReader<Cmpm>;
impl CmpmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpm {
        match self.bits {
            0 => Cmpm::HighSpeed,
            1 => Cmpm::MediumSpeed,
            2 => Cmpm::LowSpeed,
            3 => Cmpm::VeryLowSpeed,
            _ => unreachable!(),
        }
    }
    #[doc = "High speed / full power"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == Cmpm::HighSpeed
    }
    #[doc = "Medium speed / medium power"]
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == Cmpm::MediumSpeed
    }
    #[doc = "Low speed / low power"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == Cmpm::LowSpeed
    }
    #[doc = "Very-low speed / ultra-low power"]
    #[inline(always)]
    pub fn is_very_low_speed(&self) -> bool {
        *self == Cmpm::VeryLowSpeed
    }
}
#[doc = "Field `CMPM` writer - Comparator mode"]
pub type CmpmW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Cmpm>;
impl<'a, REG> CmpmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High speed / full power"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpm::HighSpeed)
    }
    #[doc = "Medium speed / medium power"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpm::MediumSpeed)
    }
    #[doc = "Low speed / low power"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpm::LowSpeed)
    }
    #[doc = "Very-low speed / ultra-low power"]
    #[inline(always)]
    pub fn very_low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpm::VeryLowSpeed)
    }
}
#[doc = "Comparator input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmpmsel {
    #[doc = "0: 1/4 of VRefint"]
    OneQuarterVref = 0,
    #[doc = "1: 1/2 of VRefint"]
    OneHalfVref = 1,
    #[doc = "2: 3/4 of VRefint"]
    ThreeQuarterVref = 2,
    #[doc = "3: VRefint"]
    Vref = 3,
    #[doc = "4: PA4 (DAC0"]
    Pa4 = 4,
    #[doc = "5: PA5"]
    Pa5 = 5,
    #[doc = "6: PA0"]
    Pa0 = 6,
}
impl From<Cmpmsel> for u8 {
    #[inline(always)]
    fn from(variant: Cmpmsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmpmsel {
    type Ux = u8;
}
#[doc = "Field `CMPMSEL` reader - Comparator input selection"]
pub type CmpmselR = crate::FieldReader<Cmpmsel>;
impl CmpmselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmpmsel> {
        match self.bits {
            0 => Some(Cmpmsel::OneQuarterVref),
            1 => Some(Cmpmsel::OneHalfVref),
            2 => Some(Cmpmsel::ThreeQuarterVref),
            3 => Some(Cmpmsel::Vref),
            4 => Some(Cmpmsel::Pa4),
            5 => Some(Cmpmsel::Pa5),
            6 => Some(Cmpmsel::Pa0),
            _ => None,
        }
    }
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == Cmpmsel::OneQuarterVref
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == Cmpmsel::OneHalfVref
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == Cmpmsel::ThreeQuarterVref
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == Cmpmsel::Vref
    }
    #[doc = "PA4 (DAC0"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        *self == Cmpmsel::Pa4
    }
    #[doc = "PA5"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == Cmpmsel::Pa5
    }
    #[doc = "PA0"]
    #[inline(always)]
    pub fn is_pa0(&self) -> bool {
        *self == Cmpmsel::Pa0
    }
}
#[doc = "Field `CMPMSEL` writer - Comparator input selection"]
pub type CmpmselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cmpmsel>;
impl<'a, REG> CmpmselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmsel::OneQuarterVref)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmsel::OneHalfVref)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmsel::ThreeQuarterVref)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmsel::Vref)
    }
    #[doc = "PA4 (DAC0"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmsel::Pa4)
    }
    #[doc = "PA5"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmsel::Pa5)
    }
    #[doc = "PA0"]
    #[inline(always)]
    pub fn pa0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmsel::Pa0)
    }
}
#[doc = "Comparator output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmposel {
    #[doc = "0: No selection"]
    NoSelection = 0,
    #[doc = "1: Timer 0 break input"]
    Timer0breakInput = 1,
    #[doc = "2: Timer 0 Input capture 0"]
    Timer0inputCapture0 = 2,
    #[doc = "3: Timer 0 OCPRE_CLR input"]
    Timer0ocpreclearInput = 3,
    #[doc = "4: Timer 1 input capture 3"]
    Timer1inputCapture3 = 4,
    #[doc = "5: Timer 1 OCPRE_CLR input"]
    Timer1ocpreclearInput = 5,
    #[doc = "6: Timer 2 input capture 0"]
    Timer2inputCapture0 = 6,
    #[doc = "7: Timer 2 OCPRE_CLR input"]
    Timer2ocpreclearInput = 7,
}
impl From<Cmposel> for u8 {
    #[inline(always)]
    fn from(variant: Cmposel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmposel {
    type Ux = u8;
}
#[doc = "Field `CMPOSEL` reader - Comparator output selection"]
pub type CmposelR = crate::FieldReader<Cmposel>;
impl CmposelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmposel {
        match self.bits {
            0 => Cmposel::NoSelection,
            1 => Cmposel::Timer0breakInput,
            2 => Cmposel::Timer0inputCapture0,
            3 => Cmposel::Timer0ocpreclearInput,
            4 => Cmposel::Timer1inputCapture3,
            5 => Cmposel::Timer1ocpreclearInput,
            6 => Cmposel::Timer2inputCapture0,
            7 => Cmposel::Timer2ocpreclearInput,
            _ => unreachable!(),
        }
    }
    #[doc = "No selection"]
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        *self == Cmposel::NoSelection
    }
    #[doc = "Timer 0 break input"]
    #[inline(always)]
    pub fn is_timer0break_input(&self) -> bool {
        *self == Cmposel::Timer0breakInput
    }
    #[doc = "Timer 0 Input capture 0"]
    #[inline(always)]
    pub fn is_timer0input_capture0(&self) -> bool {
        *self == Cmposel::Timer0inputCapture0
    }
    #[doc = "Timer 0 OCPRE_CLR input"]
    #[inline(always)]
    pub fn is_timer0ocpreclear_input(&self) -> bool {
        *self == Cmposel::Timer0ocpreclearInput
    }
    #[doc = "Timer 1 input capture 3"]
    #[inline(always)]
    pub fn is_timer1input_capture3(&self) -> bool {
        *self == Cmposel::Timer1inputCapture3
    }
    #[doc = "Timer 1 OCPRE_CLR input"]
    #[inline(always)]
    pub fn is_timer1ocpreclear_input(&self) -> bool {
        *self == Cmposel::Timer1ocpreclearInput
    }
    #[doc = "Timer 2 input capture 0"]
    #[inline(always)]
    pub fn is_timer2input_capture0(&self) -> bool {
        *self == Cmposel::Timer2inputCapture0
    }
    #[doc = "Timer 2 OCPRE_CLR input"]
    #[inline(always)]
    pub fn is_timer2ocpreclear_input(&self) -> bool {
        *self == Cmposel::Timer2ocpreclearInput
    }
}
#[doc = "Field `CMPOSEL` writer - Comparator output selection"]
pub type CmposelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Cmposel>;
impl<'a, REG> CmposelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No selection"]
    #[inline(always)]
    pub fn no_selection(self) -> &'a mut crate::W<REG> {
        self.variant(Cmposel::NoSelection)
    }
    #[doc = "Timer 0 break input"]
    #[inline(always)]
    pub fn timer0break_input(self) -> &'a mut crate::W<REG> {
        self.variant(Cmposel::Timer0breakInput)
    }
    #[doc = "Timer 0 Input capture 0"]
    #[inline(always)]
    pub fn timer0input_capture0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmposel::Timer0inputCapture0)
    }
    #[doc = "Timer 0 OCPRE_CLR input"]
    #[inline(always)]
    pub fn timer0ocpreclear_input(self) -> &'a mut crate::W<REG> {
        self.variant(Cmposel::Timer0ocpreclearInput)
    }
    #[doc = "Timer 1 input capture 3"]
    #[inline(always)]
    pub fn timer1input_capture3(self) -> &'a mut crate::W<REG> {
        self.variant(Cmposel::Timer1inputCapture3)
    }
    #[doc = "Timer 1 OCPRE_CLR input"]
    #[inline(always)]
    pub fn timer1ocpreclear_input(self) -> &'a mut crate::W<REG> {
        self.variant(Cmposel::Timer1ocpreclearInput)
    }
    #[doc = "Timer 2 input capture 0"]
    #[inline(always)]
    pub fn timer2input_capture0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmposel::Timer2inputCapture0)
    }
    #[doc = "Timer 2 OCPRE_CLR input"]
    #[inline(always)]
    pub fn timer2ocpreclear_input(self) -> &'a mut crate::W<REG> {
        self.variant(Cmposel::Timer2ocpreclearInput)
    }
}
#[doc = "Polarity of comparator output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmppl {
    #[doc = "0: Output is not inverted"]
    NotInverted = 0,
    #[doc = "1: Output is inverted"]
    Inverted = 1,
}
impl From<Cmppl> for bool {
    #[inline(always)]
    fn from(variant: Cmppl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPPL` reader - Polarity of comparator output"]
pub type CmpplR = crate::BitReader<Cmppl>;
impl CmpplR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmppl {
        match self.bits {
            false => Cmppl::NotInverted,
            true => Cmppl::Inverted,
        }
    }
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == Cmppl::NotInverted
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Cmppl::Inverted
    }
}
#[doc = "Field `CMPPL` writer - Polarity of comparator output"]
pub type CmpplW<'a, REG> = crate::BitWriter<'a, REG, Cmppl>;
impl<'a, REG> CmpplW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Cmppl::NotInverted)
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Cmppl::Inverted)
    }
}
#[doc = "Comparator hysteresis\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmphst {
    #[doc = "0: No hysteresis"]
    NoHysteresis = 0,
    #[doc = "1: Low hysteresis"]
    LowHysteresis = 1,
    #[doc = "2: Medium hysteresis"]
    MediumHysteresis = 2,
    #[doc = "3: High hysteresis"]
    HighHysteresis = 3,
}
impl From<Cmphst> for u8 {
    #[inline(always)]
    fn from(variant: Cmphst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmphst {
    type Ux = u8;
}
#[doc = "Field `CMPHST` reader - Comparator hysteresis"]
pub type CmphstR = crate::FieldReader<Cmphst>;
impl CmphstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmphst {
        match self.bits {
            0 => Cmphst::NoHysteresis,
            1 => Cmphst::LowHysteresis,
            2 => Cmphst::MediumHysteresis,
            3 => Cmphst::HighHysteresis,
            _ => unreachable!(),
        }
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn is_no_hysteresis(&self) -> bool {
        *self == Cmphst::NoHysteresis
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn is_low_hysteresis(&self) -> bool {
        *self == Cmphst::LowHysteresis
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn is_medium_hysteresis(&self) -> bool {
        *self == Cmphst::MediumHysteresis
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn is_high_hysteresis(&self) -> bool {
        *self == Cmphst::HighHysteresis
    }
}
#[doc = "Field `CMPHST` writer - Comparator hysteresis"]
pub type CmphstW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Cmphst>;
impl<'a, REG> CmphstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn no_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(Cmphst::NoHysteresis)
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn low_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(Cmphst::LowHysteresis)
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn medium_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(Cmphst::MediumHysteresis)
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn high_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(Cmphst::HighHysteresis)
    }
}
#[doc = "Comparator 0 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpo {
    #[doc = "0: Non-inverting input below inverting input"]
    Low = 0,
    #[doc = "1: Non-inverting input above inverting input"]
    High = 1,
}
impl From<Cmpo> for bool {
    #[inline(always)]
    fn from(variant: Cmpo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPO` reader - Comparator 0 output"]
pub type CmpoR = crate::BitReader<Cmpo>;
impl CmpoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpo {
        match self.bits {
            false => Cmpo::Low,
            true => Cmpo::High,
        }
    }
    #[doc = "Non-inverting input below inverting input"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Cmpo::Low
    }
    #[doc = "Non-inverting input above inverting input"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Cmpo::High
    }
}
#[doc = "Comparator 0 lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplk {
    #[doc = "0: Control bits are read-write"]
    ReadWrite = 0,
    #[doc = "1: Control bits are read-only"]
    ReadOnly = 1,
}
impl From<Cmplk> for bool {
    #[inline(always)]
    fn from(variant: Cmplk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLK` reader - Comparator 0 lock"]
pub type CmplkR = crate::BitReader<Cmplk>;
impl CmplkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplk {
        match self.bits {
            false => Cmplk::ReadWrite,
            true => Cmplk::ReadOnly,
        }
    }
    #[doc = "Control bits are read-write"]
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        *self == Cmplk::ReadWrite
    }
    #[doc = "Control bits are read-only"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == Cmplk::ReadOnly
    }
}
#[doc = "Field `CMPLK` writer - Comparator 0 lock"]
pub type CmplkW<'a, REG> = crate::BitWriter<'a, REG, Cmplk>;
impl<'a, REG> CmplkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Control bits are read-write"]
    #[inline(always)]
    pub fn read_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplk::ReadWrite)
    }
    #[doc = "Control bits are read-only"]
    #[inline(always)]
    pub fn read_only(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplk::ReadOnly)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator enable"]
    #[inline(always)]
    pub fn cmpen(&self) -> CmpenR {
        CmpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator switch"]
    #[inline(always)]
    pub fn cmpsw(&self) -> CmpswR {
        CmpswR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator mode"]
    #[inline(always)]
    pub fn cmpm(&self) -> CmpmR {
        CmpmR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator input selection"]
    #[inline(always)]
    pub fn cmpmsel(&self) -> CmpmselR {
        CmpmselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Comparator output selection"]
    #[inline(always)]
    pub fn cmposel(&self) -> CmposelR {
        CmposelR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Polarity of comparator output"]
    #[inline(always)]
    pub fn cmppl(&self) -> CmpplR {
        CmpplR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Comparator hysteresis"]
    #[inline(always)]
    pub fn cmphst(&self) -> CmphstR {
        CmphstR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Comparator 0 output"]
    #[inline(always)]
    pub fn cmpo(&self) -> CmpoR {
        CmpoR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator 0 lock"]
    #[inline(always)]
    pub fn cmplk(&self) -> CmplkR {
        CmplkR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpen(&mut self) -> CmpenW<CsSpec> {
        CmpenW::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator switch"]
    #[inline(always)]
    #[must_use]
    pub fn cmpsw(&mut self) -> CmpswW<CsSpec> {
        CmpswW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Comparator mode"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm(&mut self) -> CmpmW<CsSpec> {
        CmpmW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator input selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmsel(&mut self) -> CmpmselW<CsSpec> {
        CmpmselW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Comparator output selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmposel(&mut self) -> CmposelW<CsSpec> {
        CmposelW::new(self, 8)
    }
    #[doc = "Bit 11 - Polarity of comparator output"]
    #[inline(always)]
    #[must_use]
    pub fn cmppl(&mut self) -> CmpplW<CsSpec> {
        CmpplW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Comparator hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn cmphst(&mut self) -> CmphstW<CsSpec> {
        CmphstW::new(self, 12)
    }
    #[doc = "Bit 15 - Comparator 0 lock"]
    #[inline(always)]
    #[must_use]
    pub fn cmplk(&mut self) -> CmplkW<CsSpec> {
        CmplkW::new(self, 15)
    }
}
#[doc = "control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
