#[doc = "Register `DMAEN` reader"]
pub type R = crate::R<DmaenSpec>;
#[doc = "Register `DMAEN` writer"]
pub type W = crate::W<DmaenSpec>;
#[doc = "Field `DMAIEN` reader - In FIFO DMA enable"]
pub type DmaienR = crate::BitReader;
#[doc = "Field `DMAIEN` writer - In FIFO DMA enable"]
pub type DmaienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAOEN` reader - Out FIFO DMA enable"]
pub type DmaoenR = crate::BitReader;
#[doc = "Field `DMAOEN` writer - Out FIFO DMA enable"]
pub type DmaoenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - In FIFO DMA enable"]
    #[inline(always)]
    pub fn dmaien(&self) -> DmaienR {
        DmaienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Out FIFO DMA enable"]
    #[inline(always)]
    pub fn dmaoen(&self) -> DmaoenR {
        DmaoenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - In FIFO DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaien(&mut self) -> DmaienW<DmaenSpec> {
        DmaienW::new(self, 0)
    }
    #[doc = "Bit 1 - Out FIFO DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaoen(&mut self) -> DmaoenW<DmaenSpec> {
        DmaoenW::new(self, 1)
    }
}
#[doc = "CAU DMA enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaenSpec;
impl crate::RegisterSpec for DmaenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaen::R`](R) reader structure"]
impl crate::Readable for DmaenSpec {}
#[doc = "`write(|w| ..)` method takes [`dmaen::W`](W) writer structure"]
impl crate::Writable for DmaenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAEN to value 0"]
impl crate::Resettable for DmaenSpec {
    const RESET_VALUE: u32 = 0;
}
