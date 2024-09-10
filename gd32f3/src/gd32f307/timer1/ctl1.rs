#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Field `DMAS` reader - DMA request source selection"]
pub type DmasR = crate::BitReader;
#[doc = "Field `DMAS` writer - DMA request source selection"]
pub type DmasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMC` reader - Master mode control"]
pub type MmcR = crate::FieldReader;
#[doc = "Field `MMC` writer - Master mode control"]
pub type MmcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TI0S` reader - Channel 0 trigger input selection"]
pub type Ti0sR = crate::BitReader;
#[doc = "Field `TI0S` writer - Channel 0 trigger input selection"]
pub type Ti0sW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - DMA request source selection"]
    #[inline(always)]
    pub fn dmas(&self) -> DmasR {
        DmasR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Master mode control"]
    #[inline(always)]
    pub fn mmc(&self) -> MmcR {
        MmcR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Channel 0 trigger input selection"]
    #[inline(always)]
    pub fn ti0s(&self) -> Ti0sR {
        Ti0sR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - DMA request source selection"]
    #[inline(always)]
    #[must_use]
    pub fn dmas(&mut self) -> DmasW<Ctl1Spec> {
        DmasW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Master mode control"]
    #[inline(always)]
    #[must_use]
    pub fn mmc(&mut self) -> MmcW<Ctl1Spec> {
        MmcW::new(self, 4)
    }
    #[doc = "Bit 7 - Channel 0 trigger input selection"]
    #[inline(always)]
    #[must_use]
    pub fn ti0s(&mut self) -> Ti0sW<Ctl1Spec> {
        Ti0sW::new(self, 7)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {
    const RESET_VALUE: u32 = 0;
}
