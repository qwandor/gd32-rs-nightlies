#[doc = "Register `DO4` reader"]
pub type R = crate::R<Do4Spec>;
#[doc = "Field `DO4` reader - message digest result of hash algorithm"]
pub type Do4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - message digest result of hash algorithm"]
    #[inline(always)]
    pub fn do4(&self) -> Do4R {
        Do4R::new(self.bits)
    }
}
#[doc = "HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Do4Spec;
impl crate::RegisterSpec for Do4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`do4::R`](R) reader structure"]
impl crate::Readable for Do4Spec {}
#[doc = "`reset()` method sets DO4 to value 0"]
impl crate::Resettable for Do4Spec {
    const RESET_VALUE: u32 = 0;
}
