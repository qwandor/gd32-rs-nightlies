#[doc = "Register `CFG2` reader"]
pub type R = crate::R<Cfg2Spec>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<Cfg2Spec>;
#[doc = "Field `CKOUT0DIV` reader - the CK_OUT0 divider"]
pub type Ckout0divR = crate::FieldReader;
#[doc = "Field `CKOUT0DIV` writer - the CK_OUT0 divider"]
pub type Ckout0divW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CKOUT1DIV` reader - the CK_OUT1 divider"]
pub type Ckout1divR = crate::FieldReader;
#[doc = "Field `CKOUT1DIV` writer - the CK_OUT1 divider"]
pub type Ckout1divW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "CK_OUT1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckout1sel {
    #[doc = "0: No clock selected"]
    Noclk = 0,
    #[doc = "1: System clock selected"]
    Sysclk = 1,
    #[doc = "2: High Speed 8M Internal Oscillator clock (IRC8M) selected"]
    Irc8m = 2,
    #[doc = "3: External High Speed oscillator clock (HXTAL) selected"]
    Hxtal = 3,
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
#[doc = "Field `CKOUT1SEL` reader - CK_OUT1 clock source selection"]
pub type Ckout1selR = crate::FieldReader<Ckout1sel>;
impl Ckout1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ckout1sel> {
        match self.bits {
            0 => Some(Ckout1sel::Noclk),
            1 => Some(Ckout1sel::Sysclk),
            2 => Some(Ckout1sel::Irc8m),
            3 => Some(Ckout1sel::Hxtal),
            _ => None,
        }
    }
    #[doc = "No clock selected"]
    #[inline(always)]
    pub fn is_noclk(&self) -> bool {
        *self == Ckout1sel::Noclk
    }
    #[doc = "System clock selected"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == Ckout1sel::Sysclk
    }
    #[doc = "High Speed 8M Internal Oscillator clock (IRC8M) selected"]
    #[inline(always)]
    pub fn is_irc8m(&self) -> bool {
        *self == Ckout1sel::Irc8m
    }
    #[doc = "External High Speed oscillator clock (HXTAL) selected"]
    #[inline(always)]
    pub fn is_hxtal(&self) -> bool {
        *self == Ckout1sel::Hxtal
    }
}
#[doc = "Field `CKOUT1SEL` writer - CK_OUT1 clock source selection"]
pub type Ckout1selW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ckout1sel>;
impl<'a, REG> Ckout1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock selected"]
    #[inline(always)]
    pub fn noclk(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout1sel::Noclk)
    }
    #[doc = "System clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout1sel::Sysclk)
    }
    #[doc = "High Speed 8M Internal Oscillator clock (IRC8M) selected"]
    #[inline(always)]
    pub fn irc8m(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout1sel::Irc8m)
    }
    #[doc = "External High Speed oscillator clock (HXTAL) selected"]
    #[inline(always)]
    pub fn hxtal(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout1sel::Hxtal)
    }
}
impl R {
    #[doc = "Bits 0:5 - the CK_OUT0 divider"]
    #[inline(always)]
    pub fn ckout0div(&self) -> Ckout0divR {
        Ckout0divR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - the CK_OUT1 divider"]
    #[inline(always)]
    pub fn ckout1div(&self) -> Ckout1divR {
        Ckout1divR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - CK_OUT1 clock source selection"]
    #[inline(always)]
    pub fn ckout1sel(&self) -> Ckout1selR {
        Ckout1selR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - the CK_OUT0 divider"]
    #[inline(always)]
    #[must_use]
    pub fn ckout0div(&mut self) -> Ckout0divW<Cfg2Spec> {
        Ckout0divW::new(self, 0)
    }
    #[doc = "Bits 8:13 - the CK_OUT1 divider"]
    #[inline(always)]
    #[must_use]
    pub fn ckout1div(&mut self) -> Ckout1divW<Cfg2Spec> {
        Ckout1divW::new(self, 8)
    }
    #[doc = "Bits 16:19 - CK_OUT1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckout1sel(&mut self) -> Ckout1selW<Cfg2Spec> {
        Ckout1selW::new(self, 16)
    }
}
#[doc = "configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
