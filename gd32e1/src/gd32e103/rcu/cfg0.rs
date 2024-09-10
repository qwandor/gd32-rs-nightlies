#[doc = "Register `CFG0` reader"]
pub type R = crate::R<Cfg0Spec>;
#[doc = "Register `CFG0` writer"]
pub type W = crate::W<Cfg0Spec>;
#[doc = "System clock switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Scs {
    #[doc = "0: IRC8M used as system clock"]
    Irc8m = 0,
    #[doc = "1: HXTAL used as system clock"]
    Hxtal = 1,
    #[doc = "2: PLL used as system clock"]
    Pll = 2,
}
impl From<Scs> for u8 {
    #[inline(always)]
    fn from(variant: Scs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Scs {
    type Ux = u8;
}
#[doc = "Field `SCS` reader - System clock switch"]
pub type ScsR = crate::FieldReader<Scs>;
impl ScsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Scs> {
        match self.bits {
            0 => Some(Scs::Irc8m),
            1 => Some(Scs::Hxtal),
            2 => Some(Scs::Pll),
            _ => None,
        }
    }
    #[doc = "IRC8M used as system clock"]
    #[inline(always)]
    pub fn is_irc8m(&self) -> bool {
        *self == Scs::Irc8m
    }
    #[doc = "HXTAL used as system clock"]
    #[inline(always)]
    pub fn is_hxtal(&self) -> bool {
        *self == Scs::Hxtal
    }
    #[doc = "PLL used as system clock"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == Scs::Pll
    }
}
#[doc = "Field `SCS` writer - System clock switch"]
pub type ScsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Scs>;
impl<'a, REG> ScsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IRC8M used as system clock"]
    #[inline(always)]
    pub fn irc8m(self) -> &'a mut crate::W<REG> {
        self.variant(Scs::Irc8m)
    }
    #[doc = "HXTAL used as system clock"]
    #[inline(always)]
    pub fn hxtal(self) -> &'a mut crate::W<REG> {
        self.variant(Scs::Hxtal)
    }
    #[doc = "PLL used as system clock"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(Scs::Pll)
    }
}
#[doc = "System clock switch status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Scss {
    #[doc = "0: IRC8M used as system clock"]
    Irc8m = 0,
    #[doc = "1: HXTAL used as system clock"]
    Hxtal = 1,
    #[doc = "2: PLL used as system clock"]
    Pll = 2,
}
impl From<Scss> for u8 {
    #[inline(always)]
    fn from(variant: Scss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Scss {
    type Ux = u8;
}
#[doc = "Field `SCSS` reader - System clock switch status"]
pub type ScssR = crate::FieldReader<Scss>;
impl ScssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Scss> {
        match self.bits {
            0 => Some(Scss::Irc8m),
            1 => Some(Scss::Hxtal),
            2 => Some(Scss::Pll),
            _ => None,
        }
    }
    #[doc = "IRC8M used as system clock"]
    #[inline(always)]
    pub fn is_irc8m(&self) -> bool {
        *self == Scss::Irc8m
    }
    #[doc = "HXTAL used as system clock"]
    #[inline(always)]
    pub fn is_hxtal(&self) -> bool {
        *self == Scss::Hxtal
    }
    #[doc = "PLL used as system clock"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == Scss::Pll
    }
}
#[doc = "AHB prescaler selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ahbpsc {
    #[doc = "0: CK_SYS"]
    Div1 = 0,
    #[doc = "8: CK_SYS divided by 2"]
    Div2 = 8,
    #[doc = "9: CK_SYS divided by 4"]
    Div4 = 9,
    #[doc = "10: CK_SYS divided by 8"]
    Div8 = 10,
    #[doc = "11: CK_SYS divided by 16"]
    Div16 = 11,
    #[doc = "12: CK_SYS divided by 64"]
    Div64 = 12,
    #[doc = "13: CK_SYS divided by 128"]
    Div128 = 13,
    #[doc = "14: CK_SYS divided by 256"]
    Div256 = 14,
    #[doc = "15: CK_SYS divided by 512"]
    Div512 = 15,
}
impl From<Ahbpsc> for u8 {
    #[inline(always)]
    fn from(variant: Ahbpsc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ahbpsc {
    type Ux = u8;
}
#[doc = "Field `AHBPSC` reader - AHB prescaler selection"]
pub type AhbpscR = crate::FieldReader<Ahbpsc>;
impl AhbpscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ahbpsc> {
        match self.bits {
            0 => Some(Ahbpsc::Div1),
            8 => Some(Ahbpsc::Div2),
            9 => Some(Ahbpsc::Div4),
            10 => Some(Ahbpsc::Div8),
            11 => Some(Ahbpsc::Div16),
            12 => Some(Ahbpsc::Div64),
            13 => Some(Ahbpsc::Div128),
            14 => Some(Ahbpsc::Div256),
            15 => Some(Ahbpsc::Div512),
            _ => None,
        }
    }
    #[doc = "CK_SYS"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Ahbpsc::Div1
    }
    #[doc = "CK_SYS divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Ahbpsc::Div2
    }
    #[doc = "CK_SYS divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Ahbpsc::Div4
    }
    #[doc = "CK_SYS divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Ahbpsc::Div8
    }
    #[doc = "CK_SYS divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Ahbpsc::Div16
    }
    #[doc = "CK_SYS divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Ahbpsc::Div64
    }
    #[doc = "CK_SYS divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Ahbpsc::Div128
    }
    #[doc = "CK_SYS divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Ahbpsc::Div256
    }
    #[doc = "CK_SYS divided by 512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == Ahbpsc::Div512
    }
}
#[doc = "Field `AHBPSC` writer - AHB prescaler selection"]
pub type AhbpscW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ahbpsc>;
impl<'a, REG> AhbpscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CK_SYS"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbpsc::Div1)
    }
    #[doc = "CK_SYS divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbpsc::Div2)
    }
    #[doc = "CK_SYS divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbpsc::Div4)
    }
    #[doc = "CK_SYS divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbpsc::Div8)
    }
    #[doc = "CK_SYS divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbpsc::Div16)
    }
    #[doc = "CK_SYS divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbpsc::Div64)
    }
    #[doc = "CK_SYS divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbpsc::Div128)
    }
    #[doc = "CK_SYS divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbpsc::Div256)
    }
    #[doc = "CK_SYS divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbpsc::Div512)
    }
}
#[doc = "APB1 prescaler selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Apb1psc {
    #[doc = "0: CK_AHB"]
    Div1 = 0,
    #[doc = "4: CK_AHB divided by 2"]
    Div2 = 4,
    #[doc = "5: CK_AHB divided by 4"]
    Div4 = 5,
    #[doc = "6: CK_AHB divided by 8"]
    Div8 = 6,
    #[doc = "7: CK_AHB divided by 16"]
    Div16 = 7,
}
impl From<Apb1psc> for u8 {
    #[inline(always)]
    fn from(variant: Apb1psc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Apb1psc {
    type Ux = u8;
}
#[doc = "Field `APB1PSC` reader - APB1 prescaler selection"]
pub type Apb1pscR = crate::FieldReader<Apb1psc>;
impl Apb1pscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Apb1psc> {
        match self.bits {
            0 => Some(Apb1psc::Div1),
            4 => Some(Apb1psc::Div2),
            5 => Some(Apb1psc::Div4),
            6 => Some(Apb1psc::Div8),
            7 => Some(Apb1psc::Div16),
            _ => None,
        }
    }
    #[doc = "CK_AHB"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Apb1psc::Div1
    }
    #[doc = "CK_AHB divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Apb1psc::Div2
    }
    #[doc = "CK_AHB divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Apb1psc::Div4
    }
    #[doc = "CK_AHB divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Apb1psc::Div8
    }
    #[doc = "CK_AHB divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Apb1psc::Div16
    }
}
#[doc = "Field `APB1PSC` writer - APB1 prescaler selection"]
pub type Apb1pscW<'a, REG> = crate::FieldWriter<'a, REG, 3, Apb1psc>;
impl<'a, REG> Apb1pscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CK_AHB"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Apb1psc::Div1)
    }
    #[doc = "CK_AHB divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Apb1psc::Div2)
    }
    #[doc = "CK_AHB divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Apb1psc::Div4)
    }
    #[doc = "CK_AHB divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Apb1psc::Div8)
    }
    #[doc = "CK_AHB divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Apb1psc::Div16)
    }
}
#[doc = "Field `APB2PSC` reader - APB2 prescaler selection"]
pub use Apb1pscR as Apb2pscR;
#[doc = "Field `APB2PSC` writer - APB2 prescaler selection"]
pub use Apb1pscW as Apb2pscW;
#[doc = "ADC clock prescaler selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adcpsc {
    #[doc = "0: CK_APB2 divided by 2"]
    Div2 = 0,
    #[doc = "1: CK_APB2 divided by 4"]
    Div4 = 1,
    #[doc = "2: CK_APB2 divided by 6"]
    Div6 = 2,
    #[doc = "3: CK_APB2 divided by 8"]
    Div8 = 3,
}
impl From<Adcpsc> for u8 {
    #[inline(always)]
    fn from(variant: Adcpsc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adcpsc {
    type Ux = u8;
}
#[doc = "Field `ADCPSC` reader - ADC clock prescaler selection"]
pub type AdcpscR = crate::FieldReader<Adcpsc>;
impl AdcpscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Adcpsc> {
        match self.bits {
            0 => Some(Adcpsc::Div2),
            1 => Some(Adcpsc::Div4),
            2 => Some(Adcpsc::Div6),
            3 => Some(Adcpsc::Div8),
            _ => None,
        }
    }
    #[doc = "CK_APB2 divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Adcpsc::Div2
    }
    #[doc = "CK_APB2 divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Adcpsc::Div4
    }
    #[doc = "CK_APB2 divided by 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == Adcpsc::Div6
    }
    #[doc = "CK_APB2 divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Adcpsc::Div8
    }
}
#[doc = "Field `ADCPSC` writer - ADC clock prescaler selection"]
pub type AdcpscW<'a, REG> = crate::FieldWriter<'a, REG, 3, Adcpsc>;
impl<'a, REG> AdcpscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CK_APB2 divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpsc::Div2)
    }
    #[doc = "CK_APB2 divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpsc::Div4)
    }
    #[doc = "CK_APB2 divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpsc::Div6)
    }
    #[doc = "CK_APB2 divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpsc::Div8)
    }
}
#[doc = "PLL Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllsel {
    #[doc = "0: IRC8M / 2 selected as PLL source clock"]
    Irc8m2 = 0,
    #[doc = "1: HXTAL selected as PLL source clock"]
    Hxtal = 1,
}
impl From<Pllsel> for bool {
    #[inline(always)]
    fn from(variant: Pllsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSEL` reader - PLL Clock Source Selection"]
pub type PllselR = crate::BitReader<Pllsel>;
impl PllselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllsel {
        match self.bits {
            false => Pllsel::Irc8m2,
            true => Pllsel::Hxtal,
        }
    }
    #[doc = "IRC8M / 2 selected as PLL source clock"]
    #[inline(always)]
    pub fn is_irc8m_2(&self) -> bool {
        *self == Pllsel::Irc8m2
    }
    #[doc = "HXTAL selected as PLL source clock"]
    #[inline(always)]
    pub fn is_hxtal(&self) -> bool {
        *self == Pllsel::Hxtal
    }
}
#[doc = "Field `PLLSEL` writer - PLL Clock Source Selection"]
pub type PllselW<'a, REG> = crate::BitWriter<'a, REG, Pllsel>;
impl<'a, REG> PllselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IRC8M / 2 selected as PLL source clock"]
    #[inline(always)]
    pub fn irc8m_2(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsel::Irc8m2)
    }
    #[doc = "HXTAL selected as PLL source clock"]
    #[inline(always)]
    pub fn hxtal(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsel::Hxtal)
    }
}
#[doc = "The LSB of PREDV0 division factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllpredv {
    #[doc = "0: HXTAL clock not divided"]
    Div1 = 0,
    #[doc = "1: HXTAL clock divided by 2"]
    Div2 = 1,
}
impl From<Pllpredv> for bool {
    #[inline(always)]
    fn from(variant: Pllpredv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLPREDV` reader - The LSB of PREDV0 division factor"]
pub type PllpredvR = crate::BitReader<Pllpredv>;
impl PllpredvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllpredv {
        match self.bits {
            false => Pllpredv::Div1,
            true => Pllpredv::Div2,
        }
    }
    #[doc = "HXTAL clock not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Pllpredv::Div1
    }
    #[doc = "HXTAL clock divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Pllpredv::Div2
    }
}
#[doc = "Field `PLLPREDV` writer - The LSB of PREDV0 division factor"]
pub type PllpredvW<'a, REG> = crate::BitWriter<'a, REG, Pllpredv>;
impl<'a, REG> PllpredvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HXTAL clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllpredv::Div1)
    }
    #[doc = "HXTAL clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Pllpredv::Div2)
    }
}
#[doc = "The PLL clock multiplication factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pllmf {
    #[doc = "0: PLL input clock x2"]
    Mul2 = 0,
    #[doc = "1: PLL input clock x3"]
    Mul3 = 1,
    #[doc = "2: PLL input clock x4"]
    Mul4 = 2,
    #[doc = "3: PLL input clock x5"]
    Mul5 = 3,
    #[doc = "4: PLL input clock x6"]
    Mul6 = 4,
    #[doc = "5: PLL input clock x7"]
    Mul7 = 5,
    #[doc = "6: PLL input clock x8"]
    Mul8 = 6,
    #[doc = "7: PLL input clock x9"]
    Mul9 = 7,
    #[doc = "8: PLL input clock x10"]
    Mul10 = 8,
    #[doc = "9: PLL input clock x11"]
    Mul11 = 9,
    #[doc = "10: PLL input clock x12"]
    Mul12 = 10,
    #[doc = "11: PLL input clock x13"]
    Mul13 = 11,
    #[doc = "12: PLL input clock x14"]
    Mul14 = 12,
    #[doc = "13: PLL input clock x15"]
    Mul15 = 13,
    #[doc = "14: PLL input clock x16"]
    Mul16 = 14,
    #[doc = "15: PLL input clock x16"]
    Mul16x = 15,
}
impl From<Pllmf> for u8 {
    #[inline(always)]
    fn from(variant: Pllmf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pllmf {
    type Ux = u8;
}
#[doc = "Field `PLLMF` reader - The PLL clock multiplication factor"]
pub type PllmfR = crate::FieldReader<Pllmf>;
impl PllmfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllmf {
        match self.bits {
            0 => Pllmf::Mul2,
            1 => Pllmf::Mul3,
            2 => Pllmf::Mul4,
            3 => Pllmf::Mul5,
            4 => Pllmf::Mul6,
            5 => Pllmf::Mul7,
            6 => Pllmf::Mul8,
            7 => Pllmf::Mul9,
            8 => Pllmf::Mul10,
            9 => Pllmf::Mul11,
            10 => Pllmf::Mul12,
            11 => Pllmf::Mul13,
            12 => Pllmf::Mul14,
            13 => Pllmf::Mul15,
            14 => Pllmf::Mul16,
            15 => Pllmf::Mul16x,
            _ => unreachable!(),
        }
    }
    #[doc = "PLL input clock x2"]
    #[inline(always)]
    pub fn is_mul2(&self) -> bool {
        *self == Pllmf::Mul2
    }
    #[doc = "PLL input clock x3"]
    #[inline(always)]
    pub fn is_mul3(&self) -> bool {
        *self == Pllmf::Mul3
    }
    #[doc = "PLL input clock x4"]
    #[inline(always)]
    pub fn is_mul4(&self) -> bool {
        *self == Pllmf::Mul4
    }
    #[doc = "PLL input clock x5"]
    #[inline(always)]
    pub fn is_mul5(&self) -> bool {
        *self == Pllmf::Mul5
    }
    #[doc = "PLL input clock x6"]
    #[inline(always)]
    pub fn is_mul6(&self) -> bool {
        *self == Pllmf::Mul6
    }
    #[doc = "PLL input clock x7"]
    #[inline(always)]
    pub fn is_mul7(&self) -> bool {
        *self == Pllmf::Mul7
    }
    #[doc = "PLL input clock x8"]
    #[inline(always)]
    pub fn is_mul8(&self) -> bool {
        *self == Pllmf::Mul8
    }
    #[doc = "PLL input clock x9"]
    #[inline(always)]
    pub fn is_mul9(&self) -> bool {
        *self == Pllmf::Mul9
    }
    #[doc = "PLL input clock x10"]
    #[inline(always)]
    pub fn is_mul10(&self) -> bool {
        *self == Pllmf::Mul10
    }
    #[doc = "PLL input clock x11"]
    #[inline(always)]
    pub fn is_mul11(&self) -> bool {
        *self == Pllmf::Mul11
    }
    #[doc = "PLL input clock x12"]
    #[inline(always)]
    pub fn is_mul12(&self) -> bool {
        *self == Pllmf::Mul12
    }
    #[doc = "PLL input clock x13"]
    #[inline(always)]
    pub fn is_mul13(&self) -> bool {
        *self == Pllmf::Mul13
    }
    #[doc = "PLL input clock x14"]
    #[inline(always)]
    pub fn is_mul14(&self) -> bool {
        *self == Pllmf::Mul14
    }
    #[doc = "PLL input clock x15"]
    #[inline(always)]
    pub fn is_mul15(&self) -> bool {
        *self == Pllmf::Mul15
    }
    #[doc = "PLL input clock x16"]
    #[inline(always)]
    pub fn is_mul16(&self) -> bool {
        *self == Pllmf::Mul16
    }
    #[doc = "PLL input clock x16"]
    #[inline(always)]
    pub fn is_mul16x(&self) -> bool {
        *self == Pllmf::Mul16x
    }
}
#[doc = "Field `PLLMF` writer - The PLL clock multiplication factor"]
pub type PllmfW<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, Pllmf>;
impl<'a, REG> PllmfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLL input clock x2"]
    #[inline(always)]
    pub fn mul2(self) -> &'a mut crate::W<REG> {
        self.variant(Pllmf::Mul2)
    }
    #[doc = "PLL input clock x3"]
    #[inline(always)]
    pub fn mul3(self) -> &'a mut crate::W<REG> {
        self.variant(Pllmf::Mul3)
    }
    #[doc = "PLL input clock x4"]
    #[inline(always)]
    pub fn mul4(self) -> &'a mut crate::W<REG> {
        self.variant(Pllmf::Mul4)
    }
    #[doc = "PLL input clock x5"]
    #[inline(always)]
    pub fn mul5(self) -> &'a mut crate::W<REG> {
        self.variant(Pllmf::Mul5)
    }
    #[doc = "PLL input clock x6"]
    #[inline(always)]
    pub fn mul6(self) -> &'a mut crate::W<REG> {
        self.variant(Pllmf::Mul6)
    }
    #[doc = "PLL input clock x7"]
    #[inline(always)]
    pub fn mul7(self) -> &'a mut crate::W<REG> {
        self.variant(Pllmf::Mul7)
    }
    #[doc = "PLL input clock x8"]
    #[inline(always)]
    pub fn mul8(self) -> &'a mut crate::W<REG> {
        self.variant(Pllmf::Mul8)
    }
    #[doc = "PLL input clock x9"]
    #[inline(always)]
    pub fn mul9(self) -> &'a mut crate::W<REG> {
        self.variant(Pllmf::Mul9)
    }
    #[doc = "PLL input clock x10"]
    #[inline(always)]
    pub fn mul10(self) -> &'a mut crate::W<REG> {
        self.variant(Pllmf::Mul10)
    }
    #[doc = "PLL input clock x11"]
    #[inline(always)]
    pub fn mul11(self) -> &'a mut crate::W<REG> {
        self.variant(Pllmf::Mul11)
    }
    #[doc = "PLL input clock x12"]
    #[inline(always)]
    pub fn mul12(self) -> &'a mut crate::W<REG> {
        self.variant(Pllmf::Mul12)
    }
    #[doc = "PLL input clock x13"]
    #[inline(always)]
    pub fn mul13(self) -> &'a mut crate::W<REG> {
        self.variant(Pllmf::Mul13)
    }
    #[doc = "PLL input clock x14"]
    #[inline(always)]
    pub fn mul14(self) -> &'a mut crate::W<REG> {
        self.variant(Pllmf::Mul14)
    }
    #[doc = "PLL input clock x15"]
    #[inline(always)]
    pub fn mul15(self) -> &'a mut crate::W<REG> {
        self.variant(Pllmf::Mul15)
    }
    #[doc = "PLL input clock x16"]
    #[inline(always)]
    pub fn mul16(self) -> &'a mut crate::W<REG> {
        self.variant(Pllmf::Mul16)
    }
    #[doc = "PLL input clock x16"]
    #[inline(always)]
    pub fn mul16x(self) -> &'a mut crate::W<REG> {
        self.variant(Pllmf::Mul16x)
    }
}
#[doc = "USBFS clock prescaler selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usbfspsc {
    #[doc = "0: PLL clock is divided by 1.5"]
    Div1_5 = 0,
    #[doc = "1: PLL clock is not divided"]
    Div1 = 1,
    #[doc = "2: PLL clock is divided by 2.5"]
    Div2_5 = 2,
    #[doc = "3: PLL clock is divided by 2"]
    Div2 = 3,
}
impl From<Usbfspsc> for u8 {
    #[inline(always)]
    fn from(variant: Usbfspsc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usbfspsc {
    type Ux = u8;
}
#[doc = "Field `USBFSPSC` reader - USBFS clock prescaler selection"]
pub type UsbfspscR = crate::FieldReader<Usbfspsc>;
impl UsbfspscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbfspsc {
        match self.bits {
            0 => Usbfspsc::Div1_5,
            1 => Usbfspsc::Div1,
            2 => Usbfspsc::Div2_5,
            3 => Usbfspsc::Div2,
            _ => unreachable!(),
        }
    }
    #[doc = "PLL clock is divided by 1.5"]
    #[inline(always)]
    pub fn is_div1_5(&self) -> bool {
        *self == Usbfspsc::Div1_5
    }
    #[doc = "PLL clock is not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Usbfspsc::Div1
    }
    #[doc = "PLL clock is divided by 2.5"]
    #[inline(always)]
    pub fn is_div2_5(&self) -> bool {
        *self == Usbfspsc::Div2_5
    }
    #[doc = "PLL clock is divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Usbfspsc::Div2
    }
}
#[doc = "Field `USBFSPSC` writer - USBFS clock prescaler selection"]
pub type UsbfspscW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Usbfspsc>;
impl<'a, REG> UsbfspscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLL clock is divided by 1.5"]
    #[inline(always)]
    pub fn div1_5(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfspsc::Div1_5)
    }
    #[doc = "PLL clock is not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfspsc::Div1)
    }
    #[doc = "PLL clock is divided by 2.5"]
    #[inline(always)]
    pub fn div2_5(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfspsc::Div2_5)
    }
    #[doc = "PLL clock is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfspsc::Div2)
    }
}
#[doc = "CKOUT0 Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckoutsel {
    #[doc = "0: No clock selected"]
    None = 0,
    #[doc = "1: Internal 14 MHz RC oscillator clock selected"]
    Irc14m = 1,
    #[doc = "2: Internal 40 kHz RC oscillator clock selected"]
    Lsi40k = 2,
    #[doc = "3: External low speed oscillator clock selected"]
    Lxtal = 3,
    #[doc = "4: System clock selected"]
    Sysclk = 4,
    #[doc = "5: Internal RC 8 MHz (HSI) oscillator clock selected"]
    Irc8m = 5,
    #[doc = "6: External 4-32 MHz (HSE) oscillator clock selected"]
    Hxtal = 6,
    #[doc = "7: PLL clock selected (divided by 1 or 2, depending on PLLDV)"]
    Pll = 7,
}
impl From<Ckoutsel> for u8 {
    #[inline(always)]
    fn from(variant: Ckoutsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckoutsel {
    type Ux = u8;
}
#[doc = "Field `CKOUTSEL` reader - CKOUT0 Clock Source Selection"]
pub type CkoutselR = crate::FieldReader<Ckoutsel>;
impl CkoutselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ckoutsel> {
        match self.bits {
            0 => Some(Ckoutsel::None),
            1 => Some(Ckoutsel::Irc14m),
            2 => Some(Ckoutsel::Lsi40k),
            3 => Some(Ckoutsel::Lxtal),
            4 => Some(Ckoutsel::Sysclk),
            5 => Some(Ckoutsel::Irc8m),
            6 => Some(Ckoutsel::Hxtal),
            7 => Some(Ckoutsel::Pll),
            _ => None,
        }
    }
    #[doc = "No clock selected"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Ckoutsel::None
    }
    #[doc = "Internal 14 MHz RC oscillator clock selected"]
    #[inline(always)]
    pub fn is_irc14m(&self) -> bool {
        *self == Ckoutsel::Irc14m
    }
    #[doc = "Internal 40 kHz RC oscillator clock selected"]
    #[inline(always)]
    pub fn is_lsi40k(&self) -> bool {
        *self == Ckoutsel::Lsi40k
    }
    #[doc = "External low speed oscillator clock selected"]
    #[inline(always)]
    pub fn is_lxtal(&self) -> bool {
        *self == Ckoutsel::Lxtal
    }
    #[doc = "System clock selected"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == Ckoutsel::Sysclk
    }
    #[doc = "Internal RC 8 MHz (HSI) oscillator clock selected"]
    #[inline(always)]
    pub fn is_irc8m(&self) -> bool {
        *self == Ckoutsel::Irc8m
    }
    #[doc = "External 4-32 MHz (HSE) oscillator clock selected"]
    #[inline(always)]
    pub fn is_hxtal(&self) -> bool {
        *self == Ckoutsel::Hxtal
    }
    #[doc = "PLL clock selected (divided by 1 or 2, depending on PLLDV)"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == Ckoutsel::Pll
    }
}
#[doc = "Field `CKOUTSEL` writer - CKOUT0 Clock Source Selection"]
pub type CkoutselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ckoutsel>;
impl<'a, REG> CkoutselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock selected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Ckoutsel::None)
    }
    #[doc = "Internal 14 MHz RC oscillator clock selected"]
    #[inline(always)]
    pub fn irc14m(self) -> &'a mut crate::W<REG> {
        self.variant(Ckoutsel::Irc14m)
    }
    #[doc = "Internal 40 kHz RC oscillator clock selected"]
    #[inline(always)]
    pub fn lsi40k(self) -> &'a mut crate::W<REG> {
        self.variant(Ckoutsel::Lsi40k)
    }
    #[doc = "External low speed oscillator clock selected"]
    #[inline(always)]
    pub fn lxtal(self) -> &'a mut crate::W<REG> {
        self.variant(Ckoutsel::Lxtal)
    }
    #[doc = "System clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(Ckoutsel::Sysclk)
    }
    #[doc = "Internal RC 8 MHz (HSI) oscillator clock selected"]
    #[inline(always)]
    pub fn irc8m(self) -> &'a mut crate::W<REG> {
        self.variant(Ckoutsel::Irc8m)
    }
    #[doc = "External 4-32 MHz (HSE) oscillator clock selected"]
    #[inline(always)]
    pub fn hxtal(self) -> &'a mut crate::W<REG> {
        self.variant(Ckoutsel::Hxtal)
    }
    #[doc = "PLL clock selected (divided by 1 or 2, depending on PLLDV)"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(Ckoutsel::Pll)
    }
}
#[doc = "Bit 5 and Bit 4 of PLLMF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PllmfMsb {
    #[doc = "0: Value of PLLMF is as set"]
    None = 0,
    #[doc = "1: Add 15 to the value of PLLMF"]
    Plus15 = 1,
}
impl From<PllmfMsb> for u8 {
    #[inline(always)]
    fn from(variant: PllmfMsb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PllmfMsb {
    type Ux = u8;
}
#[doc = "Field `PLLMF_MSB` reader - Bit 5 and Bit 4 of PLLMF"]
pub type PllmfMsbR = crate::FieldReader<PllmfMsb>;
impl PllmfMsbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PllmfMsb> {
        match self.bits {
            0 => Some(PllmfMsb::None),
            1 => Some(PllmfMsb::Plus15),
            _ => None,
        }
    }
    #[doc = "Value of PLLMF is as set"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PllmfMsb::None
    }
    #[doc = "Add 15 to the value of PLLMF"]
    #[inline(always)]
    pub fn is_plus15(&self) -> bool {
        *self == PllmfMsb::Plus15
    }
}
#[doc = "Field `PLLMF_MSB` writer - Bit 5 and Bit 4 of PLLMF"]
pub type PllmfMsbW<'a, REG> = crate::FieldWriter<'a, REG, 2, PllmfMsb>;
impl<'a, REG> PllmfMsbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Value of PLLMF is as set"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PllmfMsb::None)
    }
    #[doc = "Add 15 to the value of PLLMF"]
    #[inline(always)]
    pub fn plus15(self) -> &'a mut crate::W<REG> {
        self.variant(PllmfMsb::Plus15)
    }
}
#[doc = "Field `USBFSPSC_3` reader - Bit 2 of USBFSPSC"]
pub type Usbfspsc3R = crate::BitReader;
#[doc = "Field `USBFSPSC_3` writer - Bit 2 of USBFSPSC"]
pub type Usbfspsc3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn scs(&self) -> ScsR {
        ScsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - System clock switch status"]
    #[inline(always)]
    pub fn scss(&self) -> ScssR {
        ScssR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB prescaler selection"]
    #[inline(always)]
    pub fn ahbpsc(&self) -> AhbpscR {
        AhbpscR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - APB1 prescaler selection"]
    #[inline(always)]
    pub fn apb1psc(&self) -> Apb1pscR {
        Apb1pscR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - APB2 prescaler selection"]
    #[inline(always)]
    pub fn apb2psc(&self) -> Apb2pscR {
        Apb2pscR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - ADC clock prescaler selection"]
    #[inline(always)]
    pub fn adcpsc(&self) -> AdcpscR {
        AdcpscR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 16 - PLL Clock Source Selection"]
    #[inline(always)]
    pub fn pllsel(&self) -> PllselR {
        PllselR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The LSB of PREDV0 division factor"]
    #[inline(always)]
    pub fn pllpredv(&self) -> PllpredvR {
        PllpredvR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - The PLL clock multiplication factor"]
    #[inline(always)]
    pub fn pllmf(&self) -> PllmfR {
        PllmfR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - USBFS clock prescaler selection"]
    #[inline(always)]
    pub fn usbfspsc(&self) -> UsbfspscR {
        UsbfspscR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - CKOUT0 Clock Source Selection"]
    #[inline(always)]
    pub fn ckoutsel(&self) -> CkoutselR {
        CkoutselR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 29:30 - Bit 5 and Bit 4 of PLLMF"]
    #[inline(always)]
    pub fn pllmf_msb(&self) -> PllmfMsbR {
        PllmfMsbR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Bit 2 of USBFSPSC"]
    #[inline(always)]
    pub fn usbfspsc_3(&self) -> Usbfspsc3R {
        Usbfspsc3R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    #[must_use]
    pub fn scs(&mut self) -> ScsW<Cfg0Spec> {
        ScsW::new(self, 0)
    }
    #[doc = "Bits 4:7 - AHB prescaler selection"]
    #[inline(always)]
    #[must_use]
    pub fn ahbpsc(&mut self) -> AhbpscW<Cfg0Spec> {
        AhbpscW::new(self, 4)
    }
    #[doc = "Bits 8:10 - APB1 prescaler selection"]
    #[inline(always)]
    #[must_use]
    pub fn apb1psc(&mut self) -> Apb1pscW<Cfg0Spec> {
        Apb1pscW::new(self, 8)
    }
    #[doc = "Bits 11:13 - APB2 prescaler selection"]
    #[inline(always)]
    #[must_use]
    pub fn apb2psc(&mut self) -> Apb2pscW<Cfg0Spec> {
        Apb2pscW::new(self, 11)
    }
    #[doc = "Bits 14:16 - ADC clock prescaler selection"]
    #[inline(always)]
    #[must_use]
    pub fn adcpsc(&mut self) -> AdcpscW<Cfg0Spec> {
        AdcpscW::new(self, 14)
    }
    #[doc = "Bit 16 - PLL Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn pllsel(&mut self) -> PllselW<Cfg0Spec> {
        PllselW::new(self, 16)
    }
    #[doc = "Bit 17 - The LSB of PREDV0 division factor"]
    #[inline(always)]
    #[must_use]
    pub fn pllpredv(&mut self) -> PllpredvW<Cfg0Spec> {
        PllpredvW::new(self, 17)
    }
    #[doc = "Bits 18:21 - The PLL clock multiplication factor"]
    #[inline(always)]
    #[must_use]
    pub fn pllmf(&mut self) -> PllmfW<Cfg0Spec> {
        PllmfW::new(self, 18)
    }
    #[doc = "Bits 22:23 - USBFS clock prescaler selection"]
    #[inline(always)]
    #[must_use]
    pub fn usbfspsc(&mut self) -> UsbfspscW<Cfg0Spec> {
        UsbfspscW::new(self, 22)
    }
    #[doc = "Bits 24:27 - CKOUT0 Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckoutsel(&mut self) -> CkoutselW<Cfg0Spec> {
        CkoutselW::new(self, 24)
    }
    #[doc = "Bits 29:30 - Bit 5 and Bit 4 of PLLMF"]
    #[inline(always)]
    #[must_use]
    pub fn pllmf_msb(&mut self) -> PllmfMsbW<Cfg0Spec> {
        PllmfMsbW::new(self, 29)
    }
    #[doc = "Bit 31 - Bit 2 of USBFSPSC"]
    #[inline(always)]
    #[must_use]
    pub fn usbfspsc_3(&mut self) -> Usbfspsc3W<Cfg0Spec> {
        Usbfspsc3W::new(self, 31)
    }
}
#[doc = "Clock configuration register 0 (RCU_CFG0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Spec;
impl crate::RegisterSpec for Cfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0::R`](R) reader structure"]
impl crate::Readable for Cfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0::W`](W) writer structure"]
impl crate::Writable for Cfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0 to value 0"]
impl crate::Resettable for Cfg0Spec {
    const RESET_VALUE: u32 = 0;
}
