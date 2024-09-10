#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `LMIE` reader - Line Mark Interrupt Enable"]
pub type LmieR = crate::BitReader;
#[doc = "Field `LMIE` writer - Line Mark Interrupt Enable"]
pub type LmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEIE` reader - FIFO Error Interrupt Enable"]
pub type FeieR = crate::BitReader;
#[doc = "Field `FEIE` writer - FIFO Error Interrupt Enable"]
pub type FeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIE` reader - Transaction Error Interrupt Enable"]
pub type TeieR = crate::BitReader;
#[doc = "Field `TEIE` writer - Transaction Error Interrupt Enable"]
pub type TeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCRIE` reader - Layer Configuration Reloaded Interrupt Enable"]
pub type LcrieR = crate::BitReader;
#[doc = "Field `LCRIE` writer - Layer Configuration Reloaded Interrupt Enable"]
pub type LcrieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Line Mark Interrupt Enable"]
    #[inline(always)]
    pub fn lmie(&self) -> LmieR {
        LmieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&self) -> FeieR {
        FeieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transaction Error Interrupt Enable"]
    #[inline(always)]
    pub fn teie(&self) -> TeieR {
        TeieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Layer Configuration Reloaded Interrupt Enable"]
    #[inline(always)]
    pub fn lcrie(&self) -> LcrieR {
        LcrieR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Line Mark Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lmie(&mut self) -> LmieW<IntenSpec> {
        LmieW::new(self, 0)
    }
    #[doc = "Bit 1 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn feie(&mut self) -> FeieW<IntenSpec> {
        FeieW::new(self, 1)
    }
    #[doc = "Bit 2 - Transaction Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TeieW<IntenSpec> {
        TeieW::new(self, 2)
    }
    #[doc = "Bit 3 - Layer Configuration Reloaded Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lcrie(&mut self) -> LcrieW<IntenSpec> {
        LcrieW::new(self, 3)
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
