#[doc = "Register `IVREF_CTL` reader"]
pub type R = crate::R<IvrefCtlSpec>;
#[doc = "Register `IVREF_CTL` writer"]
pub type W = crate::W<IvrefCtlSpec>;
#[doc = "Field `CSDT` reader - Current step data"]
pub type CsdtR = crate::FieldReader;
#[doc = "Field `CSDT` writer - Current step data"]
pub type CsdtW<'a, REG> = crate::FieldWriterSafe<'a, REG, 6>;
#[doc = "Sink current mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scmod {
    #[doc = "0: Source current"]
    Source = 0,
    #[doc = "1: Sink current"]
    Sink = 1,
}
impl From<Scmod> for bool {
    #[inline(always)]
    fn from(variant: Scmod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCMOD` reader - Sink current mode"]
pub type ScmodR = crate::BitReader<Scmod>;
impl ScmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scmod {
        match self.bits {
            false => Scmod::Source,
            true => Scmod::Sink,
        }
    }
    #[doc = "Source current"]
    #[inline(always)]
    pub fn is_source(&self) -> bool {
        *self == Scmod::Source
    }
    #[doc = "Sink current"]
    #[inline(always)]
    pub fn is_sink(&self) -> bool {
        *self == Scmod::Sink
    }
}
#[doc = "Field `SCMOD` writer - Sink current mode"]
pub type ScmodW<'a, REG> = crate::BitWriter<'a, REG, Scmod>;
impl<'a, REG> ScmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Source current"]
    #[inline(always)]
    pub fn source(self) -> &'a mut crate::W<REG> {
        self.variant(Scmod::Source)
    }
    #[doc = "Sink current"]
    #[inline(always)]
    pub fn sink(self) -> &'a mut crate::W<REG> {
        self.variant(Scmod::Sink)
    }
}
#[doc = "Current precision trim\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpt {
    #[doc = "0: Trim -15%"]
    Minus15 = 0,
    #[doc = "1: Trim -14%"]
    Minus14 = 1,
    #[doc = "2: Trim -13%"]
    Minus13 = 2,
    #[doc = "3: Trim -12%"]
    Minus12 = 3,
    #[doc = "4: Trim -11%"]
    Minus11 = 4,
    #[doc = "5: Trim -10%"]
    Minus10 = 5,
    #[doc = "6: Trim -9%"]
    Minus9 = 6,
    #[doc = "7: Trim -8%"]
    Minus8 = 7,
    #[doc = "8: Trim -7%"]
    Minus7 = 8,
    #[doc = "9: Trim -6%"]
    Minus6 = 9,
    #[doc = "10: Trim -5%"]
    Minus5 = 10,
    #[doc = "11: Trim -4%"]
    Minus4 = 11,
    #[doc = "12: Trim -3%"]
    Minus3 = 12,
    #[doc = "13: Trim -2%"]
    Minus2 = 13,
    #[doc = "14: Trim -1%"]
    Minus1 = 14,
    #[doc = "15: Trim 0%"]
    Zero = 15,
    #[doc = "16: Trim +1%"]
    Plus1 = 16,
    #[doc = "17: Trim +2%"]
    Plus2 = 17,
    #[doc = "18: Trim +3%"]
    Plus3 = 18,
    #[doc = "19: Trim +4%"]
    Plus4 = 19,
    #[doc = "20: Trim +5%"]
    Plus5 = 20,
    #[doc = "21: Trim +6%"]
    Plus6 = 21,
    #[doc = "22: Trim +7%"]
    Plus7 = 22,
    #[doc = "23: Trim +8%"]
    Plus8 = 23,
    #[doc = "24: Trim +9%"]
    Plus9 = 24,
    #[doc = "25: Trim +10%"]
    Plus10 = 25,
    #[doc = "26: Trim +11%"]
    Plus11 = 26,
    #[doc = "27: Trim +12%"]
    Plus12 = 27,
    #[doc = "28: Trim +13%"]
    Plus13 = 28,
    #[doc = "29: Trim +14%"]
    Plus14 = 29,
    #[doc = "30: Trim +15%"]
    Plus15 = 30,
    #[doc = "31: Trim +16%"]
    Plus16 = 31,
}
impl From<Cpt> for u8 {
    #[inline(always)]
    fn from(variant: Cpt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpt {
    type Ux = u8;
}
#[doc = "Field `CPT` reader - Current precision trim"]
pub type CptR = crate::FieldReader<Cpt>;
impl CptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpt {
        match self.bits {
            0 => Cpt::Minus15,
            1 => Cpt::Minus14,
            2 => Cpt::Minus13,
            3 => Cpt::Minus12,
            4 => Cpt::Minus11,
            5 => Cpt::Minus10,
            6 => Cpt::Minus9,
            7 => Cpt::Minus8,
            8 => Cpt::Minus7,
            9 => Cpt::Minus6,
            10 => Cpt::Minus5,
            11 => Cpt::Minus4,
            12 => Cpt::Minus3,
            13 => Cpt::Minus2,
            14 => Cpt::Minus1,
            15 => Cpt::Zero,
            16 => Cpt::Plus1,
            17 => Cpt::Plus2,
            18 => Cpt::Plus3,
            19 => Cpt::Plus4,
            20 => Cpt::Plus5,
            21 => Cpt::Plus6,
            22 => Cpt::Plus7,
            23 => Cpt::Plus8,
            24 => Cpt::Plus9,
            25 => Cpt::Plus10,
            26 => Cpt::Plus11,
            27 => Cpt::Plus12,
            28 => Cpt::Plus13,
            29 => Cpt::Plus14,
            30 => Cpt::Plus15,
            31 => Cpt::Plus16,
            _ => unreachable!(),
        }
    }
    #[doc = "Trim -15%"]
    #[inline(always)]
    pub fn is_minus15(&self) -> bool {
        *self == Cpt::Minus15
    }
    #[doc = "Trim -14%"]
    #[inline(always)]
    pub fn is_minus14(&self) -> bool {
        *self == Cpt::Minus14
    }
    #[doc = "Trim -13%"]
    #[inline(always)]
    pub fn is_minus13(&self) -> bool {
        *self == Cpt::Minus13
    }
    #[doc = "Trim -12%"]
    #[inline(always)]
    pub fn is_minus12(&self) -> bool {
        *self == Cpt::Minus12
    }
    #[doc = "Trim -11%"]
    #[inline(always)]
    pub fn is_minus11(&self) -> bool {
        *self == Cpt::Minus11
    }
    #[doc = "Trim -10%"]
    #[inline(always)]
    pub fn is_minus10(&self) -> bool {
        *self == Cpt::Minus10
    }
    #[doc = "Trim -9%"]
    #[inline(always)]
    pub fn is_minus9(&self) -> bool {
        *self == Cpt::Minus9
    }
    #[doc = "Trim -8%"]
    #[inline(always)]
    pub fn is_minus8(&self) -> bool {
        *self == Cpt::Minus8
    }
    #[doc = "Trim -7%"]
    #[inline(always)]
    pub fn is_minus7(&self) -> bool {
        *self == Cpt::Minus7
    }
    #[doc = "Trim -6%"]
    #[inline(always)]
    pub fn is_minus6(&self) -> bool {
        *self == Cpt::Minus6
    }
    #[doc = "Trim -5%"]
    #[inline(always)]
    pub fn is_minus5(&self) -> bool {
        *self == Cpt::Minus5
    }
    #[doc = "Trim -4%"]
    #[inline(always)]
    pub fn is_minus4(&self) -> bool {
        *self == Cpt::Minus4
    }
    #[doc = "Trim -3%"]
    #[inline(always)]
    pub fn is_minus3(&self) -> bool {
        *self == Cpt::Minus3
    }
    #[doc = "Trim -2%"]
    #[inline(always)]
    pub fn is_minus2(&self) -> bool {
        *self == Cpt::Minus2
    }
    #[doc = "Trim -1%"]
    #[inline(always)]
    pub fn is_minus1(&self) -> bool {
        *self == Cpt::Minus1
    }
    #[doc = "Trim 0%"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Cpt::Zero
    }
    #[doc = "Trim +1%"]
    #[inline(always)]
    pub fn is_plus1(&self) -> bool {
        *self == Cpt::Plus1
    }
    #[doc = "Trim +2%"]
    #[inline(always)]
    pub fn is_plus2(&self) -> bool {
        *self == Cpt::Plus2
    }
    #[doc = "Trim +3%"]
    #[inline(always)]
    pub fn is_plus3(&self) -> bool {
        *self == Cpt::Plus3
    }
    #[doc = "Trim +4%"]
    #[inline(always)]
    pub fn is_plus4(&self) -> bool {
        *self == Cpt::Plus4
    }
    #[doc = "Trim +5%"]
    #[inline(always)]
    pub fn is_plus5(&self) -> bool {
        *self == Cpt::Plus5
    }
    #[doc = "Trim +6%"]
    #[inline(always)]
    pub fn is_plus6(&self) -> bool {
        *self == Cpt::Plus6
    }
    #[doc = "Trim +7%"]
    #[inline(always)]
    pub fn is_plus7(&self) -> bool {
        *self == Cpt::Plus7
    }
    #[doc = "Trim +8%"]
    #[inline(always)]
    pub fn is_plus8(&self) -> bool {
        *self == Cpt::Plus8
    }
    #[doc = "Trim +9%"]
    #[inline(always)]
    pub fn is_plus9(&self) -> bool {
        *self == Cpt::Plus9
    }
    #[doc = "Trim +10%"]
    #[inline(always)]
    pub fn is_plus10(&self) -> bool {
        *self == Cpt::Plus10
    }
    #[doc = "Trim +11%"]
    #[inline(always)]
    pub fn is_plus11(&self) -> bool {
        *self == Cpt::Plus11
    }
    #[doc = "Trim +12%"]
    #[inline(always)]
    pub fn is_plus12(&self) -> bool {
        *self == Cpt::Plus12
    }
    #[doc = "Trim +13%"]
    #[inline(always)]
    pub fn is_plus13(&self) -> bool {
        *self == Cpt::Plus13
    }
    #[doc = "Trim +14%"]
    #[inline(always)]
    pub fn is_plus14(&self) -> bool {
        *self == Cpt::Plus14
    }
    #[doc = "Trim +15%"]
    #[inline(always)]
    pub fn is_plus15(&self) -> bool {
        *self == Cpt::Plus15
    }
    #[doc = "Trim +16%"]
    #[inline(always)]
    pub fn is_plus16(&self) -> bool {
        *self == Cpt::Plus16
    }
}
#[doc = "Field `CPT` writer - Current precision trim"]
pub type CptW<'a, REG> = crate::FieldWriterSafe<'a, REG, 5, Cpt>;
impl<'a, REG> CptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trim -15%"]
    #[inline(always)]
    pub fn minus15(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Minus15)
    }
    #[doc = "Trim -14%"]
    #[inline(always)]
    pub fn minus14(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Minus14)
    }
    #[doc = "Trim -13%"]
    #[inline(always)]
    pub fn minus13(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Minus13)
    }
    #[doc = "Trim -12%"]
    #[inline(always)]
    pub fn minus12(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Minus12)
    }
    #[doc = "Trim -11%"]
    #[inline(always)]
    pub fn minus11(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Minus11)
    }
    #[doc = "Trim -10%"]
    #[inline(always)]
    pub fn minus10(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Minus10)
    }
    #[doc = "Trim -9%"]
    #[inline(always)]
    pub fn minus9(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Minus9)
    }
    #[doc = "Trim -8%"]
    #[inline(always)]
    pub fn minus8(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Minus8)
    }
    #[doc = "Trim -7%"]
    #[inline(always)]
    pub fn minus7(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Minus7)
    }
    #[doc = "Trim -6%"]
    #[inline(always)]
    pub fn minus6(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Minus6)
    }
    #[doc = "Trim -5%"]
    #[inline(always)]
    pub fn minus5(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Minus5)
    }
    #[doc = "Trim -4%"]
    #[inline(always)]
    pub fn minus4(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Minus4)
    }
    #[doc = "Trim -3%"]
    #[inline(always)]
    pub fn minus3(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Minus3)
    }
    #[doc = "Trim -2%"]
    #[inline(always)]
    pub fn minus2(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Minus2)
    }
    #[doc = "Trim -1%"]
    #[inline(always)]
    pub fn minus1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Minus1)
    }
    #[doc = "Trim 0%"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Zero)
    }
    #[doc = "Trim +1%"]
    #[inline(always)]
    pub fn plus1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Plus1)
    }
    #[doc = "Trim +2%"]
    #[inline(always)]
    pub fn plus2(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Plus2)
    }
    #[doc = "Trim +3%"]
    #[inline(always)]
    pub fn plus3(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Plus3)
    }
    #[doc = "Trim +4%"]
    #[inline(always)]
    pub fn plus4(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Plus4)
    }
    #[doc = "Trim +5%"]
    #[inline(always)]
    pub fn plus5(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Plus5)
    }
    #[doc = "Trim +6%"]
    #[inline(always)]
    pub fn plus6(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Plus6)
    }
    #[doc = "Trim +7%"]
    #[inline(always)]
    pub fn plus7(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Plus7)
    }
    #[doc = "Trim +8%"]
    #[inline(always)]
    pub fn plus8(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Plus8)
    }
    #[doc = "Trim +9%"]
    #[inline(always)]
    pub fn plus9(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Plus9)
    }
    #[doc = "Trim +10%"]
    #[inline(always)]
    pub fn plus10(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Plus10)
    }
    #[doc = "Trim +11%"]
    #[inline(always)]
    pub fn plus11(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Plus11)
    }
    #[doc = "Trim +12%"]
    #[inline(always)]
    pub fn plus12(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Plus12)
    }
    #[doc = "Trim +13%"]
    #[inline(always)]
    pub fn plus13(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Plus13)
    }
    #[doc = "Trim +14%"]
    #[inline(always)]
    pub fn plus14(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Plus14)
    }
    #[doc = "Trim +15%"]
    #[inline(always)]
    pub fn plus15(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Plus15)
    }
    #[doc = "Trim +16%"]
    #[inline(always)]
    pub fn plus16(self) -> &'a mut crate::W<REG> {
        self.variant(Cpt::Plus16)
    }
}
#[doc = "Step selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssel {
    #[doc = "0: Low power, 1uA step"]
    LowPower = 0,
    #[doc = "1: Low power, 8uA step"]
    HighPower = 1,
}
impl From<Ssel> for bool {
    #[inline(always)]
    fn from(variant: Ssel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSEL` reader - Step selection"]
pub type SselR = crate::BitReader<Ssel>;
impl SselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssel {
        match self.bits {
            false => Ssel::LowPower,
            true => Ssel::HighPower,
        }
    }
    #[doc = "Low power, 1uA step"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == Ssel::LowPower
    }
    #[doc = "Low power, 8uA step"]
    #[inline(always)]
    pub fn is_high_power(&self) -> bool {
        *self == Ssel::HighPower
    }
}
#[doc = "Field `SSEL` writer - Step selection"]
pub type SselW<'a, REG> = crate::BitWriter<'a, REG, Ssel>;
impl<'a, REG> SselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low power, 1uA step"]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut crate::W<REG> {
        self.variant(Ssel::LowPower)
    }
    #[doc = "Low power, 8uA step"]
    #[inline(always)]
    pub fn high_power(self) -> &'a mut crate::W<REG> {
        self.variant(Ssel::HighPower)
    }
}
#[doc = "Current reference enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cren {
    #[doc = "0: Disable current reference"]
    Disabled = 0,
    #[doc = "1: Enable current reference"]
    Enabled = 1,
}
impl From<Cren> for bool {
    #[inline(always)]
    fn from(variant: Cren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CREN` reader - Current reference enable"]
pub type CrenR = crate::BitReader<Cren>;
impl CrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cren {
        match self.bits {
            false => Cren::Disabled,
            true => Cren::Enabled,
        }
    }
    #[doc = "Disable current reference"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cren::Disabled
    }
    #[doc = "Enable current reference"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cren::Enabled
    }
}
#[doc = "Field `CREN` writer - Current reference enable"]
pub type CrenW<'a, REG> = crate::BitWriter<'a, REG, Cren>;
impl<'a, REG> CrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable current reference"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cren::Disabled)
    }
    #[doc = "Enable current reference"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cren::Enabled)
    }
}
#[doc = "Voltage precision tirm\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vpt {
    #[doc = "0: Trim -6.4%"]
    Minus6_4 = 0,
    #[doc = "1: Trim -6.0%"]
    Minus6_0 = 1,
    #[doc = "2: Trim -5.6%"]
    Minus5_6 = 2,
    #[doc = "3: Trim -5.2%"]
    Minus5_2 = 3,
    #[doc = "4: Trim -4.8%"]
    Minus4_8 = 4,
    #[doc = "5: Trim -4.4%"]
    Minus4_4 = 5,
    #[doc = "6: Trim -4.0%"]
    Minus4_0 = 6,
    #[doc = "7: Trim -3.6%"]
    Minus3_6 = 7,
    #[doc = "8: Trim -3.2%"]
    Minus3_2 = 8,
    #[doc = "9: Trim -2.8%"]
    Minus2_8 = 9,
    #[doc = "10: Trim -2.4%"]
    Minus2_4 = 10,
    #[doc = "11: Trim -2.0%"]
    Minus2_0 = 11,
    #[doc = "12: Trim -1.6%"]
    Minus1_6 = 12,
    #[doc = "13: Trim -1.2%"]
    Minus1_2 = 13,
    #[doc = "14: Trim -0.8%"]
    Minus0_8 = 14,
    #[doc = "15: Trim -0.4%"]
    Minus0_4 = 15,
    #[doc = "16: Trim 0%"]
    Zero = 16,
    #[doc = "17: Trim +0.4%"]
    Plus0_4 = 17,
    #[doc = "18: Trim +0.8%"]
    Plus0_8 = 18,
    #[doc = "19: Trim +1.2%"]
    Plus1_2 = 19,
    #[doc = "20: Trim +1.6%"]
    Plus1_6 = 20,
    #[doc = "22: Trim +2.4%"]
    Plus2_4 = 22,
    #[doc = "21: Trim +2.0%"]
    Plus2_0 = 21,
    #[doc = "23: Trim +2.8%"]
    Plus2_8 = 23,
    #[doc = "24: Trim +3.2%"]
    Plus3_2 = 24,
    #[doc = "25: Trim +3.6%"]
    Plus3_6 = 25,
    #[doc = "26: Trim +4.0%"]
    Plus4_0 = 26,
    #[doc = "27: Trim +4.4%"]
    Plus4_4 = 27,
    #[doc = "28: Trim +4.8%"]
    Plus4_8 = 28,
    #[doc = "29: Trim +5.2%"]
    Plus5_2 = 29,
    #[doc = "30: Trim +5.6%"]
    Plus5_6 = 30,
    #[doc = "31: Trim +6.0%"]
    Plus6_0 = 31,
}
impl From<Vpt> for u8 {
    #[inline(always)]
    fn from(variant: Vpt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vpt {
    type Ux = u8;
}
#[doc = "Field `VPT` reader - Voltage precision tirm"]
pub type VptR = crate::FieldReader<Vpt>;
impl VptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vpt {
        match self.bits {
            0 => Vpt::Minus6_4,
            1 => Vpt::Minus6_0,
            2 => Vpt::Minus5_6,
            3 => Vpt::Minus5_2,
            4 => Vpt::Minus4_8,
            5 => Vpt::Minus4_4,
            6 => Vpt::Minus4_0,
            7 => Vpt::Minus3_6,
            8 => Vpt::Minus3_2,
            9 => Vpt::Minus2_8,
            10 => Vpt::Minus2_4,
            11 => Vpt::Minus2_0,
            12 => Vpt::Minus1_6,
            13 => Vpt::Minus1_2,
            14 => Vpt::Minus0_8,
            15 => Vpt::Minus0_4,
            16 => Vpt::Zero,
            17 => Vpt::Plus0_4,
            18 => Vpt::Plus0_8,
            19 => Vpt::Plus1_2,
            20 => Vpt::Plus1_6,
            22 => Vpt::Plus2_4,
            21 => Vpt::Plus2_0,
            23 => Vpt::Plus2_8,
            24 => Vpt::Plus3_2,
            25 => Vpt::Plus3_6,
            26 => Vpt::Plus4_0,
            27 => Vpt::Plus4_4,
            28 => Vpt::Plus4_8,
            29 => Vpt::Plus5_2,
            30 => Vpt::Plus5_6,
            31 => Vpt::Plus6_0,
            _ => unreachable!(),
        }
    }
    #[doc = "Trim -6.4%"]
    #[inline(always)]
    pub fn is_minus6_4(&self) -> bool {
        *self == Vpt::Minus6_4
    }
    #[doc = "Trim -6.0%"]
    #[inline(always)]
    pub fn is_minus6_0(&self) -> bool {
        *self == Vpt::Minus6_0
    }
    #[doc = "Trim -5.6%"]
    #[inline(always)]
    pub fn is_minus5_6(&self) -> bool {
        *self == Vpt::Minus5_6
    }
    #[doc = "Trim -5.2%"]
    #[inline(always)]
    pub fn is_minus5_2(&self) -> bool {
        *self == Vpt::Minus5_2
    }
    #[doc = "Trim -4.8%"]
    #[inline(always)]
    pub fn is_minus4_8(&self) -> bool {
        *self == Vpt::Minus4_8
    }
    #[doc = "Trim -4.4%"]
    #[inline(always)]
    pub fn is_minus4_4(&self) -> bool {
        *self == Vpt::Minus4_4
    }
    #[doc = "Trim -4.0%"]
    #[inline(always)]
    pub fn is_minus4_0(&self) -> bool {
        *self == Vpt::Minus4_0
    }
    #[doc = "Trim -3.6%"]
    #[inline(always)]
    pub fn is_minus3_6(&self) -> bool {
        *self == Vpt::Minus3_6
    }
    #[doc = "Trim -3.2%"]
    #[inline(always)]
    pub fn is_minus3_2(&self) -> bool {
        *self == Vpt::Minus3_2
    }
    #[doc = "Trim -2.8%"]
    #[inline(always)]
    pub fn is_minus2_8(&self) -> bool {
        *self == Vpt::Minus2_8
    }
    #[doc = "Trim -2.4%"]
    #[inline(always)]
    pub fn is_minus2_4(&self) -> bool {
        *self == Vpt::Minus2_4
    }
    #[doc = "Trim -2.0%"]
    #[inline(always)]
    pub fn is_minus2_0(&self) -> bool {
        *self == Vpt::Minus2_0
    }
    #[doc = "Trim -1.6%"]
    #[inline(always)]
    pub fn is_minus1_6(&self) -> bool {
        *self == Vpt::Minus1_6
    }
    #[doc = "Trim -1.2%"]
    #[inline(always)]
    pub fn is_minus1_2(&self) -> bool {
        *self == Vpt::Minus1_2
    }
    #[doc = "Trim -0.8%"]
    #[inline(always)]
    pub fn is_minus0_8(&self) -> bool {
        *self == Vpt::Minus0_8
    }
    #[doc = "Trim -0.4%"]
    #[inline(always)]
    pub fn is_minus0_4(&self) -> bool {
        *self == Vpt::Minus0_4
    }
    #[doc = "Trim 0%"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Vpt::Zero
    }
    #[doc = "Trim +0.4%"]
    #[inline(always)]
    pub fn is_plus0_4(&self) -> bool {
        *self == Vpt::Plus0_4
    }
    #[doc = "Trim +0.8%"]
    #[inline(always)]
    pub fn is_plus0_8(&self) -> bool {
        *self == Vpt::Plus0_8
    }
    #[doc = "Trim +1.2%"]
    #[inline(always)]
    pub fn is_plus1_2(&self) -> bool {
        *self == Vpt::Plus1_2
    }
    #[doc = "Trim +1.6%"]
    #[inline(always)]
    pub fn is_plus1_6(&self) -> bool {
        *self == Vpt::Plus1_6
    }
    #[doc = "Trim +2.4%"]
    #[inline(always)]
    pub fn is_plus2_4(&self) -> bool {
        *self == Vpt::Plus2_4
    }
    #[doc = "Trim +2.0%"]
    #[inline(always)]
    pub fn is_plus2_0(&self) -> bool {
        *self == Vpt::Plus2_0
    }
    #[doc = "Trim +2.8%"]
    #[inline(always)]
    pub fn is_plus2_8(&self) -> bool {
        *self == Vpt::Plus2_8
    }
    #[doc = "Trim +3.2%"]
    #[inline(always)]
    pub fn is_plus3_2(&self) -> bool {
        *self == Vpt::Plus3_2
    }
    #[doc = "Trim +3.6%"]
    #[inline(always)]
    pub fn is_plus3_6(&self) -> bool {
        *self == Vpt::Plus3_6
    }
    #[doc = "Trim +4.0%"]
    #[inline(always)]
    pub fn is_plus4_0(&self) -> bool {
        *self == Vpt::Plus4_0
    }
    #[doc = "Trim +4.4%"]
    #[inline(always)]
    pub fn is_plus4_4(&self) -> bool {
        *self == Vpt::Plus4_4
    }
    #[doc = "Trim +4.8%"]
    #[inline(always)]
    pub fn is_plus4_8(&self) -> bool {
        *self == Vpt::Plus4_8
    }
    #[doc = "Trim +5.2%"]
    #[inline(always)]
    pub fn is_plus5_2(&self) -> bool {
        *self == Vpt::Plus5_2
    }
    #[doc = "Trim +5.6%"]
    #[inline(always)]
    pub fn is_plus5_6(&self) -> bool {
        *self == Vpt::Plus5_6
    }
    #[doc = "Trim +6.0%"]
    #[inline(always)]
    pub fn is_plus6_0(&self) -> bool {
        *self == Vpt::Plus6_0
    }
}
#[doc = "Field `VPT` writer - Voltage precision tirm"]
pub type VptW<'a, REG> = crate::FieldWriterSafe<'a, REG, 5, Vpt>;
impl<'a, REG> VptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trim -6.4%"]
    #[inline(always)]
    pub fn minus6_4(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Minus6_4)
    }
    #[doc = "Trim -6.0%"]
    #[inline(always)]
    pub fn minus6_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Minus6_0)
    }
    #[doc = "Trim -5.6%"]
    #[inline(always)]
    pub fn minus5_6(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Minus5_6)
    }
    #[doc = "Trim -5.2%"]
    #[inline(always)]
    pub fn minus5_2(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Minus5_2)
    }
    #[doc = "Trim -4.8%"]
    #[inline(always)]
    pub fn minus4_8(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Minus4_8)
    }
    #[doc = "Trim -4.4%"]
    #[inline(always)]
    pub fn minus4_4(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Minus4_4)
    }
    #[doc = "Trim -4.0%"]
    #[inline(always)]
    pub fn minus4_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Minus4_0)
    }
    #[doc = "Trim -3.6%"]
    #[inline(always)]
    pub fn minus3_6(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Minus3_6)
    }
    #[doc = "Trim -3.2%"]
    #[inline(always)]
    pub fn minus3_2(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Minus3_2)
    }
    #[doc = "Trim -2.8%"]
    #[inline(always)]
    pub fn minus2_8(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Minus2_8)
    }
    #[doc = "Trim -2.4%"]
    #[inline(always)]
    pub fn minus2_4(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Minus2_4)
    }
    #[doc = "Trim -2.0%"]
    #[inline(always)]
    pub fn minus2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Minus2_0)
    }
    #[doc = "Trim -1.6%"]
    #[inline(always)]
    pub fn minus1_6(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Minus1_6)
    }
    #[doc = "Trim -1.2%"]
    #[inline(always)]
    pub fn minus1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Minus1_2)
    }
    #[doc = "Trim -0.8%"]
    #[inline(always)]
    pub fn minus0_8(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Minus0_8)
    }
    #[doc = "Trim -0.4%"]
    #[inline(always)]
    pub fn minus0_4(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Minus0_4)
    }
    #[doc = "Trim 0%"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Zero)
    }
    #[doc = "Trim +0.4%"]
    #[inline(always)]
    pub fn plus0_4(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Plus0_4)
    }
    #[doc = "Trim +0.8%"]
    #[inline(always)]
    pub fn plus0_8(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Plus0_8)
    }
    #[doc = "Trim +1.2%"]
    #[inline(always)]
    pub fn plus1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Plus1_2)
    }
    #[doc = "Trim +1.6%"]
    #[inline(always)]
    pub fn plus1_6(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Plus1_6)
    }
    #[doc = "Trim +2.4%"]
    #[inline(always)]
    pub fn plus2_4(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Plus2_4)
    }
    #[doc = "Trim +2.0%"]
    #[inline(always)]
    pub fn plus2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Plus2_0)
    }
    #[doc = "Trim +2.8%"]
    #[inline(always)]
    pub fn plus2_8(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Plus2_8)
    }
    #[doc = "Trim +3.2%"]
    #[inline(always)]
    pub fn plus3_2(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Plus3_2)
    }
    #[doc = "Trim +3.6%"]
    #[inline(always)]
    pub fn plus3_6(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Plus3_6)
    }
    #[doc = "Trim +4.0%"]
    #[inline(always)]
    pub fn plus4_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Plus4_0)
    }
    #[doc = "Trim +4.4%"]
    #[inline(always)]
    pub fn plus4_4(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Plus4_4)
    }
    #[doc = "Trim +4.8%"]
    #[inline(always)]
    pub fn plus4_8(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Plus4_8)
    }
    #[doc = "Trim +5.2%"]
    #[inline(always)]
    pub fn plus5_2(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Plus5_2)
    }
    #[doc = "Trim +5.6%"]
    #[inline(always)]
    pub fn plus5_6(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Plus5_6)
    }
    #[doc = "Trim +6.0%"]
    #[inline(always)]
    pub fn plus6_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vpt::Plus6_0)
    }
}
#[doc = "Disconnect external capacitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decap {
    #[doc = "0: External capacitor connected"]
    Connected = 0,
    #[doc = "1: External capacitor disonnected"]
    Disconnected = 1,
}
impl From<Decap> for bool {
    #[inline(always)]
    fn from(variant: Decap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DECAP` reader - Disconnect external capacitor"]
pub type DecapR = crate::BitReader<Decap>;
impl DecapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Decap {
        match self.bits {
            false => Decap::Connected,
            true => Decap::Disconnected,
        }
    }
    #[doc = "External capacitor connected"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == Decap::Connected
    }
    #[doc = "External capacitor disonnected"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == Decap::Disconnected
    }
}
#[doc = "Field `DECAP` writer - Disconnect external capacitor"]
pub type DecapW<'a, REG> = crate::BitWriter<'a, REG, Decap>;
impl<'a, REG> DecapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External capacitor connected"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(Decap::Connected)
    }
    #[doc = "External capacitor disonnected"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(Decap::Disconnected)
    }
}
#[doc = "Voltage reference enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vren {
    #[doc = "0: Disable voltage reference"]
    Disabled = 0,
    #[doc = "1: Enable voltage reference"]
    Enabled = 1,
}
impl From<Vren> for bool {
    #[inline(always)]
    fn from(variant: Vren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREN` reader - Voltage reference enable"]
pub type VrenR = crate::BitReader<Vren>;
impl VrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vren {
        match self.bits {
            false => Vren::Disabled,
            true => Vren::Enabled,
        }
    }
    #[doc = "Disable voltage reference"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Vren::Disabled
    }
    #[doc = "Enable voltage reference"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Vren::Enabled
    }
}
#[doc = "Field `VREN` writer - Voltage reference enable"]
pub type VrenW<'a, REG> = crate::BitWriter<'a, REG, Vren>;
impl<'a, REG> VrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable voltage reference"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Vren::Disabled)
    }
    #[doc = "Enable voltage reference"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Vren::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:5 - Current step data"]
    #[inline(always)]
    pub fn csdt(&self) -> CsdtR {
        CsdtR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Sink current mode"]
    #[inline(always)]
    pub fn scmod(&self) -> ScmodR {
        ScmodR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Current precision trim"]
    #[inline(always)]
    pub fn cpt(&self) -> CptR {
        CptR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - Step selection"]
    #[inline(always)]
    pub fn ssel(&self) -> SselR {
        SselR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Current reference enable"]
    #[inline(always)]
    pub fn cren(&self) -> CrenR {
        CrenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Voltage precision tirm"]
    #[inline(always)]
    pub fn vpt(&self) -> VptR {
        VptR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - Disconnect external capacitor"]
    #[inline(always)]
    pub fn decap(&self) -> DecapR {
        DecapR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Voltage reference enable"]
    #[inline(always)]
    pub fn vren(&self) -> VrenR {
        VrenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Current step data"]
    #[inline(always)]
    #[must_use]
    pub fn csdt(&mut self) -> CsdtW<IvrefCtlSpec> {
        CsdtW::new(self, 0)
    }
    #[doc = "Bit 7 - Sink current mode"]
    #[inline(always)]
    #[must_use]
    pub fn scmod(&mut self) -> ScmodW<IvrefCtlSpec> {
        ScmodW::new(self, 7)
    }
    #[doc = "Bits 8:12 - Current precision trim"]
    #[inline(always)]
    #[must_use]
    pub fn cpt(&mut self) -> CptW<IvrefCtlSpec> {
        CptW::new(self, 8)
    }
    #[doc = "Bit 14 - Step selection"]
    #[inline(always)]
    #[must_use]
    pub fn ssel(&mut self) -> SselW<IvrefCtlSpec> {
        SselW::new(self, 14)
    }
    #[doc = "Bit 15 - Current reference enable"]
    #[inline(always)]
    #[must_use]
    pub fn cren(&mut self) -> CrenW<IvrefCtlSpec> {
        CrenW::new(self, 15)
    }
    #[doc = "Bits 24:28 - Voltage precision tirm"]
    #[inline(always)]
    #[must_use]
    pub fn vpt(&mut self) -> VptW<IvrefCtlSpec> {
        VptW::new(self, 24)
    }
    #[doc = "Bit 30 - Disconnect external capacitor"]
    #[inline(always)]
    #[must_use]
    pub fn decap(&mut self) -> DecapW<IvrefCtlSpec> {
        DecapW::new(self, 30)
    }
    #[doc = "Bit 31 - Voltage reference enable"]
    #[inline(always)]
    #[must_use]
    pub fn vren(&mut self) -> VrenW<IvrefCtlSpec> {
        VrenW::new(self, 31)
    }
}
#[doc = "IVREF control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivref_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivref_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IvrefCtlSpec;
impl crate::RegisterSpec for IvrefCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ivref_ctl::R`](R) reader structure"]
impl crate::Readable for IvrefCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ivref_ctl::W`](W) writer structure"]
impl crate::Writable for IvrefCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IVREF_CTL to value 0x1000_0f00"]
impl crate::Resettable for IvrefCtlSpec {
    const RESET_VALUE: u32 = 0x1000_0f00;
}
