#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `RBRIE` reader - RBR Interrupt Enable"]
pub type RbrieR = crate::BitReader;
#[doc = "Field `RBRIE` writer - RBR Interrupt Enable"]
pub type RbrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RENDIE` reader - REND Interrupt Enable"]
pub type RendieR = crate::BitReader;
#[doc = "Field `RENDIE` writer - REND Interrupt Enable"]
pub type RendieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROIE` reader - RO Interrupt Enable"]
pub type RoieR = crate::BitReader;
#[doc = "Field `ROIE` writer - RO Interrupt Enable"]
pub type RoieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBREIE` reader - RBRE Interrupt Enable"]
pub type RbreieR = crate::BitReader;
#[doc = "Field `RBREIE` writer - RBRE Interrupt Enable"]
pub type RbreieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSBPEIE` reader - RSBPE Interrupt Enable"]
pub type RsbpeieR = crate::BitReader;
#[doc = "Field `RSBPEIE` writer - RSBPE Interrupt Enable"]
pub type RsbpeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RLBPEIE` reader - RLBPE Interrupt Enable"]
pub type RlbpeieR = crate::BitReader;
#[doc = "Field `RLBPEIE` writer - RLBPE Interrupt Enable"]
pub type RlbpeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAEIE` reader - RAE Interrupt Enable"]
pub type RaeieR = crate::BitReader;
#[doc = "Field `RAEIE` writer - RAE Interrupt Enable"]
pub type RaeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSTARBIE` reader - ALRLST Interrupt Enable"]
pub type LstarbieR = crate::BitReader;
#[doc = "Field `LSTARBIE` writer - ALRLST Interrupt Enable"]
pub type LstarbieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBRIE` reader - TBR Interrupt Enable"]
pub type TbrieR = crate::BitReader;
#[doc = "Field `TBRIE` writer - TBR Interrupt Enable"]
pub type TbrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXENDIE` reader - TEND Interrupt Enable"]
pub type TxendieR = crate::BitReader;
#[doc = "Field `TXENDIE` writer - TEND Interrupt Enable"]
pub type TxendieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUIE` reader - TU Interrupt Enable"]
pub type TuieR = crate::BitReader;
#[doc = "Field `TUIE` writer - TU Interrupt Enable"]
pub type TuieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERRIE` reader - TERR Interrupt Enable"]
pub type TerrieR = crate::BitReader;
#[doc = "Field `TERRIE` writer - TERR Interrupt Enable"]
pub type TerrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAERRIE` reader - TAERR Interrupt Enable"]
pub type TaerrieR = crate::BitReader;
#[doc = "Field `TAERRIE` writer - TAERR Interrupt Enable"]
pub type TaerrieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RBR Interrupt Enable"]
    #[inline(always)]
    pub fn rbrie(&self) -> RbrieR {
        RbrieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REND Interrupt Enable"]
    #[inline(always)]
    pub fn rendie(&self) -> RendieR {
        RendieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RO Interrupt Enable"]
    #[inline(always)]
    pub fn roie(&self) -> RoieR {
        RoieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RBRE Interrupt Enable"]
    #[inline(always)]
    pub fn rbreie(&self) -> RbreieR {
        RbreieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RSBPE Interrupt Enable"]
    #[inline(always)]
    pub fn rsbpeie(&self) -> RsbpeieR {
        RsbpeieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RLBPE Interrupt Enable"]
    #[inline(always)]
    pub fn rlbpeie(&self) -> RlbpeieR {
        RlbpeieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RAE Interrupt Enable"]
    #[inline(always)]
    pub fn raeie(&self) -> RaeieR {
        RaeieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ALRLST Interrupt Enable"]
    #[inline(always)]
    pub fn lstarbie(&self) -> LstarbieR {
        LstarbieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TBR Interrupt Enable"]
    #[inline(always)]
    pub fn tbrie(&self) -> TbrieR {
        TbrieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TEND Interrupt Enable"]
    #[inline(always)]
    pub fn txendie(&self) -> TxendieR {
        TxendieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TU Interrupt Enable"]
    #[inline(always)]
    pub fn tuie(&self) -> TuieR {
        TuieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TERR Interrupt Enable"]
    #[inline(always)]
    pub fn terrie(&self) -> TerrieR {
        TerrieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TAERR Interrupt Enable"]
    #[inline(always)]
    pub fn taerrie(&self) -> TaerrieR {
        TaerrieR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RBR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rbrie(&mut self) -> RbrieW<IntenSpec> {
        RbrieW::new(self, 0)
    }
    #[doc = "Bit 1 - REND Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rendie(&mut self) -> RendieW<IntenSpec> {
        RendieW::new(self, 1)
    }
    #[doc = "Bit 2 - RO Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn roie(&mut self) -> RoieW<IntenSpec> {
        RoieW::new(self, 2)
    }
    #[doc = "Bit 3 - RBRE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rbreie(&mut self) -> RbreieW<IntenSpec> {
        RbreieW::new(self, 3)
    }
    #[doc = "Bit 4 - RSBPE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rsbpeie(&mut self) -> RsbpeieW<IntenSpec> {
        RsbpeieW::new(self, 4)
    }
    #[doc = "Bit 5 - RLBPE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rlbpeie(&mut self) -> RlbpeieW<IntenSpec> {
        RlbpeieW::new(self, 5)
    }
    #[doc = "Bit 6 - RAE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn raeie(&mut self) -> RaeieW<IntenSpec> {
        RaeieW::new(self, 6)
    }
    #[doc = "Bit 7 - ALRLST Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lstarbie(&mut self) -> LstarbieW<IntenSpec> {
        LstarbieW::new(self, 7)
    }
    #[doc = "Bit 8 - TBR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbrie(&mut self) -> TbrieW<IntenSpec> {
        TbrieW::new(self, 8)
    }
    #[doc = "Bit 9 - TEND Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txendie(&mut self) -> TxendieW<IntenSpec> {
        TxendieW::new(self, 9)
    }
    #[doc = "Bit 10 - TU Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tuie(&mut self) -> TuieW<IntenSpec> {
        TuieW::new(self, 10)
    }
    #[doc = "Bit 11 - TERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn terrie(&mut self) -> TerrieW<IntenSpec> {
        TerrieW::new(self, 11)
    }
    #[doc = "Bit 12 - TAERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn taerrie(&mut self) -> TaerrieW<IntenSpec> {
        TaerrieW::new(self, 12)
    }
}
#[doc = "interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
