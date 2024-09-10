#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "ADC on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcon {
    #[doc = "0: Disable ADC conversion/calibration and go to power down mode"]
    Disabled = 0,
    #[doc = "1: Enable ADC and to start conversion"]
    Enabled = 1,
}
impl From<Adcon> for bool {
    #[inline(always)]
    fn from(variant: Adcon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCON` reader - ADC on"]
pub type AdconR = crate::BitReader<Adcon>;
impl AdconR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcon {
        match self.bits {
            false => Adcon::Disabled,
            true => Adcon::Enabled,
        }
    }
    #[doc = "Disable ADC conversion/calibration and go to power down mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Adcon::Disabled
    }
    #[doc = "Enable ADC and to start conversion"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Adcon::Enabled
    }
}
#[doc = "Field `ADCON` writer - ADC on"]
pub type AdconW<'a, REG> = crate::BitWriter<'a, REG, Adcon>;
impl<'a, REG> AdconW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable ADC conversion/calibration and go to power down mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Adcon::Disabled)
    }
    #[doc = "Enable ADC and to start conversion"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Adcon::Enabled)
    }
}
#[doc = "Continuous mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctn {
    #[doc = "0: Single conversion mode"]
    Single = 0,
    #[doc = "1: Continuous conversion mode"]
    Continuous = 1,
}
impl From<Ctn> for bool {
    #[inline(always)]
    fn from(variant: Ctn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTN` reader - Continuous mode"]
pub type CtnR = crate::BitReader<Ctn>;
impl CtnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctn {
        match self.bits {
            false => Ctn::Single,
            true => Ctn::Continuous,
        }
    }
    #[doc = "Single conversion mode"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Ctn::Single
    }
    #[doc = "Continuous conversion mode"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Ctn::Continuous
    }
}
#[doc = "Field `CTN` writer - Continuous mode"]
pub type CtnW<'a, REG> = crate::BitWriter<'a, REG, Ctn>;
impl<'a, REG> CtnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single conversion mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Ctn::Single)
    }
    #[doc = "Continuous conversion mode"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Ctn::Continuous)
    }
}
#[doc = "ADC calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clbr {
    #[doc = "0: Calibration completed"]
    Complete = 0,
    #[doc = "1: Calibrating"]
    NotComplete = 1,
}
impl From<Clbr> for bool {
    #[inline(always)]
    fn from(variant: Clbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLB` reader - ADC calibration"]
pub type ClbR = crate::BitReader<Clbr>;
impl ClbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clbr {
        match self.bits {
            false => Clbr::Complete,
            true => Clbr::NotComplete,
        }
    }
    #[doc = "Calibration completed"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == Clbr::Complete
    }
    #[doc = "Calibrating"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == Clbr::NotComplete
    }
}
#[doc = "ADC calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClbwWO {
    #[doc = "1: Enable calibration"]
    Start = 1,
}
impl From<ClbwWO> for bool {
    #[inline(always)]
    fn from(variant: ClbwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLB` writer - ADC calibration"]
pub type ClbW<'a, REG> = crate::BitWriter<'a, REG, ClbwWO>;
impl<'a, REG> ClbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable calibration"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(ClbwWO::Start)
    }
}
#[doc = "Reset calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rstclbr {
    #[doc = "0: Calibration completed"]
    Complete = 0,
    #[doc = "1: Calibrating"]
    NotComplete = 1,
}
impl From<Rstclbr> for bool {
    #[inline(always)]
    fn from(variant: Rstclbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTCLB` reader - Reset calibration"]
pub type RstclbR = crate::BitReader<Rstclbr>;
impl RstclbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rstclbr {
        match self.bits {
            false => Rstclbr::Complete,
            true => Rstclbr::NotComplete,
        }
    }
    #[doc = "Calibration completed"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == Rstclbr::Complete
    }
    #[doc = "Calibrating"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == Rstclbr::NotComplete
    }
}
#[doc = "Reset calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RstclbwWO {
    #[doc = "1: Enable calibration"]
    Start = 1,
}
impl From<RstclbwWO> for bool {
    #[inline(always)]
    fn from(variant: RstclbwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTCLB` writer - Reset calibration"]
pub type RstclbW<'a, REG> = crate::BitWriter<'a, REG, RstclbwWO>;
impl<'a, REG> RstclbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable calibration"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(RstclbwWO::Start)
    }
}
#[doc = "DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma {
    #[doc = "0: DMA mode disabled"]
    Disabled = 0,
    #[doc = "1: DMA mode enabled"]
    Enabled = 1,
}
impl From<Dma> for bool {
    #[inline(always)]
    fn from(variant: Dma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` reader - DMA request enable"]
pub type DmaR = crate::BitReader<Dma>;
impl DmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma {
        match self.bits {
            false => Dma::Disabled,
            true => Dma::Enabled,
        }
    }
    #[doc = "DMA mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dma::Disabled
    }
    #[doc = "DMA mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dma::Enabled
    }
}
#[doc = "Field `DMA` writer - DMA request enable"]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG, Dma>;
impl<'a, REG> DmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dma::Disabled)
    }
    #[doc = "DMA mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dma::Enabled)
    }
}
#[doc = "Data alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dal {
    #[doc = "0: Right alignment"]
    Right = 0,
    #[doc = "1: Left alignment"]
    Left = 1,
}
impl From<Dal> for bool {
    #[inline(always)]
    fn from(variant: Dal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAL` reader - Data alignment"]
pub type DalR = crate::BitReader<Dal>;
impl DalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dal {
        match self.bits {
            false => Dal::Right,
            true => Dal::Left,
        }
    }
    #[doc = "Right alignment"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == Dal::Right
    }
    #[doc = "Left alignment"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == Dal::Left
    }
}
#[doc = "Field `DAL` writer - Data alignment"]
pub type DalW<'a, REG> = crate::BitWriter<'a, REG, Dal>;
impl<'a, REG> DalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Right alignment"]
    #[inline(always)]
    pub fn right(self) -> &'a mut crate::W<REG> {
        self.variant(Dal::Right)
    }
    #[doc = "Left alignment"]
    #[inline(always)]
    pub fn left(self) -> &'a mut crate::W<REG> {
        self.variant(Dal::Left)
    }
}
#[doc = "External trigger select for inserted channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Etsic {
    #[doc = "0: Timer 0 TRGO event"]
    Timer0trgo = 0,
    #[doc = "1: Timer 0 channel 3 event"]
    Timer0ch3 = 1,
    #[doc = "2: Timer 1 TRGO event"]
    Timer1trgo = 2,
    #[doc = "3: Timer 1 channel 0 event"]
    Timer1ch0 = 3,
    #[doc = "4: Timer 2 channel 3 event"]
    Timer2ch2 = 4,
    #[doc = "5: Timer 14 TRGO event"]
    Timer14trgo = 5,
    #[doc = "6: EXTI line 15"]
    Exti15 = 6,
    #[doc = "7: SWICST"]
    Swicst = 7,
}
impl From<Etsic> for u8 {
    #[inline(always)]
    fn from(variant: Etsic) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Etsic {
    type Ux = u8;
}
#[doc = "Field `ETSIC` reader - External trigger select for inserted channel"]
pub type EtsicR = crate::FieldReader<Etsic>;
impl EtsicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Etsic {
        match self.bits {
            0 => Etsic::Timer0trgo,
            1 => Etsic::Timer0ch3,
            2 => Etsic::Timer1trgo,
            3 => Etsic::Timer1ch0,
            4 => Etsic::Timer2ch2,
            5 => Etsic::Timer14trgo,
            6 => Etsic::Exti15,
            7 => Etsic::Swicst,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer 0 TRGO event"]
    #[inline(always)]
    pub fn is_timer0trgo(&self) -> bool {
        *self == Etsic::Timer0trgo
    }
    #[doc = "Timer 0 channel 3 event"]
    #[inline(always)]
    pub fn is_timer0ch3(&self) -> bool {
        *self == Etsic::Timer0ch3
    }
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn is_timer1trgo(&self) -> bool {
        *self == Etsic::Timer1trgo
    }
    #[doc = "Timer 1 channel 0 event"]
    #[inline(always)]
    pub fn is_timer1ch0(&self) -> bool {
        *self == Etsic::Timer1ch0
    }
    #[doc = "Timer 2 channel 3 event"]
    #[inline(always)]
    pub fn is_timer2ch2(&self) -> bool {
        *self == Etsic::Timer2ch2
    }
    #[doc = "Timer 14 TRGO event"]
    #[inline(always)]
    pub fn is_timer14trgo(&self) -> bool {
        *self == Etsic::Timer14trgo
    }
    #[doc = "EXTI line 15"]
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        *self == Etsic::Exti15
    }
    #[doc = "SWICST"]
    #[inline(always)]
    pub fn is_swicst(&self) -> bool {
        *self == Etsic::Swicst
    }
}
#[doc = "Field `ETSIC` writer - External trigger select for inserted channel"]
pub type EtsicW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Etsic>;
impl<'a, REG> EtsicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer 0 TRGO event"]
    #[inline(always)]
    pub fn timer0trgo(self) -> &'a mut crate::W<REG> {
        self.variant(Etsic::Timer0trgo)
    }
    #[doc = "Timer 0 channel 3 event"]
    #[inline(always)]
    pub fn timer0ch3(self) -> &'a mut crate::W<REG> {
        self.variant(Etsic::Timer0ch3)
    }
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn timer1trgo(self) -> &'a mut crate::W<REG> {
        self.variant(Etsic::Timer1trgo)
    }
    #[doc = "Timer 1 channel 0 event"]
    #[inline(always)]
    pub fn timer1ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Etsic::Timer1ch0)
    }
    #[doc = "Timer 2 channel 3 event"]
    #[inline(always)]
    pub fn timer2ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Etsic::Timer2ch2)
    }
    #[doc = "Timer 14 TRGO event"]
    #[inline(always)]
    pub fn timer14trgo(self) -> &'a mut crate::W<REG> {
        self.variant(Etsic::Timer14trgo)
    }
    #[doc = "EXTI line 15"]
    #[inline(always)]
    pub fn exti15(self) -> &'a mut crate::W<REG> {
        self.variant(Etsic::Exti15)
    }
    #[doc = "SWICST"]
    #[inline(always)]
    pub fn swicst(self) -> &'a mut crate::W<REG> {
        self.variant(Etsic::Swicst)
    }
}
#[doc = "External trigger enable for insert channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eteic {
    #[doc = "0: Conversion on external event disabled"]
    Disabled = 0,
    #[doc = "1: Conversion on external event enabled"]
    Enabled = 1,
}
impl From<Eteic> for bool {
    #[inline(always)]
    fn from(variant: Eteic) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETEIC` reader - External trigger enable for insert channel"]
pub type EteicR = crate::BitReader<Eteic>;
impl EteicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eteic {
        match self.bits {
            false => Eteic::Disabled,
            true => Eteic::Enabled,
        }
    }
    #[doc = "Conversion on external event disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Eteic::Disabled
    }
    #[doc = "Conversion on external event enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Eteic::Enabled
    }
}
#[doc = "Field `ETEIC` writer - External trigger enable for insert channel"]
pub type EteicW<'a, REG> = crate::BitWriter<'a, REG, Eteic>;
impl<'a, REG> EteicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Conversion on external event disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Eteic::Disabled)
    }
    #[doc = "Conversion on external event enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Eteic::Enabled)
    }
}
#[doc = "External trigger select for regular channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Etsrc {
    #[doc = "0: Timer 0 channel 0 event"]
    Timer0ch0 = 0,
    #[doc = "1: Timer 0 channel 1 event"]
    Timer0ch1 = 1,
    #[doc = "2: Timer 0 channel 2 event"]
    Timer0ch2 = 2,
    #[doc = "3: Timer 1 channel 1 event"]
    Timer1ch1 = 3,
    #[doc = "4: Timer 2 TRGO event"]
    Timer2trgo = 4,
    #[doc = "5: Timer 14 channel 0 event"]
    Timer14ch0 = 5,
    #[doc = "6: EXTI line 11"]
    Exti11 = 6,
    #[doc = "7: SWRCST"]
    Swrcst = 7,
}
impl From<Etsrc> for u8 {
    #[inline(always)]
    fn from(variant: Etsrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Etsrc {
    type Ux = u8;
}
#[doc = "Field `ETSRC` reader - External trigger select for regular channel"]
pub type EtsrcR = crate::FieldReader<Etsrc>;
impl EtsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Etsrc {
        match self.bits {
            0 => Etsrc::Timer0ch0,
            1 => Etsrc::Timer0ch1,
            2 => Etsrc::Timer0ch2,
            3 => Etsrc::Timer1ch1,
            4 => Etsrc::Timer2trgo,
            5 => Etsrc::Timer14ch0,
            6 => Etsrc::Exti11,
            7 => Etsrc::Swrcst,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer 0 channel 0 event"]
    #[inline(always)]
    pub fn is_timer0ch0(&self) -> bool {
        *self == Etsrc::Timer0ch0
    }
    #[doc = "Timer 0 channel 1 event"]
    #[inline(always)]
    pub fn is_timer0ch1(&self) -> bool {
        *self == Etsrc::Timer0ch1
    }
    #[doc = "Timer 0 channel 2 event"]
    #[inline(always)]
    pub fn is_timer0ch2(&self) -> bool {
        *self == Etsrc::Timer0ch2
    }
    #[doc = "Timer 1 channel 1 event"]
    #[inline(always)]
    pub fn is_timer1ch1(&self) -> bool {
        *self == Etsrc::Timer1ch1
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn is_timer2trgo(&self) -> bool {
        *self == Etsrc::Timer2trgo
    }
    #[doc = "Timer 14 channel 0 event"]
    #[inline(always)]
    pub fn is_timer14ch0(&self) -> bool {
        *self == Etsrc::Timer14ch0
    }
    #[doc = "EXTI line 11"]
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        *self == Etsrc::Exti11
    }
    #[doc = "SWRCST"]
    #[inline(always)]
    pub fn is_swrcst(&self) -> bool {
        *self == Etsrc::Swrcst
    }
}
#[doc = "Field `ETSRC` writer - External trigger select for regular channel"]
pub type EtsrcW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Etsrc>;
impl<'a, REG> EtsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer 0 channel 0 event"]
    #[inline(always)]
    pub fn timer0ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Etsrc::Timer0ch0)
    }
    #[doc = "Timer 0 channel 1 event"]
    #[inline(always)]
    pub fn timer0ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Etsrc::Timer0ch1)
    }
    #[doc = "Timer 0 channel 2 event"]
    #[inline(always)]
    pub fn timer0ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Etsrc::Timer0ch2)
    }
    #[doc = "Timer 1 channel 1 event"]
    #[inline(always)]
    pub fn timer1ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Etsrc::Timer1ch1)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn timer2trgo(self) -> &'a mut crate::W<REG> {
        self.variant(Etsrc::Timer2trgo)
    }
    #[doc = "Timer 14 channel 0 event"]
    #[inline(always)]
    pub fn timer14ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Etsrc::Timer14ch0)
    }
    #[doc = "EXTI line 11"]
    #[inline(always)]
    pub fn exti11(self) -> &'a mut crate::W<REG> {
        self.variant(Etsrc::Exti11)
    }
    #[doc = "SWRCST"]
    #[inline(always)]
    pub fn swrcst(self) -> &'a mut crate::W<REG> {
        self.variant(Etsrc::Swrcst)
    }
}
#[doc = "External trigger enable for regular channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eterc {
    #[doc = "0: Conversion on external event disabled"]
    Disabled = 0,
    #[doc = "1: Conversion on external event enabled"]
    Enabled = 1,
}
impl From<Eterc> for bool {
    #[inline(always)]
    fn from(variant: Eterc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETERC` reader - External trigger enable for regular channel"]
pub type EtercR = crate::BitReader<Eterc>;
impl EtercR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eterc {
        match self.bits {
            false => Eterc::Disabled,
            true => Eterc::Enabled,
        }
    }
    #[doc = "Conversion on external event disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Eterc::Disabled
    }
    #[doc = "Conversion on external event enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Eterc::Enabled
    }
}
#[doc = "Field `ETERC` writer - External trigger enable for regular channel"]
pub type EtercW<'a, REG> = crate::BitWriter<'a, REG, Eterc>;
impl<'a, REG> EtercW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Conversion on external event disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Eterc::Disabled)
    }
    #[doc = "Conversion on external event enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Eterc::Enabled)
    }
}
#[doc = "Start on inserted channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swicstr {
    #[doc = "0: Reset state"]
    Started = 0,
    #[doc = "1: Starting conversion of inserted channels"]
    NotStarted = 1,
}
impl From<Swicstr> for bool {
    #[inline(always)]
    fn from(variant: Swicstr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWICST` reader - Start on inserted channel"]
pub type SwicstR = crate::BitReader<Swicstr>;
impl SwicstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swicstr {
        match self.bits {
            false => Swicstr::Started,
            true => Swicstr::NotStarted,
        }
    }
    #[doc = "Reset state"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == Swicstr::Started
    }
    #[doc = "Starting conversion of inserted channels"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == Swicstr::NotStarted
    }
}
#[doc = "Start on inserted channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwicstwWO {
    #[doc = "1: Start conversion of inserted channels"]
    Start = 1,
}
impl From<SwicstwWO> for bool {
    #[inline(always)]
    fn from(variant: SwicstwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWICST` writer - Start on inserted channel"]
pub type SwicstW<'a, REG> = crate::BitWriter<'a, REG, SwicstwWO>;
impl<'a, REG> SwicstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start conversion of inserted channels"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(SwicstwWO::Start)
    }
}
#[doc = "Channel 16 and 17 enable of ADC0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsvren {
    #[doc = "0: Channel 16 and 17 disabled"]
    Disabled = 0,
    #[doc = "1: Channel 16 and 17 enabled"]
    Enabled = 1,
}
impl From<Tsvren> for bool {
    #[inline(always)]
    fn from(variant: Tsvren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSVREN` reader - Channel 16 and 17 enable of ADC0"]
pub type TsvrenR = crate::BitReader<Tsvren>;
impl TsvrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsvren {
        match self.bits {
            false => Tsvren::Disabled,
            true => Tsvren::Enabled,
        }
    }
    #[doc = "Channel 16 and 17 disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tsvren::Disabled
    }
    #[doc = "Channel 16 and 17 enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tsvren::Enabled
    }
}
#[doc = "Field `TSVREN` writer - Channel 16 and 17 enable of ADC0"]
pub type TsvrenW<'a, REG> = crate::BitWriter<'a, REG, Tsvren>;
impl<'a, REG> TsvrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 16 and 17 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tsvren::Disabled)
    }
    #[doc = "Channel 16 and 17 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tsvren::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - ADC on"]
    #[inline(always)]
    pub fn adcon(&self) -> AdconR {
        AdconR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Continuous mode"]
    #[inline(always)]
    pub fn ctn(&self) -> CtnR {
        CtnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC calibration"]
    #[inline(always)]
    pub fn clb(&self) -> ClbR {
        ClbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    pub fn rstclb(&self) -> RstclbR {
        RstclbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA request enable"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn dal(&self) -> DalR {
        DalR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - External trigger select for inserted channel"]
    #[inline(always)]
    pub fn etsic(&self) -> EtsicR {
        EtsicR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - External trigger enable for insert channel"]
    #[inline(always)]
    pub fn eteic(&self) -> EteicR {
        EteicR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:19 - External trigger select for regular channel"]
    #[inline(always)]
    pub fn etsrc(&self) -> EtsrcR {
        EtsrcR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - External trigger enable for regular channel"]
    #[inline(always)]
    pub fn eterc(&self) -> EtercR {
        EtercR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Start on inserted channel"]
    #[inline(always)]
    pub fn swicst(&self) -> SwicstR {
        SwicstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel 16 and 17 enable of ADC0"]
    #[inline(always)]
    pub fn tsvren(&self) -> TsvrenR {
        TsvrenR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC on"]
    #[inline(always)]
    #[must_use]
    pub fn adcon(&mut self) -> AdconW<Ctl1Spec> {
        AdconW::new(self, 0)
    }
    #[doc = "Bit 1 - Continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn ctn(&mut self) -> CtnW<Ctl1Spec> {
        CtnW::new(self, 1)
    }
    #[doc = "Bit 2 - ADC calibration"]
    #[inline(always)]
    #[must_use]
    pub fn clb(&mut self) -> ClbW<Ctl1Spec> {
        ClbW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    #[must_use]
    pub fn rstclb(&mut self) -> RstclbW<Ctl1Spec> {
        RstclbW::new(self, 3)
    }
    #[doc = "Bit 8 - DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DmaW<Ctl1Spec> {
        DmaW::new(self, 8)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    #[must_use]
    pub fn dal(&mut self) -> DalW<Ctl1Spec> {
        DalW::new(self, 11)
    }
    #[doc = "Bits 12:14 - External trigger select for inserted channel"]
    #[inline(always)]
    #[must_use]
    pub fn etsic(&mut self) -> EtsicW<Ctl1Spec> {
        EtsicW::new(self, 12)
    }
    #[doc = "Bit 15 - External trigger enable for insert channel"]
    #[inline(always)]
    #[must_use]
    pub fn eteic(&mut self) -> EteicW<Ctl1Spec> {
        EteicW::new(self, 15)
    }
    #[doc = "Bits 17:19 - External trigger select for regular channel"]
    #[inline(always)]
    #[must_use]
    pub fn etsrc(&mut self) -> EtsrcW<Ctl1Spec> {
        EtsrcW::new(self, 17)
    }
    #[doc = "Bit 20 - External trigger enable for regular channel"]
    #[inline(always)]
    #[must_use]
    pub fn eterc(&mut self) -> EtercW<Ctl1Spec> {
        EtercW::new(self, 20)
    }
    #[doc = "Bit 21 - Start on inserted channel"]
    #[inline(always)]
    #[must_use]
    pub fn swicst(&mut self) -> SwicstW<Ctl1Spec> {
        SwicstW::new(self, 21)
    }
    #[doc = "Bit 23 - Channel 16 and 17 enable of ADC0"]
    #[inline(always)]
    #[must_use]
    pub fn tsvren(&mut self) -> TsvrenW<Ctl1Spec> {
        TsvrenW::new(self, 23)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {
    const RESET_VALUE: u32 = 0;
}
