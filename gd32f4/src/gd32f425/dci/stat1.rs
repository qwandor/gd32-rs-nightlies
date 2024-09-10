#[doc = "Register `STAT1` reader"]
pub type R = crate::R<Stat1Spec>;
#[doc = "Field `EFF` reader - End of Frame Flag"]
pub type EffR = crate::BitReader;
#[doc = "Field `OVRF` reader - FIFO Overrun Flag"]
pub type OvrfR = crate::BitReader;
#[doc = "Field `ESEF` reader - Embedded Synchronous Error Flag"]
pub type EsefR = crate::BitReader;
#[doc = "Field `VSF` reader - Vsync Flag"]
pub type VsfR = crate::BitReader;
#[doc = "Field `ELF` reader - End of Line Flag"]
pub type ElfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - End of Frame Flag"]
    #[inline(always)]
    pub fn eff(&self) -> EffR {
        EffR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Overrun Flag"]
    #[inline(always)]
    pub fn ovrf(&self) -> OvrfR {
        OvrfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Embedded Synchronous Error Flag"]
    #[inline(always)]
    pub fn esef(&self) -> EsefR {
        EsefR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Vsync Flag"]
    #[inline(always)]
    pub fn vsf(&self) -> VsfR {
        VsfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Line Flag"]
    #[inline(always)]
    pub fn elf(&self) -> ElfR {
        ElfR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stat1Spec;
impl crate::RegisterSpec for Stat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat1::R`](R) reader structure"]
impl crate::Readable for Stat1Spec {}
#[doc = "`reset()` method sets STAT1 to value 0"]
impl crate::Resettable for Stat1Spec {
    const RESET_VALUE: u32 = 0;
}
