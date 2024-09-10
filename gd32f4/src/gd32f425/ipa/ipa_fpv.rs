#[doc = "Register `IPA_FPV` reader"]
pub type R = crate::R<IpaFpvSpec>;
#[doc = "Register `IPA_FPV` writer"]
pub type W = crate::W<IpaFpvSpec>;
#[doc = "Field `FPDBV` reader - Foreground pre-defined blue value"]
pub type FpdbvR = crate::FieldReader;
#[doc = "Field `FPDBV` writer - Foreground pre-defined blue value"]
pub type FpdbvW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FPDGV` reader - Foreground pre-defined green value"]
pub type FpdgvR = crate::FieldReader;
#[doc = "Field `FPDGV` writer - Foreground pre-defined green value"]
pub type FpdgvW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FPDRV` reader - Foreground pre-defined red value"]
pub type FpdrvR = crate::FieldReader;
#[doc = "Field `FPDRV` writer - Foreground pre-defined red value"]
pub type FpdrvW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Foreground pre-defined blue value"]
    #[inline(always)]
    pub fn fpdbv(&self) -> FpdbvR {
        FpdbvR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Foreground pre-defined green value"]
    #[inline(always)]
    pub fn fpdgv(&self) -> FpdgvR {
        FpdgvR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Foreground pre-defined red value"]
    #[inline(always)]
    pub fn fpdrv(&self) -> FpdrvR {
        FpdrvR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Foreground pre-defined blue value"]
    #[inline(always)]
    #[must_use]
    pub fn fpdbv(&mut self) -> FpdbvW<IpaFpvSpec> {
        FpdbvW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Foreground pre-defined green value"]
    #[inline(always)]
    #[must_use]
    pub fn fpdgv(&mut self) -> FpdgvW<IpaFpvSpec> {
        FpdgvW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Foreground pre-defined red value"]
    #[inline(always)]
    #[must_use]
    pub fn fpdrv(&mut self) -> FpdrvW<IpaFpvSpec> {
        FpdrvW::new(self, 16)
    }
}
#[doc = "Foreground pixel value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_fpv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_fpv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpaFpvSpec;
impl crate::RegisterSpec for IpaFpvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipa_fpv::R`](R) reader structure"]
impl crate::Readable for IpaFpvSpec {}
#[doc = "`write(|w| ..)` method takes [`ipa_fpv::W`](W) writer structure"]
impl crate::Writable for IpaFpvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPA_FPV to value 0"]
impl crate::Resettable for IpaFpvSpec {
    const RESET_VALUE: u32 = 0;
}
