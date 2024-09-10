#[doc = "Register `WND` reader"]
pub type R = crate::R<WndSpec>;
#[doc = "Field `WND` reader - Watchdog counter window value"]
pub type WndR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Watchdog counter window value"]
    #[inline(always)]
    pub fn wnd(&self) -> WndR {
        WndR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Window register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wnd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WndSpec;
impl crate::RegisterSpec for WndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wnd::R`](R) reader structure"]
impl crate::Readable for WndSpec {}
#[doc = "`reset()` method sets WND to value 0"]
impl crate::Resettable for WndSpec {
    const RESET_VALUE: u32 = 0;
}
