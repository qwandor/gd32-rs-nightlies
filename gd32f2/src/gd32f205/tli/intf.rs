#[doc = "Register `INTF` reader"]
pub type R = crate::R<IntfSpec>;
#[doc = "Field `LMF` reader - Line Mark Flag"]
pub type LmfR = crate::BitReader;
#[doc = "Field `FEF` reader - FIFO Error Flag"]
pub type FefR = crate::BitReader;
#[doc = "Field `TEF` reader - Transaction Error Flag"]
pub type TefR = crate::BitReader;
#[doc = "Field `LCRF` reader - Layer Configuration Reloaded Flag"]
pub type LcrfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Line Mark Flag"]
    #[inline(always)]
    pub fn lmf(&self) -> LmfR {
        LmfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Error Flag"]
    #[inline(always)]
    pub fn fef(&self) -> FefR {
        FefR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transaction Error Flag"]
    #[inline(always)]
    pub fn tef(&self) -> TefR {
        TefR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Layer Configuration Reloaded Flag"]
    #[inline(always)]
    pub fn lcrf(&self) -> LcrfR {
        LcrfR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
