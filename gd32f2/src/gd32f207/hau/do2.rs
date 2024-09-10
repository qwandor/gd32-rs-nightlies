#[doc = "Register `DO2` reader"]
pub type R = crate::R<Do2Spec>;
#[doc = "Field `DO2` reader - message digest result of hash algorithm"]
pub type Do2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - message digest result of hash algorithm"]
    #[inline(always)]
    pub fn do2(&self) -> Do2R {
        Do2R::new(self.bits)
    }
}
#[doc = "HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Do2Spec;
impl crate::RegisterSpec for Do2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`do2::R`](R) reader structure"]
impl crate::Readable for Do2Spec {}
#[doc = "`reset()` method sets DO2 to value 0"]
impl crate::Resettable for Do2Spec {
    const RESET_VALUE: u32 = 0;
}
