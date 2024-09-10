#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `EFIE` reader - End of Frame Interrupt Enable"]
pub type EfieR = crate::BitReader;
#[doc = "Field `EFIE` writer - End of Frame Interrupt Enable"]
pub type EfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRIE` reader - FIFO Overrun interrupt Enable"]
pub type OvrieR = crate::BitReader;
#[doc = "Field `OVRIE` writer - FIFO Overrun interrupt Enable"]
pub type OvrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESEIE` reader - Embedded Synchronous Error Interrupt Enable"]
pub type EseieR = crate::BitReader;
#[doc = "Field `ESEIE` writer - Embedded Synchronous Error Interrupt Enable"]
pub type EseieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSIE` reader - Vsync Interrupt Enable"]
pub type VsieR = crate::BitReader;
#[doc = "Field `VSIE` writer - Vsync Interrupt Enable"]
pub type VsieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELIE` reader - End of line interrupt Enable"]
pub type ElieR = crate::BitReader;
#[doc = "Field `ELIE` writer - End of line interrupt Enable"]
pub type ElieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - End of Frame Interrupt Enable"]
    #[inline(always)]
    pub fn efie(&self) -> EfieR {
        EfieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Overrun interrupt Enable"]
    #[inline(always)]
    pub fn ovrie(&self) -> OvrieR {
        OvrieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Embedded Synchronous Error Interrupt Enable"]
    #[inline(always)]
    pub fn eseie(&self) -> EseieR {
        EseieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Vsync Interrupt Enable"]
    #[inline(always)]
    pub fn vsie(&self) -> VsieR {
        VsieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of line interrupt Enable"]
    #[inline(always)]
    pub fn elie(&self) -> ElieR {
        ElieR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of Frame Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn efie(&mut self) -> EfieW<IntenSpec> {
        EfieW::new(self, 0)
    }
    #[doc = "Bit 1 - FIFO Overrun interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OvrieW<IntenSpec> {
        OvrieW::new(self, 1)
    }
    #[doc = "Bit 2 - Embedded Synchronous Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eseie(&mut self) -> EseieW<IntenSpec> {
        EseieW::new(self, 2)
    }
    #[doc = "Bit 3 - Vsync Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vsie(&mut self) -> VsieW<IntenSpec> {
        VsieW::new(self, 3)
    }
    #[doc = "Bit 4 - End of line interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn elie(&mut self) -> ElieW<IntenSpec> {
        ElieW::new(self, 4)
    }
}
#[doc = "DCI inrerrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
