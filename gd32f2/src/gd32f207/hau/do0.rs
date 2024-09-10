#[doc = "Register `DO0` reader"]
pub type R = crate::R<Do0Spec>;
#[doc = "Field `DO0` reader - message digest result of hash algorithm"]
pub type Do0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - message digest result of hash algorithm"]
    #[inline(always)]
    pub fn do0(&self) -> Do0R {
        Do0R::new(self.bits)
    }
}
#[doc = "HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Do0Spec;
impl crate::RegisterSpec for Do0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`do0::R`](R) reader structure"]
impl crate::Readable for Do0Spec {}
#[doc = "`reset()` method sets DO0 to value 0"]
impl crate::Resettable for Do0Spec {
    const RESET_VALUE: u32 = 0;
}
