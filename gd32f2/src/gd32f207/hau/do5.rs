#[doc = "Register `DO5` reader"]
pub type R = crate::R<Do5Spec>;
#[doc = "Field `DO5` reader - message digest result of hash algorithm"]
pub type Do5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - message digest result of hash algorithm"]
    #[inline(always)]
    pub fn do5(&self) -> Do5R {
        Do5R::new(self.bits)
    }
}
#[doc = "HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Do5Spec;
impl crate::RegisterSpec for Do5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`do5::R`](R) reader structure"]
impl crate::Readable for Do5Spec {}
#[doc = "`reset()` method sets DO5 to value 0"]
impl crate::Resettable for Do5Spec {
    const RESET_VALUE: u32 = 0;
}
