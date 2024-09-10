#[doc = "Register `IPA_INTF` reader"]
pub type R = crate::R<IpaIntfSpec>;
#[doc = "Field `TAEIF` reader - Transfer access error interrupt flag"]
pub type TaeifR = crate::BitReader;
#[doc = "Field `FTFIF` reader - Full transfer finish interrupt flag"]
pub type FtfifR = crate::BitReader;
#[doc = "Field `TLMIF` reader - Transfer line mark interrupt flag"]
pub type TlmifR = crate::BitReader;
#[doc = "Field `LACIF` reader - LUT access conflict interrupt flag"]
pub type LacifR = crate::BitReader;
#[doc = "Field `LLFIF` reader - LUT loading finish interrupt flag"]
pub type LlfifR = crate::BitReader;
#[doc = "Field `WCFIF` reader - Wrong configuration interrupt flag"]
pub type WcfifR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transfer access error interrupt flag"]
    #[inline(always)]
    pub fn taeif(&self) -> TaeifR {
        TaeifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Full transfer finish interrupt flag"]
    #[inline(always)]
    pub fn ftfif(&self) -> FtfifR {
        FtfifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer line mark interrupt flag"]
    #[inline(always)]
    pub fn tlmif(&self) -> TlmifR {
        TlmifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LUT access conflict interrupt flag"]
    #[inline(always)]
    pub fn lacif(&self) -> LacifR {
        LacifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LUT loading finish interrupt flag"]
    #[inline(always)]
    pub fn llfif(&self) -> LlfifR {
        LlfifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wrong configuration interrupt flag"]
    #[inline(always)]
    pub fn wcfif(&self) -> WcfifR {
        WcfifR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_intf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpaIntfSpec;
impl crate::RegisterSpec for IpaIntfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipa_intf::R`](R) reader structure"]
impl crate::Readable for IpaIntfSpec {}
#[doc = "`reset()` method sets IPA_INTF to value 0"]
impl crate::Resettable for IpaIntfSpec {
    const RESET_VALUE: u32 = 0;
}
