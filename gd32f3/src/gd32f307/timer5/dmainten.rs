#[doc = "Register `DMAINTEN` reader"]
pub type R = crate::R<DmaintenSpec>;
#[doc = "Register `DMAINTEN` writer"]
pub type W = crate::W<DmaintenSpec>;
#[doc = "Field `UPIE` reader - Update interrupt enable"]
pub type UpieR = crate::BitReader;
#[doc = "Field `UPIE` writer - Update interrupt enable"]
pub type UpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDEN` reader - Update DMA request enable"]
pub type UpdenR = crate::BitReader;
#[doc = "Field `UPDEN` writer - Update DMA request enable"]
pub type UpdenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn upie(&self) -> UpieR {
        UpieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn upden(&self) -> UpdenR {
        UpdenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn upie(&mut self) -> UpieW<DmaintenSpec> {
        UpieW::new(self, 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn upden(&mut self) -> UpdenW<DmaintenSpec> {
        UpdenW::new(self, 8)
    }
}
#[doc = "DMA/Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmainten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmainten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaintenSpec;
impl crate::RegisterSpec for DmaintenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmainten::R`](R) reader structure"]
impl crate::Readable for DmaintenSpec {}
#[doc = "`write(|w| ..)` method takes [`dmainten::W`](W) writer structure"]
impl crate::Writable for DmaintenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAINTEN to value 0"]
impl crate::Resettable for DmaintenSpec {
    const RESET_VALUE: u32 = 0;
}
