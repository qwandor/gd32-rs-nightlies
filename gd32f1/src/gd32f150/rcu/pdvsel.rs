#[doc = "Register `PDVSEL` reader"]
pub type R = crate::R<PdvselSpec>;
#[doc = "Register `PDVSEL` writer"]
pub type W = crate::W<PdvselSpec>;
#[doc = "Power down voltage select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdrvs {
    #[doc = "0: The power down voltage is 2.6 V"]
    V2_6 = 0,
    #[doc = "1: The power down voltage is 1.8 V"]
    V1_8 = 1,
}
impl From<Pdrvs> for bool {
    #[inline(always)]
    fn from(variant: Pdrvs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDRVS` reader - Power down voltage select"]
pub type PdrvsR = crate::BitReader<Pdrvs>;
impl PdrvsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdrvs {
        match self.bits {
            false => Pdrvs::V2_6,
            true => Pdrvs::V1_8,
        }
    }
    #[doc = "The power down voltage is 2.6 V"]
    #[inline(always)]
    pub fn is_v2_6(&self) -> bool {
        *self == Pdrvs::V2_6
    }
    #[doc = "The power down voltage is 1.8 V"]
    #[inline(always)]
    pub fn is_v1_8(&self) -> bool {
        *self == Pdrvs::V1_8
    }
}
#[doc = "Field `PDRVS` writer - Power down voltage select"]
pub type PdrvsW<'a, REG> = crate::BitWriter<'a, REG, Pdrvs>;
impl<'a, REG> PdrvsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The power down voltage is 2.6 V"]
    #[inline(always)]
    pub fn v2_6(self) -> &'a mut crate::W<REG> {
        self.variant(Pdrvs::V2_6)
    }
    #[doc = "The power down voltage is 1.8 V"]
    #[inline(always)]
    pub fn v1_8(self) -> &'a mut crate::W<REG> {
        self.variant(Pdrvs::V1_8)
    }
}
impl R {
    #[doc = "Bit 0 - Power down voltage select"]
    #[inline(always)]
    pub fn pdrvs(&self) -> PdrvsR {
        PdrvsR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power down voltage select"]
    #[inline(always)]
    #[must_use]
    pub fn pdrvs(&mut self) -> PdrvsW<PdvselSpec> {
        PdrvsW::new(self, 0)
    }
}
#[doc = "Power down voltage select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdvsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdvsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdvselSpec;
impl crate::RegisterSpec for PdvselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdvsel::R`](R) reader structure"]
impl crate::Readable for PdvselSpec {}
#[doc = "`write(|w| ..)` method takes [`pdvsel::W`](W) writer structure"]
impl crate::Writable for PdvselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDVSEL to value 0"]
impl crate::Resettable for PdvselSpec {
    const RESET_VALUE: u32 = 0;
}
