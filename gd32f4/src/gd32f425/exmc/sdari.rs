#[doc = "Register `SDARI` reader"]
pub type R = crate::R<SdariSpec>;
#[doc = "Register `SDARI` writer"]
pub type W = crate::W<SdariSpec>;
#[doc = "Field `REC` reader - Refresh error flag clear"]
pub type RecR = crate::BitReader;
#[doc = "Field `REC` writer - Refresh error flag clear"]
pub type RecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARINTV` reader - Auto-Refresh Interval"]
pub type ArintvR = crate::FieldReader<u16>;
#[doc = "Field `ARINTV` writer - Auto-Refresh Interval"]
pub type ArintvW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `REIE` reader - Refresh error interrupt Enable"]
pub type ReieR = crate::BitReader;
#[doc = "Field `REIE` writer - Refresh error interrupt Enable"]
pub type ReieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Refresh error flag clear"]
    #[inline(always)]
    pub fn rec(&self) -> RecR {
        RecR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:13 - Auto-Refresh Interval"]
    #[inline(always)]
    pub fn arintv(&self) -> ArintvR {
        ArintvR::new(((self.bits >> 1) & 0x1fff) as u16)
    }
    #[doc = "Bit 14 - Refresh error interrupt Enable"]
    #[inline(always)]
    pub fn reie(&self) -> ReieR {
        ReieR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Refresh error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rec(&mut self) -> RecW<SdariSpec> {
        RecW::new(self, 0)
    }
    #[doc = "Bits 1:13 - Auto-Refresh Interval"]
    #[inline(always)]
    #[must_use]
    pub fn arintv(&mut self) -> ArintvW<SdariSpec> {
        ArintvW::new(self, 1)
    }
    #[doc = "Bit 14 - Refresh error interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn reie(&mut self) -> ReieW<SdariSpec> {
        ReieW::new(self, 14)
    }
}
#[doc = "SDRAM auto-refresh interval register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdari::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdari::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdariSpec;
impl crate::RegisterSpec for SdariSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdari::R`](R) reader structure"]
impl crate::Readable for SdariSpec {}
#[doc = "`write(|w| ..)` method takes [`sdari::W`](W) writer structure"]
impl crate::Writable for SdariSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDARI to value 0"]
impl crate::Resettable for SdariSpec {
    const RESET_VALUE: u32 = 0;
}
