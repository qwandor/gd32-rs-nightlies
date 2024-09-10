#[doc = "Register `STAT1` reader"]
pub type R = crate::R<Stat1Spec>;
#[doc = "Field `ISTA` reader - In FIFO interrupt flag"]
pub type IstaR = crate::BitReader;
#[doc = "Field `OSTA` reader - Out FIFO interrupt flag"]
pub type OstaR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - In FIFO interrupt flag"]
    #[inline(always)]
    pub fn ista(&self) -> IstaR {
        IstaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Out FIFO interrupt flag"]
    #[inline(always)]
    pub fn osta(&self) -> OstaR {
        OstaR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "CAU interrupt status flag register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stat1Spec;
impl crate::RegisterSpec for Stat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat1::R`](R) reader structure"]
impl crate::Readable for Stat1Spec {}
#[doc = "`reset()` method sets STAT1 to value 0x01"]
impl crate::Resettable for Stat1Spec {
    const RESET_VALUE: u32 = 0x01;
}
