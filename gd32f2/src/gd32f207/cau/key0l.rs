#[doc = "Register `KEY0L` writer"]
pub type W = crate::W<Key0lSpec>;
#[doc = "Field `KEY0L` writer - Key for DES,TDES,AES"]
pub type Key0lW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Key for DES,TDES,AES"]
    #[inline(always)]
    #[must_use]
    pub fn key0l(&mut self) -> Key0lW<Key0lSpec> {
        Key0lW::new(self, 0)
    }
}
#[doc = "CAU key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key0l::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key0lSpec;
impl crate::RegisterSpec for Key0lSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key0l::W`](W) writer structure"]
impl crate::Writable for Key0lSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY0L to value 0"]
impl crate::Resettable for Key0lSpec {
    const RESET_VALUE: u32 = 0;
}
