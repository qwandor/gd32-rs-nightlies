#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Field `WDCHSEL` reader - Analog watchdog channel select"]
pub type WdchselR = crate::FieldReader;
#[doc = "Field `WDCHSEL` writer - Analog watchdog channel select"]
pub type WdchselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Interrupt enable for EOC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eocie {
    #[doc = "0: EOC interrupt disabled"]
    Disabled = 0,
    #[doc = "1: EOC interrupt enabled. An interrupt is generated when the EOC bit is set"]
    Enabled = 1,
}
impl From<Eocie> for bool {
    #[inline(always)]
    fn from(variant: Eocie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOCIE` reader - Interrupt enable for EOC"]
pub type EocieR = crate::BitReader<Eocie>;
impl EocieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eocie {
        match self.bits {
            false => Eocie::Disabled,
            true => Eocie::Enabled,
        }
    }
    #[doc = "EOC interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Eocie::Disabled
    }
    #[doc = "EOC interrupt enabled. An interrupt is generated when the EOC bit is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Eocie::Enabled
    }
}
#[doc = "Field `EOCIE` writer - Interrupt enable for EOC"]
pub type EocieW<'a, REG> = crate::BitWriter<'a, REG, Eocie>;
impl<'a, REG> EocieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EOC interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Eocie::Disabled)
    }
    #[doc = "EOC interrupt enabled. An interrupt is generated when the EOC bit is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Eocie::Enabled)
    }
}
#[doc = "Interrupt enable for WDE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdeie {
    #[doc = "0: WDE interrupt disabled"]
    Disabled = 0,
    #[doc = "1: WDE interrupt enabled. An interrupt is generated when the WDE bit is set"]
    Enabled = 1,
}
impl From<Wdeie> for bool {
    #[inline(always)]
    fn from(variant: Wdeie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDEIE` reader - Interrupt enable for WDE"]
pub type WdeieR = crate::BitReader<Wdeie>;
impl WdeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdeie {
        match self.bits {
            false => Wdeie::Disabled,
            true => Wdeie::Enabled,
        }
    }
    #[doc = "WDE interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Wdeie::Disabled
    }
    #[doc = "WDE interrupt enabled. An interrupt is generated when the WDE bit is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Wdeie::Enabled
    }
}
#[doc = "Field `WDEIE` writer - Interrupt enable for WDE"]
pub type WdeieW<'a, REG> = crate::BitWriter<'a, REG, Wdeie>;
impl<'a, REG> WdeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WDE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wdeie::Disabled)
    }
    #[doc = "WDE interrupt enabled. An interrupt is generated when the WDE bit is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wdeie::Enabled)
    }
}
#[doc = "Interrupt enable for EOIC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eoicie {
    #[doc = "0: EOIC interrupt disabled"]
    Disabled = 0,
    #[doc = "1: EOIC interrupt enabled. An interrupt is generated when the EOIC bit is set"]
    Enabled = 1,
}
impl From<Eoicie> for bool {
    #[inline(always)]
    fn from(variant: Eoicie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOICIE` reader - Interrupt enable for EOIC"]
pub type EoicieR = crate::BitReader<Eoicie>;
impl EoicieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eoicie {
        match self.bits {
            false => Eoicie::Disabled,
            true => Eoicie::Enabled,
        }
    }
    #[doc = "EOIC interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Eoicie::Disabled
    }
    #[doc = "EOIC interrupt enabled. An interrupt is generated when the EOIC bit is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Eoicie::Enabled
    }
}
#[doc = "Field `EOICIE` writer - Interrupt enable for EOIC"]
pub type EoicieW<'a, REG> = crate::BitWriter<'a, REG, Eoicie>;
impl<'a, REG> EoicieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EOIC interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Eoicie::Disabled)
    }
    #[doc = "EOIC interrupt enabled. An interrupt is generated when the EOIC bit is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Eoicie::Enabled)
    }
}
#[doc = "Scan mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sm {
    #[doc = "0: Scan mode disabled"]
    Disabled = 0,
    #[doc = "1: Scan mode enabled"]
    Enabled = 1,
}
impl From<Sm> for bool {
    #[inline(always)]
    fn from(variant: Sm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SM` reader - Scan mode"]
pub type SmR = crate::BitReader<Sm>;
impl SmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sm {
        match self.bits {
            false => Sm::Disabled,
            true => Sm::Enabled,
        }
    }
    #[doc = "Scan mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sm::Disabled
    }
    #[doc = "Scan mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sm::Enabled
    }
}
#[doc = "Field `SM` writer - Scan mode"]
pub type SmW<'a, REG> = crate::BitWriter<'a, REG, Sm>;
impl<'a, REG> SmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Scan mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sm::Disabled)
    }
    #[doc = "Scan mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sm::Enabled)
    }
}
#[doc = "When in scan mode, analog watchdog is effective on a single channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdsc {
    #[doc = "0: Analog watchdog enabled on all channels"]
    All = 0,
    #[doc = "1: Analog watchdog enabled on a single channel"]
    Single = 1,
}
impl From<Wdsc> for bool {
    #[inline(always)]
    fn from(variant: Wdsc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDSC` reader - When in scan mode, analog watchdog is effective on a single channel"]
pub type WdscR = crate::BitReader<Wdsc>;
impl WdscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdsc {
        match self.bits {
            false => Wdsc::All,
            true => Wdsc::Single,
        }
    }
    #[doc = "Analog watchdog enabled on all channels"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == Wdsc::All
    }
    #[doc = "Analog watchdog enabled on a single channel"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Wdsc::Single
    }
}
#[doc = "Field `WDSC` writer - When in scan mode, analog watchdog is effective on a single channel"]
pub type WdscW<'a, REG> = crate::BitWriter<'a, REG, Wdsc>;
impl<'a, REG> WdscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog enabled on all channels"]
    #[inline(always)]
    pub fn all(self) -> &'a mut crate::W<REG> {
        self.variant(Wdsc::All)
    }
    #[doc = "Analog watchdog enabled on a single channel"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Wdsc::Single)
    }
}
#[doc = "Inserted channel group convert automatically\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ica {
    #[doc = "0: Automatic inserted group conversion disabled"]
    Disabled = 0,
    #[doc = "1: Automatic inserted group conversion enabled"]
    Enabled = 1,
}
impl From<Ica> for bool {
    #[inline(always)]
    fn from(variant: Ica) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICA` reader - Inserted channel group convert automatically"]
pub type IcaR = crate::BitReader<Ica>;
impl IcaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ica {
        match self.bits {
            false => Ica::Disabled,
            true => Ica::Enabled,
        }
    }
    #[doc = "Automatic inserted group conversion disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ica::Disabled
    }
    #[doc = "Automatic inserted group conversion enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ica::Enabled
    }
}
#[doc = "Field `ICA` writer - Inserted channel group convert automatically"]
pub type IcaW<'a, REG> = crate::BitWriter<'a, REG, Ica>;
impl<'a, REG> IcaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic inserted group conversion disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ica::Disabled)
    }
    #[doc = "Automatic inserted group conversion enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ica::Enabled)
    }
}
#[doc = "Discontinuous mode on regular channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disrc {
    #[doc = "0: Discontinuous mode on regular channels disabled"]
    Disabled = 0,
    #[doc = "1: Discontinuous mode on regular channels enabled"]
    Enabled = 1,
}
impl From<Disrc> for bool {
    #[inline(always)]
    fn from(variant: Disrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISRC` reader - Discontinuous mode on regular channels"]
pub type DisrcR = crate::BitReader<Disrc>;
impl DisrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Disrc {
        match self.bits {
            false => Disrc::Disabled,
            true => Disrc::Enabled,
        }
    }
    #[doc = "Discontinuous mode on regular channels disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Disrc::Disabled
    }
    #[doc = "Discontinuous mode on regular channels enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Disrc::Enabled
    }
}
#[doc = "Field `DISRC` writer - Discontinuous mode on regular channels"]
pub type DisrcW<'a, REG> = crate::BitWriter<'a, REG, Disrc>;
impl<'a, REG> DisrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Discontinuous mode on regular channels disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Disrc::Disabled)
    }
    #[doc = "Discontinuous mode on regular channels enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Disrc::Enabled)
    }
}
#[doc = "Discontinuous mode on injected channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disic {
    #[doc = "0: Discontinuous mode on inserted channels disabled"]
    Disabled = 0,
    #[doc = "1: Discontinuous mode on inserted channels enabled"]
    Enabled = 1,
}
impl From<Disic> for bool {
    #[inline(always)]
    fn from(variant: Disic) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISIC` reader - Discontinuous mode on injected channels"]
pub type DisicR = crate::BitReader<Disic>;
impl DisicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Disic {
        match self.bits {
            false => Disic::Disabled,
            true => Disic::Enabled,
        }
    }
    #[doc = "Discontinuous mode on inserted channels disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Disic::Disabled
    }
    #[doc = "Discontinuous mode on inserted channels enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Disic::Enabled
    }
}
#[doc = "Field `DISIC` writer - Discontinuous mode on injected channels"]
pub type DisicW<'a, REG> = crate::BitWriter<'a, REG, Disic>;
impl<'a, REG> DisicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Discontinuous mode on inserted channels disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Disic::Disabled)
    }
    #[doc = "Discontinuous mode on inserted channels enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Disic::Enabled)
    }
}
#[doc = "Field `DISNUM` reader - Number of conversions in discontinuous mode"]
pub type DisnumR = crate::FieldReader;
#[doc = "Field `DISNUM` writer - Number of conversions in discontinuous mode"]
pub type DisnumW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
#[doc = "Inserted channel analog watchdog enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iwden {
    #[doc = "0: Analog watchdog disabled on inserted channels"]
    Disabled = 0,
    #[doc = "1: Analog watchdog enabled on inserted channels"]
    Enabled = 1,
}
impl From<Iwden> for bool {
    #[inline(always)]
    fn from(variant: Iwden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDEN` reader - Inserted channel analog watchdog enable"]
pub type IwdenR = crate::BitReader<Iwden>;
impl IwdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iwden {
        match self.bits {
            false => Iwden::Disabled,
            true => Iwden::Enabled,
        }
    }
    #[doc = "Analog watchdog disabled on inserted channels"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Iwden::Disabled
    }
    #[doc = "Analog watchdog enabled on inserted channels"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Iwden::Enabled
    }
}
#[doc = "Field `IWDEN` writer - Inserted channel analog watchdog enable"]
pub type IwdenW<'a, REG> = crate::BitWriter<'a, REG, Iwden>;
impl<'a, REG> IwdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog disabled on inserted channels"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Iwden::Disabled)
    }
    #[doc = "Analog watchdog enabled on inserted channels"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Iwden::Enabled)
    }
}
#[doc = "Regular channel analog watchdog enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rwden {
    #[doc = "0: Analog watchdog disabled on regular channels"]
    Disabled = 0,
    #[doc = "1: Analog watchdog enabled on regular channels"]
    Enabled = 1,
}
impl From<Rwden> for bool {
    #[inline(always)]
    fn from(variant: Rwden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWDEN` reader - Regular channel analog watchdog enable"]
pub type RwdenR = crate::BitReader<Rwden>;
impl RwdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rwden {
        match self.bits {
            false => Rwden::Disabled,
            true => Rwden::Enabled,
        }
    }
    #[doc = "Analog watchdog disabled on regular channels"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rwden::Disabled
    }
    #[doc = "Analog watchdog enabled on regular channels"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rwden::Enabled
    }
}
#[doc = "Field `RWDEN` writer - Regular channel analog watchdog enable"]
pub type RwdenW<'a, REG> = crate::BitWriter<'a, REG, Rwden>;
impl<'a, REG> RwdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog disabled on regular channels"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rwden::Disabled)
    }
    #[doc = "Analog watchdog enabled on regular channels"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rwden::Enabled)
    }
}
#[doc = "ADC resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dres {
    #[doc = "0: 12 bit resolution"]
    Bits12 = 0,
    #[doc = "1: 10 bit resolution"]
    Bits10 = 1,
    #[doc = "2: 8 bit resolution"]
    Bits8 = 2,
    #[doc = "3: 6 bit resolution"]
    Bits6 = 3,
}
impl From<Dres> for u8 {
    #[inline(always)]
    fn from(variant: Dres) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dres {
    type Ux = u8;
}
#[doc = "Field `DRES` reader - ADC resolution"]
pub type DresR = crate::FieldReader<Dres>;
impl DresR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dres {
        match self.bits {
            0 => Dres::Bits12,
            1 => Dres::Bits10,
            2 => Dres::Bits8,
            3 => Dres::Bits6,
            _ => unreachable!(),
        }
    }
    #[doc = "12 bit resolution"]
    #[inline(always)]
    pub fn is_bits12(&self) -> bool {
        *self == Dres::Bits12
    }
    #[doc = "10 bit resolution"]
    #[inline(always)]
    pub fn is_bits10(&self) -> bool {
        *self == Dres::Bits10
    }
    #[doc = "8 bit resolution"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == Dres::Bits8
    }
    #[doc = "6 bit resolution"]
    #[inline(always)]
    pub fn is_bits6(&self) -> bool {
        *self == Dres::Bits6
    }
}
#[doc = "Field `DRES` writer - ADC resolution"]
pub type DresW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Dres>;
impl<'a, REG> DresW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12 bit resolution"]
    #[inline(always)]
    pub fn bits12(self) -> &'a mut crate::W<REG> {
        self.variant(Dres::Bits12)
    }
    #[doc = "10 bit resolution"]
    #[inline(always)]
    pub fn bits10(self) -> &'a mut crate::W<REG> {
        self.variant(Dres::Bits10)
    }
    #[doc = "8 bit resolution"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut crate::W<REG> {
        self.variant(Dres::Bits8)
    }
    #[doc = "6 bit resolution"]
    #[inline(always)]
    pub fn bits6(self) -> &'a mut crate::W<REG> {
        self.variant(Dres::Bits6)
    }
}
impl R {
    #[doc = "Bits 0:4 - Analog watchdog channel select"]
    #[inline(always)]
    pub fn wdchsel(&self) -> WdchselR {
        WdchselR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline(always)]
    pub fn eocie(&self) -> EocieR {
        EocieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt enable for WDE"]
    #[inline(always)]
    pub fn wdeie(&self) -> WdeieR {
        WdeieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable for EOIC"]
    #[inline(always)]
    pub fn eoicie(&self) -> EoicieR {
        EoicieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Scan mode"]
    #[inline(always)]
    pub fn sm(&self) -> SmR {
        SmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When in scan mode, analog watchdog is effective on a single channel"]
    #[inline(always)]
    pub fn wdsc(&self) -> WdscR {
        WdscR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Inserted channel group convert automatically"]
    #[inline(always)]
    pub fn ica(&self) -> IcaR {
        IcaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Discontinuous mode on regular channels"]
    #[inline(always)]
    pub fn disrc(&self) -> DisrcR {
        DisrcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Discontinuous mode on injected channels"]
    #[inline(always)]
    pub fn disic(&self) -> DisicR {
        DisicR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Number of conversions in discontinuous mode"]
    #[inline(always)]
    pub fn disnum(&self) -> DisnumR {
        DisnumR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 22 - Inserted channel analog watchdog enable"]
    #[inline(always)]
    pub fn iwden(&self) -> IwdenR {
        IwdenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Regular channel analog watchdog enable"]
    #[inline(always)]
    pub fn rwden(&self) -> RwdenR {
        RwdenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - ADC resolution"]
    #[inline(always)]
    pub fn dres(&self) -> DresR {
        DresR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Analog watchdog channel select"]
    #[inline(always)]
    #[must_use]
    pub fn wdchsel(&mut self) -> WdchselW<Ctl0Spec> {
        WdchselW::new(self, 0)
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EocieW<Ctl0Spec> {
        EocieW::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt enable for WDE"]
    #[inline(always)]
    #[must_use]
    pub fn wdeie(&mut self) -> WdeieW<Ctl0Spec> {
        WdeieW::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt enable for EOIC"]
    #[inline(always)]
    #[must_use]
    pub fn eoicie(&mut self) -> EoicieW<Ctl0Spec> {
        EoicieW::new(self, 7)
    }
    #[doc = "Bit 8 - Scan mode"]
    #[inline(always)]
    #[must_use]
    pub fn sm(&mut self) -> SmW<Ctl0Spec> {
        SmW::new(self, 8)
    }
    #[doc = "Bit 9 - When in scan mode, analog watchdog is effective on a single channel"]
    #[inline(always)]
    #[must_use]
    pub fn wdsc(&mut self) -> WdscW<Ctl0Spec> {
        WdscW::new(self, 9)
    }
    #[doc = "Bit 10 - Inserted channel group convert automatically"]
    #[inline(always)]
    #[must_use]
    pub fn ica(&mut self) -> IcaW<Ctl0Spec> {
        IcaW::new(self, 10)
    }
    #[doc = "Bit 11 - Discontinuous mode on regular channels"]
    #[inline(always)]
    #[must_use]
    pub fn disrc(&mut self) -> DisrcW<Ctl0Spec> {
        DisrcW::new(self, 11)
    }
    #[doc = "Bit 12 - Discontinuous mode on injected channels"]
    #[inline(always)]
    #[must_use]
    pub fn disic(&mut self) -> DisicW<Ctl0Spec> {
        DisicW::new(self, 12)
    }
    #[doc = "Bits 13:15 - Number of conversions in discontinuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn disnum(&mut self) -> DisnumW<Ctl0Spec> {
        DisnumW::new(self, 13)
    }
    #[doc = "Bit 22 - Inserted channel analog watchdog enable"]
    #[inline(always)]
    #[must_use]
    pub fn iwden(&mut self) -> IwdenW<Ctl0Spec> {
        IwdenW::new(self, 22)
    }
    #[doc = "Bit 23 - Regular channel analog watchdog enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwden(&mut self) -> RwdenW<Ctl0Spec> {
        RwdenW::new(self, 23)
    }
    #[doc = "Bits 24:25 - ADC resolution"]
    #[inline(always)]
    #[must_use]
    pub fn dres(&mut self) -> DresW<Ctl0Spec> {
        DresW::new(self, 24)
    }
}
#[doc = "control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for Ctl0Spec {
    const RESET_VALUE: u32 = 0;
}
