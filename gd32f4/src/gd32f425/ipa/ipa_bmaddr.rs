#[doc = "Register `IPA_BMADDR` reader"]
pub type R = crate::R<IpaBmaddrSpec>;
#[doc = "Register `IPA_BMADDR` writer"]
pub type W = crate::W<IpaBmaddrSpec>;
#[doc = "Field `BMADDR` reader - Background memory base address"]
pub type BmaddrR = crate::FieldReader<u32>;
#[doc = "Field `BMADDR` writer - Background memory base address"]
pub type BmaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Background memory base address"]
    #[inline(always)]
    pub fn bmaddr(&self) -> BmaddrR {
        BmaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Background memory base address"]
    #[inline(always)]
    #[must_use]
    pub fn bmaddr(&mut self) -> BmaddrW<IpaBmaddrSpec> {
        BmaddrW::new(self, 0)
    }
}
#[doc = "Background memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_bmaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_bmaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpaBmaddrSpec;
impl crate::RegisterSpec for IpaBmaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipa_bmaddr::R`](R) reader structure"]
impl crate::Readable for IpaBmaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`ipa_bmaddr::W`](W) writer structure"]
impl crate::Writable for IpaBmaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPA_BMADDR to value 0"]
impl crate::Resettable for IpaBmaddrSpec {
    const RESET_VALUE: u32 = 0;
}
