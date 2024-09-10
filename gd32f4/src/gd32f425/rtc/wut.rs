#[doc = "Register `WUT` reader"]
pub type R = crate::R<WutSpec>;
#[doc = "Register `WUT` writer"]
pub type W = crate::W<WutSpec>;
#[doc = "Field `WTRV` reader - Auto-wakeup timer reloads value"]
pub type WtrvR = crate::FieldReader<u16>;
#[doc = "Field `WTRV` writer - Auto-wakeup timer reloads value"]
pub type WtrvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Auto-wakeup timer reloads value"]
    #[inline(always)]
    pub fn wtrv(&self) -> WtrvR {
        WtrvR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Auto-wakeup timer reloads value"]
    #[inline(always)]
    #[must_use]
    pub fn wtrv(&mut self) -> WtrvW<WutSpec> {
        WtrvW::new(self, 0)
    }
}
#[doc = "Wakeup timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wut::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wut::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WutSpec;
impl crate::RegisterSpec for WutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wut::R`](R) reader structure"]
impl crate::Readable for WutSpec {}
#[doc = "`write(|w| ..)` method takes [`wut::W`](W) writer structure"]
impl crate::Writable for WutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WUT to value 0xffff"]
impl crate::Resettable for WutSpec {
    const RESET_VALUE: u32 = 0xffff;
}
