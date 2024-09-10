#[doc = "Register `INTC` writer"]
pub type W = crate::W<IntcSpec>;
#[doc = "Field `LMC` writer - Line Mark Flag Clear"]
pub type LmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEC` writer - FIFO Error Flag Clear"]
pub type FecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEC` writer - Transaction Error Flag Clear"]
pub type TecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCRC` writer - Layer Configuration Reloaded Flag Clear"]
pub type LcrcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Line Mark Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lmc(&mut self) -> LmcW<IntcSpec> {
        LmcW::new(self, 0)
    }
    #[doc = "Bit 1 - FIFO Error Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fec(&mut self) -> FecW<IntcSpec> {
        FecW::new(self, 1)
    }
    #[doc = "Bit 2 - Transaction Error Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tec(&mut self) -> TecW<IntcSpec> {
        TecW::new(self, 2)
    }
    #[doc = "Bit 3 - Layer Configuration Reloaded Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lcrc(&mut self) -> LcrcW<IntcSpec> {
        LcrcW::new(self, 3)
    }
}
#[doc = "Interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
