#[doc = "Register `KEY2H` writer"]
pub type W = crate::W<Key2hSpec>;
#[doc = "Field `KEY2H` writer - Key for DES,TDES,AES"]
pub type Key2hW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Key for DES,TDES,AES"]
    #[inline(always)]
    #[must_use]
    pub fn key2h(&mut self) -> Key2hW<Key2hSpec> {
        Key2hW::new(self, 0)
    }
}
#[doc = "CAU key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key2h::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key2hSpec;
impl crate::RegisterSpec for Key2hSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key2h::W`](W) writer structure"]
impl crate::Writable for Key2hSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY2H to value 0"]
impl crate::Resettable for Key2hSpec {
    const RESET_VALUE: u32 = 0;
}
