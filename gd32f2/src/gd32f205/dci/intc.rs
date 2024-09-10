#[doc = "Register `INTC` writer"]
pub type W = crate::W<IntcSpec>;
#[doc = "Field `EFFC` writer - Clear End of Frame Flag"]
pub type EffcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRFC` writer - Clear FIFO Overrun Flag"]
pub type OvrfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESEFC` writer - Clear embedded synchronous Error Flag"]
pub type EsefcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSFC` writer - Vsync flag clear"]
pub type VsfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELFC` writer - End of Line Flag Clear"]
pub type ElfcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear End of Frame Flag"]
    #[inline(always)]
    #[must_use]
    pub fn effc(&mut self) -> EffcW<IntcSpec> {
        EffcW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear FIFO Overrun Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ovrfc(&mut self) -> OvrfcW<IntcSpec> {
        OvrfcW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear embedded synchronous Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn esefc(&mut self) -> EsefcW<IntcSpec> {
        EsefcW::new(self, 2)
    }
    #[doc = "Bit 3 - Vsync flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn vsfc(&mut self) -> VsfcW<IntcSpec> {
        VsfcW::new(self, 3)
    }
    #[doc = "Bit 4 - End of Line Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn elfc(&mut self) -> ElfcW<IntcSpec> {
        ElfcW::new(self, 4)
    }
}
#[doc = "DCI Interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcSpec;
impl crate::RegisterSpec for IntcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intc::W`](W) writer structure"]
impl crate::Writable for IntcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTC to value 0"]
impl crate::Resettable for IntcSpec {
    const RESET_VALUE: u32 = 0;
}
