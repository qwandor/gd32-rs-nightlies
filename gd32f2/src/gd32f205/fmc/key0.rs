#[doc = "Register `KEY0` writer"]
pub type W = crate::W<Key0Spec>;
#[doc = "Field `KEY` writer - FMC_CTL0 unlock key"]
pub type KeyW<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - FMC_CTL0 unlock key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<Key0Spec> {
        KeyW::new(self, 0)
    }
}
#[doc = "Flash unlock key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key0Spec;
impl crate::RegisterSpec for Key0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key0::W`](W) writer structure"]
impl crate::Writable for Key0Spec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY0 to value 0"]
impl crate::Resettable for Key0Spec {
    const RESET_VALUE: u32 = 0;
}
