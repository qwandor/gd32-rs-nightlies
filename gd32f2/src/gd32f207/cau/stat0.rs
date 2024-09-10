#[doc = "Register `STAT0` reader"]
pub type R = crate::R<Stat0Spec>;
#[doc = "Field `IEM` reader - IN FIFO empty flag"]
pub type IemR = crate::BitReader;
#[doc = "Field `INF` reader - IN FIFO not full flag"]
pub type InfR = crate::BitReader;
#[doc = "Field `ONE` reader - OUT FIFO not empty flag"]
pub type OneR = crate::BitReader;
#[doc = "Field `OFU` reader - OUT FIFO full flag"]
pub type OfuR = crate::BitReader;
#[doc = "Field `BUSY` reader - BUSY flag"]
pub type BusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - IN FIFO empty flag"]
    #[inline(always)]
    pub fn iem(&self) -> IemR {
        IemR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IN FIFO not full flag"]
    #[inline(always)]
    pub fn inf(&self) -> InfR {
        InfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OUT FIFO not empty flag"]
    #[inline(always)]
    pub fn one(&self) -> OneR {
        OneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OUT FIFO full flag"]
    #[inline(always)]
    pub fn ofu(&self) -> OfuR {
        OfuR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BUSY flag"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "CAU status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stat0Spec;
impl crate::RegisterSpec for Stat0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat0::R`](R) reader structure"]
impl crate::Readable for Stat0Spec {}
#[doc = "`reset()` method sets STAT0 to value 0x03"]
impl crate::Resettable for Stat0Spec {
    const RESET_VALUE: u32 = 0x03;
}
