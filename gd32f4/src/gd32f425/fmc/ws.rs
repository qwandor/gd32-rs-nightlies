#[doc = "Register `WS` reader"]
pub type R = crate::R<WsSpec>;
#[doc = "Register `WS` writer"]
pub type W = crate::W<WsSpec>;
#[doc = "Field `WSCNT` reader - wait state counter register"]
pub type WscntR = crate::FieldReader;
#[doc = "Field `WSCNT` writer - wait state counter register"]
pub type WscntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - wait state counter register"]
    #[inline(always)]
    pub fn wscnt(&self) -> WscntR {
        WscntR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - wait state counter register"]
    #[inline(always)]
    #[must_use]
    pub fn wscnt(&mut self) -> WscntW<WsSpec> {
        WscntW::new(self, 0)
    }
}
#[doc = "wait state counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ws::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ws::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WsSpec;
impl crate::RegisterSpec for WsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ws::R`](R) reader structure"]
impl crate::Readable for WsSpec {}
#[doc = "`write(|w| ..)` method takes [`ws::W`](W) writer structure"]
impl crate::Writable for WsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WS to value 0"]
impl crate::Resettable for WsSpec {
    const RESET_VALUE: u32 = 0;
}
