#[doc = "Register `IPA_DPV_ARGB1555` reader"]
pub type R = crate::R<IpaDpvArgb1555Spec>;
#[doc = "Register `IPA_DPV_ARGB1555` writer"]
pub type W = crate::W<IpaDpvArgb1555Spec>;
#[doc = "Field `DPDBV` reader - Destination pre-defined blue value"]
pub type DpdbvR = crate::FieldReader;
#[doc = "Field `DPDBV` writer - Destination pre-defined blue value"]
pub type DpdbvW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DPDGV` reader - Destination pre-defined green value"]
pub type DpdgvR = crate::FieldReader;
#[doc = "Field `DPDGV` writer - Destination pre-defined green value"]
pub type DpdgvW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DPDRV` reader - Destination pre-defined red value"]
pub type DpdrvR = crate::FieldReader;
#[doc = "Field `DPDRV` writer - Destination pre-defined red value"]
pub type DpdrvW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DPDAV` reader - Destination pre-defined alpha value"]
pub type DpdavR = crate::BitReader;
#[doc = "Field `DPDAV` writer - Destination pre-defined alpha value"]
pub type DpdavW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Destination pre-defined blue value"]
    #[inline(always)]
    pub fn dpdbv(&self) -> DpdbvR {
        DpdbvR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Destination pre-defined green value"]
    #[inline(always)]
    pub fn dpdgv(&self) -> DpdgvR {
        DpdgvR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Destination pre-defined red value"]
    #[inline(always)]
    pub fn dpdrv(&self) -> DpdrvR {
        DpdrvR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Destination pre-defined alpha value"]
    #[inline(always)]
    pub fn dpdav(&self) -> DpdavR {
        DpdavR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Destination pre-defined blue value"]
    #[inline(always)]
    #[must_use]
    pub fn dpdbv(&mut self) -> DpdbvW<IpaDpvArgb1555Spec> {
        DpdbvW::new(self, 0)
    }
    #[doc = "Bits 5:9 - Destination pre-defined green value"]
    #[inline(always)]
    #[must_use]
    pub fn dpdgv(&mut self) -> DpdgvW<IpaDpvArgb1555Spec> {
        DpdgvW::new(self, 5)
    }
    #[doc = "Bits 10:14 - Destination pre-defined red value"]
    #[inline(always)]
    #[must_use]
    pub fn dpdrv(&mut self) -> DpdrvW<IpaDpvArgb1555Spec> {
        DpdrvW::new(self, 10)
    }
    #[doc = "Bit 15 - Destination pre-defined alpha value"]
    #[inline(always)]
    #[must_use]
    pub fn dpdav(&mut self) -> DpdavW<IpaDpvArgb1555Spec> {
        DpdavW::new(self, 15)
    }
}
#[doc = "Destination pixel value register(When the destination pixel format is ARGB1555)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_dpv_argb1555::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_dpv_argb1555::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpaDpvArgb1555Spec;
impl crate::RegisterSpec for IpaDpvArgb1555Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipa_dpv_argb1555::R`](R) reader structure"]
impl crate::Readable for IpaDpvArgb1555Spec {}
#[doc = "`write(|w| ..)` method takes [`ipa_dpv_argb1555::W`](W) writer structure"]
impl crate::Writable for IpaDpvArgb1555Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPA_DPV_ARGB1555 to value 0"]
impl crate::Resettable for IpaDpvArgb1555Spec {
    const RESET_VALUE: u32 = 0;
}
