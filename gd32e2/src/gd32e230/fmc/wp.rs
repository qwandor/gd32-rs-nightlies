#[doc = "Register `WP` reader"]
pub type R = crate::R<WpSpec>;
#[doc = "Field `OB_WP` reader - Store WP\\[15:0\\]
of option bytes block after system reset"]
pub type ObWpR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Store WP\\[15:0\\]
of option bytes block after system reset"]
    #[inline(always)]
    pub fn ob_wp(&self) -> ObWpR {
        ObWpR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Erase/Program Protection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WpSpec;
impl crate::RegisterSpec for WpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wp::R`](R) reader structure"]
impl crate::Readable for WpSpec {}
#[doc = "`reset()` method sets WP to value 0"]
impl crate::Resettable for WpSpec {
    const RESET_VALUE: u32 = 0;
}
