#[doc = "Register `INTF` reader"]
pub type R = crate::R<IntfSpec>;
#[doc = "Field `IINTF` reader - In FIFO enabled interrupt flag"]
pub type IintfR = crate::BitReader;
#[doc = "Field `OINTF` reader - Out FIFO enabled interrupt flag"]
pub type OintfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - In FIFO enabled interrupt flag"]
    #[inline(always)]
    pub fn iintf(&self) -> IintfR {
        IintfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Out FIFO enabled interrupt flag"]
    #[inline(always)]
    pub fn ointf(&self) -> OintfR {
        OintfR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "CAU enable interrupt status flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfSpec;
impl crate::RegisterSpec for IntfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf::R`](R) reader structure"]
impl crate::Readable for IntfSpec {}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for IntfSpec {
    const RESET_VALUE: u32 = 0;
}
