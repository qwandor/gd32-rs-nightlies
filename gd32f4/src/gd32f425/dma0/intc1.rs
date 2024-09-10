#[doc = "Register `INTC1` writer"]
pub type W = crate::W<Intc1Spec>;
#[doc = "Field `FEEIFC4` writer - Clear bit for FIFO error and exception of channel 4"]
pub type Feeifc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDEIFC4` writer - Clear bit for single data mode exception of channel 4"]
pub type Sdeifc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAEIFC4` writer - Clear bit for transfer access error flag of channel 4"]
pub type Taeifc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIFC4` writer - Clear bit for half transfer finish flag of channel 4"]
pub type Htfifc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIFC4` writer - Clear bit for Full transfer finish flag of channel 4"]
pub type Ftfifc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEEIFC5` writer - Clear bit for FIFO error and exception of channel 5"]
pub type Feeifc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDEIFC5` writer - Clear bit for single data mode exception of channel 5"]
pub type Sdeifc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAEIFC5` writer - Clear bit for transfer access error flag of channel 5"]
pub type Taeifc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIFC5` writer - Clear bit for half transfer finish flag of channel 5"]
pub type Htfifc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIFC5` writer - Clear bit for Full transfer finish flag of channel 5"]
pub type Ftfifc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEEIFC6` writer - Clear bit for FIFO error and exception of channel 6"]
pub type Feeifc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDEIFC6` writer - Clear bit for single data mode exception of channel 6"]
pub type Sdeifc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAEIFC6` writer - Clear bit for transfer access error flag of channel 6"]
pub type Taeifc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIFC6` writer - Clear bit for half transfer finish flag of channel 6"]
pub type Htfifc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIFC6` writer - Clear bit for Full transfer finish flag of channel 6"]
pub type Ftfifc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEEIFC7` writer - Clear bit for FIFO error and exception of channel 7"]
pub type Feeifc7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDEIFC7` writer - Clear bit for single data mode exception of channel 7"]
pub type Sdeifc7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAEIFC7` writer - Clear bit for transfer access error flag of channel 7"]
pub type Taeifc7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIFC7` writer - Clear bit for half transfer finish flag of channel 7"]
pub type Htfifc7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIFC7` writer - Clear bit for Full transfer finish flag of channel 7"]
pub type Ftfifc7W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear bit for FIFO error and exception of channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn feeifc4(&mut self) -> Feeifc4W<Intc1Spec> {
        Feeifc4W::new(self, 0)
    }
    #[doc = "Bit 2 - Clear bit for single data mode exception of channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn sdeifc4(&mut self) -> Sdeifc4W<Intc1Spec> {
        Sdeifc4W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear bit for transfer access error flag of channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn taeifc4(&mut self) -> Taeifc4W<Intc1Spec> {
        Taeifc4W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear bit for half transfer finish flag of channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc4(&mut self) -> Htfifc4W<Intc1Spec> {
        Htfifc4W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear bit for Full transfer finish flag of channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc4(&mut self) -> Ftfifc4W<Intc1Spec> {
        Ftfifc4W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear bit for FIFO error and exception of channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn feeifc5(&mut self) -> Feeifc5W<Intc1Spec> {
        Feeifc5W::new(self, 6)
    }
    #[doc = "Bit 8 - Clear bit for single data mode exception of channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn sdeifc5(&mut self) -> Sdeifc5W<Intc1Spec> {
        Sdeifc5W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear bit for transfer access error flag of channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn taeifc5(&mut self) -> Taeifc5W<Intc1Spec> {
        Taeifc5W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear bit for half transfer finish flag of channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc5(&mut self) -> Htfifc5W<Intc1Spec> {
        Htfifc5W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear bit for Full transfer finish flag of channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc5(&mut self) -> Ftfifc5W<Intc1Spec> {
        Ftfifc5W::new(self, 11)
    }
    #[doc = "Bit 16 - Clear bit for FIFO error and exception of channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn feeifc6(&mut self) -> Feeifc6W<Intc1Spec> {
        Feeifc6W::new(self, 16)
    }
    #[doc = "Bit 18 - Clear bit for single data mode exception of channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn sdeifc6(&mut self) -> Sdeifc6W<Intc1Spec> {
        Sdeifc6W::new(self, 18)
    }
    #[doc = "Bit 19 - Clear bit for transfer access error flag of channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn taeifc6(&mut self) -> Taeifc6W<Intc1Spec> {
        Taeifc6W::new(self, 19)
    }
    #[doc = "Bit 20 - Clear bit for half transfer finish flag of channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc6(&mut self) -> Htfifc6W<Intc1Spec> {
        Htfifc6W::new(self, 20)
    }
    #[doc = "Bit 21 - Clear bit for Full transfer finish flag of channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc6(&mut self) -> Ftfifc6W<Intc1Spec> {
        Ftfifc6W::new(self, 21)
    }
    #[doc = "Bit 22 - Clear bit for FIFO error and exception of channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn feeifc7(&mut self) -> Feeifc7W<Intc1Spec> {
        Feeifc7W::new(self, 22)
    }
    #[doc = "Bit 24 - Clear bit for single data mode exception of channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn sdeifc7(&mut self) -> Sdeifc7W<Intc1Spec> {
        Sdeifc7W::new(self, 24)
    }
    #[doc = "Bit 25 - Clear bit for transfer access error flag of channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn taeifc7(&mut self) -> Taeifc7W<Intc1Spec> {
        Taeifc7W::new(self, 25)
    }
    #[doc = "Bit 26 - Clear bit for half transfer finish flag of channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc7(&mut self) -> Htfifc7W<Intc1Spec> {
        Htfifc7W::new(self, 26)
    }
    #[doc = "Bit 27 - Clear bit for Full transfer finish flag of channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc7(&mut self) -> Ftfifc7W<Intc1Spec> {
        Ftfifc7W::new(self, 27)
    }
}
#[doc = "Interrupt flag clear register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intc1Spec;
impl crate::RegisterSpec for Intc1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intc1::W`](W) writer structure"]
impl crate::Writable for Intc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTC1 to value 0"]
impl crate::Resettable for Intc1Spec {
    const RESET_VALUE: u32 = 0;
}
