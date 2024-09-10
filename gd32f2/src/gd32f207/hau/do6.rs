#[doc = "Register `DO6` reader"]
pub type R = crate::R<Do6Spec>;
#[doc = "Field `DO6` reader - message digest result of hash algorithm"]
pub type Do6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - message digest result of hash algorithm"]
    #[inline(always)]
    pub fn do6(&self) -> Do6R {
        Do6R::new(self.bits)
    }
}
#[doc = "HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Do6Spec;
impl crate::RegisterSpec for Do6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`do6::R`](R) reader structure"]
impl crate::Readable for Do6Spec {}
#[doc = "`reset()` method sets DO6 to value 0"]
impl crate::Resettable for Do6Spec {
    const RESET_VALUE: u32 = 0;
}
