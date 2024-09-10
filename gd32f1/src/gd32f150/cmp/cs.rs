#[doc = "Register `CS` reader"]
pub type R = crate::R<CsSpec>;
#[doc = "Register `CS` writer"]
pub type W = crate::W<CsSpec>;
#[doc = "CMP0 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmp0en {
    #[doc = "0: Comparator disabled"]
    Disabled = 0,
    #[doc = "1: Comparator enabled"]
    Enabled = 1,
}
impl From<Cmp0en> for bool {
    #[inline(always)]
    fn from(variant: Cmp0en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP0EN` reader - CMP0 enable"]
pub type Cmp0enR = crate::BitReader<Cmp0en>;
impl Cmp0enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmp0en {
        match self.bits {
            false => Cmp0en::Disabled,
            true => Cmp0en::Enabled,
        }
    }
    #[doc = "Comparator disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cmp0en::Disabled
    }
    #[doc = "Comparator enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cmp0en::Enabled
    }
}
#[doc = "Field `CMP0EN` writer - CMP0 enable"]
pub type Cmp0enW<'a, REG> = crate::BitWriter<'a, REG, Cmp0en>;
impl<'a, REG> Cmp0enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0en::Disabled)
    }
    #[doc = "Comparator enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0en::Enabled)
    }
}
#[doc = "CMP0 switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmp0sw {
    #[doc = "0: Switch open"]
    Open = 0,
    #[doc = "1: Switch closed"]
    Closed = 1,
}
impl From<Cmp0sw> for bool {
    #[inline(always)]
    fn from(variant: Cmp0sw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP0SW` reader - CMP0 switch"]
pub type Cmp0swR = crate::BitReader<Cmp0sw>;
impl Cmp0swR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmp0sw {
        match self.bits {
            false => Cmp0sw::Open,
            true => Cmp0sw::Closed,
        }
    }
    #[doc = "Switch open"]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == Cmp0sw::Open
    }
    #[doc = "Switch closed"]
    #[inline(always)]
    pub fn is_closed(&self) -> bool {
        *self == Cmp0sw::Closed
    }
}
#[doc = "Field `CMP0SW` writer - CMP0 switch"]
pub type Cmp0swW<'a, REG> = crate::BitWriter<'a, REG, Cmp0sw>;
impl<'a, REG> Cmp0swW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Switch open"]
    #[inline(always)]
    pub fn open(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0sw::Open)
    }
    #[doc = "Switch closed"]
    #[inline(always)]
    pub fn closed(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0sw::Closed)
    }
}
#[doc = "CMP0 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmp0m {
    #[doc = "0: High speed / full power"]
    HighSpeed = 0,
    #[doc = "1: Medium speed / medium power"]
    MediumSpeed = 1,
    #[doc = "2: Low speed / low power"]
    LowSpeed = 2,
    #[doc = "3: Very-low speed / ultra-low power"]
    VeryLowSpeed = 3,
}
impl From<Cmp0m> for u8 {
    #[inline(always)]
    fn from(variant: Cmp0m) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmp0m {
    type Ux = u8;
}
#[doc = "Field `CMP0M` reader - CMP0 mode"]
pub type Cmp0mR = crate::FieldReader<Cmp0m>;
impl Cmp0mR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmp0m {
        match self.bits {
            0 => Cmp0m::HighSpeed,
            1 => Cmp0m::MediumSpeed,
            2 => Cmp0m::LowSpeed,
            3 => Cmp0m::VeryLowSpeed,
            _ => unreachable!(),
        }
    }
    #[doc = "High speed / full power"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == Cmp0m::HighSpeed
    }
    #[doc = "Medium speed / medium power"]
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == Cmp0m::MediumSpeed
    }
    #[doc = "Low speed / low power"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == Cmp0m::LowSpeed
    }
    #[doc = "Very-low speed / ultra-low power"]
    #[inline(always)]
    pub fn is_very_low_speed(&self) -> bool {
        *self == Cmp0m::VeryLowSpeed
    }
}
#[doc = "Field `CMP0M` writer - CMP0 mode"]
pub type Cmp0mW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Cmp0m>;
impl<'a, REG> Cmp0mW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High speed / full power"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0m::HighSpeed)
    }
    #[doc = "Medium speed / medium power"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0m::MediumSpeed)
    }
    #[doc = "Low speed / low power"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0m::LowSpeed)
    }
    #[doc = "Very-low speed / ultra-low power"]
    #[inline(always)]
    pub fn very_low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0m::VeryLowSpeed)
    }
}
#[doc = "CMP0_M input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmp0msel {
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
impl From<Cmp0msel> for u8 {
    #[inline(always)]
    fn from(variant: Cmp0msel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmp0msel {
    type Ux = u8;
}
#[doc = "Field `CMP0MSEL` reader - CMP0_M input selection"]
pub type Cmp0mselR = crate::FieldReader<Cmp0msel>;
impl Cmp0mselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmp0msel> {
        match self.bits {
            0 => Some(Cmp0msel::OneQuarterVref),
            1 => Some(Cmp0msel::OneHalfVref),
            2 => Some(Cmp0msel::ThreeQuarterVref),
            3 => Some(Cmp0msel::Vref),
            4 => Some(Cmp0msel::Pa4),
            5 => Some(Cmp0msel::Pa5),
            6 => Some(Cmp0msel::Pa0),
            _ => None,
        }
    }
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == Cmp0msel::OneQuarterVref
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == Cmp0msel::OneHalfVref
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == Cmp0msel::ThreeQuarterVref
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == Cmp0msel::Vref
    }
    #[doc = "PA4 (DAC0"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        *self == Cmp0msel::Pa4
    }
    #[doc = "PA5"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == Cmp0msel::Pa5
    }
    #[doc = "PA0"]
    #[inline(always)]
    pub fn is_pa0(&self) -> bool {
        *self == Cmp0msel::Pa0
    }
}
#[doc = "Field `CMP0MSEL` writer - CMP0_M input selection"]
pub type Cmp0mselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cmp0msel>;
impl<'a, REG> Cmp0mselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0msel::OneQuarterVref)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0msel::OneHalfVref)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0msel::ThreeQuarterVref)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0msel::Vref)
    }
    #[doc = "PA4 (DAC0"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0msel::Pa4)
    }
    #[doc = "PA5"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0msel::Pa5)
    }
    #[doc = "PA0"]
    #[inline(always)]
    pub fn pa0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0msel::Pa0)
    }
}
#[doc = "Comparator 0 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmp0osel {
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
impl From<Cmp0osel> for u8 {
    #[inline(always)]
    fn from(variant: Cmp0osel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmp0osel {
    type Ux = u8;
}
#[doc = "Field `CMP0OSEL` reader - Comparator 0 output selection"]
pub type Cmp0oselR = crate::FieldReader<Cmp0osel>;
impl Cmp0oselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmp0osel {
        match self.bits {
            0 => Cmp0osel::NoSelection,
            1 => Cmp0osel::Timer0breakInput,
            2 => Cmp0osel::Timer0inputCapture0,
            3 => Cmp0osel::Timer0ocpreclearInput,
            4 => Cmp0osel::Timer1inputCapture3,
            5 => Cmp0osel::Timer1ocpreclearInput,
            6 => Cmp0osel::Timer2inputCapture0,
            7 => Cmp0osel::Timer2ocpreclearInput,
            _ => unreachable!(),
        }
    }
    #[doc = "No selection"]
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        *self == Cmp0osel::NoSelection
    }
    #[doc = "Timer 0 break input"]
    #[inline(always)]
    pub fn is_timer0break_input(&self) -> bool {
        *self == Cmp0osel::Timer0breakInput
    }
    #[doc = "Timer 0 Input capture 0"]
    #[inline(always)]
    pub fn is_timer0input_capture0(&self) -> bool {
        *self == Cmp0osel::Timer0inputCapture0
    }
    #[doc = "Timer 0 OCPRE_CLR input"]
    #[inline(always)]
    pub fn is_timer0ocpreclear_input(&self) -> bool {
        *self == Cmp0osel::Timer0ocpreclearInput
    }
    #[doc = "Timer 1 input capture 3"]
    #[inline(always)]
    pub fn is_timer1input_capture3(&self) -> bool {
        *self == Cmp0osel::Timer1inputCapture3
    }
    #[doc = "Timer 1 OCPRE_CLR input"]
    #[inline(always)]
    pub fn is_timer1ocpreclear_input(&self) -> bool {
        *self == Cmp0osel::Timer1ocpreclearInput
    }
    #[doc = "Timer 2 input capture 0"]
    #[inline(always)]
    pub fn is_timer2input_capture0(&self) -> bool {
        *self == Cmp0osel::Timer2inputCapture0
    }
    #[doc = "Timer 2 OCPRE_CLR input"]
    #[inline(always)]
    pub fn is_timer2ocpreclear_input(&self) -> bool {
        *self == Cmp0osel::Timer2ocpreclearInput
    }
}
#[doc = "Field `CMP0OSEL` writer - Comparator 0 output selection"]
pub type Cmp0oselW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Cmp0osel>;
impl<'a, REG> Cmp0oselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No selection"]
    #[inline(always)]
    pub fn no_selection(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0osel::NoSelection)
    }
    #[doc = "Timer 0 break input"]
    #[inline(always)]
    pub fn timer0break_input(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0osel::Timer0breakInput)
    }
    #[doc = "Timer 0 Input capture 0"]
    #[inline(always)]
    pub fn timer0input_capture0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0osel::Timer0inputCapture0)
    }
    #[doc = "Timer 0 OCPRE_CLR input"]
    #[inline(always)]
    pub fn timer0ocpreclear_input(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0osel::Timer0ocpreclearInput)
    }
    #[doc = "Timer 1 input capture 3"]
    #[inline(always)]
    pub fn timer1input_capture3(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0osel::Timer1inputCapture3)
    }
    #[doc = "Timer 1 OCPRE_CLR input"]
    #[inline(always)]
    pub fn timer1ocpreclear_input(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0osel::Timer1ocpreclearInput)
    }
    #[doc = "Timer 2 input capture 0"]
    #[inline(always)]
    pub fn timer2input_capture0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0osel::Timer2inputCapture0)
    }
    #[doc = "Timer 2 OCPRE_CLR input"]
    #[inline(always)]
    pub fn timer2ocpreclear_input(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0osel::Timer2ocpreclearInput)
    }
}
#[doc = "Polarity of CMP0 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmp0pl {
    #[doc = "0: Output is not inverted"]
    NotInverted = 0,
    #[doc = "1: Output is inverted"]
    Inverted = 1,
}
impl From<Cmp0pl> for bool {
    #[inline(always)]
    fn from(variant: Cmp0pl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP0PL` reader - Polarity of CMP0 output"]
pub type Cmp0plR = crate::BitReader<Cmp0pl>;
impl Cmp0plR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmp0pl {
        match self.bits {
            false => Cmp0pl::NotInverted,
            true => Cmp0pl::Inverted,
        }
    }
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == Cmp0pl::NotInverted
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Cmp0pl::Inverted
    }
}
#[doc = "Field `CMP0PL` writer - Polarity of CMP0 output"]
pub type Cmp0plW<'a, REG> = crate::BitWriter<'a, REG, Cmp0pl>;
impl<'a, REG> Cmp0plW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0pl::NotInverted)
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0pl::Inverted)
    }
}
#[doc = "CMP0 hysteresis\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmp0hst {
    #[doc = "0: No hysteresis"]
    NoHysteresis = 0,
    #[doc = "1: Low hysteresis"]
    LowHysteresis = 1,
    #[doc = "2: Medium hysteresis"]
    MediumHysteresis = 2,
    #[doc = "3: High hysteresis"]
    HighHysteresis = 3,
}
impl From<Cmp0hst> for u8 {
    #[inline(always)]
    fn from(variant: Cmp0hst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmp0hst {
    type Ux = u8;
}
#[doc = "Field `CMP0HST` reader - CMP0 hysteresis"]
pub type Cmp0hstR = crate::FieldReader<Cmp0hst>;
impl Cmp0hstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmp0hst {
        match self.bits {
            0 => Cmp0hst::NoHysteresis,
            1 => Cmp0hst::LowHysteresis,
            2 => Cmp0hst::MediumHysteresis,
            3 => Cmp0hst::HighHysteresis,
            _ => unreachable!(),
        }
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn is_no_hysteresis(&self) -> bool {
        *self == Cmp0hst::NoHysteresis
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn is_low_hysteresis(&self) -> bool {
        *self == Cmp0hst::LowHysteresis
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn is_medium_hysteresis(&self) -> bool {
        *self == Cmp0hst::MediumHysteresis
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn is_high_hysteresis(&self) -> bool {
        *self == Cmp0hst::HighHysteresis
    }
}
#[doc = "Field `CMP0HST` writer - CMP0 hysteresis"]
pub type Cmp0hstW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Cmp0hst>;
impl<'a, REG> Cmp0hstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn no_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0hst::NoHysteresis)
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn low_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0hst::LowHysteresis)
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn medium_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0hst::MediumHysteresis)
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn high_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0hst::HighHysteresis)
    }
}
#[doc = "CMP0 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmp0o {
    #[doc = "0: Non-inverting input below inverting input"]
    Low = 0,
    #[doc = "1: Non-inverting input above inverting input"]
    High = 1,
}
impl From<Cmp0o> for bool {
    #[inline(always)]
    fn from(variant: Cmp0o) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP0O` reader - CMP0 output"]
pub type Cmp0oR = crate::BitReader<Cmp0o>;
impl Cmp0oR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmp0o {
        match self.bits {
            false => Cmp0o::Low,
            true => Cmp0o::High,
        }
    }
    #[doc = "Non-inverting input below inverting input"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Cmp0o::Low
    }
    #[doc = "Non-inverting input above inverting input"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Cmp0o::High
    }
}
#[doc = "CMP0 lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmp0lk {
    #[doc = "0: Control bits are read-write"]
    ReadWrite = 0,
    #[doc = "1: Control bits are read-only"]
    ReadOnly = 1,
}
impl From<Cmp0lk> for bool {
    #[inline(always)]
    fn from(variant: Cmp0lk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP0LK` reader - CMP0 lock"]
pub type Cmp0lkR = crate::BitReader<Cmp0lk>;
impl Cmp0lkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmp0lk {
        match self.bits {
            false => Cmp0lk::ReadWrite,
            true => Cmp0lk::ReadOnly,
        }
    }
    #[doc = "Control bits are read-write"]
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        *self == Cmp0lk::ReadWrite
    }
    #[doc = "Control bits are read-only"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == Cmp0lk::ReadOnly
    }
}
#[doc = "Field `CMP0LK` writer - CMP0 lock"]
pub type Cmp0lkW<'a, REG> = crate::BitWriter<'a, REG, Cmp0lk>;
impl<'a, REG> Cmp0lkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Control bits are read-write"]
    #[inline(always)]
    pub fn read_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0lk::ReadWrite)
    }
    #[doc = "Control bits are read-only"]
    #[inline(always)]
    pub fn read_only(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0lk::ReadOnly)
    }
}
#[doc = "Field `CMP1EN` reader - CMP1 enable"]
pub use Cmp0enR as Cmp1enR;
#[doc = "Field `CMP1EN` writer - CMP1 enable"]
pub use Cmp0enW as Cmp1enW;
#[doc = "Field `CMP1M` reader - CMP1 mode"]
pub use Cmp0mR as Cmp1mR;
#[doc = "Field `CMP1M` writer - CMP1 mode"]
pub use Cmp0mW as Cmp1mW;
#[doc = "Field `CMP1MSEL` reader - CMP1_M input selection"]
pub use Cmp0mselR as Cmp1mselR;
#[doc = "Field `CMP1MSEL` writer - CMP1_M input selection"]
pub use Cmp0mselW as Cmp1mselW;
#[doc = "Window mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wnden {
    #[doc = "0: Window mode disabled"]
    Disabled = 0,
    #[doc = "1: Window mode enabled"]
    Enabled = 1,
}
impl From<Wnden> for bool {
    #[inline(always)]
    fn from(variant: Wnden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WNDEN` reader - Window mode enable"]
pub type WndenR = crate::BitReader<Wnden>;
impl WndenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wnden {
        match self.bits {
            false => Wnden::Disabled,
            true => Wnden::Enabled,
        }
    }
    #[doc = "Window mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Wnden::Disabled
    }
    #[doc = "Window mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Wnden::Enabled
    }
}
#[doc = "Field `WNDEN` writer - Window mode enable"]
pub type WndenW<'a, REG> = crate::BitWriter<'a, REG, Wnden>;
impl<'a, REG> WndenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Window mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wnden::Disabled)
    }
    #[doc = "Window mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wnden::Enabled)
    }
}
#[doc = "Field `CMP1HST` reader - CMP1 hysteresis"]
pub use Cmp0hstR as Cmp1hstR;
#[doc = "Field `CMP1HST` writer - CMP1 hysteresis"]
pub use Cmp0hstW as Cmp1hstW;
#[doc = "Field `CMP1LK` reader - CMP1 lock"]
pub use Cmp0lkR as Cmp1lkR;
#[doc = "Field `CMP1LK` writer - CMP1 lock"]
pub use Cmp0lkW as Cmp1lkW;
#[doc = "Field `CMP1O` reader - CMP1 output"]
pub use Cmp0oR as Cmp1oR;
#[doc = "Field `CMP1OSEL` reader - CMP1 output selection"]
pub use Cmp0oselR as Cmp1oselR;
#[doc = "Field `CMP1OSEL` writer - CMP1 output selection"]
pub use Cmp0oselW as Cmp1oselW;
#[doc = "Field `CMP1PL` reader - Polarity of CMP1 output"]
pub use Cmp0plR as Cmp1plR;
#[doc = "Field `CMP1PL` writer - Polarity of CMP1 output"]
pub use Cmp0plW as Cmp1plW;
impl R {
    #[doc = "Bit 0 - CMP0 enable"]
    #[inline(always)]
    pub fn cmp0en(&self) -> Cmp0enR {
        Cmp0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMP0 switch"]
    #[inline(always)]
    pub fn cmp0sw(&self) -> Cmp0swR {
        Cmp0swR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - CMP0 mode"]
    #[inline(always)]
    pub fn cmp0m(&self) -> Cmp0mR {
        Cmp0mR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - CMP0_M input selection"]
    #[inline(always)]
    pub fn cmp0msel(&self) -> Cmp0mselR {
        Cmp0mselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Comparator 0 output selection"]
    #[inline(always)]
    pub fn cmp0osel(&self) -> Cmp0oselR {
        Cmp0oselR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Polarity of CMP0 output"]
    #[inline(always)]
    pub fn cmp0pl(&self) -> Cmp0plR {
        Cmp0plR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - CMP0 hysteresis"]
    #[inline(always)]
    pub fn cmp0hst(&self) -> Cmp0hstR {
        Cmp0hstR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - CMP0 output"]
    #[inline(always)]
    pub fn cmp0o(&self) -> Cmp0oR {
        Cmp0oR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CMP0 lock"]
    #[inline(always)]
    pub fn cmp0lk(&self) -> Cmp0lkR {
        Cmp0lkR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CMP1 enable"]
    #[inline(always)]
    pub fn cmp1en(&self) -> Cmp1enR {
        Cmp1enR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:19 - CMP1 mode"]
    #[inline(always)]
    pub fn cmp1m(&self) -> Cmp1mR {
        Cmp1mR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - CMP1_M input selection"]
    #[inline(always)]
    pub fn cmp1msel(&self) -> Cmp1mselR {
        Cmp1mselR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Window mode enable"]
    #[inline(always)]
    pub fn wnden(&self) -> WndenR {
        WndenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - CMP1 output selection"]
    #[inline(always)]
    pub fn cmp1osel(&self) -> Cmp1oselR {
        Cmp1oselR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Polarity of CMP1 output"]
    #[inline(always)]
    pub fn cmp1pl(&self) -> Cmp1plR {
        Cmp1plR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - CMP1 hysteresis"]
    #[inline(always)]
    pub fn cmp1hst(&self) -> Cmp1hstR {
        Cmp1hstR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - CMP1 output"]
    #[inline(always)]
    pub fn cmp1o(&self) -> Cmp1oR {
        Cmp1oR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CMP1 lock"]
    #[inline(always)]
    pub fn cmp1lk(&self) -> Cmp1lkR {
        Cmp1lkR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMP0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0en(&mut self) -> Cmp0enW<CsSpec> {
        Cmp0enW::new(self, 0)
    }
    #[doc = "Bit 1 - CMP0 switch"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0sw(&mut self) -> Cmp0swW<CsSpec> {
        Cmp0swW::new(self, 1)
    }
    #[doc = "Bits 2:3 - CMP0 mode"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0m(&mut self) -> Cmp0mW<CsSpec> {
        Cmp0mW::new(self, 2)
    }
    #[doc = "Bits 4:6 - CMP0_M input selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0msel(&mut self) -> Cmp0mselW<CsSpec> {
        Cmp0mselW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Comparator 0 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0osel(&mut self) -> Cmp0oselW<CsSpec> {
        Cmp0oselW::new(self, 8)
    }
    #[doc = "Bit 11 - Polarity of CMP0 output"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0pl(&mut self) -> Cmp0plW<CsSpec> {
        Cmp0plW::new(self, 11)
    }
    #[doc = "Bits 12:13 - CMP0 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0hst(&mut self) -> Cmp0hstW<CsSpec> {
        Cmp0hstW::new(self, 12)
    }
    #[doc = "Bit 15 - CMP0 lock"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0lk(&mut self) -> Cmp0lkW<CsSpec> {
        Cmp0lkW::new(self, 15)
    }
    #[doc = "Bit 16 - CMP1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1en(&mut self) -> Cmp1enW<CsSpec> {
        Cmp1enW::new(self, 16)
    }
    #[doc = "Bits 18:19 - CMP1 mode"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1m(&mut self) -> Cmp1mW<CsSpec> {
        Cmp1mW::new(self, 18)
    }
    #[doc = "Bits 20:22 - CMP1_M input selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1msel(&mut self) -> Cmp1mselW<CsSpec> {
        Cmp1mselW::new(self, 20)
    }
    #[doc = "Bit 23 - Window mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn wnden(&mut self) -> WndenW<CsSpec> {
        WndenW::new(self, 23)
    }
    #[doc = "Bits 24:26 - CMP1 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1osel(&mut self) -> Cmp1oselW<CsSpec> {
        Cmp1oselW::new(self, 24)
    }
    #[doc = "Bit 27 - Polarity of CMP1 output"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1pl(&mut self) -> Cmp1plW<CsSpec> {
        Cmp1plW::new(self, 27)
    }
    #[doc = "Bits 28:29 - CMP1 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1hst(&mut self) -> Cmp1hstW<CsSpec> {
        Cmp1hstW::new(self, 28)
    }
    #[doc = "Bit 31 - CMP1 lock"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1lk(&mut self) -> Cmp1lkW<CsSpec> {
        Cmp1lkW::new(self, 31)
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
