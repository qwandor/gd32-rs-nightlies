#[doc = "Register `G2CYCN` reader"]
pub type R = crate::R<G2cycnSpec>;
#[doc = "Field `CYCN` reader - Cycle number"]
pub type CycnR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Cycle number"]
    #[inline(always)]
    pub fn cycn(&self) -> CycnR {
        CycnR::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "I/O group x cycle number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g2cycn::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct G2cycnSpec;
impl crate::RegisterSpec for G2cycnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`g2cycn::R`](R) reader structure"]
impl crate::Readable for G2cycnSpec {}
#[doc = "`reset()` method sets G2CYCN to value 0"]
impl crate::Resettable for G2cycnSpec {
    const RESET_VALUE: u32 = 0;
}
