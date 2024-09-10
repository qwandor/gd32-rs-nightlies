#[doc = "Register `IPA_BLOFF` reader"]
pub type R = crate::R<IpaBloffSpec>;
#[doc = "Register `IPA_BLOFF` writer"]
pub type W = crate::W<IpaBloffSpec>;
#[doc = "Field `BLOFF` reader - Background line offset"]
pub type BloffR = crate::FieldReader<u16>;
#[doc = "Field `BLOFF` writer - Background line offset"]
pub type BloffW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Background line offset"]
    #[inline(always)]
    pub fn bloff(&self) -> BloffR {
        BloffR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Background line offset"]
    #[inline(always)]
    #[must_use]
    pub fn bloff(&mut self) -> BloffW<IpaBloffSpec> {
        BloffW::new(self, 0)
    }
}
#[doc = "Background line offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_bloff::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_bloff::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpaBloffSpec;
impl crate::RegisterSpec for IpaBloffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipa_bloff::R`](R) reader structure"]
impl crate::Readable for IpaBloffSpec {}
#[doc = "`write(|w| ..)` method takes [`ipa_bloff::W`](W) writer structure"]
impl crate::Writable for IpaBloffSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPA_BLOFF to value 0"]
impl crate::Resettable for IpaBloffSpec {
    const RESET_VALUE: u32 = 0;
}
