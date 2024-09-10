#[doc = "Register `BKP0` reader"]
pub type R = crate::R<Bkp0Spec>;
#[doc = "Register `BKP0` writer"]
pub type W = crate::W<Bkp0Spec>;
#[doc = "Field `DATA` reader - Backup domain registers"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Backup domain registers"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Backup domain registers"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Backup domain registers"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Bkp0Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bkp0Spec;
impl crate::RegisterSpec for Bkp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp0::R`](R) reader structure"]
impl crate::Readable for Bkp0Spec {}
#[doc = "`write(|w| ..)` method takes [`bkp0::W`](W) writer structure"]
impl crate::Writable for Bkp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BKP0 to value 0"]
impl crate::Resettable for Bkp0Spec {
    const RESET_VALUE: u32 = 0;
}
