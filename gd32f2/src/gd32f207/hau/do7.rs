#[doc = "Register `DO7` reader"]
pub type R = crate::R<Do7Spec>;
#[doc = "Field `DO7` reader - message digest result of hash algorithm"]
pub type Do7R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - message digest result of hash algorithm"]
    #[inline(always)]
    pub fn do7(&self) -> Do7R {
        Do7R::new(self.bits)
    }
}
#[doc = "HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Do7Spec;
impl crate::RegisterSpec for Do7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`do7::R`](R) reader structure"]
impl crate::Readable for Do7Spec {}
#[doc = "`reset()` method sets DO7 to value 0"]
impl crate::Resettable for Do7Spec {
    const RESET_VALUE: u32 = 0;
}
