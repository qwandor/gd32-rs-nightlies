#[doc = "Register `IPA_DPV_ARGB4444` reader"]
pub type R = crate::R<IpaDpvArgb4444Spec>;
#[doc = "Register `IPA_DPV_ARGB4444` writer"]
pub type W = crate::W<IpaDpvArgb4444Spec>;
#[doc = "Field `DPDBV` reader - Destination pre-defined blue value"]
pub type DpdbvR = crate::FieldReader;
#[doc = "Field `DPDBV` writer - Destination pre-defined blue value"]
pub type DpdbvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DPDGV` reader - Destination pre-defined green value"]
pub type DpdgvR = crate::FieldReader;
#[doc = "Field `DPDGV` writer - Destination pre-defined green value"]
pub type DpdgvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DPDRV` reader - Destination pre-defined red value"]
pub type DpdrvR = crate::FieldReader;
#[doc = "Field `DPDRV` writer - Destination pre-defined red value"]
pub type DpdrvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DPDAV` reader - Destination pre-defined alpha value"]
pub type DpdavR = crate::FieldReader;
#[doc = "Field `DPDAV` writer - Destination pre-defined alpha value"]
pub type DpdavW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Destination pre-defined blue value"]
    #[inline(always)]
    pub fn dpdbv(&self) -> DpdbvR {
        DpdbvR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Destination pre-defined green value"]
    #[inline(always)]
    pub fn dpdgv(&self) -> DpdgvR {
        DpdgvR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Destination pre-defined red value"]
    #[inline(always)]
    pub fn dpdrv(&self) -> DpdrvR {
        DpdrvR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Destination pre-defined alpha value"]
    #[inline(always)]
    pub fn dpdav(&self) -> DpdavR {
        DpdavR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Destination pre-defined blue value"]
    #[inline(always)]
    #[must_use]
    pub fn dpdbv(&mut self) -> DpdbvW<IpaDpvArgb4444Spec> {
        DpdbvW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Destination pre-defined green value"]
    #[inline(always)]
    #[must_use]
    pub fn dpdgv(&mut self) -> DpdgvW<IpaDpvArgb4444Spec> {
        DpdgvW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Destination pre-defined red value"]
    #[inline(always)]
    #[must_use]
    pub fn dpdrv(&mut self) -> DpdrvW<IpaDpvArgb4444Spec> {
        DpdrvW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Destination pre-defined alpha value"]
    #[inline(always)]
    #[must_use]
    pub fn dpdav(&mut self) -> DpdavW<IpaDpvArgb4444Spec> {
        DpdavW::new(self, 12)
    }
}
#[doc = "Destination pixel value register(When the destination pixel format is ARGB4444,)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_dpv_argb4444::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_dpv_argb4444::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpaDpvArgb4444Spec;
impl crate::RegisterSpec for IpaDpvArgb4444Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipa_dpv_argb4444::R`](R) reader structure"]
impl crate::Readable for IpaDpvArgb4444Spec {}
#[doc = "`write(|w| ..)` method takes [`ipa_dpv_argb4444::W`](W) writer structure"]
impl crate::Writable for IpaDpvArgb4444Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPA_DPV_ARGB4444 to value 0"]
impl crate::Resettable for IpaDpvArgb4444Spec {
    const RESET_VALUE: u32 = 0;
}
