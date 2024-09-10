#[doc = "Register `EXTISS1` reader"]
pub type R = crate::R<Extiss1Spec>;
#[doc = "Register `EXTISS1` writer"]
pub type W = crate::W<Extiss1Spec>;
#[doc = "EXTI 4 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti4Ss {
    #[doc = "0: PA4 pin"]
    Pa4 = 0,
    #[doc = "1: PB4 pin"]
    Pb4 = 1,
    #[doc = "2: PC4 pin"]
    Pc4 = 2,
    #[doc = "5: PF4 pin"]
    Pf4 = 5,
}
impl From<Exti4Ss> for u8 {
    #[inline(always)]
    fn from(variant: Exti4Ss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti4Ss {
    type Ux = u8;
}
#[doc = "Field `EXTI4_SS` reader - EXTI 4 sources selection"]
pub type Exti4SsR = crate::FieldReader<Exti4Ss>;
impl Exti4SsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti4Ss> {
        match self.bits {
            0 => Some(Exti4Ss::Pa4),
            1 => Some(Exti4Ss::Pb4),
            2 => Some(Exti4Ss::Pc4),
            5 => Some(Exti4Ss::Pf4),
            _ => None,
        }
    }
    #[doc = "PA4 pin"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        *self == Exti4Ss::Pa4
    }
    #[doc = "PB4 pin"]
    #[inline(always)]
    pub fn is_pb4(&self) -> bool {
        *self == Exti4Ss::Pb4
    }
    #[doc = "PC4 pin"]
    #[inline(always)]
    pub fn is_pc4(&self) -> bool {
        *self == Exti4Ss::Pc4
    }
    #[doc = "PF4 pin"]
    #[inline(always)]
    pub fn is_pf4(&self) -> bool {
        *self == Exti4Ss::Pf4
    }
}
#[doc = "Field `EXTI4_SS` writer - EXTI 4 sources selection"]
pub type Exti4SsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Exti4Ss>;
impl<'a, REG> Exti4SsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA4 pin"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut crate::W<REG> {
        self.variant(Exti4Ss::Pa4)
    }
    #[doc = "PB4 pin"]
    #[inline(always)]
    pub fn pb4(self) -> &'a mut crate::W<REG> {
        self.variant(Exti4Ss::Pb4)
    }
    #[doc = "PC4 pin"]
    #[inline(always)]
    pub fn pc4(self) -> &'a mut crate::W<REG> {
        self.variant(Exti4Ss::Pc4)
    }
    #[doc = "PF4 pin"]
    #[inline(always)]
    pub fn pf4(self) -> &'a mut crate::W<REG> {
        self.variant(Exti4Ss::Pf4)
    }
}
#[doc = "EXTI 5 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti5Ss {
    #[doc = "0: PA5 pin"]
    Pa5 = 0,
    #[doc = "1: PB5 pin"]
    Pb5 = 1,
    #[doc = "2: PC5 pin"]
    Pc5 = 2,
    #[doc = "5: PF5 pin"]
    Pf5 = 5,
}
impl From<Exti5Ss> for u8 {
    #[inline(always)]
    fn from(variant: Exti5Ss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti5Ss {
    type Ux = u8;
}
#[doc = "Field `EXTI5_SS` reader - EXTI 5 sources selection"]
pub type Exti5SsR = crate::FieldReader<Exti5Ss>;
impl Exti5SsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti5Ss> {
        match self.bits {
            0 => Some(Exti5Ss::Pa5),
            1 => Some(Exti5Ss::Pb5),
            2 => Some(Exti5Ss::Pc5),
            5 => Some(Exti5Ss::Pf5),
            _ => None,
        }
    }
    #[doc = "PA5 pin"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == Exti5Ss::Pa5
    }
    #[doc = "PB5 pin"]
    #[inline(always)]
    pub fn is_pb5(&self) -> bool {
        *self == Exti5Ss::Pb5
    }
    #[doc = "PC5 pin"]
    #[inline(always)]
    pub fn is_pc5(&self) -> bool {
        *self == Exti5Ss::Pc5
    }
    #[doc = "PF5 pin"]
    #[inline(always)]
    pub fn is_pf5(&self) -> bool {
        *self == Exti5Ss::Pf5
    }
}
#[doc = "Field `EXTI5_SS` writer - EXTI 5 sources selection"]
pub type Exti5SsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Exti5Ss>;
impl<'a, REG> Exti5SsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA5 pin"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut crate::W<REG> {
        self.variant(Exti5Ss::Pa5)
    }
    #[doc = "PB5 pin"]
    #[inline(always)]
    pub fn pb5(self) -> &'a mut crate::W<REG> {
        self.variant(Exti5Ss::Pb5)
    }
    #[doc = "PC5 pin"]
    #[inline(always)]
    pub fn pc5(self) -> &'a mut crate::W<REG> {
        self.variant(Exti5Ss::Pc5)
    }
    #[doc = "PF5 pin"]
    #[inline(always)]
    pub fn pf5(self) -> &'a mut crate::W<REG> {
        self.variant(Exti5Ss::Pf5)
    }
}
#[doc = "EXTI 6 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti6Ss {
    #[doc = "0: PA6 pin"]
    Pa6 = 0,
    #[doc = "1: PB6 pin"]
    Pb6 = 1,
    #[doc = "2: PC6 pin"]
    Pc6 = 2,
    #[doc = "5: PF6 pin"]
    Pf6 = 5,
}
impl From<Exti6Ss> for u8 {
    #[inline(always)]
    fn from(variant: Exti6Ss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti6Ss {
    type Ux = u8;
}
#[doc = "Field `EXTI6_SS` reader - EXTI 6 sources selection"]
pub type Exti6SsR = crate::FieldReader<Exti6Ss>;
impl Exti6SsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti6Ss> {
        match self.bits {
            0 => Some(Exti6Ss::Pa6),
            1 => Some(Exti6Ss::Pb6),
            2 => Some(Exti6Ss::Pc6),
            5 => Some(Exti6Ss::Pf6),
            _ => None,
        }
    }
    #[doc = "PA6 pin"]
    #[inline(always)]
    pub fn is_pa6(&self) -> bool {
        *self == Exti6Ss::Pa6
    }
    #[doc = "PB6 pin"]
    #[inline(always)]
    pub fn is_pb6(&self) -> bool {
        *self == Exti6Ss::Pb6
    }
    #[doc = "PC6 pin"]
    #[inline(always)]
    pub fn is_pc6(&self) -> bool {
        *self == Exti6Ss::Pc6
    }
    #[doc = "PF6 pin"]
    #[inline(always)]
    pub fn is_pf6(&self) -> bool {
        *self == Exti6Ss::Pf6
    }
}
#[doc = "Field `EXTI6_SS` writer - EXTI 6 sources selection"]
pub type Exti6SsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Exti6Ss>;
impl<'a, REG> Exti6SsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA6 pin"]
    #[inline(always)]
    pub fn pa6(self) -> &'a mut crate::W<REG> {
        self.variant(Exti6Ss::Pa6)
    }
    #[doc = "PB6 pin"]
    #[inline(always)]
    pub fn pb6(self) -> &'a mut crate::W<REG> {
        self.variant(Exti6Ss::Pb6)
    }
    #[doc = "PC6 pin"]
    #[inline(always)]
    pub fn pc6(self) -> &'a mut crate::W<REG> {
        self.variant(Exti6Ss::Pc6)
    }
    #[doc = "PF6 pin"]
    #[inline(always)]
    pub fn pf6(self) -> &'a mut crate::W<REG> {
        self.variant(Exti6Ss::Pf6)
    }
}
#[doc = "EXTI 7 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti7Ss {
    #[doc = "0: PA7 pin"]
    Pa7 = 0,
    #[doc = "1: PB7 pin"]
    Pb7 = 1,
    #[doc = "2: PC7 pin"]
    Pc7 = 2,
    #[doc = "5: PF7 pin"]
    Pf7 = 5,
}
impl From<Exti7Ss> for u8 {
    #[inline(always)]
    fn from(variant: Exti7Ss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti7Ss {
    type Ux = u8;
}
#[doc = "Field `EXTI7_SS` reader - EXTI 7 sources selection"]
pub type Exti7SsR = crate::FieldReader<Exti7Ss>;
impl Exti7SsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti7Ss> {
        match self.bits {
            0 => Some(Exti7Ss::Pa7),
            1 => Some(Exti7Ss::Pb7),
            2 => Some(Exti7Ss::Pc7),
            5 => Some(Exti7Ss::Pf7),
            _ => None,
        }
    }
    #[doc = "PA7 pin"]
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        *self == Exti7Ss::Pa7
    }
    #[doc = "PB7 pin"]
    #[inline(always)]
    pub fn is_pb7(&self) -> bool {
        *self == Exti7Ss::Pb7
    }
    #[doc = "PC7 pin"]
    #[inline(always)]
    pub fn is_pc7(&self) -> bool {
        *self == Exti7Ss::Pc7
    }
    #[doc = "PF7 pin"]
    #[inline(always)]
    pub fn is_pf7(&self) -> bool {
        *self == Exti7Ss::Pf7
    }
}
#[doc = "Field `EXTI7_SS` writer - EXTI 7 sources selection"]
pub type Exti7SsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Exti7Ss>;
impl<'a, REG> Exti7SsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA7 pin"]
    #[inline(always)]
    pub fn pa7(self) -> &'a mut crate::W<REG> {
        self.variant(Exti7Ss::Pa7)
    }
    #[doc = "PB7 pin"]
    #[inline(always)]
    pub fn pb7(self) -> &'a mut crate::W<REG> {
        self.variant(Exti7Ss::Pb7)
    }
    #[doc = "PC7 pin"]
    #[inline(always)]
    pub fn pc7(self) -> &'a mut crate::W<REG> {
        self.variant(Exti7Ss::Pc7)
    }
    #[doc = "PF7 pin"]
    #[inline(always)]
    pub fn pf7(self) -> &'a mut crate::W<REG> {
        self.variant(Exti7Ss::Pf7)
    }
}
impl R {
    #[doc = "Bits 0:3 - EXTI 4 sources selection"]
    #[inline(always)]
    pub fn exti4_ss(&self) -> Exti4SsR {
        Exti4SsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 5 sources selection"]
    #[inline(always)]
    pub fn exti5_ss(&self) -> Exti5SsR {
        Exti5SsR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 6 sources selection"]
    #[inline(always)]
    pub fn exti6_ss(&self) -> Exti6SsR {
        Exti6SsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 7 sources selection"]
    #[inline(always)]
    pub fn exti7_ss(&self) -> Exti7SsR {
        Exti7SsR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 4 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti4_ss(&mut self) -> Exti4SsW<Extiss1Spec> {
        Exti4SsW::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXTI 5 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti5_ss(&mut self) -> Exti5SsW<Extiss1Spec> {
        Exti5SsW::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXTI 6 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti6_ss(&mut self) -> Exti6SsW<Extiss1Spec> {
        Exti6SsW::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXTI 7 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti7_ss(&mut self) -> Exti7SsW<Extiss1Spec> {
        Exti7SsW::new(self, 12)
    }
}
#[doc = "EXTI sources selection register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extiss1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extiss1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Extiss1Spec;
impl crate::RegisterSpec for Extiss1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extiss1::R`](R) reader structure"]
impl crate::Readable for Extiss1Spec {}
#[doc = "`write(|w| ..)` method takes [`extiss1::W`](W) writer structure"]
impl crate::Writable for Extiss1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTISS1 to value 0"]
impl crate::Resettable for Extiss1Spec {
    const RESET_VALUE: u32 = 0;
}
