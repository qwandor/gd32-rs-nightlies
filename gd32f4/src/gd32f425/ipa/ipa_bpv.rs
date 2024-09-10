#[doc = "Register `IPA_BPV` reader"]
pub type R = crate::R<IpaBpvSpec>;
#[doc = "Register `IPA_BPV` writer"]
pub type W = crate::W<IpaBpvSpec>;
#[doc = "Field `BPDBV` reader - Background pre-defined blue value"]
pub type BpdbvR = crate::FieldReader;
#[doc = "Field `BPDBV` writer - Background pre-defined blue value"]
pub type BpdbvW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BPDGV` reader - Background pre-defined green value"]
pub type BpdgvR = crate::FieldReader;
#[doc = "Field `BPDGV` writer - Background pre-defined green value"]
pub type BpdgvW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BPDRV` reader - Background pre-defined red value"]
pub type BpdrvR = crate::FieldReader;
#[doc = "Field `BPDRV` writer - Background pre-defined red value"]
pub type BpdrvW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Background pre-defined blue value"]
    #[inline(always)]
    pub fn bpdbv(&self) -> BpdbvR {
        BpdbvR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Background pre-defined green value"]
    #[inline(always)]
    pub fn bpdgv(&self) -> BpdgvR {
        BpdgvR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Background pre-defined red value"]
    #[inline(always)]
    pub fn bpdrv(&self) -> BpdrvR {
        BpdrvR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Background pre-defined blue value"]
    #[inline(always)]
    #[must_use]
    pub fn bpdbv(&mut self) -> BpdbvW<IpaBpvSpec> {
        BpdbvW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Background pre-defined green value"]
    #[inline(always)]
    #[must_use]
    pub fn bpdgv(&mut self) -> BpdgvW<IpaBpvSpec> {
        BpdgvW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Background pre-defined red value"]
    #[inline(always)]
    #[must_use]
    pub fn bpdrv(&mut self) -> BpdrvW<IpaBpvSpec> {
        BpdrvW::new(self, 16)
    }
}
#[doc = "Background pixel value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_bpv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_bpv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpaBpvSpec;
impl crate::RegisterSpec for IpaBpvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipa_bpv::R`](R) reader structure"]
impl crate::Readable for IpaBpvSpec {}
#[doc = "`write(|w| ..)` method takes [`ipa_bpv::W`](W) writer structure"]
impl crate::Writable for IpaBpvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPA_BPV to value 0"]
impl crate::Resettable for IpaBpvSpec {
    const RESET_VALUE: u32 = 0;
}
