#[doc = "Register `HCH9INTEN` reader"]
pub type R = crate::R<Hch9intenSpec>;
#[doc = "Register `HCH9INTEN` writer"]
pub type W = crate::W<Hch9intenSpec>;
#[doc = "Field `TFIE` reader - Transfer finished interrupt enable"]
pub type TfieR = crate::BitReader;
#[doc = "Field `TFIE` writer - Transfer finished interrupt enable"]
pub type TfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHIE` reader - Channel halted interrupt enable"]
pub type ChieR = crate::BitReader;
#[doc = "Field `CHIE` writer - Channel halted interrupt enable"]
pub type ChieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAERIE` reader - DMA error interrupt enable"]
pub type DmaerieR = crate::BitReader;
#[doc = "Field `DMAERIE` writer - DMA error interrupt enable"]
pub type DmaerieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLIE` reader - STALL interrupt enable"]
pub type StallieR = crate::BitReader;
#[doc = "Field `STALLIE` writer - STALL interrupt enable"]
pub type StallieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKIE` reader - NAK interrupt enable"]
pub type NakieR = crate::BitReader;
#[doc = "Field `NAKIE` writer - NAK interrupt enable"]
pub type NakieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKIE` reader - ACK interrupt enable"]
pub type AckieR = crate::BitReader;
#[doc = "Field `ACKIE` writer - ACK interrupt enable"]
pub type AckieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYETIE` reader - NYET interrupt enable"]
pub type NyetieR = crate::BitReader;
#[doc = "Field `NYETIE` writer - NYET interrupt enable"]
pub type NyetieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBERIE` reader - USB bus error interrupt enable"]
pub type UsberieR = crate::BitReader;
#[doc = "Field `USBERIE` writer - USB bus error interrupt enable"]
pub type UsberieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBERIE` reader - Babble error interrupt enable"]
pub type BberieR = crate::BitReader;
#[doc = "Field `BBERIE` writer - Babble error interrupt enable"]
pub type BberieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REQOVRIE` reader - request queue overrun interrupt enable"]
pub type ReqovrieR = crate::BitReader;
#[doc = "Field `REQOVRIE` writer - request queue overrun interrupt enable"]
pub type ReqovrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERIE` reader - Data toggle error interrupt enable"]
pub type DterieR = crate::BitReader;
#[doc = "Field `DTERIE` writer - Data toggle error interrupt enable"]
pub type DterieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer finished interrupt enable"]
    #[inline(always)]
    pub fn tfie(&self) -> TfieR {
        TfieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel halted interrupt enable"]
    #[inline(always)]
    pub fn chie(&self) -> ChieR {
        ChieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA error interrupt enable"]
    #[inline(always)]
    pub fn dmaerie(&self) -> DmaerieR {
        DmaerieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL interrupt enable"]
    #[inline(always)]
    pub fn stallie(&self) -> StallieR {
        StallieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK interrupt enable"]
    #[inline(always)]
    pub fn nakie(&self) -> NakieR {
        NakieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK interrupt enable"]
    #[inline(always)]
    pub fn ackie(&self) -> AckieR {
        AckieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NYET interrupt enable"]
    #[inline(always)]
    pub fn nyetie(&self) -> NyetieR {
        NyetieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB bus error interrupt enable"]
    #[inline(always)]
    pub fn usberie(&self) -> UsberieR {
        UsberieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble error interrupt enable"]
    #[inline(always)]
    pub fn bberie(&self) -> BberieR {
        BberieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - request queue overrun interrupt enable"]
    #[inline(always)]
    pub fn reqovrie(&self) -> ReqovrieR {
        ReqovrieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data toggle error interrupt enable"]
    #[inline(always)]
    pub fn dterie(&self) -> DterieR {
        DterieR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer finished interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfie(&mut self) -> TfieW<Hch9intenSpec> {
        TfieW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel halted interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn chie(&mut self) -> ChieW<Hch9intenSpec> {
        ChieW::new(self, 1)
    }
    #[doc = "Bit 2 - DMA error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaerie(&mut self) -> DmaerieW<Hch9intenSpec> {
        DmaerieW::new(self, 2)
    }
    #[doc = "Bit 3 - STALL interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn stallie(&mut self) -> StallieW<Hch9intenSpec> {
        StallieW::new(self, 3)
    }
    #[doc = "Bit 4 - NAK interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn nakie(&mut self) -> NakieW<Hch9intenSpec> {
        NakieW::new(self, 4)
    }
    #[doc = "Bit 5 - ACK interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ackie(&mut self) -> AckieW<Hch9intenSpec> {
        AckieW::new(self, 5)
    }
    #[doc = "Bit 6 - NYET interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn nyetie(&mut self) -> NyetieW<Hch9intenSpec> {
        NyetieW::new(self, 6)
    }
    #[doc = "Bit 7 - USB bus error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn usberie(&mut self) -> UsberieW<Hch9intenSpec> {
        UsberieW::new(self, 7)
    }
    #[doc = "Bit 8 - Babble error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bberie(&mut self) -> BberieW<Hch9intenSpec> {
        BberieW::new(self, 8)
    }
    #[doc = "Bit 9 - request queue overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn reqovrie(&mut self) -> ReqovrieW<Hch9intenSpec> {
        ReqovrieW::new(self, 9)
    }
    #[doc = "Bit 10 - Data toggle error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dterie(&mut self) -> DterieW<Hch9intenSpec> {
        DterieW::new(self, 10)
    }
}
#[doc = "host channel-9 interrupt enable register (HCH9INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch9inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch9inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hch9intenSpec;
impl crate::RegisterSpec for Hch9intenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hch9inten::R`](R) reader structure"]
impl crate::Readable for Hch9intenSpec {}
#[doc = "`write(|w| ..)` method takes [`hch9inten::W`](W) writer structure"]
impl crate::Writable for Hch9intenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCH9INTEN to value 0"]
impl crate::Resettable for Hch9intenSpec {
    const RESET_VALUE: u32 = 0;
}
