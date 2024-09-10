#[doc = "Register `KEY1` writer"]
pub type W = crate::W<Key1Spec>;
#[doc = "Field `KEY` writer - FMC_CTL1 unlock register"]
pub type KeyW<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - FMC_CTL1 unlock register"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<Key1Spec> {
        KeyW::new(self, 0)
    }
}
#[doc = "Unlock key register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key1Spec;
impl crate::RegisterSpec for Key1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key1::W`](W) writer structure"]
impl crate::Writable for Key1Spec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY1 to value 0"]
impl crate::Resettable for Key1Spec {
    const RESET_VALUE: u32 = 0;
}
