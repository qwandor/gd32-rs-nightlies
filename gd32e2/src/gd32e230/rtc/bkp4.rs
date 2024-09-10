#[doc = "Register `BKP4` reader"]
pub type R = crate::R<Bkp4Spec>;
#[doc = "Register `BKP4` writer"]
pub type W = crate::W<Bkp4Spec>;
#[doc = "Field `DATA` reader - BKP data"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - BKP data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - BKP data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BKP data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Bkp4Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bkp4Spec;
impl crate::RegisterSpec for Bkp4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp4::R`](R) reader structure"]
impl crate::Readable for Bkp4Spec {}
#[doc = "`write(|w| ..)` method takes [`bkp4::W`](W) writer structure"]
impl crate::Writable for Bkp4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BKP4 to value 0"]
impl crate::Resettable for Bkp4Spec {
    const RESET_VALUE: u32 = 0;
}
