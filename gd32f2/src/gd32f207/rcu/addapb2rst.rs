#[doc = "Register `ADDAPB2RST` reader"]
pub type R = crate::R<Addapb2rstSpec>;
#[doc = "Register `ADDAPB2RST` writer"]
pub type W = crate::W<Addapb2rstSpec>;
#[doc = "Field `USART5RST` reader - USART5 reset"]
pub type Usart5rstR = crate::BitReader;
#[doc = "Field `USART5RST` writer - USART5 reset"]
pub type Usart5rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TLIRST` reader - TLI reset"]
pub type TlirstR = crate::BitReader;
#[doc = "Field `TLIRST` writer - TLI reset"]
pub type TlirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHRST` reader - GPIO port H reset"]
pub type PhrstR = crate::BitReader;
#[doc = "Field `PHRST` writer - GPIO port H reset"]
pub type PhrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIRST` reader - GPIO port I reset"]
pub type PirstR = crate::BitReader;
#[doc = "Field `PIRST` writer - GPIO port I reset"]
pub type PirstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - USART5 reset"]
    #[inline(always)]
    pub fn usart5rst(&self) -> Usart5rstR {
        Usart5rstR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - TLI reset"]
    #[inline(always)]
    pub fn tlirst(&self) -> TlirstR {
        TlirstR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO port H reset"]
    #[inline(always)]
    pub fn phrst(&self) -> PhrstR {
        PhrstR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO port I reset"]
    #[inline(always)]
    pub fn pirst(&self) -> PirstR {
        PirstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - USART5 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart5rst(&mut self) -> Usart5rstW<Addapb2rstSpec> {
        Usart5rstW::new(self, 24)
    }
    #[doc = "Bit 26 - TLI reset"]
    #[inline(always)]
    #[must_use]
    pub fn tlirst(&mut self) -> TlirstW<Addapb2rstSpec> {
        TlirstW::new(self, 26)
    }
    #[doc = "Bit 30 - GPIO port H reset"]
    #[inline(always)]
    #[must_use]
    pub fn phrst(&mut self) -> PhrstW<Addapb2rstSpec> {
        PhrstW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO port I reset"]
    #[inline(always)]
    #[must_use]
    pub fn pirst(&mut self) -> PirstW<Addapb2rstSpec> {
        PirstW::new(self, 31)
    }
}
#[doc = "APB2 additional enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addapb2rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addapb2rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addapb2rstSpec;
impl crate::RegisterSpec for Addapb2rstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addapb2rst::R`](R) reader structure"]
impl crate::Readable for Addapb2rstSpec {}
#[doc = "`write(|w| ..)` method takes [`addapb2rst::W`](W) writer structure"]
impl crate::Writable for Addapb2rstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDAPB2RST to value 0"]
impl crate::Resettable for Addapb2rstSpec {
    const RESET_VALUE: u32 = 0;
}
