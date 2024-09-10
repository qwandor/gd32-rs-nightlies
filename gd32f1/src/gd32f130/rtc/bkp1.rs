#[doc = "Register `BKP1` reader"]
pub type R = crate::R<Bkp1Spec>;
#[doc = "Register `BKP1` writer"]
pub type W = crate::W<Bkp1Spec>;
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
    pub fn data(&mut self) -> DataW<Bkp1Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bkp1Spec;
impl crate::RegisterSpec for Bkp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp1::R`](R) reader structure"]
impl crate::Readable for Bkp1Spec {}
#[doc = "`write(|w| ..)` method takes [`bkp1::W`](W) writer structure"]
impl crate::Writable for Bkp1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BKP1 to value 0"]
impl crate::Resettable for Bkp1Spec {
    const RESET_VALUE: u32 = 0;
}
