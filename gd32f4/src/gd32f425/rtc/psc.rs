#[doc = "Register `PSC` reader"]
pub type R = crate::R<PscSpec>;
#[doc = "Register `PSC` writer"]
pub type W = crate::W<PscSpec>;
#[doc = "Field `FACTOR_S` reader - Synchronous prescaler factor"]
pub type FactorSR = crate::FieldReader<u16>;
#[doc = "Field `FACTOR_S` writer - Synchronous prescaler factor"]
pub type FactorSW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `FACTOR_A` reader - Asynchronous prescaler factor"]
pub type FactorAR = crate::FieldReader;
#[doc = "Field `FACTOR_A` writer - Asynchronous prescaler factor"]
pub type FactorAW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:14 - Synchronous prescaler factor"]
    #[inline(always)]
    pub fn factor_s(&self) -> FactorSR {
        FactorSR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:22 - Asynchronous prescaler factor"]
    #[inline(always)]
    pub fn factor_a(&self) -> FactorAR {
        FactorAR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - Synchronous prescaler factor"]
    #[inline(always)]
    #[must_use]
    pub fn factor_s(&mut self) -> FactorSW<PscSpec> {
        FactorSW::new(self, 0)
    }
    #[doc = "Bits 16:22 - Asynchronous prescaler factor"]
    #[inline(always)]
    #[must_use]
    pub fn factor_a(&mut self) -> FactorAW<PscSpec> {
        FactorAW::new(self, 16)
    }
}
#[doc = "prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PscSpec;
impl crate::RegisterSpec for PscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psc::R`](R) reader structure"]
impl crate::Readable for PscSpec {}
#[doc = "`write(|w| ..)` method takes [`psc::W`](W) writer structure"]
impl crate::Writable for PscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSC to value 0x007f_00ff"]
impl crate::Resettable for PscSpec {
    const RESET_VALUE: u32 = 0x007f_00ff;
}
