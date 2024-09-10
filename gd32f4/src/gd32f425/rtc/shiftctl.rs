#[doc = "Register `SHIFTCTL` writer"]
pub type W = crate::W<ShiftctlSpec>;
#[doc = "Field `SFS` writer - Subtract a fraction of a second"]
pub type SfsW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `A1S` writer - One second add"]
pub type A1sW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:14 - Subtract a fraction of a second"]
    #[inline(always)]
    #[must_use]
    pub fn sfs(&mut self) -> SfsW<ShiftctlSpec> {
        SfsW::new(self, 0)
    }
    #[doc = "Bit 31 - One second add"]
    #[inline(always)]
    #[must_use]
    pub fn a1s(&mut self) -> A1sW<ShiftctlSpec> {
        A1sW::new(self, 31)
    }
}
#[doc = "shift function control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shiftctl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShiftctlSpec;
impl crate::RegisterSpec for ShiftctlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`shiftctl::W`](W) writer structure"]
impl crate::Writable for ShiftctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHIFTCTL to value 0"]
impl crate::Resettable for ShiftctlSpec {
    const RESET_VALUE: u32 = 0;
}
