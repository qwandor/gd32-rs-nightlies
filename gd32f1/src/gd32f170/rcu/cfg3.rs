#[doc = "Register `CFG3` reader"]
pub type R = crate::R<Cfg3Spec>;
#[doc = "Register `CFG3` writer"]
pub type W = crate::W<Cfg3Spec>;
#[doc = "CKOUT1 Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckout1sel {
    #[doc = "0: No clock selected"]
    None = 0,
    #[doc = "1: Internal 28 MHz RC oscillator clock selected"]
    Irc28m = 1,
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
impl From<Ckout1sel> for u8 {
    #[inline(always)]
    fn from(variant: Ckout1sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckout1sel {
    type Ux = u8;
}
#[doc = "Field `CKOUT1SEL` reader - CKOUT1 Clock Source Selection"]
pub type Ckout1selR = crate::FieldReader<Ckout1sel>;
impl Ckout1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckout1sel {
        match self.bits {
            0 => Ckout1sel::None,
            1 => Ckout1sel::Irc28m,
            2 => Ckout1sel::Lsi40k,
            3 => Ckout1sel::Lxtal,
            4 => Ckout1sel::Sysclk,
            5 => Ckout1sel::Irc8m,
            6 => Ckout1sel::Hxtal,
            7 => Ckout1sel::Pll,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock selected"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Ckout1sel::None
    }
    #[doc = "Internal 28 MHz RC oscillator clock selected"]
    #[inline(always)]
    pub fn is_irc28m(&self) -> bool {
        *self == Ckout1sel::Irc28m
    }
    #[doc = "Internal 40 kHz RC oscillator clock selected"]
    #[inline(always)]
    pub fn is_lsi40k(&self) -> bool {
        *self == Ckout1sel::Lsi40k
    }
    #[doc = "External low speed oscillator clock selected"]
    #[inline(always)]
    pub fn is_lxtal(&self) -> bool {
        *self == Ckout1sel::Lxtal
    }
    #[doc = "System clock selected"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == Ckout1sel::Sysclk
    }
    #[doc = "Internal RC 8 MHz (HSI) oscillator clock selected"]
    #[inline(always)]
    pub fn is_irc8m(&self) -> bool {
        *self == Ckout1sel::Irc8m
    }
    #[doc = "External 4-32 MHz (HSE) oscillator clock selected"]
    #[inline(always)]
    pub fn is_hxtal(&self) -> bool {
        *self == Ckout1sel::Hxtal
    }
    #[doc = "PLL clock selected (divided by 1 or 2, depending on PLLDV)"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == Ckout1sel::Pll
    }
}
#[doc = "Field `CKOUT1SEL` writer - CKOUT1 Clock Source Selection"]
pub type Ckout1selW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Ckout1sel>;
impl<'a, REG> Ckout1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock selected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout1sel::None)
    }
    #[doc = "Internal 28 MHz RC oscillator clock selected"]
    #[inline(always)]
    pub fn irc28m(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout1sel::Irc28m)
    }
    #[doc = "Internal 40 kHz RC oscillator clock selected"]
    #[inline(always)]
    pub fn lsi40k(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout1sel::Lsi40k)
    }
    #[doc = "External low speed oscillator clock selected"]
    #[inline(always)]
    pub fn lxtal(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout1sel::Lxtal)
    }
    #[doc = "System clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout1sel::Sysclk)
    }
    #[doc = "Internal RC 8 MHz (HSI) oscillator clock selected"]
    #[inline(always)]
    pub fn irc8m(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout1sel::Irc8m)
    }
    #[doc = "External 4-32 MHz (HSE) oscillator clock selected"]
    #[inline(always)]
    pub fn hxtal(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout1sel::Hxtal)
    }
    #[doc = "PLL clock selected (divided by 1 or 2, depending on PLLDV)"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout1sel::Pll)
    }
}
#[doc = "Field `CKOUT1DIV` reader - The CK_OUT1 divider which the CK_OUT1 frequency can be reduced see bits 2:0 of RCU_CFG3 for CK_OUT1"]
pub type Ckout1divR = crate::FieldReader;
#[doc = "Field `CKOUT1DIV` writer - The CK_OUT1 divider which the CK_OUT1 frequency can be reduced see bits 2:0 of RCU_CFG3 for CK_OUT1"]
pub type Ckout1divW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:2 - CKOUT1 Clock Source Selection"]
    #[inline(always)]
    pub fn ckout1sel(&self) -> Ckout1selR {
        Ckout1selR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:13 - The CK_OUT1 divider which the CK_OUT1 frequency can be reduced see bits 2:0 of RCU_CFG3 for CK_OUT1"]
    #[inline(always)]
    pub fn ckout1div(&self) -> Ckout1divR {
        Ckout1divR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CKOUT1 Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckout1sel(&mut self) -> Ckout1selW<Cfg3Spec> {
        Ckout1selW::new(self, 0)
    }
    #[doc = "Bits 8:13 - The CK_OUT1 divider which the CK_OUT1 frequency can be reduced see bits 2:0 of RCU_CFG3 for CK_OUT1"]
    #[inline(always)]
    #[must_use]
    pub fn ckout1div(&mut self) -> Ckout1divW<Cfg3Spec> {
        Ckout1divW::new(self, 8)
    }
}
#[doc = "Configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg3Spec;
impl crate::RegisterSpec for Cfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg3::R`](R) reader structure"]
impl crate::Readable for Cfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg3::W`](W) writer structure"]
impl crate::Writable for Cfg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG3 to value 0"]
impl crate::Resettable for Cfg3Spec {
    const RESET_VALUE: u32 = 0;
}
