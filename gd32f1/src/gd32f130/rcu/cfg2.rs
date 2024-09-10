#[doc = "Register `CFG2` reader"]
pub type R = crate::R<Cfg2Spec>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<Cfg2Spec>;
#[doc = "CK_USART0 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usart0sel {
    #[doc = "0: APB2 selected as USART0 clock source"]
    Apb2 = 0,
    #[doc = "1: SYS selected as USART0 clock source"]
    Sys = 1,
    #[doc = "2: LXTAL selected as USART0 clock source"]
    Lxtal = 2,
    #[doc = "3: IRC8M selected as USART0 clock source"]
    Irc8m = 3,
}
impl From<Usart0sel> for u8 {
    #[inline(always)]
    fn from(variant: Usart0sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usart0sel {
    type Ux = u8;
}
#[doc = "Field `USART0SEL` reader - CK_USART0 clock source selection"]
pub type Usart0selR = crate::FieldReader<Usart0sel>;
impl Usart0selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart0sel {
        match self.bits {
            0 => Usart0sel::Apb2,
            1 => Usart0sel::Sys,
            2 => Usart0sel::Lxtal,
            3 => Usart0sel::Irc8m,
            _ => unreachable!(),
        }
    }
    #[doc = "APB2 selected as USART0 clock source"]
    #[inline(always)]
    pub fn is_apb2(&self) -> bool {
        *self == Usart0sel::Apb2
    }
    #[doc = "SYS selected as USART0 clock source"]
    #[inline(always)]
    pub fn is_sys(&self) -> bool {
        *self == Usart0sel::Sys
    }
    #[doc = "LXTAL selected as USART0 clock source"]
    #[inline(always)]
    pub fn is_lxtal(&self) -> bool {
        *self == Usart0sel::Lxtal
    }
    #[doc = "IRC8M selected as USART0 clock source"]
    #[inline(always)]
    pub fn is_irc8m(&self) -> bool {
        *self == Usart0sel::Irc8m
    }
}
#[doc = "Field `USART0SEL` writer - CK_USART0 clock source selection"]
pub type Usart0selW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Usart0sel>;
impl<'a, REG> Usart0selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "APB2 selected as USART0 clock source"]
    #[inline(always)]
    pub fn apb2(self) -> &'a mut crate::W<REG> {
        self.variant(Usart0sel::Apb2)
    }
    #[doc = "SYS selected as USART0 clock source"]
    #[inline(always)]
    pub fn sys(self) -> &'a mut crate::W<REG> {
        self.variant(Usart0sel::Sys)
    }
    #[doc = "LXTAL selected as USART0 clock source"]
    #[inline(always)]
    pub fn lxtal(self) -> &'a mut crate::W<REG> {
        self.variant(Usart0sel::Lxtal)
    }
    #[doc = "IRC8M selected as USART0 clock source"]
    #[inline(always)]
    pub fn irc8m(self) -> &'a mut crate::W<REG> {
        self.variant(Usart0sel::Irc8m)
    }
}
#[doc = "CK_CEC clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cecsel {
    #[doc = "0: IRC8M clock divided by 244 selected as CEC clock source"]
    Irc8mDiv244 = 0,
    #[doc = "1: LXTAL clock selected as CEC clock source"]
    Lxtal = 1,
}
impl From<Cecsel> for bool {
    #[inline(always)]
    fn from(variant: Cecsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CECSEL` reader - CK_CEC clock source selection"]
pub type CecselR = crate::BitReader<Cecsel>;
impl CecselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cecsel {
        match self.bits {
            false => Cecsel::Irc8mDiv244,
            true => Cecsel::Lxtal,
        }
    }
    #[doc = "IRC8M clock divided by 244 selected as CEC clock source"]
    #[inline(always)]
    pub fn is_irc8m_div244(&self) -> bool {
        *self == Cecsel::Irc8mDiv244
    }
    #[doc = "LXTAL clock selected as CEC clock source"]
    #[inline(always)]
    pub fn is_lxtal(&self) -> bool {
        *self == Cecsel::Lxtal
    }
}
#[doc = "Field `CECSEL` writer - CK_CEC clock source selection"]
pub type CecselW<'a, REG> = crate::BitWriter<'a, REG, Cecsel>;
impl<'a, REG> CecselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IRC8M clock divided by 244 selected as CEC clock source"]
    #[inline(always)]
    pub fn irc8m_div244(self) -> &'a mut crate::W<REG> {
        self.variant(Cecsel::Irc8mDiv244)
    }
    #[doc = "LXTAL clock selected as CEC clock source"]
    #[inline(always)]
    pub fn lxtal(self) -> &'a mut crate::W<REG> {
        self.variant(Cecsel::Lxtal)
    }
}
#[doc = "CK_ADC clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcsel {
    #[doc = "0: IRC14M selected as ADC clock source"]
    Irc14m = 0,
    #[doc = "1: APB2 divided by prescaler selected as ADC clock source"]
    Apb2 = 1,
}
impl From<Adcsel> for bool {
    #[inline(always)]
    fn from(variant: Adcsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCSEL` reader - CK_ADC clock source selection"]
pub type AdcselR = crate::BitReader<Adcsel>;
impl AdcselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcsel {
        match self.bits {
            false => Adcsel::Irc14m,
            true => Adcsel::Apb2,
        }
    }
    #[doc = "IRC14M selected as ADC clock source"]
    #[inline(always)]
    pub fn is_irc14m(&self) -> bool {
        *self == Adcsel::Irc14m
    }
    #[doc = "APB2 divided by prescaler selected as ADC clock source"]
    #[inline(always)]
    pub fn is_apb2(&self) -> bool {
        *self == Adcsel::Apb2
    }
}
#[doc = "Field `ADCSEL` writer - CK_ADC clock source selection"]
pub type AdcselW<'a, REG> = crate::BitWriter<'a, REG, Adcsel>;
impl<'a, REG> AdcselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IRC14M selected as ADC clock source"]
    #[inline(always)]
    pub fn irc14m(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsel::Irc14m)
    }
    #[doc = "APB2 divided by prescaler selected as ADC clock source"]
    #[inline(always)]
    pub fn apb2(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsel::Apb2)
    }
}
impl R {
    #[doc = "Bits 0:1 - CK_USART0 clock source selection"]
    #[inline(always)]
    pub fn usart0sel(&self) -> Usart0selR {
        Usart0selR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 6 - CK_CEC clock source selection"]
    #[inline(always)]
    pub fn cecsel(&self) -> CecselR {
        CecselR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - CK_ADC clock source selection"]
    #[inline(always)]
    pub fn adcsel(&self) -> AdcselR {
        AdcselR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CK_USART0 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart0sel(&mut self) -> Usart0selW<Cfg2Spec> {
        Usart0selW::new(self, 0)
    }
    #[doc = "Bit 6 - CK_CEC clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn cecsel(&mut self) -> CecselW<Cfg2Spec> {
        CecselW::new(self, 6)
    }
    #[doc = "Bit 8 - CK_ADC clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn adcsel(&mut self) -> AdcselW<Cfg2Spec> {
        AdcselW::new(self, 8)
    }
}
#[doc = "Configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg2Spec;
impl crate::RegisterSpec for Cfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg2::R`](R) reader structure"]
impl crate::Readable for Cfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg2::W`](W) writer structure"]
impl crate::Writable for Cfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG2 to value 0"]
impl crate::Resettable for Cfg2Spec {
    const RESET_VALUE: u32 = 0;
}
