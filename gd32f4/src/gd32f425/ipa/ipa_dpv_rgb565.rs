#[doc = "Register `IPA_DPV_RGB565` reader"]
pub type R = crate::R<IpaDpvRgb565Spec>;
#[doc = "Register `IPA_DPV_RGB565` writer"]
pub type W = crate::W<IpaDpvRgb565Spec>;
#[doc = "Field `DPDBV` reader - Destination pre-defined blue value"]
pub type DpdbvR = crate::FieldReader;
#[doc = "Field `DPDBV` writer - Destination pre-defined blue value"]
pub type DpdbvW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DPDGV` reader - Destination pre-defined green value"]
pub type DpdgvR = crate::FieldReader;
#[doc = "Field `DPDGV` writer - Destination pre-defined green value"]
pub type DpdgvW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DPDRV` reader - Destination pre-defined red value"]
pub type DpdrvR = crate::FieldReader;
#[doc = "Field `DPDRV` writer - Destination pre-defined red value"]
pub type DpdrvW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Destination pre-defined blue value"]
    #[inline(always)]
    pub fn dpdbv(&self) -> DpdbvR {
        DpdbvR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:10 - Destination pre-defined green value"]
    #[inline(always)]
    pub fn dpdgv(&self) -> DpdgvR {
        DpdgvR::new(((self.bits >> 5) & 0x3f) as u8)
    }
    #[doc = "Bits 11:15 - Destination pre-defined red value"]
    #[inline(always)]
    pub fn dpdrv(&self) -> DpdrvR {
        DpdrvR::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Destination pre-defined blue value"]
    #[inline(always)]
    #[must_use]
    pub fn dpdbv(&mut self) -> DpdbvW<IpaDpvRgb565Spec> {
        DpdbvW::new(self, 0)
    }
    #[doc = "Bits 5:10 - Destination pre-defined green value"]
    #[inline(always)]
    #[must_use]
    pub fn dpdgv(&mut self) -> DpdgvW<IpaDpvRgb565Spec> {
        DpdgvW::new(self, 5)
    }
    #[doc = "Bits 11:15 - Destination pre-defined red value"]
    #[inline(always)]
    #[must_use]
    pub fn dpdrv(&mut self) -> DpdrvW<IpaDpvRgb565Spec> {
        DpdrvW::new(self, 11)
    }
}
#[doc = "Destination pixel value register(When the destination pixel format is RGB565)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_dpv_rgb565::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_dpv_rgb565::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpaDpvRgb565Spec;
impl crate::RegisterSpec for IpaDpvRgb565Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipa_dpv_rgb565::R`](R) reader structure"]
impl crate::Readable for IpaDpvRgb565Spec {}
#[doc = "`write(|w| ..)` method takes [`ipa_dpv_rgb565::W`](W) writer structure"]
impl crate::Writable for IpaDpvRgb565Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPA_DPV_RGB565 to value 0"]
impl crate::Resettable for IpaDpvRgb565Spec {
    const RESET_VALUE: u32 = 0;
}
