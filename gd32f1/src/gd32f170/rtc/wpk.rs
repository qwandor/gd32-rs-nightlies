#[doc = "Register `WPK` writer"]
pub type W = crate::W<WpkSpec>;
#[doc = "Field `WPK` writer - Write protection key"]
pub type WpkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Write protection key"]
    #[inline(always)]
    #[must_use]
    pub fn wpk(&mut self) -> WpkW<WpkSpec> {
        WpkW::new(self, 0)
    }
}
#[doc = "Write protection key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpk::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WpkSpec;
impl crate::RegisterSpec for WpkSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wpk::W`](W) writer structure"]
impl crate::Writable for WpkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WPK to value 0"]
impl crate::Resettable for WpkSpec {
    const RESET_VALUE: u32 = 0;
}
