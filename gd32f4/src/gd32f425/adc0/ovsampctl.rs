#[doc = "Register `OVSAMPCTL` reader"]
pub type R = crate::R<OvsampctlSpec>;
#[doc = "Field `OVSEN` reader - Oversampling Enable"]
pub type OvsenR = crate::BitReader;
#[doc = "Field `OVSR` reader - Oversampling ratio"]
pub type OvsrR = crate::FieldReader;
#[doc = "Field `OVSS` reader - Oversampling shift"]
pub type OvssR = crate::FieldReader;
#[doc = "Field `TOVS` reader - Triggered Oversampling"]
pub type TovsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Oversampling Enable"]
    #[inline(always)]
    pub fn ovsen(&self) -> OvsenR {
        OvsenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:4 - Oversampling ratio"]
    #[inline(always)]
    pub fn ovsr(&self) -> OvsrR {
        OvsrR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:8 - Oversampling shift"]
    #[inline(always)]
    pub fn ovss(&self) -> OvssR {
        OvssR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Triggered Oversampling"]
    #[inline(always)]
    pub fn tovs(&self) -> TovsR {
        TovsR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Oversample control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ovsampctl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OvsampctlSpec;
impl crate::RegisterSpec for OvsampctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ovsampctl::R`](R) reader structure"]
impl crate::Readable for OvsampctlSpec {}
#[doc = "`reset()` method sets OVSAMPCTL to value 0"]
impl crate::Resettable for OvsampctlSpec {
    const RESET_VALUE: u32 = 0;
}
