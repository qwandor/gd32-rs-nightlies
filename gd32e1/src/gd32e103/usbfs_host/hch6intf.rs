#[doc = "Register `HCH6INTF` reader"]
pub type R = crate::R<Hch6intfSpec>;
#[doc = "Register `HCH6INTF` writer"]
pub type W = crate::W<Hch6intfSpec>;
#[doc = "Field `TF` reader - Transfer finished"]
pub type TfR = crate::BitReader;
#[doc = "Field `TF` writer - Transfer finished"]
pub type TfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH` reader - Channel halted"]
pub type ChR = crate::BitReader;
#[doc = "Field `CH` writer - Channel halted"]
pub type ChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - STALL response received interrupt"]
pub type StallR = crate::BitReader;
#[doc = "Field `STALL` writer - STALL response received interrupt"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK` reader - NAK response received interrupt"]
pub type NakR = crate::BitReader;
#[doc = "Field `NAK` writer - NAK response received interrupt"]
pub type NakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - ACK response received/transmitted interrupt"]
pub type AckR = crate::BitReader;
#[doc = "Field `ACK` writer - ACK response received/transmitted interrupt"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBER` reader - USB bus error"]
pub type UsberR = crate::BitReader;
#[doc = "Field `USBER` writer - USB bus error"]
pub type UsberW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBER` reader - Babble error"]
pub type BberR = crate::BitReader;
#[doc = "Field `BBER` writer - Babble error"]
pub type BberW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REQOVR` reader - Request queue overrun"]
pub type ReqovrR = crate::BitReader;
#[doc = "Field `REQOVR` writer - Request queue overrun"]
pub type ReqovrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTER` reader - Data toggle error"]
pub type DterR = crate::BitReader;
#[doc = "Field `DTER` writer - Data toggle error"]
pub type DterW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer finished"]
    #[inline(always)]
    pub fn tf(&self) -> TfR {
        TfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel halted"]
    #[inline(always)]
    pub fn ch(&self) -> ChR {
        ChR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL response received interrupt"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK response received interrupt"]
    #[inline(always)]
    pub fn nak(&self) -> NakR {
        NakR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - USB bus error"]
    #[inline(always)]
    pub fn usber(&self) -> UsberR {
        UsberR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble error"]
    #[inline(always)]
    pub fn bber(&self) -> BberR {
        BberR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Request queue overrun"]
    #[inline(always)]
    pub fn reqovr(&self) -> ReqovrR {
        ReqovrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data toggle error"]
    #[inline(always)]
    pub fn dter(&self) -> DterR {
        DterR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer finished"]
    #[inline(always)]
    #[must_use]
    pub fn tf(&mut self) -> TfW<Hch6intfSpec> {
        TfW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel halted"]
    #[inline(always)]
    #[must_use]
    pub fn ch(&mut self) -> ChW<Hch6intfSpec> {
        ChW::new(self, 1)
    }
    #[doc = "Bit 3 - STALL response received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> StallW<Hch6intfSpec> {
        StallW::new(self, 3)
    }
    #[doc = "Bit 4 - NAK response received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NakW<Hch6intfSpec> {
        NakW::new(self, 4)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> AckW<Hch6intfSpec> {
        AckW::new(self, 5)
    }
    #[doc = "Bit 7 - USB bus error"]
    #[inline(always)]
    #[must_use]
    pub fn usber(&mut self) -> UsberW<Hch6intfSpec> {
        UsberW::new(self, 7)
    }
    #[doc = "Bit 8 - Babble error"]
    #[inline(always)]
    #[must_use]
    pub fn bber(&mut self) -> BberW<Hch6intfSpec> {
        BberW::new(self, 8)
    }
    #[doc = "Bit 9 - Request queue overrun"]
    #[inline(always)]
    #[must_use]
    pub fn reqovr(&mut self) -> ReqovrW<Hch6intfSpec> {
        ReqovrW::new(self, 9)
    }
    #[doc = "Bit 10 - Data toggle error"]
    #[inline(always)]
    #[must_use]
    pub fn dter(&mut self) -> DterW<Hch6intfSpec> {
        DterW::new(self, 10)
    }
}
#[doc = "host channel-6 interrupt register (HCH6INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch6intf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch6intf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hch6intfSpec;
impl crate::RegisterSpec for Hch6intfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hch6intf::R`](R) reader structure"]
impl crate::Readable for Hch6intfSpec {}
#[doc = "`write(|w| ..)` method takes [`hch6intf::W`](W) writer structure"]
impl crate::Writable for Hch6intfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCH6INTF to value 0"]
impl crate::Resettable for Hch6intfSpec {
    const RESET_VALUE: u32 = 0;
}
