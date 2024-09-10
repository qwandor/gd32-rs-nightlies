#[doc = "Register `STAT0` reader"]
pub type R = crate::R<Stat0Spec>;
#[doc = "Field `HS` reader - HS line status"]
pub type HsR = crate::BitReader;
#[doc = "Field `VS` reader - VS line status"]
pub type VsR = crate::BitReader;
#[doc = "Field `FV` reader - FIFO Valid"]
pub type FvR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - HS line status"]
    #[inline(always)]
    pub fn hs(&self) -> HsR {
        HsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VS line status"]
    #[inline(always)]
    pub fn vs(&self) -> VsR {
        VsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO Valid"]
    #[inline(always)]
    pub fn fv(&self) -> FvR {
        FvR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stat0Spec;
impl crate::RegisterSpec for Stat0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat0::R`](R) reader structure"]
impl crate::Readable for Stat0Spec {}
#[doc = "`reset()` method sets STAT0 to value 0"]
impl crate::Resettable for Stat0Spec {
    const RESET_VALUE: u32 = 0;
}
