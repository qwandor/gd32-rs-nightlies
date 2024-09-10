#[doc = "Register `DO1` reader"]
pub type R = crate::R<Do1Spec>;
#[doc = "Field `DO1` reader - message digest result of hash algorithm"]
pub type Do1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - message digest result of hash algorithm"]
    #[inline(always)]
    pub fn do1(&self) -> Do1R {
        Do1R::new(self.bits)
    }
}
#[doc = "HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Do1Spec;
impl crate::RegisterSpec for Do1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`do1::R`](R) reader structure"]
impl crate::Readable for Do1Spec {}
#[doc = "`reset()` method sets DO1 to value 0"]
impl crate::Resettable for Do1Spec {
    const RESET_VALUE: u32 = 0;
}
