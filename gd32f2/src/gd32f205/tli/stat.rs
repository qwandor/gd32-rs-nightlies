#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Field `VDE` reader - Current VDE status"]
pub type VdeR = crate::BitReader;
#[doc = "Field `HDE` reader - Current HDE status"]
pub type HdeR = crate::BitReader;
#[doc = "Field `VS` reader - Current VS staus of the TLI"]
pub type VsR = crate::BitReader;
#[doc = "Field `HS` reader - Current HS staus of the TLI"]
pub type HsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Current VDE status"]
    #[inline(always)]
    pub fn vde(&self) -> VdeR {
        VdeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Current HDE status"]
    #[inline(always)]
    pub fn hde(&self) -> HdeR {
        HdeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Current VS staus of the TLI"]
    #[inline(always)]
    pub fn vs(&self) -> VsR {
        VsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Current HS staus of the TLI"]
    #[inline(always)]
    pub fn hs(&self) -> HsR {
        HsR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`reset()` method sets STAT to value 0x0f"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0x0f;
}
