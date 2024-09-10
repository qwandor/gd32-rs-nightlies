#[doc = "Register `L1SA` reader"]
pub type R = crate::R<L1saSpec>;
#[doc = "Register `L1SA` writer"]
pub type W = crate::W<L1saSpec>;
#[doc = "Field `SA` reader - Specified alpha"]
pub type SaR = crate::FieldReader;
#[doc = "Field `SA` writer - Specified alpha"]
pub type SaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Specified alpha"]
    #[inline(always)]
    pub fn sa(&self) -> SaR {
        SaR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specified alpha"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SaW<L1saSpec> {
        SaW::new(self, 0)
    }
}
#[doc = "Layer 1 specified alpha register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1sa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1sa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1saSpec;
impl crate::RegisterSpec for L1saSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1sa::R`](R) reader structure"]
impl crate::Readable for L1saSpec {}
#[doc = "`write(|w| ..)` method takes [`l1sa::W`](W) writer structure"]
impl crate::Writable for L1saSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1SA to value 0xff"]
impl crate::Resettable for L1saSpec {
    const RESET_VALUE: u32 = 0xff;
}
