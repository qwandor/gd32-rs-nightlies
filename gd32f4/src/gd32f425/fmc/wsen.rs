#[doc = "Register `WSEN` reader"]
pub type R = crate::R<WsenSpec>;
#[doc = "Field `WSEN` reader - FMC wait state enable register"]
pub type WsenR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - FMC wait state enable register"]
    #[inline(always)]
    pub fn wsen(&self) -> WsenR {
        WsenR::new((self.bits & 1) != 0)
    }
}
#[doc = "Wait state enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wsen::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WsenSpec;
impl crate::RegisterSpec for WsenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wsen::R`](R) reader structure"]
impl crate::Readable for WsenSpec {}
#[doc = "`reset()` method sets WSEN to value 0"]
impl crate::Resettable for WsenSpec {
    const RESET_VALUE: u32 = 0;
}
