#[doc = "Register `BKP6` reader"]
pub type R = crate::R<Bkp6Spec>;
#[doc = "Register `BKP6` writer"]
pub type W = crate::W<Bkp6Spec>;
#[doc = "Field `DATA` reader - Data"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Bkp6Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bkp6Spec;
impl crate::RegisterSpec for Bkp6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp6::R`](R) reader structure"]
impl crate::Readable for Bkp6Spec {}
#[doc = "`write(|w| ..)` method takes [`bkp6::W`](W) writer structure"]
impl crate::Writable for Bkp6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BKP6 to value 0"]
impl crate::Resettable for Bkp6Spec {
    const RESET_VALUE: u32 = 0;
}
