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
#[doc = "DAC0 DMA Underrun Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ddudrie0 {
    #[doc = "0: DAC DMA Underrun Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: DAC DMA Underrun Interrupt enabled"]
    Enabled = 1,
}
impl From<Ddudrie0> for bool {
    #[inline(always)]
    fn from(variant: Ddudrie0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDUDRIE0` reader - DAC0 DMA Underrun Interrupt enable"]
pub type Ddudrie0R = crate::BitReader<Ddudrie0>;
impl Ddudrie0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ddudrie0 {
        match self.bits {
            false => Ddudrie0::Disabled,
            true => Ddudrie0::Enabled,
        }
    }
    #[doc = "DAC DMA Underrun Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ddudrie0::Disabled
    }
    #[doc = "DAC DMA Underrun Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ddudrie0::Enabled
    }
}
#[doc = "Field `DDUDRIE0` writer - DAC0 DMA Underrun Interrupt enable"]
pub type Ddudrie0W<'a, REG> = crate::BitWriter<'a, REG, Ddudrie0>;
impl<'a, REG> Ddudrie0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC DMA Underrun Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ddudrie0::Disabled)
    }
    #[doc = "DAC DMA Underrun Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ddudrie0::Enabled)
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
#[doc = "Field `DDUDRIE1` reader - DAC1 DMA Underrun Interrupt enable"]
pub use Ddudrie0R as Ddudrie1R;
#[doc = "Field `DDUDRIE1` writer - DAC1 DMA Underrun Interrupt enable"]
pub use Ddudrie0W as Ddudrie1W;
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
    #[doc = "Bit 12 - DAC0 DMA enable"]
    #[inline(always)]
    pub fn ddmaen0(&self) -> Ddmaen0R {
        Ddmaen0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DAC0 DMA Underrun Interrupt enable"]
    #[inline(always)]
    pub fn ddudrie0(&self) -> Ddudrie0R {
        Ddudrie0R::new(((self.bits >> 13) & 1) != 0)
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
    #[doc = "Bit 28 - DAC1 DMA enable"]
    #[inline(always)]
    pub fn ddmaen1(&self) -> Ddmaen1R {
        Ddmaen1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1 DMA Underrun Interrupt enable"]
    #[inline(always)]
    pub fn ddudrie1(&self) -> Ddudrie1R {
        Ddudrie1R::new(((self.bits >> 29) & 1) != 0)
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
    #[doc = "Bit 12 - DAC0 DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddmaen0(&mut self) -> Ddmaen0W<CtlSpec> {
        Ddmaen0W::new(self, 12)
    }
    #[doc = "Bit 13 - DAC0 DMA Underrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddudrie0(&mut self) -> Ddudrie0W<CtlSpec> {
        Ddudrie0W::new(self, 13)
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
    #[doc = "Bit 28 - DAC1 DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddmaen1(&mut self) -> Ddmaen1W<CtlSpec> {
        Ddmaen1W::new(self, 28)
    }
    #[doc = "Bit 29 - DAC1 DMA Underrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddudrie1(&mut self) -> Ddudrie1W<CtlSpec> {
        Ddudrie1W::new(self, 29)
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
