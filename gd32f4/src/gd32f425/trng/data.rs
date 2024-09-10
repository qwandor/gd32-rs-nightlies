#[doc = "Register `DATA` reader"]
pub type R = crate::R<DataSpec>;
#[doc = "Field `TRNDATA` reader - 32-bit random data"]
pub type TrndataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit random data"]
    #[inline(always)]
    pub fn trndata(&self) -> TrndataR {
        TrndataR::new(self.bits)
    }
}
#[doc = "data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataSpec;
impl crate::RegisterSpec for DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DataSpec {}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DataSpec {
    const RESET_VALUE: u32 = 0;
}
