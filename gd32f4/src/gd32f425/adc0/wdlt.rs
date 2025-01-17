#[doc = "Register `WDLT` reader"]
pub type R = crate::R<WdltSpec>;
#[doc = "Register `WDLT` writer"]
pub type W = crate::W<WdltSpec>;
#[doc = "Field `WDLT` reader - Analog watchdog lower threshold"]
pub type WdltR = crate::FieldReader<u16>;
#[doc = "Field `WDLT` writer - Analog watchdog lower threshold"]
pub type WdltW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Analog watchdog lower threshold"]
    #[inline(always)]
    pub fn wdlt(&self) -> WdltR {
        WdltR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog lower threshold"]
    #[inline(always)]
    #[must_use]
    pub fn wdlt(&mut self) -> WdltW<WdltSpec> {
        WdltW::new(self, 0)
    }
}
#[doc = "watchdog lower threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdlt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdlt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdltSpec;
impl crate::RegisterSpec for WdltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdlt::R`](R) reader structure"]
impl crate::Readable for WdltSpec {}
#[doc = "`write(|w| ..)` method takes [`wdlt::W`](W) writer structure"]
impl crate::Writable for WdltSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDLT to value 0"]
impl crate::Resettable for WdltSpec {
    const RESET_VALUE: u32 = 0;
}
