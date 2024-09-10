#[doc = "Register `KEY0H` writer"]
pub type W = crate::W<Key0hSpec>;
#[doc = "Field `KEY0H` writer - Key for DES,TDES,AES"]
pub type Key0hW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Key for DES,TDES,AES"]
    #[inline(always)]
    #[must_use]
    pub fn key0h(&mut self) -> Key0hW<Key0hSpec> {
        Key0hW::new(self, 0)
    }
}
#[doc = "CAU key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key0h::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key0hSpec;
impl crate::RegisterSpec for Key0hSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key0h::W`](W) writer structure"]
impl crate::Writable for Key0hSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY0H to value 0"]
impl crate::Resettable for Key0hSpec {
    const RESET_VALUE: u32 = 0;
}
