#[doc = "Register `BKP17` reader"]
pub type R = crate::R<Bkp17Spec>;
#[doc = "Register `BKP17` writer"]
pub type W = crate::W<Bkp17Spec>;
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
    pub fn data(&mut self) -> DataW<Bkp17Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp17::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp17::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bkp17Spec;
impl crate::RegisterSpec for Bkp17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp17::R`](R) reader structure"]
impl crate::Readable for Bkp17Spec {}
#[doc = "`write(|w| ..)` method takes [`bkp17::W`](W) writer structure"]
impl crate::Writable for Bkp17Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BKP17 to value 0"]
impl crate::Resettable for Bkp17Spec {
    const RESET_VALUE: u32 = 0;
}