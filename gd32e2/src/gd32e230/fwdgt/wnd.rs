#[doc = "Register `WND` reader"]
pub type R = crate::R<WndSpec>;
#[doc = "Register `WND` writer"]
pub type W = crate::W<WndSpec>;
#[doc = "Field `WND` reader - Watchdog counter window value"]
pub type WndR = crate::FieldReader<u16>;
#[doc = "Field `WND` writer - Watchdog counter window value"]
pub type WndW<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Watchdog counter window value"]
    #[inline(always)]
    pub fn wnd(&self) -> WndR {
        WndR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog counter window value"]
    #[inline(always)]
    #[must_use]
    pub fn wnd(&mut self) -> WndW<WndSpec> {
        WndW::new(self, 0)
    }
}
#[doc = "Window register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wnd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wnd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WndSpec;
impl crate::RegisterSpec for WndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wnd::R`](R) reader structure"]
impl crate::Readable for WndSpec {}
#[doc = "`write(|w| ..)` method takes [`wnd::W`](W) writer structure"]
impl crate::Writable for WndSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WND to value 0x0fff"]
impl crate::Resettable for WndSpec {
    const RESET_VALUE: u32 = 0x0fff;
}
