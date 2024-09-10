#[doc = "Register `DO3` reader"]
pub type R = crate::R<Do3Spec>;
#[doc = "Field `DO3` reader - message digest result of hash algorithm"]
pub type Do3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - message digest result of hash algorithm"]
    #[inline(always)]
    pub fn do3(&self) -> Do3R {
        Do3R::new(self.bits)
    }
}
#[doc = "HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Do3Spec;
impl crate::RegisterSpec for Do3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`do3::R`](R) reader structure"]
impl crate::Readable for Do3Spec {}
#[doc = "`reset()` method sets DO3 to value 0"]
impl crate::Resettable for Do3Spec {
    const RESET_VALUE: u32 = 0;
}
