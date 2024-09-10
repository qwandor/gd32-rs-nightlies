#[doc = "Register `DO` reader"]
pub type R = crate::R<DoSpec>;
#[doc = "Field `DO` reader - Data output"]
pub type DoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data output"]
    #[inline(always)]
    pub fn do_(&self) -> DoR {
        DoR::new(self.bits)
    }
}
#[doc = "CAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoSpec;
impl crate::RegisterSpec for DoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`do_::R`](R) reader structure"]
impl crate::Readable for DoSpec {}
#[doc = "`reset()` method sets DO to value 0"]
impl crate::Resettable for DoSpec {
    const RESET_VALUE: u32 = 0;
}
