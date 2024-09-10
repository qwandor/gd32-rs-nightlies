#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Reset bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rstw {
    #[doc = "1: Resets the DATA register to IDATA, with no effect on FDATA"]
    Reset = 1,
}
impl From<Rstw> for bool {
    #[inline(always)]
    fn from(variant: Rstw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST` writer - Reset bit"]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG, Rstw>;
impl<'a, REG> RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resets the DATA register to IDATA, with no effect on FDATA"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Rstw::Reset)
    }
}
impl W {
    #[doc = "Bit 0 - Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RstW<CtlSpec> {
        RstW::new(self, 0)
    }
}
#[doc = "Control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
