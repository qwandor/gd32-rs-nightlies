#[doc = "Register `KEY1H` writer"]
pub type W = crate::W<Key1hSpec>;
#[doc = "Field `KEY1H` writer - Key for DES,TDES,AES"]
pub type Key1hW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Key for DES,TDES,AES"]
    #[inline(always)]
    #[must_use]
    pub fn key1h(&mut self) -> Key1hW<Key1hSpec> {
        Key1hW::new(self, 0)
    }
}
#[doc = "CAU key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key1h::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key1hSpec;
impl crate::RegisterSpec for Key1hSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key1h::W`](W) writer structure"]
impl crate::Writable for Key1hSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY1H to value 0"]
impl crate::Resettable for Key1hSpec {
    const RESET_VALUE: u32 = 0;
}
