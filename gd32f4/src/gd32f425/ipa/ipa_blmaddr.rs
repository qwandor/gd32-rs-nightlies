#[doc = "Register `IPA_BLMADDR` reader"]
pub type R = crate::R<IpaBlmaddrSpec>;
#[doc = "Register `IPA_BLMADDR` writer"]
pub type W = crate::W<IpaBlmaddrSpec>;
#[doc = "Field `BLMADDR` reader - Background LUT memory base address"]
pub type BlmaddrR = crate::FieldReader<u32>;
#[doc = "Field `BLMADDR` writer - Background LUT memory base address"]
pub type BlmaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Background LUT memory base address"]
    #[inline(always)]
    pub fn blmaddr(&self) -> BlmaddrR {
        BlmaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Background LUT memory base address"]
    #[inline(always)]
    #[must_use]
    pub fn blmaddr(&mut self) -> BlmaddrW<IpaBlmaddrSpec> {
        BlmaddrW::new(self, 0)
    }
}
#[doc = "Background LUT memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_blmaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_blmaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpaBlmaddrSpec;
impl crate::RegisterSpec for IpaBlmaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipa_blmaddr::R`](R) reader structure"]
impl crate::Readable for IpaBlmaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`ipa_blmaddr::W`](W) writer structure"]
impl crate::Writable for IpaBlmaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPA_BLMADDR to value 0"]
impl crate::Resettable for IpaBlmaddrSpec {
    const RESET_VALUE: u32 = 0;
}
