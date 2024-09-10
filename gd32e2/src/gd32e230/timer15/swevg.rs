#[doc = "Register `SWEVG` writer"]
pub type W = crate::W<SwevgSpec>;
#[doc = "Break generation"]
pub use crate::gd32e230::timer0::swevg::Brkg;
#[doc = "Field `BRKG` writer - Break generation"]
pub use crate::gd32e230::timer0::swevg::BrkgW;
#[doc = "Capture/compare 0 generation"]
pub use crate::gd32e230::timer0::swevg::Ch0g;
#[doc = "Field `CH0G` writer - Capture/compare 0 generation"]
pub use crate::gd32e230::timer0::swevg::Ch0gW;
#[doc = "Capture/Compare control update generation"]
pub use crate::gd32e230::timer0::swevg::Cmtg;
#[doc = "Field `CMTG` writer - Capture/Compare control update generation"]
pub use crate::gd32e230::timer0::swevg::CmtgW;
#[doc = "Update generation"]
pub use crate::gd32e230::timer0::swevg::Upg;
#[doc = "Field `UPG` writer - Update generation"]
pub use crate::gd32e230::timer0::swevg::UpgW;
impl W {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    #[must_use]
    pub fn upg(&mut self) -> UpgW<SwevgSpec> {
        UpgW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/compare 0 generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch0g(&mut self) -> Ch0gW<SwevgSpec> {
        Ch0gW::new(self, 1)
    }
    #[doc = "Bit 5 - Capture/Compare control update generation"]
    #[inline(always)]
    #[must_use]
    pub fn cmtg(&mut self) -> CmtgW<SwevgSpec> {
        CmtgW::new(self, 5)
    }
    #[doc = "Bit 7 - Break generation"]
    #[inline(always)]
    #[must_use]
    pub fn brkg(&mut self) -> BrkgW<SwevgSpec> {
        BrkgW::new(self, 7)
    }
}
#[doc = "event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swevg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwevgSpec;
impl crate::RegisterSpec for SwevgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swevg::W`](W) writer structure"]
impl crate::Writable for SwevgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWEVG to value 0"]
impl crate::Resettable for SwevgSpec {
    const RESET_VALUE: u32 = 0;
}
