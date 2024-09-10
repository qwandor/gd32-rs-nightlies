#[doc = "Register `CHC` reader"]
pub type R = crate::R<ChcSpec>;
#[doc = "Register `CHC` writer"]
pub type W = crate::W<ChcSpec>;
#[doc = "Field `HCM` reader - Hardware flow control coherence mode"]
pub type HcmR = crate::BitReader;
#[doc = "Field `HCM` writer - Hardware flow control coherence mode"]
pub type HcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCM` reader - Parity check coherence mode"]
pub type PcmR = crate::BitReader;
#[doc = "Field `PCM` writer - Parity check coherence mode"]
pub type PcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCM` reader - Break frame coherence mode"]
pub type BcmR = crate::BitReader;
#[doc = "Field `BCM` writer - Break frame coherence mode"]
pub type BcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPERR` reader - Early parity error flag"]
pub type EperrR = crate::BitReader;
#[doc = "Field `EPERR` writer - Early parity error flag"]
pub type EperrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Hardware flow control coherence mode"]
    #[inline(always)]
    pub fn hcm(&self) -> HcmR {
        HcmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity check coherence mode"]
    #[inline(always)]
    pub fn pcm(&self) -> PcmR {
        PcmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Break frame coherence mode"]
    #[inline(always)]
    pub fn bcm(&self) -> BcmR {
        BcmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Early parity error flag"]
    #[inline(always)]
    pub fn eperr(&self) -> EperrR {
        EperrR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hardware flow control coherence mode"]
    #[inline(always)]
    #[must_use]
    pub fn hcm(&mut self) -> HcmW<ChcSpec> {
        HcmW::new(self, 0)
    }
    #[doc = "Bit 1 - Parity check coherence mode"]
    #[inline(always)]
    #[must_use]
    pub fn pcm(&mut self) -> PcmW<ChcSpec> {
        PcmW::new(self, 1)
    }
    #[doc = "Bit 2 - Break frame coherence mode"]
    #[inline(always)]
    #[must_use]
    pub fn bcm(&mut self) -> BcmW<ChcSpec> {
        BcmW::new(self, 2)
    }
    #[doc = "Bit 8 - Early parity error flag"]
    #[inline(always)]
    #[must_use]
    pub fn eperr(&mut self) -> EperrW<ChcSpec> {
        EperrW::new(self, 8)
    }
}
#[doc = "Coherence control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChcSpec;
impl crate::RegisterSpec for ChcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chc::R`](R) reader structure"]
impl crate::Readable for ChcSpec {}
#[doc = "`write(|w| ..)` method takes [`chc::W`](W) writer structure"]
impl crate::Writable for ChcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHC to value 0"]
impl crate::Resettable for ChcSpec {
    const RESET_VALUE: u32 = 0;
}
