#[doc = "Register `EXTISS3` reader"]
pub type R = crate::R<Extiss3Spec>;
#[doc = "Register `EXTISS3` writer"]
pub type W = crate::W<Extiss3Spec>;
#[doc = "EXTI 12 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti12Ss {
    #[doc = "0: PA12 pin"]
    Pa12 = 0,
    #[doc = "1: PB12 pin"]
    Pb12 = 1,
    #[doc = "2: PC12 pin"]
    Pc12 = 2,
}
impl From<Exti12Ss> for u8 {
    #[inline(always)]
    fn from(variant: Exti12Ss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti12Ss {
    type Ux = u8;
}
#[doc = "Field `EXTI12_SS` reader - EXTI 12 sources selection"]
pub type Exti12SsR = crate::FieldReader<Exti12Ss>;
impl Exti12SsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti12Ss> {
        match self.bits {
            0 => Some(Exti12Ss::Pa12),
            1 => Some(Exti12Ss::Pb12),
            2 => Some(Exti12Ss::Pc12),
            _ => None,
        }
    }
    #[doc = "PA12 pin"]
    #[inline(always)]
    pub fn is_pa12(&self) -> bool {
        *self == Exti12Ss::Pa12
    }
    #[doc = "PB12 pin"]
    #[inline(always)]
    pub fn is_pb12(&self) -> bool {
        *self == Exti12Ss::Pb12
    }
    #[doc = "PC12 pin"]
    #[inline(always)]
    pub fn is_pc12(&self) -> bool {
        *self == Exti12Ss::Pc12
    }
}
#[doc = "Field `EXTI12_SS` writer - EXTI 12 sources selection"]
pub type Exti12SsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Exti12Ss>;
impl<'a, REG> Exti12SsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA12 pin"]
    #[inline(always)]
    pub fn pa12(self) -> &'a mut crate::W<REG> {
        self.variant(Exti12Ss::Pa12)
    }
    #[doc = "PB12 pin"]
    #[inline(always)]
    pub fn pb12(self) -> &'a mut crate::W<REG> {
        self.variant(Exti12Ss::Pb12)
    }
    #[doc = "PC12 pin"]
    #[inline(always)]
    pub fn pc12(self) -> &'a mut crate::W<REG> {
        self.variant(Exti12Ss::Pc12)
    }
}
#[doc = "EXTI 13 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti13Ss {
    #[doc = "0: PA13 pin"]
    Pa13 = 0,
    #[doc = "1: PB13 pin"]
    Pb13 = 1,
    #[doc = "2: PC13 pin"]
    Pc13 = 2,
}
impl From<Exti13Ss> for u8 {
    #[inline(always)]
    fn from(variant: Exti13Ss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti13Ss {
    type Ux = u8;
}
#[doc = "Field `EXTI13_SS` reader - EXTI 13 sources selection"]
pub type Exti13SsR = crate::FieldReader<Exti13Ss>;
impl Exti13SsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti13Ss> {
        match self.bits {
            0 => Some(Exti13Ss::Pa13),
            1 => Some(Exti13Ss::Pb13),
            2 => Some(Exti13Ss::Pc13),
            _ => None,
        }
    }
    #[doc = "PA13 pin"]
    #[inline(always)]
    pub fn is_pa13(&self) -> bool {
        *self == Exti13Ss::Pa13
    }
    #[doc = "PB13 pin"]
    #[inline(always)]
    pub fn is_pb13(&self) -> bool {
        *self == Exti13Ss::Pb13
    }
    #[doc = "PC13 pin"]
    #[inline(always)]
    pub fn is_pc13(&self) -> bool {
        *self == Exti13Ss::Pc13
    }
}
#[doc = "Field `EXTI13_SS` writer - EXTI 13 sources selection"]
pub type Exti13SsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Exti13Ss>;
impl<'a, REG> Exti13SsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA13 pin"]
    #[inline(always)]
    pub fn pa13(self) -> &'a mut crate::W<REG> {
        self.variant(Exti13Ss::Pa13)
    }
    #[doc = "PB13 pin"]
    #[inline(always)]
    pub fn pb13(self) -> &'a mut crate::W<REG> {
        self.variant(Exti13Ss::Pb13)
    }
    #[doc = "PC13 pin"]
    #[inline(always)]
    pub fn pc13(self) -> &'a mut crate::W<REG> {
        self.variant(Exti13Ss::Pc13)
    }
}
#[doc = "EXTI 14 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti14Ss {
    #[doc = "0: PA14 pin"]
    Pa14 = 0,
    #[doc = "1: PB14 pin"]
    Pb14 = 1,
    #[doc = "2: PC14 pin"]
    Pc14 = 2,
}
impl From<Exti14Ss> for u8 {
    #[inline(always)]
    fn from(variant: Exti14Ss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti14Ss {
    type Ux = u8;
}
#[doc = "Field `EXTI14_SS` reader - EXTI 14 sources selection"]
pub type Exti14SsR = crate::FieldReader<Exti14Ss>;
impl Exti14SsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti14Ss> {
        match self.bits {
            0 => Some(Exti14Ss::Pa14),
            1 => Some(Exti14Ss::Pb14),
            2 => Some(Exti14Ss::Pc14),
            _ => None,
        }
    }
    #[doc = "PA14 pin"]
    #[inline(always)]
    pub fn is_pa14(&self) -> bool {
        *self == Exti14Ss::Pa14
    }
    #[doc = "PB14 pin"]
    #[inline(always)]
    pub fn is_pb14(&self) -> bool {
        *self == Exti14Ss::Pb14
    }
    #[doc = "PC14 pin"]
    #[inline(always)]
    pub fn is_pc14(&self) -> bool {
        *self == Exti14Ss::Pc14
    }
}
#[doc = "Field `EXTI14_SS` writer - EXTI 14 sources selection"]
pub type Exti14SsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Exti14Ss>;
impl<'a, REG> Exti14SsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA14 pin"]
    #[inline(always)]
    pub fn pa14(self) -> &'a mut crate::W<REG> {
        self.variant(Exti14Ss::Pa14)
    }
    #[doc = "PB14 pin"]
    #[inline(always)]
    pub fn pb14(self) -> &'a mut crate::W<REG> {
        self.variant(Exti14Ss::Pb14)
    }
    #[doc = "PC14 pin"]
    #[inline(always)]
    pub fn pc14(self) -> &'a mut crate::W<REG> {
        self.variant(Exti14Ss::Pc14)
    }
}
#[doc = "EXTI 15 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti15Ss {
    #[doc = "0: PA15 pin"]
    Pa15 = 0,
    #[doc = "1: PB15 pin"]
    Pb15 = 1,
    #[doc = "2: PC15 pin"]
    Pc15 = 2,
}
impl From<Exti15Ss> for u8 {
    #[inline(always)]
    fn from(variant: Exti15Ss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti15Ss {
    type Ux = u8;
}
#[doc = "Field `EXTI15_SS` reader - EXTI 15 sources selection"]
pub type Exti15SsR = crate::FieldReader<Exti15Ss>;
impl Exti15SsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti15Ss> {
        match self.bits {
            0 => Some(Exti15Ss::Pa15),
            1 => Some(Exti15Ss::Pb15),
            2 => Some(Exti15Ss::Pc15),
            _ => None,
        }
    }
    #[doc = "PA15 pin"]
    #[inline(always)]
    pub fn is_pa15(&self) -> bool {
        *self == Exti15Ss::Pa15
    }
    #[doc = "PB15 pin"]
    #[inline(always)]
    pub fn is_pb15(&self) -> bool {
        *self == Exti15Ss::Pb15
    }
    #[doc = "PC15 pin"]
    #[inline(always)]
    pub fn is_pc15(&self) -> bool {
        *self == Exti15Ss::Pc15
    }
}
#[doc = "Field `EXTI15_SS` writer - EXTI 15 sources selection"]
pub type Exti15SsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Exti15Ss>;
impl<'a, REG> Exti15SsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA15 pin"]
    #[inline(always)]
    pub fn pa15(self) -> &'a mut crate::W<REG> {
        self.variant(Exti15Ss::Pa15)
    }
    #[doc = "PB15 pin"]
    #[inline(always)]
    pub fn pb15(self) -> &'a mut crate::W<REG> {
        self.variant(Exti15Ss::Pb15)
    }
    #[doc = "PC15 pin"]
    #[inline(always)]
    pub fn pc15(self) -> &'a mut crate::W<REG> {
        self.variant(Exti15Ss::Pc15)
    }
}
impl R {
    #[doc = "Bits 0:3 - EXTI 12 sources selection"]
    #[inline(always)]
    pub fn exti12_ss(&self) -> Exti12SsR {
        Exti12SsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 13 sources selection"]
    #[inline(always)]
    pub fn exti13_ss(&self) -> Exti13SsR {
        Exti13SsR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 14 sources selection"]
    #[inline(always)]
    pub fn exti14_ss(&self) -> Exti14SsR {
        Exti14SsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 15 sources selection"]
    #[inline(always)]
    pub fn exti15_ss(&self) -> Exti15SsR {
        Exti15SsR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 12 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti12_ss(&mut self) -> Exti12SsW<Extiss3Spec> {
        Exti12SsW::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXTI 13 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti13_ss(&mut self) -> Exti13SsW<Extiss3Spec> {
        Exti13SsW::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXTI 14 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti14_ss(&mut self) -> Exti14SsW<Extiss3Spec> {
        Exti14SsW::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXTI 15 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti15_ss(&mut self) -> Exti15SsW<Extiss3Spec> {
        Exti15SsW::new(self, 12)
    }
}
#[doc = "EXTI sources selection register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extiss3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extiss3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Extiss3Spec;
impl crate::RegisterSpec for Extiss3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extiss3::R`](R) reader structure"]
impl crate::Readable for Extiss3Spec {}
#[doc = "`write(|w| ..)` method takes [`extiss3::W`](W) writer structure"]
impl crate::Writable for Extiss3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTISS3 to value 0"]
impl crate::Resettable for Extiss3Spec {
    const RESET_VALUE: u32 = 0;
}
