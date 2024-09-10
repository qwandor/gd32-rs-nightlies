#[doc = "Register `IPA_FMADDR` reader"]
pub type R = crate::R<IpaFmaddrSpec>;
#[doc = "Register `IPA_FMADDR` writer"]
pub type W = crate::W<IpaFmaddrSpec>;
#[doc = "Field `FMADDR` reader - Foreground memory base address"]
pub type FmaddrR = crate::FieldReader<u32>;
#[doc = "Field `FMADDR` writer - Foreground memory base address"]
pub type FmaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Foreground memory base address"]
    #[inline(always)]
    pub fn fmaddr(&self) -> FmaddrR {
        FmaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Foreground memory base address"]
    #[inline(always)]
    #[must_use]
    pub fn fmaddr(&mut self) -> FmaddrW<IpaFmaddrSpec> {
        FmaddrW::new(self, 0)
    }
}
#[doc = "Foreground memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_fmaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_fmaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpaFmaddrSpec;
impl crate::RegisterSpec for IpaFmaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipa_fmaddr::R`](R) reader structure"]
impl crate::Readable for IpaFmaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`ipa_fmaddr::W`](W) writer structure"]
impl crate::Writable for IpaFmaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPA_FMADDR to value 0"]
impl crate::Resettable for IpaFmaddrSpec {
    const RESET_VALUE: u32 = 0;
}
