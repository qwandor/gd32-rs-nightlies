#[doc = "Register `EXTISS0` reader"]
pub type R = crate::R<Extiss0Spec>;
#[doc = "Register `EXTISS0` writer"]
pub type W = crate::W<Extiss0Spec>;
#[doc = "EXTI 0 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti0Ss {
    #[doc = "0: PA0 pin"]
    Pa0 = 0,
    #[doc = "1: PB0 pin"]
    Pb0 = 1,
    #[doc = "2: PC0 pin"]
    Pc0 = 2,
    #[doc = "5: PF0 pin"]
    Pf0 = 5,
}
impl From<Exti0Ss> for u8 {
    #[inline(always)]
    fn from(variant: Exti0Ss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti0Ss {
    type Ux = u8;
}
#[doc = "Field `EXTI0_SS` reader - EXTI 0 sources selection"]
pub type Exti0SsR = crate::FieldReader<Exti0Ss>;
impl Exti0SsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti0Ss> {
        match self.bits {
            0 => Some(Exti0Ss::Pa0),
            1 => Some(Exti0Ss::Pb0),
            2 => Some(Exti0Ss::Pc0),
            5 => Some(Exti0Ss::Pf0),
            _ => None,
        }
    }
    #[doc = "PA0 pin"]
    #[inline(always)]
    pub fn is_pa0(&self) -> bool {
        *self == Exti0Ss::Pa0
    }
    #[doc = "PB0 pin"]
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == Exti0Ss::Pb0
    }
    #[doc = "PC0 pin"]
    #[inline(always)]
    pub fn is_pc0(&self) -> bool {
        *self == Exti0Ss::Pc0
    }
    #[doc = "PF0 pin"]
    #[inline(always)]
    pub fn is_pf0(&self) -> bool {
        *self == Exti0Ss::Pf0
    }
}
#[doc = "Field `EXTI0_SS` writer - EXTI 0 sources selection"]
pub type Exti0SsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Exti0Ss>;
impl<'a, REG> Exti0SsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA0 pin"]
    #[inline(always)]
    pub fn pa0(self) -> &'a mut crate::W<REG> {
        self.variant(Exti0Ss::Pa0)
    }
    #[doc = "PB0 pin"]
    #[inline(always)]
    pub fn pb0(self) -> &'a mut crate::W<REG> {
        self.variant(Exti0Ss::Pb0)
    }
    #[doc = "PC0 pin"]
    #[inline(always)]
    pub fn pc0(self) -> &'a mut crate::W<REG> {
        self.variant(Exti0Ss::Pc0)
    }
    #[doc = "PF0 pin"]
    #[inline(always)]
    pub fn pf0(self) -> &'a mut crate::W<REG> {
        self.variant(Exti0Ss::Pf0)
    }
}
#[doc = "EXTI 1 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti1Ss {
    #[doc = "0: PA1 pin"]
    Pa1 = 0,
    #[doc = "1: PB1 pin"]
    Pb1 = 1,
    #[doc = "2: PC1 pin"]
    Pc1 = 2,
    #[doc = "5: PF1 pin"]
    Pf1 = 5,
}
impl From<Exti1Ss> for u8 {
    #[inline(always)]
    fn from(variant: Exti1Ss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti1Ss {
    type Ux = u8;
}
#[doc = "Field `EXTI1_SS` reader - EXTI 1 sources selection"]
pub type Exti1SsR = crate::FieldReader<Exti1Ss>;
impl Exti1SsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti1Ss> {
        match self.bits {
            0 => Some(Exti1Ss::Pa1),
            1 => Some(Exti1Ss::Pb1),
            2 => Some(Exti1Ss::Pc1),
            5 => Some(Exti1Ss::Pf1),
            _ => None,
        }
    }
    #[doc = "PA1 pin"]
    #[inline(always)]
    pub fn is_pa1(&self) -> bool {
        *self == Exti1Ss::Pa1
    }
    #[doc = "PB1 pin"]
    #[inline(always)]
    pub fn is_pb1(&self) -> bool {
        *self == Exti1Ss::Pb1
    }
    #[doc = "PC1 pin"]
    #[inline(always)]
    pub fn is_pc1(&self) -> bool {
        *self == Exti1Ss::Pc1
    }
    #[doc = "PF1 pin"]
    #[inline(always)]
    pub fn is_pf1(&self) -> bool {
        *self == Exti1Ss::Pf1
    }
}
#[doc = "Field `EXTI1_SS` writer - EXTI 1 sources selection"]
pub type Exti1SsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Exti1Ss>;
impl<'a, REG> Exti1SsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA1 pin"]
    #[inline(always)]
    pub fn pa1(self) -> &'a mut crate::W<REG> {
        self.variant(Exti1Ss::Pa1)
    }
    #[doc = "PB1 pin"]
    #[inline(always)]
    pub fn pb1(self) -> &'a mut crate::W<REG> {
        self.variant(Exti1Ss::Pb1)
    }
    #[doc = "PC1 pin"]
    #[inline(always)]
    pub fn pc1(self) -> &'a mut crate::W<REG> {
        self.variant(Exti1Ss::Pc1)
    }
    #[doc = "PF1 pin"]
    #[inline(always)]
    pub fn pf1(self) -> &'a mut crate::W<REG> {
        self.variant(Exti1Ss::Pf1)
    }
}
#[doc = "EXTI 2 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti2Ss {
    #[doc = "0: PA2 pin"]
    Pa2 = 0,
    #[doc = "1: PB2 pin"]
    Pb2 = 1,
    #[doc = "2: PC2 pin"]
    Pc2 = 2,
    #[doc = "3: PD2 pin"]
    Pd2 = 3,
}
impl From<Exti2Ss> for u8 {
    #[inline(always)]
    fn from(variant: Exti2Ss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti2Ss {
    type Ux = u8;
}
#[doc = "Field `EXTI2_SS` reader - EXTI 2 sources selection"]
pub type Exti2SsR = crate::FieldReader<Exti2Ss>;
impl Exti2SsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti2Ss> {
        match self.bits {
            0 => Some(Exti2Ss::Pa2),
            1 => Some(Exti2Ss::Pb2),
            2 => Some(Exti2Ss::Pc2),
            3 => Some(Exti2Ss::Pd2),
            _ => None,
        }
    }
    #[doc = "PA2 pin"]
    #[inline(always)]
    pub fn is_pa2(&self) -> bool {
        *self == Exti2Ss::Pa2
    }
    #[doc = "PB2 pin"]
    #[inline(always)]
    pub fn is_pb2(&self) -> bool {
        *self == Exti2Ss::Pb2
    }
    #[doc = "PC2 pin"]
    #[inline(always)]
    pub fn is_pc2(&self) -> bool {
        *self == Exti2Ss::Pc2
    }
    #[doc = "PD2 pin"]
    #[inline(always)]
    pub fn is_pd2(&self) -> bool {
        *self == Exti2Ss::Pd2
    }
}
#[doc = "Field `EXTI2_SS` writer - EXTI 2 sources selection"]
pub type Exti2SsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Exti2Ss>;
impl<'a, REG> Exti2SsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA2 pin"]
    #[inline(always)]
    pub fn pa2(self) -> &'a mut crate::W<REG> {
        self.variant(Exti2Ss::Pa2)
    }
    #[doc = "PB2 pin"]
    #[inline(always)]
    pub fn pb2(self) -> &'a mut crate::W<REG> {
        self.variant(Exti2Ss::Pb2)
    }
    #[doc = "PC2 pin"]
    #[inline(always)]
    pub fn pc2(self) -> &'a mut crate::W<REG> {
        self.variant(Exti2Ss::Pc2)
    }
    #[doc = "PD2 pin"]
    #[inline(always)]
    pub fn pd2(self) -> &'a mut crate::W<REG> {
        self.variant(Exti2Ss::Pd2)
    }
}
#[doc = "EXTI 3 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti3Ss {
    #[doc = "0: PA3 pin"]
    Pa3 = 0,
    #[doc = "1: PB3 pin"]
    Pb3 = 1,
    #[doc = "2: PC3 pin"]
    Pc3 = 2,
}
impl From<Exti3Ss> for u8 {
    #[inline(always)]
    fn from(variant: Exti3Ss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti3Ss {
    type Ux = u8;
}
#[doc = "Field `EXTI3_SS` reader - EXTI 3 sources selection"]
pub type Exti3SsR = crate::FieldReader<Exti3Ss>;
impl Exti3SsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti3Ss> {
        match self.bits {
            0 => Some(Exti3Ss::Pa3),
            1 => Some(Exti3Ss::Pb3),
            2 => Some(Exti3Ss::Pc3),
            _ => None,
        }
    }
    #[doc = "PA3 pin"]
    #[inline(always)]
    pub fn is_pa3(&self) -> bool {
        *self == Exti3Ss::Pa3
    }
    #[doc = "PB3 pin"]
    #[inline(always)]
    pub fn is_pb3(&self) -> bool {
        *self == Exti3Ss::Pb3
    }
    #[doc = "PC3 pin"]
    #[inline(always)]
    pub fn is_pc3(&self) -> bool {
        *self == Exti3Ss::Pc3
    }
}
#[doc = "Field `EXTI3_SS` writer - EXTI 3 sources selection"]
pub type Exti3SsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Exti3Ss>;
impl<'a, REG> Exti3SsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA3 pin"]
    #[inline(always)]
    pub fn pa3(self) -> &'a mut crate::W<REG> {
        self.variant(Exti3Ss::Pa3)
    }
    #[doc = "PB3 pin"]
    #[inline(always)]
    pub fn pb3(self) -> &'a mut crate::W<REG> {
        self.variant(Exti3Ss::Pb3)
    }
    #[doc = "PC3 pin"]
    #[inline(always)]
    pub fn pc3(self) -> &'a mut crate::W<REG> {
        self.variant(Exti3Ss::Pc3)
    }
}
impl R {
    #[doc = "Bits 0:3 - EXTI 0 sources selection"]
    #[inline(always)]
    pub fn exti0_ss(&self) -> Exti0SsR {
        Exti0SsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 1 sources selection"]
    #[inline(always)]
    pub fn exti1_ss(&self) -> Exti1SsR {
        Exti1SsR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 2 sources selection"]
    #[inline(always)]
    pub fn exti2_ss(&self) -> Exti2SsR {
        Exti2SsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 3 sources selection"]
    #[inline(always)]
    pub fn exti3_ss(&self) -> Exti3SsR {
        Exti3SsR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 0 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti0_ss(&mut self) -> Exti0SsW<Extiss0Spec> {
        Exti0SsW::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXTI 1 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti1_ss(&mut self) -> Exti1SsW<Extiss0Spec> {
        Exti1SsW::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXTI 2 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti2_ss(&mut self) -> Exti2SsW<Extiss0Spec> {
        Exti2SsW::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXTI 3 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti3_ss(&mut self) -> Exti3SsW<Extiss0Spec> {
        Exti3SsW::new(self, 12)
    }
}
#[doc = "EXTI sources selection register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extiss0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extiss0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Extiss0Spec;
impl crate::RegisterSpec for Extiss0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extiss0::R`](R) reader structure"]
impl crate::Readable for Extiss0Spec {}
#[doc = "`write(|w| ..)` method takes [`extiss0::W`](W) writer structure"]
impl crate::Writable for Extiss0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTISS0 to value 0"]
impl crate::Resettable for Extiss0Spec {
    const RESET_VALUE: u32 = 0;
}
