#[doc = "Register `SWT` writer"]
pub type W = crate::W<SwtSpec>;
#[doc = "DAC0 software trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swtr0 {
    #[doc = "0: DAC channel X software trigger disabled"]
    Disabled = 0,
    #[doc = "1: DAC channel X software trigger enabled"]
    Enabled = 1,
}
impl From<Swtr0> for bool {
    #[inline(always)]
    fn from(variant: Swtr0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWTR0` writer - DAC0 software trigger"]
pub type Swtr0W<'a, REG> = crate::BitWriter<'a, REG, Swtr0>;
impl<'a, REG> Swtr0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC channel X software trigger disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Swtr0::Disabled)
    }
    #[doc = "DAC channel X software trigger enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Swtr0::Enabled)
    }
}
impl W {
    #[doc = "Bit 0 - DAC0 software trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtr0(&mut self) -> Swtr0W<SwtSpec> {
        Swtr0W::new(self, 0)
    }
}
#[doc = "software trigger register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwtSpec;
impl crate::RegisterSpec for SwtSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swt::W`](W) writer structure"]
impl crate::Writable for SwtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWT to value 0"]
impl crate::Resettable for SwtSpec {
    const RESET_VALUE: u32 = 0;
}
