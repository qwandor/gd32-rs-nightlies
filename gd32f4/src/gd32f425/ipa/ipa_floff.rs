#[doc = "Register `IPA_FLOFF` reader"]
pub type R = crate::R<IpaFloffSpec>;
#[doc = "Register `IPA_FLOFF` writer"]
pub type W = crate::W<IpaFloffSpec>;
#[doc = "Field `FLOFF` reader - Foreground line offset"]
pub type FloffR = crate::FieldReader<u16>;
#[doc = "Field `FLOFF` writer - Foreground line offset"]
pub type FloffW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Foreground line offset"]
    #[inline(always)]
    pub fn floff(&self) -> FloffR {
        FloffR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Foreground line offset"]
    #[inline(always)]
    #[must_use]
    pub fn floff(&mut self) -> FloffW<IpaFloffSpec> {
        FloffW::new(self, 0)
    }
}
#[doc = "Foreground line offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_floff::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_floff::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpaFloffSpec;
impl crate::RegisterSpec for IpaFloffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipa_floff::R`](R) reader structure"]
impl crate::Readable for IpaFloffSpec {}
#[doc = "`write(|w| ..)` method takes [`ipa_floff::W`](W) writer structure"]
impl crate::Writable for IpaFloffSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPA_FLOFF to value 0"]
impl crate::Resettable for IpaFloffSpec {
    const RESET_VALUE: u32 = 0;
}
