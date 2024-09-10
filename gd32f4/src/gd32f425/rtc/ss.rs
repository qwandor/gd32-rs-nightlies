#[doc = "Register `SS` reader"]
pub type R = crate::R<SsSpec>;
#[doc = "Field `SSC` reader - Sub second value"]
pub type SscR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Sub second value"]
    #[inline(always)]
    pub fn ssc(&self) -> SscR {
        SscR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ss::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsSpec;
impl crate::RegisterSpec for SsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss::R`](R) reader structure"]
impl crate::Readable for SsSpec {}
#[doc = "`reset()` method sets SS to value 0"]
impl crate::Resettable for SsSpec {
    const RESET_VALUE: u32 = 0;
}
