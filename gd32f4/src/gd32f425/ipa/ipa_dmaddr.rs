#[doc = "Register `IPA_DMADDR` reader"]
pub type R = crate::R<IpaDmaddrSpec>;
#[doc = "Register `IPA_DMADDR` writer"]
pub type W = crate::W<IpaDmaddrSpec>;
#[doc = "Field `DMADDR` reader - Destination memory base address"]
pub type DmaddrR = crate::FieldReader<u32>;
#[doc = "Field `DMADDR` writer - Destination memory base address"]
pub type DmaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination memory base address"]
    #[inline(always)]
    pub fn dmaddr(&self) -> DmaddrR {
        DmaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination memory base address"]
    #[inline(always)]
    #[must_use]
    pub fn dmaddr(&mut self) -> DmaddrW<IpaDmaddrSpec> {
        DmaddrW::new(self, 0)
    }
}
#[doc = "Destination memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_dmaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_dmaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpaDmaddrSpec;
impl crate::RegisterSpec for IpaDmaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipa_dmaddr::R`](R) reader structure"]
impl crate::Readable for IpaDmaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`ipa_dmaddr::W`](W) writer structure"]
impl crate::Writable for IpaDmaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPA_DMADDR to value 0"]
impl crate::Resettable for IpaDmaddrSpec {
    const RESET_VALUE: u32 = 0;
}
