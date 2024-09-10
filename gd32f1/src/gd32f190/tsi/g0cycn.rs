#[doc = "Register `G0CYCN` reader"]
pub type R = crate::R<G0cycnSpec>;
#[doc = "Field `CYCN` reader - Cycle number"]
pub type CycnR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Cycle number"]
    #[inline(always)]
    pub fn cycn(&self) -> CycnR {
        CycnR::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "I/O group x cycle number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g0cycn::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct G0cycnSpec;
impl crate::RegisterSpec for G0cycnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`g0cycn::R`](R) reader structure"]
impl crate::Readable for G0cycnSpec {}
#[doc = "`reset()` method sets G0CYCN to value 0"]
impl crate::Resettable for G0cycnSpec {
    const RESET_VALUE: u32 = 0;
}
