#[doc = "Register `INTC0` writer"]
pub type W = crate::W<Intc0Spec>;
#[doc = "Field `FEEIFC0` writer - Clear bit for FIFO error and exception of channel 0"]
pub type Feeifc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDEIFC0` writer - Clear bit for single data mode exception of channel 0"]
pub type Sdeifc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAEIFC0` writer - Clear bit for transfer access error flag of channel 0"]
pub type Taeifc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIFC0` writer - Clear bit for half transfer finish flag of channel 0"]
pub type Htfifc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIFC0` writer - Clear bit for Full transfer finish flag of channel 0"]
pub type Ftfifc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEEIFC1` writer - Clear bit for FIFO error and exception of channel 1"]
pub type Feeifc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDEIFC1` writer - Clear bit for single data mode exception of channel 1"]
pub type Sdeifc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAEIFC1` writer - Clear bit for transfer access error flag of channel 1"]
pub type Taeifc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIFC1` writer - Clear bit for half transfer finish flag of channel 1"]
pub type Htfifc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIFC1` writer - Clear bit for Full transfer finish flag of channel 1"]
pub type Ftfifc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEEIFC2` writer - Clear bit for FIFO error and exception of channel 2"]
pub type Feeifc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDEIFC2` writer - Clear bit for single data mode exception of channel 2"]
pub type Sdeifc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAEIFC2` writer - Clear bit for transfer access error flag of channel 2"]
pub type Taeifc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIFC2` writer - Clear bit for half transfer finish flag of channel 2"]
pub type Htfifc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIFC2` writer - Clear bit for Full transfer finish flag of channel 2"]
pub type Ftfifc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEEIFC3` writer - Clear bit for FIFO error and exception of channel 3"]
pub type Feeifc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDEIFC3` writer - Clear bit for single data mode exception of channel 3"]
pub type Sdeifc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAEIFC3` writer - Clear bit for transfer access error flag of channel 3"]
pub type Taeifc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIFC3` writer - Clear bit for half transfer finish flag of channel 3"]
pub type Htfifc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIFC3` writer - Clear bit for Full transfer finish flag of channel 3"]
pub type Ftfifc3W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear bit for FIFO error and exception of channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn feeifc0(&mut self) -> Feeifc0W<Intc0Spec> {
        Feeifc0W::new(self, 0)
    }
    #[doc = "Bit 2 - Clear bit for single data mode exception of channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn sdeifc0(&mut self) -> Sdeifc0W<Intc0Spec> {
        Sdeifc0W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear bit for transfer access error flag of channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn taeifc0(&mut self) -> Taeifc0W<Intc0Spec> {
        Taeifc0W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear bit for half transfer finish flag of channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc0(&mut self) -> Htfifc0W<Intc0Spec> {
        Htfifc0W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear bit for Full transfer finish flag of channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc0(&mut self) -> Ftfifc0W<Intc0Spec> {
        Ftfifc0W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear bit for FIFO error and exception of channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn feeifc1(&mut self) -> Feeifc1W<Intc0Spec> {
        Feeifc1W::new(self, 6)
    }
    #[doc = "Bit 8 - Clear bit for single data mode exception of channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn sdeifc1(&mut self) -> Sdeifc1W<Intc0Spec> {
        Sdeifc1W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear bit for transfer access error flag of channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn taeifc1(&mut self) -> Taeifc1W<Intc0Spec> {
        Taeifc1W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear bit for half transfer finish flag of channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc1(&mut self) -> Htfifc1W<Intc0Spec> {
        Htfifc1W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear bit for Full transfer finish flag of channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc1(&mut self) -> Ftfifc1W<Intc0Spec> {
        Ftfifc1W::new(self, 11)
    }
    #[doc = "Bit 16 - Clear bit for FIFO error and exception of channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn feeifc2(&mut self) -> Feeifc2W<Intc0Spec> {
        Feeifc2W::new(self, 16)
    }
    #[doc = "Bit 18 - Clear bit for single data mode exception of channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn sdeifc2(&mut self) -> Sdeifc2W<Intc0Spec> {
        Sdeifc2W::new(self, 18)
    }
    #[doc = "Bit 19 - Clear bit for transfer access error flag of channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn taeifc2(&mut self) -> Taeifc2W<Intc0Spec> {
        Taeifc2W::new(self, 19)
    }
    #[doc = "Bit 20 - Clear bit for half transfer finish flag of channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc2(&mut self) -> Htfifc2W<Intc0Spec> {
        Htfifc2W::new(self, 20)
    }
    #[doc = "Bit 21 - Clear bit for Full transfer finish flag of channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc2(&mut self) -> Ftfifc2W<Intc0Spec> {
        Ftfifc2W::new(self, 21)
    }
    #[doc = "Bit 22 - Clear bit for FIFO error and exception of channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn feeifc3(&mut self) -> Feeifc3W<Intc0Spec> {
        Feeifc3W::new(self, 22)
    }
    #[doc = "Bit 24 - Clear bit for single data mode exception of channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn sdeifc3(&mut self) -> Sdeifc3W<Intc0Spec> {
        Sdeifc3W::new(self, 24)
    }
    #[doc = "Bit 25 - Clear bit for transfer access error flag of channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn taeifc3(&mut self) -> Taeifc3W<Intc0Spec> {
        Taeifc3W::new(self, 25)
    }
    #[doc = "Bit 26 - Clear bit for half transfer finish flag of channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc3(&mut self) -> Htfifc3W<Intc0Spec> {
        Htfifc3W::new(self, 26)
    }
    #[doc = "Bit 27 - Clear bit for Full transfer finish flag of channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc3(&mut self) -> Ftfifc3W<Intc0Spec> {
        Ftfifc3W::new(self, 27)
    }
}
#[doc = "Interrupt flag clear register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intc0Spec;
impl crate::RegisterSpec for Intc0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intc0::W`](W) writer structure"]
impl crate::Writable for Intc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTC0 to value 0"]
impl crate::Resettable for Intc0Spec {
    const RESET_VALUE: u32 = 0;
}
