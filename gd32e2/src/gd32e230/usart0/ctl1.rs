#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "7-bit Address Detection/4-bit Address Detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addm {
    #[doc = "0: 4-bit address detection"]
    Bit4 = 0,
    #[doc = "1: Full-bit address detection"]
    Full = 1,
}
impl From<Addm> for bool {
    #[inline(always)]
    fn from(variant: Addm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDM` reader - 7-bit Address Detection/4-bit Address Detection"]
pub type AddmR = crate::BitReader<Addm>;
impl AddmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addm {
        match self.bits {
            false => Addm::Bit4,
            true => Addm::Full,
        }
    }
    #[doc = "4-bit address detection"]
    #[inline(always)]
    pub fn is_bit4(&self) -> bool {
        *self == Addm::Bit4
    }
    #[doc = "Full-bit address detection"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Addm::Full
    }
}
#[doc = "Field `ADDM` writer - 7-bit Address Detection/4-bit Address Detection"]
pub type AddmW<'a, REG> = crate::BitWriter<'a, REG, Addm>;
impl<'a, REG> AddmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "4-bit address detection"]
    #[inline(always)]
    pub fn bit4(self) -> &'a mut crate::W<REG> {
        self.variant(Addm::Bit4)
    }
    #[doc = "Full-bit address detection"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Addm::Full)
    }
}
#[doc = "LIN break detection length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lblen {
    #[doc = "0: 10-bit break detection"]
    Bit10 = 0,
    #[doc = "1: 11-bit break detection"]
    Bit11 = 1,
}
impl From<Lblen> for bool {
    #[inline(always)]
    fn from(variant: Lblen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBLEN` reader - LIN break detection length"]
pub type LblenR = crate::BitReader<Lblen>;
impl LblenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lblen {
        match self.bits {
            false => Lblen::Bit10,
            true => Lblen::Bit11,
        }
    }
    #[doc = "10-bit break detection"]
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == Lblen::Bit10
    }
    #[doc = "11-bit break detection"]
    #[inline(always)]
    pub fn is_bit11(&self) -> bool {
        *self == Lblen::Bit11
    }
}
#[doc = "Field `LBLEN` writer - LIN break detection length"]
pub type LblenW<'a, REG> = crate::BitWriter<'a, REG, Lblen>;
impl<'a, REG> LblenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "10-bit break detection"]
    #[inline(always)]
    pub fn bit10(self) -> &'a mut crate::W<REG> {
        self.variant(Lblen::Bit10)
    }
    #[doc = "11-bit break detection"]
    #[inline(always)]
    pub fn bit11(self) -> &'a mut crate::W<REG> {
        self.variant(Lblen::Bit11)
    }
}
#[doc = "LIN break detection interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbdie {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated whenever LBDF=1 in the STAT register"]
    Enabled = 1,
}
impl From<Lbdie> for bool {
    #[inline(always)]
    fn from(variant: Lbdie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBDIE` reader - LIN break detection interrupt enable"]
pub type LbdieR = crate::BitReader<Lbdie>;
impl LbdieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbdie {
        match self.bits {
            false => Lbdie::Disabled,
            true => Lbdie::Enabled,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lbdie::Disabled
    }
    #[doc = "An interrupt is generated whenever LBDF=1 in the STAT register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lbdie::Enabled
    }
}
#[doc = "Field `LBDIE` writer - LIN break detection interrupt enable"]
pub type LbdieW<'a, REG> = crate::BitWriter<'a, REG, Lbdie>;
impl<'a, REG> LbdieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lbdie::Disabled)
    }
    #[doc = "An interrupt is generated whenever LBDF=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lbdie::Enabled)
    }
}
#[doc = "Last bit clock pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clen {
    #[doc = "0: The clock pulse of the last data bit is not output to the CK pin"]
    NotOutput = 0,
    #[doc = "1: The clock pulse of the last data bit is output to the CK pin"]
    Output = 1,
}
impl From<Clen> for bool {
    #[inline(always)]
    fn from(variant: Clen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLEN` reader - Last bit clock pulse"]
pub type ClenR = crate::BitReader<Clen>;
impl ClenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clen {
        match self.bits {
            false => Clen::NotOutput,
            true => Clen::Output,
        }
    }
    #[doc = "The clock pulse of the last data bit is not output to the CK pin"]
    #[inline(always)]
    pub fn is_not_output(&self) -> bool {
        *self == Clen::NotOutput
    }
    #[doc = "The clock pulse of the last data bit is output to the CK pin"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Clen::Output
    }
}
#[doc = "Field `CLEN` writer - Last bit clock pulse"]
pub type ClenW<'a, REG> = crate::BitWriter<'a, REG, Clen>;
impl<'a, REG> ClenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The clock pulse of the last data bit is not output to the CK pin"]
    #[inline(always)]
    pub fn not_output(self) -> &'a mut crate::W<REG> {
        self.variant(Clen::NotOutput)
    }
    #[doc = "The clock pulse of the last data bit is output to the CK pin"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Clen::Output)
    }
}
#[doc = "Clock phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cph {
    #[doc = "0: The first clock transition is the first data capture edge"]
    First = 0,
    #[doc = "1: The second clock transition is the first data capture edge"]
    Second = 1,
}
impl From<Cph> for bool {
    #[inline(always)]
    fn from(variant: Cph) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPH` reader - Clock phase"]
pub type CphR = crate::BitReader<Cph>;
impl CphR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cph {
        match self.bits {
            false => Cph::First,
            true => Cph::Second,
        }
    }
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn is_first(&self) -> bool {
        *self == Cph::First
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == Cph::Second
    }
}
#[doc = "Field `CPH` writer - Clock phase"]
pub type CphW<'a, REG> = crate::BitWriter<'a, REG, Cph>;
impl<'a, REG> CphW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn first(self) -> &'a mut crate::W<REG> {
        self.variant(Cph::First)
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn second(self) -> &'a mut crate::W<REG> {
        self.variant(Cph::Second)
    }
}
#[doc = "Clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpl {
    #[doc = "0: Steady low value on CK pin outside tranmission window"]
    NotInverted = 0,
    #[doc = "1: Steady high value on CK pin outside tranmission window"]
    Inverted = 1,
}
impl From<Cpl> for bool {
    #[inline(always)]
    fn from(variant: Cpl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPL` reader - Clock polarity"]
pub type CplR = crate::BitReader<Cpl>;
impl CplR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpl {
        match self.bits {
            false => Cpl::NotInverted,
            true => Cpl::Inverted,
        }
    }
    #[doc = "Steady low value on CK pin outside tranmission window"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == Cpl::NotInverted
    }
    #[doc = "Steady high value on CK pin outside tranmission window"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Cpl::Inverted
    }
}
#[doc = "Field `CPL` writer - Clock polarity"]
pub type CplW<'a, REG> = crate::BitWriter<'a, REG, Cpl>;
impl<'a, REG> CplW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Steady low value on CK pin outside tranmission window"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Cpl::NotInverted)
    }
    #[doc = "Steady high value on CK pin outside tranmission window"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Cpl::Inverted)
    }
}
#[doc = "Clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cken {
    #[doc = "0: CK pin disabled"]
    Disabled = 0,
    #[doc = "1: CK pin enabled"]
    Enabled = 1,
}
impl From<Cken> for bool {
    #[inline(always)]
    fn from(variant: Cken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKEN` reader - Clock enable"]
pub type CkenR = crate::BitReader<Cken>;
impl CkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cken {
        match self.bits {
            false => Cken::Disabled,
            true => Cken::Enabled,
        }
    }
    #[doc = "CK pin disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cken::Disabled
    }
    #[doc = "CK pin enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cken::Enabled
    }
}
#[doc = "Field `CKEN` writer - Clock enable"]
pub type CkenW<'a, REG> = crate::BitWriter<'a, REG, Cken>;
impl<'a, REG> CkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CK pin disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cken::Disabled)
    }
    #[doc = "CK pin enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cken::Enabled)
    }
}
#[doc = "STOP bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stb {
    #[doc = "0: 1 stop bit"]
    Stop1 = 0,
    #[doc = "1: 0.5 stop bit"]
    Stop0p5 = 1,
    #[doc = "2: 2 stop bit"]
    Stop2 = 2,
    #[doc = "3: 1.5 stop bit"]
    Stop1p5 = 3,
}
impl From<Stb> for u8 {
    #[inline(always)]
    fn from(variant: Stb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stb {
    type Ux = u8;
}
#[doc = "Field `STB` reader - STOP bits"]
pub type StbR = crate::FieldReader<Stb>;
impl StbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stb {
        match self.bits {
            0 => Stb::Stop1,
            1 => Stb::Stop0p5,
            2 => Stb::Stop2,
            3 => Stb::Stop1p5,
            _ => unreachable!(),
        }
    }
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == Stb::Stop1
    }
    #[doc = "0.5 stop bit"]
    #[inline(always)]
    pub fn is_stop0p5(&self) -> bool {
        *self == Stb::Stop0p5
    }
    #[doc = "2 stop bit"]
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == Stb::Stop2
    }
    #[doc = "1.5 stop bit"]
    #[inline(always)]
    pub fn is_stop1p5(&self) -> bool {
        *self == Stb::Stop1p5
    }
}
#[doc = "Field `STB` writer - STOP bits"]
pub type StbW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Stb>;
impl<'a, REG> StbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn stop1(self) -> &'a mut crate::W<REG> {
        self.variant(Stb::Stop1)
    }
    #[doc = "0.5 stop bit"]
    #[inline(always)]
    pub fn stop0p5(self) -> &'a mut crate::W<REG> {
        self.variant(Stb::Stop0p5)
    }
    #[doc = "2 stop bit"]
    #[inline(always)]
    pub fn stop2(self) -> &'a mut crate::W<REG> {
        self.variant(Stb::Stop2)
    }
    #[doc = "1.5 stop bit"]
    #[inline(always)]
    pub fn stop1p5(self) -> &'a mut crate::W<REG> {
        self.variant(Stb::Stop1p5)
    }
}
#[doc = "LIN mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lmen {
    #[doc = "0: LIN mode disabled"]
    Disabled = 0,
    #[doc = "1: LIN mode enabled"]
    Enabled = 1,
}
impl From<Lmen> for bool {
    #[inline(always)]
    fn from(variant: Lmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LMEN` reader - LIN mode enable"]
pub type LmenR = crate::BitReader<Lmen>;
impl LmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lmen {
        match self.bits {
            false => Lmen::Disabled,
            true => Lmen::Enabled,
        }
    }
    #[doc = "LIN mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lmen::Disabled
    }
    #[doc = "LIN mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lmen::Enabled
    }
}
#[doc = "Field `LMEN` writer - LIN mode enable"]
pub type LmenW<'a, REG> = crate::BitWriter<'a, REG, Lmen>;
impl<'a, REG> LmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LIN mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lmen::Disabled)
    }
    #[doc = "LIN mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lmen::Enabled)
    }
}
#[doc = "Swap TX/RX pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Strp {
    #[doc = "0: TX/RX pins are used as defined in standard pinout"]
    Standard = 0,
    #[doc = "1: The TX and RX pins functions are swapped"]
    Swapped = 1,
}
impl From<Strp> for bool {
    #[inline(always)]
    fn from(variant: Strp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRP` reader - Swap TX/RX pins"]
pub type StrpR = crate::BitReader<Strp>;
impl StrpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Strp {
        match self.bits {
            false => Strp::Standard,
            true => Strp::Swapped,
        }
    }
    #[doc = "TX/RX pins are used as defined in standard pinout"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == Strp::Standard
    }
    #[doc = "The TX and RX pins functions are swapped"]
    #[inline(always)]
    pub fn is_swapped(&self) -> bool {
        *self == Strp::Swapped
    }
}
#[doc = "Field `STRP` writer - Swap TX/RX pins"]
pub type StrpW<'a, REG> = crate::BitWriter<'a, REG, Strp>;
impl<'a, REG> StrpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX/RX pins are used as defined in standard pinout"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(Strp::Standard)
    }
    #[doc = "The TX and RX pins functions are swapped"]
    #[inline(always)]
    pub fn swapped(self) -> &'a mut crate::W<REG> {
        self.variant(Strp::Swapped)
    }
}
#[doc = "RX pin active level inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rinv {
    #[doc = "0: RX pin signal works using the standard logic levels"]
    Standard = 0,
    #[doc = "1: RX pin signal values are inverted"]
    Inverted = 1,
}
impl From<Rinv> for bool {
    #[inline(always)]
    fn from(variant: Rinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RINV` reader - RX pin active level inversion"]
pub type RinvR = crate::BitReader<Rinv>;
impl RinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rinv {
        match self.bits {
            false => Rinv::Standard,
            true => Rinv::Inverted,
        }
    }
    #[doc = "RX pin signal works using the standard logic levels"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == Rinv::Standard
    }
    #[doc = "RX pin signal values are inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Rinv::Inverted
    }
}
#[doc = "Field `RINV` writer - RX pin active level inversion"]
pub type RinvW<'a, REG> = crate::BitWriter<'a, REG, Rinv>;
impl<'a, REG> RinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RX pin signal works using the standard logic levels"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(Rinv::Standard)
    }
    #[doc = "RX pin signal values are inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Rinv::Inverted)
    }
}
#[doc = "TX pin active level inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tinv {
    #[doc = "0: TX pin signal works using the standard logic levels"]
    Standard = 0,
    #[doc = "1: TX pin signal values are inverted"]
    Inverted = 1,
}
impl From<Tinv> for bool {
    #[inline(always)]
    fn from(variant: Tinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TINV` reader - TX pin active level inversion"]
pub type TinvR = crate::BitReader<Tinv>;
impl TinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tinv {
        match self.bits {
            false => Tinv::Standard,
            true => Tinv::Inverted,
        }
    }
    #[doc = "TX pin signal works using the standard logic levels"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == Tinv::Standard
    }
    #[doc = "TX pin signal values are inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Tinv::Inverted
    }
}
#[doc = "Field `TINV` writer - TX pin active level inversion"]
pub type TinvW<'a, REG> = crate::BitWriter<'a, REG, Tinv>;
impl<'a, REG> TinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX pin signal works using the standard logic levels"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(Tinv::Standard)
    }
    #[doc = "TX pin signal values are inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Tinv::Inverted)
    }
}
#[doc = "Binary data inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dinv {
    #[doc = "0: Logical data from the data register are send/received in positive/direct logic"]
    Positive = 0,
    #[doc = "1: Logical data from the data register are send/received in negative/inverse logic"]
    Negative = 1,
}
impl From<Dinv> for bool {
    #[inline(always)]
    fn from(variant: Dinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINV` reader - Binary data inversion"]
pub type DinvR = crate::BitReader<Dinv>;
impl DinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dinv {
        match self.bits {
            false => Dinv::Positive,
            true => Dinv::Negative,
        }
    }
    #[doc = "Logical data from the data register are send/received in positive/direct logic"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == Dinv::Positive
    }
    #[doc = "Logical data from the data register are send/received in negative/inverse logic"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == Dinv::Negative
    }
}
#[doc = "Field `DINV` writer - Binary data inversion"]
pub type DinvW<'a, REG> = crate::BitWriter<'a, REG, Dinv>;
impl<'a, REG> DinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Logical data from the data register are send/received in positive/direct logic"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut crate::W<REG> {
        self.variant(Dinv::Positive)
    }
    #[doc = "Logical data from the data register are send/received in negative/inverse logic"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut crate::W<REG> {
        self.variant(Dinv::Negative)
    }
}
#[doc = "Most significant bit first\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msbf {
    #[doc = "0: Data is transmitted/received with data bit 0 first, following the start bit"]
    Lsb = 0,
    #[doc = "1: Data is transmitted/received with MSB (bit 7/8/9) first, following the start bit"]
    Msb = 1,
}
impl From<Msbf> for bool {
    #[inline(always)]
    fn from(variant: Msbf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSBF` reader - Most significant bit first"]
pub type MsbfR = crate::BitReader<Msbf>;
impl MsbfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msbf {
        match self.bits {
            false => Msbf::Lsb,
            true => Msbf::Msb,
        }
    }
    #[doc = "Data is transmitted/received with data bit 0 first, following the start bit"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == Msbf::Lsb
    }
    #[doc = "Data is transmitted/received with MSB (bit 7/8/9) first, following the start bit"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == Msbf::Msb
    }
}
#[doc = "Field `MSBF` writer - Most significant bit first"]
pub type MsbfW<'a, REG> = crate::BitWriter<'a, REG, Msbf>;
impl<'a, REG> MsbfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is transmitted/received with data bit 0 first, following the start bit"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(Msbf::Lsb)
    }
    #[doc = "Data is transmitted/received with MSB (bit 7/8/9) first, following the start bit"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(Msbf::Msb)
    }
}
#[doc = "Auto baud rate enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Abden {
    #[doc = "0: Auto baud rate detection is disabled"]
    Disabled = 0,
    #[doc = "1: Auto baud rate detection is enabled"]
    Enabled = 1,
}
impl From<Abden> for bool {
    #[inline(always)]
    fn from(variant: Abden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABDEN` reader - Auto baud rate enable"]
pub type AbdenR = crate::BitReader<Abden>;
impl AbdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Abden {
        match self.bits {
            false => Abden::Disabled,
            true => Abden::Enabled,
        }
    }
    #[doc = "Auto baud rate detection is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Abden::Disabled
    }
    #[doc = "Auto baud rate detection is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Abden::Enabled
    }
}
#[doc = "Field `ABDEN` writer - Auto baud rate enable"]
pub type AbdenW<'a, REG> = crate::BitWriter<'a, REG, Abden>;
impl<'a, REG> AbdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto baud rate detection is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Abden::Disabled)
    }
    #[doc = "Auto baud rate detection is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Abden::Enabled)
    }
}
#[doc = "Auto baud rate mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Abdm {
    #[doc = "0: Measurement of the start bit is used to detect the baud rate"]
    Start = 0,
    #[doc = "1: Falling edge to falling edge measurement"]
    Edge = 1,
}
impl From<Abdm> for u8 {
    #[inline(always)]
    fn from(variant: Abdm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Abdm {
    type Ux = u8;
}
#[doc = "Field `ABDM` reader - Auto baud rate mode"]
pub type AbdmR = crate::FieldReader<Abdm>;
impl AbdmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Abdm> {
        match self.bits {
            0 => Some(Abdm::Start),
            1 => Some(Abdm::Edge),
            _ => None,
        }
    }
    #[doc = "Measurement of the start bit is used to detect the baud rate"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Abdm::Start
    }
    #[doc = "Falling edge to falling edge measurement"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Abdm::Edge
    }
}
#[doc = "Field `ABDM` writer - Auto baud rate mode"]
pub type AbdmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Abdm>;
impl<'a, REG> AbdmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Measurement of the start bit is used to detect the baud rate"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Abdm::Start)
    }
    #[doc = "Falling edge to falling edge measurement"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Abdm::Edge)
    }
}
#[doc = "Receiver timeout enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rten {
    #[doc = "0: Receiver timeout feature disabled"]
    Disabled = 0,
    #[doc = "1: Receiver timeout feature enabled"]
    Enabled = 1,
}
impl From<Rten> for bool {
    #[inline(always)]
    fn from(variant: Rten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTEN` reader - Receiver timeout enable"]
pub type RtenR = crate::BitReader<Rten>;
impl RtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rten {
        match self.bits {
            false => Rten::Disabled,
            true => Rten::Enabled,
        }
    }
    #[doc = "Receiver timeout feature disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rten::Disabled
    }
    #[doc = "Receiver timeout feature enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rten::Enabled
    }
}
#[doc = "Field `RTEN` writer - Receiver timeout enable"]
pub type RtenW<'a, REG> = crate::BitWriter<'a, REG, Rten>;
impl<'a, REG> RtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver timeout feature disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rten::Disabled)
    }
    #[doc = "Receiver timeout feature enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rten::Enabled)
    }
}
#[doc = "Field `ADDR` reader - Address of the USART node"]
pub type AddrR = crate::FieldReader;
#[doc = "Field `ADDR` writer - Address of the USART node"]
pub type AddrW<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline(always)]
    pub fn addm(&self) -> AddmR {
        AddmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LIN break detection length"]
    #[inline(always)]
    pub fn lblen(&self) -> LblenR {
        LblenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn lbdie(&self) -> LbdieR {
        LbdieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    pub fn clen(&self) -> ClenR {
        ClenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn cph(&self) -> CphR {
        CphR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn cpl(&self) -> CplR {
        CplR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    pub fn cken(&self) -> CkenR {
        CkenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    pub fn stb(&self) -> StbR {
        StbR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn lmen(&self) -> LmenR {
        LmenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    pub fn strp(&self) -> StrpR {
        StrpR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline(always)]
    pub fn rinv(&self) -> RinvR {
        RinvR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline(always)]
    pub fn tinv(&self) -> TinvR {
        TinvR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline(always)]
    pub fn dinv(&self) -> DinvR {
        DinvR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    pub fn msbf(&self) -> MsbfR {
        MsbfR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Auto baud rate enable"]
    #[inline(always)]
    pub fn abden(&self) -> AbdenR {
        AbdenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Auto baud rate mode"]
    #[inline(always)]
    pub fn abdm(&self) -> AbdmR {
        AbdmR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Receiver timeout enable"]
    #[inline(always)]
    pub fn rten(&self) -> RtenR {
        RtenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Address of the USART node"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline(always)]
    #[must_use]
    pub fn addm(&mut self) -> AddmW<Ctl1Spec> {
        AddmW::new(self, 4)
    }
    #[doc = "Bit 5 - LIN break detection length"]
    #[inline(always)]
    #[must_use]
    pub fn lblen(&mut self) -> LblenW<Ctl1Spec> {
        LblenW::new(self, 5)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lbdie(&mut self) -> LbdieW<Ctl1Spec> {
        LbdieW::new(self, 6)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    #[must_use]
    pub fn clen(&mut self) -> ClenW<Ctl1Spec> {
        ClenW::new(self, 8)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn cph(&mut self) -> CphW<Ctl1Spec> {
        CphW::new(self, 9)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpl(&mut self) -> CplW<Ctl1Spec> {
        CplW::new(self, 10)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn cken(&mut self) -> CkenW<Ctl1Spec> {
        CkenW::new(self, 11)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    #[must_use]
    pub fn stb(&mut self) -> StbW<Ctl1Spec> {
        StbW::new(self, 12)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn lmen(&mut self) -> LmenW<Ctl1Spec> {
        LmenW::new(self, 14)
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    #[must_use]
    pub fn strp(&mut self) -> StrpW<Ctl1Spec> {
        StrpW::new(self, 15)
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline(always)]
    #[must_use]
    pub fn rinv(&mut self) -> RinvW<Ctl1Spec> {
        RinvW::new(self, 16)
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline(always)]
    #[must_use]
    pub fn tinv(&mut self) -> TinvW<Ctl1Spec> {
        TinvW::new(self, 17)
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline(always)]
    #[must_use]
    pub fn dinv(&mut self) -> DinvW<Ctl1Spec> {
        DinvW::new(self, 18)
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    #[must_use]
    pub fn msbf(&mut self) -> MsbfW<Ctl1Spec> {
        MsbfW::new(self, 19)
    }
    #[doc = "Bit 20 - Auto baud rate enable"]
    #[inline(always)]
    #[must_use]
    pub fn abden(&mut self) -> AbdenW<Ctl1Spec> {
        AbdenW::new(self, 20)
    }
    #[doc = "Bits 21:22 - Auto baud rate mode"]
    #[inline(always)]
    #[must_use]
    pub fn abdm(&mut self) -> AbdmW<Ctl1Spec> {
        AbdmW::new(self, 21)
    }
    #[doc = "Bit 23 - Receiver timeout enable"]
    #[inline(always)]
    #[must_use]
    pub fn rten(&mut self) -> RtenW<Ctl1Spec> {
        RtenW::new(self, 23)
    }
    #[doc = "Bits 24:31 - Address of the USART node"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<Ctl1Spec> {
        AddrW::new(self, 24)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
