#[doc = "Register `DMA_INTEN` reader"]
pub type R = crate::R<DmaIntenSpec>;
#[doc = "Register `DMA_INTEN` writer"]
pub type W = crate::W<DmaIntenSpec>;
#[doc = "Field `TIEN` reader - Transmit interrupt enable"]
pub type TienR = crate::BitReader;
#[doc = "Field `TIEN` writer - Transmit interrupt enable"]
pub type TienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPSIEN` reader - Transmit process stopped interrupt enable"]
pub type TpsienR = crate::BitReader;
#[doc = "Field `TPSIEN` writer - Transmit process stopped interrupt enable"]
pub type TpsienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBUIEN` reader - Transmit buffer unavailable interrupt enable"]
pub type TbuienR = crate::BitReader;
#[doc = "Field `TBUIEN` writer - Transmit buffer unavailable interrupt enable"]
pub type TbuienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TJTIEN` reader - Transmit jabber timeout interrupt enable"]
pub type TjtienR = crate::BitReader;
#[doc = "Field `TJTIEN` writer - Transmit jabber timeout interrupt enable"]
pub type TjtienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROIEN` reader - Receive overflow interrupt enable"]
pub type RoienR = crate::BitReader;
#[doc = "Field `ROIEN` writer - Receive overflow interrupt enable"]
pub type RoienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUIEN` reader - Transmit underflow interrupt enable"]
pub type TuienR = crate::BitReader;
#[doc = "Field `TUIEN` writer - Transmit underflow interrupt enable"]
pub type TuienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIEN` reader - Receive interrupt enable"]
pub type RienR = crate::BitReader;
#[doc = "Field `RIEN` writer - Receive interrupt enable"]
pub type RienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBUIEN` reader - Receive buffer unavailable interrupt enable"]
pub type RbuienR = crate::BitReader;
#[doc = "Field `RBUIEN` writer - Receive buffer unavailable interrupt enable"]
pub type RbuienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPSIEN` reader - Receive process stopped interrupt enable"]
pub type RpsienR = crate::BitReader;
#[doc = "Field `RPSIEN` writer - Receive process stopped interrupt enable"]
pub type RpsienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWTIEN` reader - receive watchdog timeout interrupt enable"]
pub type RwtienR = crate::BitReader;
#[doc = "Field `RWTIEN` writer - receive watchdog timeout interrupt enable"]
pub type RwtienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETIEN` reader - Early transmit interrupt enable"]
pub type EtienR = crate::BitReader;
#[doc = "Field `ETIEN` writer - Early transmit interrupt enable"]
pub type EtienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBEIEN` reader - Fatal bus error interrupt enable"]
pub type FbeienR = crate::BitReader;
#[doc = "Field `FBEIEN` writer - Fatal bus error interrupt enable"]
pub type FbeienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERIEN` reader - Early receive interrupt enable"]
pub type ErienR = crate::BitReader;
#[doc = "Field `ERIEN` writer - Early receive interrupt enable"]
pub type ErienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIEN` reader - Abnormal interrupt summary enable"]
pub type AienR = crate::BitReader;
#[doc = "Field `AIEN` writer - Abnormal interrupt summary enable"]
pub type AienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIEN` reader - Normal interrupt summary enable"]
pub type NienR = crate::BitReader;
#[doc = "Field `NIEN` writer - Normal interrupt summary enable"]
pub type NienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn tien(&self) -> TienR {
        TienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit process stopped interrupt enable"]
    #[inline(always)]
    pub fn tpsien(&self) -> TpsienR {
        TpsienR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable interrupt enable"]
    #[inline(always)]
    pub fn tbuien(&self) -> TbuienR {
        TbuienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit jabber timeout interrupt enable"]
    #[inline(always)]
    pub fn tjtien(&self) -> TjtienR {
        TjtienR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive overflow interrupt enable"]
    #[inline(always)]
    pub fn roien(&self) -> RoienR {
        RoienR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit underflow interrupt enable"]
    #[inline(always)]
    pub fn tuien(&self) -> TuienR {
        TuienR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rien(&self) -> RienR {
        RienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive buffer unavailable interrupt enable"]
    #[inline(always)]
    pub fn rbuien(&self) -> RbuienR {
        RbuienR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive process stopped interrupt enable"]
    #[inline(always)]
    pub fn rpsien(&self) -> RpsienR {
        RpsienR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - receive watchdog timeout interrupt enable"]
    #[inline(always)]
    pub fn rwtien(&self) -> RwtienR {
        RwtienR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early transmit interrupt enable"]
    #[inline(always)]
    pub fn etien(&self) -> EtienR {
        EtienR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal bus error interrupt enable"]
    #[inline(always)]
    pub fn fbeien(&self) -> FbeienR {
        FbeienR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early receive interrupt enable"]
    #[inline(always)]
    pub fn erien(&self) -> ErienR {
        ErienR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary enable"]
    #[inline(always)]
    pub fn aien(&self) -> AienR {
        AienR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal interrupt summary enable"]
    #[inline(always)]
    pub fn nien(&self) -> NienR {
        NienR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tien(&mut self) -> TienW<DmaIntenSpec> {
        TienW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit process stopped interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpsien(&mut self) -> TpsienW<DmaIntenSpec> {
        TpsienW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbuien(&mut self) -> TbuienW<DmaIntenSpec> {
        TbuienW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit jabber timeout interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tjtien(&mut self) -> TjtienW<DmaIntenSpec> {
        TjtienW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn roien(&mut self) -> RoienW<DmaIntenSpec> {
        RoienW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit underflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tuien(&mut self) -> TuienW<DmaIntenSpec> {
        TuienW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rien(&mut self) -> RienW<DmaIntenSpec> {
        RienW::new(self, 6)
    }
    #[doc = "Bit 7 - Receive buffer unavailable interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rbuien(&mut self) -> RbuienW<DmaIntenSpec> {
        RbuienW::new(self, 7)
    }
    #[doc = "Bit 8 - Receive process stopped interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rpsien(&mut self) -> RpsienW<DmaIntenSpec> {
        RpsienW::new(self, 8)
    }
    #[doc = "Bit 9 - receive watchdog timeout interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwtien(&mut self) -> RwtienW<DmaIntenSpec> {
        RwtienW::new(self, 9)
    }
    #[doc = "Bit 10 - Early transmit interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn etien(&mut self) -> EtienW<DmaIntenSpec> {
        EtienW::new(self, 10)
    }
    #[doc = "Bit 13 - Fatal bus error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fbeien(&mut self) -> FbeienW<DmaIntenSpec> {
        FbeienW::new(self, 13)
    }
    #[doc = "Bit 14 - Early receive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn erien(&mut self) -> ErienW<DmaIntenSpec> {
        ErienW::new(self, 14)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary enable"]
    #[inline(always)]
    #[must_use]
    pub fn aien(&mut self) -> AienW<DmaIntenSpec> {
        AienW::new(self, 15)
    }
    #[doc = "Bit 16 - Normal interrupt summary enable"]
    #[inline(always)]
    #[must_use]
    pub fn nien(&mut self) -> NienW<DmaIntenSpec> {
        NienW::new(self, 16)
    }
}
#[doc = "Ethernet DMA interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaIntenSpec;
impl crate::RegisterSpec for DmaIntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_inten::R`](R) reader structure"]
impl crate::Readable for DmaIntenSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_inten::W`](W) writer structure"]
impl crate::Writable for DmaIntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_INTEN to value 0"]
impl crate::Resettable for DmaIntenSpec {
    const RESET_VALUE: u32 = 0;
}
