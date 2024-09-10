#[doc = "Register `DSV` reader"]
pub type R = crate::R<DsvSpec>;
#[doc = "Register `DSV` writer"]
pub type W = crate::W<DsvSpec>;
#[doc = "Deep-sleep mode voltage select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dslpvs {
    #[doc = "0: The core voltage is 1.2 V in deep-sleep mode"]
    V1_2 = 0,
    #[doc = "1: The core voltage is 1.1 V in deep-sleep mode"]
    V1_1 = 1,
    #[doc = "2: The core voltage is 1.0 V in deep-sleep mode"]
    V1_0 = 2,
    #[doc = "3: The core voltage is 0.9 V in deep-sleep mode"]
    V0_9 = 3,
}
impl From<Dslpvs> for u8 {
    #[inline(always)]
    fn from(variant: Dslpvs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dslpvs {
    type Ux = u8;
}
#[doc = "Field `DSLPVS` reader - Deep-sleep mode voltage select"]
pub type DslpvsR = crate::FieldReader<Dslpvs>;
impl DslpvsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dslpvs> {
        match self.bits {
            0 => Some(Dslpvs::V1_2),
            1 => Some(Dslpvs::V1_1),
            2 => Some(Dslpvs::V1_0),
            3 => Some(Dslpvs::V0_9),
            _ => None,
        }
    }
    #[doc = "The core voltage is 1.2 V in deep-sleep mode"]
    #[inline(always)]
    pub fn is_v1_2(&self) -> bool {
        *self == Dslpvs::V1_2
    }
    #[doc = "The core voltage is 1.1 V in deep-sleep mode"]
    #[inline(always)]
    pub fn is_v1_1(&self) -> bool {
        *self == Dslpvs::V1_1
    }
    #[doc = "The core voltage is 1.0 V in deep-sleep mode"]
    #[inline(always)]
    pub fn is_v1_0(&self) -> bool {
        *self == Dslpvs::V1_0
    }
    #[doc = "The core voltage is 0.9 V in deep-sleep mode"]
    #[inline(always)]
    pub fn is_v0_9(&self) -> bool {
        *self == Dslpvs::V0_9
    }
}
#[doc = "Field `DSLPVS` writer - Deep-sleep mode voltage select"]
pub type DslpvsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dslpvs>;
impl<'a, REG> DslpvsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The core voltage is 1.2 V in deep-sleep mode"]
    #[inline(always)]
    pub fn v1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dslpvs::V1_2)
    }
    #[doc = "The core voltage is 1.1 V in deep-sleep mode"]
    #[inline(always)]
    pub fn v1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dslpvs::V1_1)
    }
    #[doc = "The core voltage is 1.0 V in deep-sleep mode"]
    #[inline(always)]
    pub fn v1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dslpvs::V1_0)
    }
    #[doc = "The core voltage is 0.9 V in deep-sleep mode"]
    #[inline(always)]
    pub fn v0_9(self) -> &'a mut crate::W<REG> {
        self.variant(Dslpvs::V0_9)
    }
}
impl R {
    #[doc = "Bits 0:2 - Deep-sleep mode voltage select"]
    #[inline(always)]
    pub fn dslpvs(&self) -> DslpvsR {
        DslpvsR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Deep-sleep mode voltage select"]
    #[inline(always)]
    #[must_use]
    pub fn dslpvs(&mut self) -> DslpvsW<DsvSpec> {
        DslpvsW::new(self, 0)
    }
}
#[doc = "Deep-sleep mode voltage register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsvSpec;
impl crate::RegisterSpec for DsvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsv::R`](R) reader structure"]
impl crate::Readable for DsvSpec {}
#[doc = "`write(|w| ..)` method takes [`dsv::W`](W) writer structure"]
impl crate::Writable for DsvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSV to value 0"]
impl crate::Resettable for DsvSpec {
    const RESET_VALUE: u32 = 0;
}
