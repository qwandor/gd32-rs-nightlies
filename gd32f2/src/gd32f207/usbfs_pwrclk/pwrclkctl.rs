#[doc = "Register `PWRCLKCTL` reader"]
pub type R = crate::R<PwrclkctlSpec>;
#[doc = "Register `PWRCLKCTL` writer"]
pub type W = crate::W<PwrclkctlSpec>;
#[doc = "Field `SUCLK` reader - Stop the USB clock"]
pub type SuclkR = crate::BitReader;
#[doc = "Field `SUCLK` writer - Stop the USB clock"]
pub type SuclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHCLK` reader - Stop HCLK"]
pub type ShclkR = crate::BitReader;
#[doc = "Field `SHCLK` writer - Stop HCLK"]
pub type ShclkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stop the USB clock"]
    #[inline(always)]
    pub fn suclk(&self) -> SuclkR {
        SuclkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stop HCLK"]
    #[inline(always)]
    pub fn shclk(&self) -> ShclkR {
        ShclkR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop the USB clock"]
    #[inline(always)]
    #[must_use]
    pub fn suclk(&mut self) -> SuclkW<PwrclkctlSpec> {
        SuclkW::new(self, 0)
    }
    #[doc = "Bit 1 - Stop HCLK"]
    #[inline(always)]
    #[must_use]
    pub fn shclk(&mut self) -> ShclkW<PwrclkctlSpec> {
        ShclkW::new(self, 1)
    }
}
#[doc = "power and clock gating control register (PWRCLKCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrclkctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrclkctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrclkctlSpec;
impl crate::RegisterSpec for PwrclkctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrclkctl::R`](R) reader structure"]
impl crate::Readable for PwrclkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrclkctl::W`](W) writer structure"]
impl crate::Writable for PwrclkctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRCLKCTL to value 0"]
impl crate::Resettable for PwrclkctlSpec {
    const RESET_VALUE: u32 = 0;
}
