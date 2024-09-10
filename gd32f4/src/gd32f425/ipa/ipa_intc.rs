#[doc = "Register `IPA_INTC` reader"]
pub type R = crate::R<IpaIntcSpec>;
#[doc = "Register `IPA_INTC` writer"]
pub type W = crate::W<IpaIntcSpec>;
#[doc = "Field `TAEIFC` reader - Clear bit for transfer access error interrupt flag"]
pub type TaeifcR = crate::BitReader;
#[doc = "Field `TAEIFC` writer - Clear bit for transfer access error interrupt flag"]
pub type TaeifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFIFC` reader - Clear bit for full transfer finish interrupt flag"]
pub type TfifcR = crate::BitReader;
#[doc = "Field `TFIFC` writer - Clear bit for full transfer finish interrupt flag"]
pub type TfifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TLMIF` reader - Clear bit for transfer line mark interrupt flag"]
pub type TlmifR = crate::BitReader;
#[doc = "Field `TLMIF` writer - Clear bit for transfer line mark interrupt flag"]
pub type TlmifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LACIFC` reader - Clear bit for LUT access conflict interrupt flag"]
pub type LacifcR = crate::BitReader;
#[doc = "Field `LACIFC` writer - Clear bit for LUT access conflict interrupt flag"]
pub type LacifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LLFIFC` reader - Clear bit for LUT loading finish interrupt flag"]
pub type LlfifcR = crate::BitReader;
#[doc = "Field `LLFIFC` writer - Clear bit for LUT loading finish interrupt flag"]
pub type LlfifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWCFIF` reader - Clear bit for wrong configuration interrupt flag"]
pub type CwcfifR = crate::BitReader;
#[doc = "Field `CWCFIF` writer - Clear bit for wrong configuration interrupt flag"]
pub type CwcfifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear bit for transfer access error interrupt flag"]
    #[inline(always)]
    pub fn taeifc(&self) -> TaeifcR {
        TaeifcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear bit for full transfer finish interrupt flag"]
    #[inline(always)]
    pub fn tfifc(&self) -> TfifcR {
        TfifcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear bit for transfer line mark interrupt flag"]
    #[inline(always)]
    pub fn tlmif(&self) -> TlmifR {
        TlmifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear bit for LUT access conflict interrupt flag"]
    #[inline(always)]
    pub fn lacifc(&self) -> LacifcR {
        LacifcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear bit for LUT loading finish interrupt flag"]
    #[inline(always)]
    pub fn llfifc(&self) -> LlfifcR {
        LlfifcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear bit for wrong configuration interrupt flag"]
    #[inline(always)]
    pub fn cwcfif(&self) -> CwcfifR {
        CwcfifR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear bit for transfer access error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn taeifc(&mut self) -> TaeifcW<IpaIntcSpec> {
        TaeifcW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear bit for full transfer finish interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn tfifc(&mut self) -> TfifcW<IpaIntcSpec> {
        TfifcW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear bit for transfer line mark interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn tlmif(&mut self) -> TlmifW<IpaIntcSpec> {
        TlmifW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear bit for LUT access conflict interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn lacifc(&mut self) -> LacifcW<IpaIntcSpec> {
        LacifcW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear bit for LUT loading finish interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn llfifc(&mut self) -> LlfifcW<IpaIntcSpec> {
        LlfifcW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear bit for wrong configuration interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cwcfif(&mut self) -> CwcfifW<IpaIntcSpec> {
        CwcfifW::new(self, 5)
    }
}
#[doc = "Interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_intc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_intc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpaIntcSpec;
impl crate::RegisterSpec for IpaIntcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipa_intc::R`](R) reader structure"]
impl crate::Readable for IpaIntcSpec {}
#[doc = "`write(|w| ..)` method takes [`ipa_intc::W`](W) writer structure"]
impl crate::Writable for IpaIntcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPA_INTC to value 0"]
impl crate::Resettable for IpaIntcSpec {
    const RESET_VALUE: u32 = 0;
}
