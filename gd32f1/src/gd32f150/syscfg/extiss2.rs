#[doc = "Register `EXTISS2` reader"]
pub type R = crate::R<Extiss2Spec>;
#[doc = "Register `EXTISS2` writer"]
pub type W = crate::W<Extiss2Spec>;
#[doc = "EXTI 8 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti8Ss {
    #[doc = "0: PA8 pin"]
    Pa8 = 0,
    #[doc = "1: PB8 pin"]
    Pb8 = 1,
    #[doc = "2: PC8 pin"]
    Pc8 = 2,
}
impl From<Exti8Ss> for u8 {
    #[inline(always)]
    fn from(variant: Exti8Ss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti8Ss {
    type Ux = u8;
}
#[doc = "Field `EXTI8_SS` reader - EXTI 8 sources selection"]
pub type Exti8SsR = crate::FieldReader<Exti8Ss>;
impl Exti8SsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti8Ss> {
        match self.bits {
            0 => Some(Exti8Ss::Pa8),
            1 => Some(Exti8Ss::Pb8),
            2 => Some(Exti8Ss::Pc8),
            _ => None,
        }
    }
    #[doc = "PA8 pin"]
    #[inline(always)]
    pub fn is_pa8(&self) -> bool {
        *self == Exti8Ss::Pa8
    }
    #[doc = "PB8 pin"]
    #[inline(always)]
    pub fn is_pb8(&self) -> bool {
        *self == Exti8Ss::Pb8
    }
    #[doc = "PC8 pin"]
    #[inline(always)]
    pub fn is_pc8(&self) -> bool {
        *self == Exti8Ss::Pc8
    }
}
#[doc = "Field `EXTI8_SS` writer - EXTI 8 sources selection"]
pub type Exti8SsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Exti8Ss>;
impl<'a, REG> Exti8SsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA8 pin"]
    #[inline(always)]
    pub fn pa8(self) -> &'a mut crate::W<REG> {
        self.variant(Exti8Ss::Pa8)
    }
    #[doc = "PB8 pin"]
    #[inline(always)]
    pub fn pb8(self) -> &'a mut crate::W<REG> {
        self.variant(Exti8Ss::Pb8)
    }
    #[doc = "PC8 pin"]
    #[inline(always)]
    pub fn pc8(self) -> &'a mut crate::W<REG> {
        self.variant(Exti8Ss::Pc8)
    }
}
#[doc = "EXTI 9 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti9Ss {
    #[doc = "0: PA9 pin"]
    Pa9 = 0,
    #[doc = "1: PB9 pin"]
    Pb9 = 1,
    #[doc = "2: PC9 pin"]
    Pc9 = 2,
}
impl From<Exti9Ss> for u8 {
    #[inline(always)]
    fn from(variant: Exti9Ss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti9Ss {
    type Ux = u8;
}
#[doc = "Field `EXTI9_SS` reader - EXTI 9 sources selection"]
pub type Exti9SsR = crate::FieldReader<Exti9Ss>;
impl Exti9SsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti9Ss> {
        match self.bits {
            0 => Some(Exti9Ss::Pa9),
            1 => Some(Exti9Ss::Pb9),
            2 => Some(Exti9Ss::Pc9),
            _ => None,
        }
    }
    #[doc = "PA9 pin"]
    #[inline(always)]
    pub fn is_pa9(&self) -> bool {
        *self == Exti9Ss::Pa9
    }
    #[doc = "PB9 pin"]
    #[inline(always)]
    pub fn is_pb9(&self) -> bool {
        *self == Exti9Ss::Pb9
    }
    #[doc = "PC9 pin"]
    #[inline(always)]
    pub fn is_pc9(&self) -> bool {
        *self == Exti9Ss::Pc9
    }
}
#[doc = "Field `EXTI9_SS` writer - EXTI 9 sources selection"]
pub type Exti9SsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Exti9Ss>;
impl<'a, REG> Exti9SsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA9 pin"]
    #[inline(always)]
    pub fn pa9(self) -> &'a mut crate::W<REG> {
        self.variant(Exti9Ss::Pa9)
    }
    #[doc = "PB9 pin"]
    #[inline(always)]
    pub fn pb9(self) -> &'a mut crate::W<REG> {
        self.variant(Exti9Ss::Pb9)
    }
    #[doc = "PC9 pin"]
    #[inline(always)]
    pub fn pc9(self) -> &'a mut crate::W<REG> {
        self.variant(Exti9Ss::Pc9)
    }
}
#[doc = "EXTI 10 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti10Ss {
    #[doc = "0: PA10 pin"]
    Pa10 = 0,
    #[doc = "1: PB10 pin"]
    Pb10 = 1,
    #[doc = "2: PC10 pin"]
    Pc10 = 2,
}
impl From<Exti10Ss> for u8 {
    #[inline(always)]
    fn from(variant: Exti10Ss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti10Ss {
    type Ux = u8;
}
#[doc = "Field `EXTI10_SS` reader - EXTI 10 sources selection"]
pub type Exti10SsR = crate::FieldReader<Exti10Ss>;
impl Exti10SsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti10Ss> {
        match self.bits {
            0 => Some(Exti10Ss::Pa10),
            1 => Some(Exti10Ss::Pb10),
            2 => Some(Exti10Ss::Pc10),
            _ => None,
        }
    }
    #[doc = "PA10 pin"]
    #[inline(always)]
    pub fn is_pa10(&self) -> bool {
        *self == Exti10Ss::Pa10
    }
    #[doc = "PB10 pin"]
    #[inline(always)]
    pub fn is_pb10(&self) -> bool {
        *self == Exti10Ss::Pb10
    }
    #[doc = "PC10 pin"]
    #[inline(always)]
    pub fn is_pc10(&self) -> bool {
        *self == Exti10Ss::Pc10
    }
}
#[doc = "Field `EXTI10_SS` writer - EXTI 10 sources selection"]
pub type Exti10SsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Exti10Ss>;
impl<'a, REG> Exti10SsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA10 pin"]
    #[inline(always)]
    pub fn pa10(self) -> &'a mut crate::W<REG> {
        self.variant(Exti10Ss::Pa10)
    }
    #[doc = "PB10 pin"]
    #[inline(always)]
    pub fn pb10(self) -> &'a mut crate::W<REG> {
        self.variant(Exti10Ss::Pb10)
    }
    #[doc = "PC10 pin"]
    #[inline(always)]
    pub fn pc10(self) -> &'a mut crate::W<REG> {
        self.variant(Exti10Ss::Pc10)
    }
}
#[doc = "EXTI 11 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti11Ss {
    #[doc = "0: PA11 pin"]
    Pa11 = 0,
    #[doc = "1: PB11 pin"]
    Pb11 = 1,
    #[doc = "2: PC11 pin"]
    Pc11 = 2,
}
impl From<Exti11Ss> for u8 {
    #[inline(always)]
    fn from(variant: Exti11Ss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti11Ss {
    type Ux = u8;
}
#[doc = "Field `EXTI11_SS` reader - EXTI 11 sources selection"]
pub type Exti11SsR = crate::FieldReader<Exti11Ss>;
impl Exti11SsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti11Ss> {
        match self.bits {
            0 => Some(Exti11Ss::Pa11),
            1 => Some(Exti11Ss::Pb11),
            2 => Some(Exti11Ss::Pc11),
            _ => None,
        }
    }
    #[doc = "PA11 pin"]
    #[inline(always)]
    pub fn is_pa11(&self) -> bool {
        *self == Exti11Ss::Pa11
    }
    #[doc = "PB11 pin"]
    #[inline(always)]
    pub fn is_pb11(&self) -> bool {
        *self == Exti11Ss::Pb11
    }
    #[doc = "PC11 pin"]
    #[inline(always)]
    pub fn is_pc11(&self) -> bool {
        *self == Exti11Ss::Pc11
    }
}
#[doc = "Field `EXTI11_SS` writer - EXTI 11 sources selection"]
pub type Exti11SsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Exti11Ss>;
impl<'a, REG> Exti11SsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA11 pin"]
    #[inline(always)]
    pub fn pa11(self) -> &'a mut crate::W<REG> {
        self.variant(Exti11Ss::Pa11)
    }
    #[doc = "PB11 pin"]
    #[inline(always)]
    pub fn pb11(self) -> &'a mut crate::W<REG> {
        self.variant(Exti11Ss::Pb11)
    }
    #[doc = "PC11 pin"]
    #[inline(always)]
    pub fn pc11(self) -> &'a mut crate::W<REG> {
        self.variant(Exti11Ss::Pc11)
    }
}
impl R {
    #[doc = "Bits 0:3 - EXTI 8 sources selection"]
    #[inline(always)]
    pub fn exti8_ss(&self) -> Exti8SsR {
        Exti8SsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 9 sources selection"]
    #[inline(always)]
    pub fn exti9_ss(&self) -> Exti9SsR {
        Exti9SsR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 10 sources selection"]
    #[inline(always)]
    pub fn exti10_ss(&self) -> Exti10SsR {
        Exti10SsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 11 sources selection"]
    #[inline(always)]
    pub fn exti11_ss(&self) -> Exti11SsR {
        Exti11SsR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 8 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti8_ss(&mut self) -> Exti8SsW<Extiss2Spec> {
        Exti8SsW::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXTI 9 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti9_ss(&mut self) -> Exti9SsW<Extiss2Spec> {
        Exti9SsW::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXTI 10 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti10_ss(&mut self) -> Exti10SsW<Extiss2Spec> {
        Exti10SsW::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXTI 11 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti11_ss(&mut self) -> Exti11SsW<Extiss2Spec> {
        Exti11SsW::new(self, 12)
    }
}
#[doc = "EXTI sources selection register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extiss2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extiss2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Extiss2Spec;
impl crate::RegisterSpec for Extiss2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extiss2::R`](R) reader structure"]
impl crate::Readable for Extiss2Spec {}
#[doc = "`write(|w| ..)` method takes [`extiss2::W`](W) writer structure"]
impl crate::Writable for Extiss2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTISS2 to value 0"]
impl crate::Resettable for Extiss2Spec {
    const RESET_VALUE: u32 = 0;
}
