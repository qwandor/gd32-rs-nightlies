#[doc = "Register `INTF1` reader"]
pub type R = crate::R<Intf1Spec>;
#[doc = "Field `FEEIF4` reader - FIFO error and exception of channel 4"]
pub type Feeif4R = crate::BitReader;
#[doc = "Field `SDEIF4` reader - Single data mode exception of channel 4"]
pub type Sdeif4R = crate::BitReader;
#[doc = "Field `TAEIF4` reader - Transfer access error flag of channel 4"]
pub type Taeif4R = crate::BitReader;
#[doc = "Field `HTFIF4` reader - Half transfer finish flag of channel 4"]
pub type Htfif4R = crate::BitReader;
#[doc = "Field `FTFIF4` reader - Full Transfer finish flag of channel 4"]
pub type Ftfif4R = crate::BitReader;
#[doc = "Field `FEEIF5` reader - FIFO error and exception of channel 5"]
pub type Feeif5R = crate::BitReader;
#[doc = "Field `SDEIF5` reader - Single data mode exception of channel 5"]
pub type Sdeif5R = crate::BitReader;
#[doc = "Field `TAEIF5` reader - Transfer access error flag of channel 5"]
pub type Taeif5R = crate::BitReader;
#[doc = "Field `HTFIF5` reader - Half transfer finish flag of channel 5"]
pub type Htfif5R = crate::BitReader;
#[doc = "Field `FTFIF5` reader - Full Transfer finish flag of channel 5"]
pub type Ftfif5R = crate::BitReader;
#[doc = "Field `FEEIF6` reader - FIFO error and exception of channel 6"]
pub type Feeif6R = crate::BitReader;
#[doc = "Field `SDEIF6` reader - Single data mode exception of channel 6"]
pub type Sdeif6R = crate::BitReader;
#[doc = "Field `TAEIF6` reader - Transfer access error flag of channel 6"]
pub type Taeif6R = crate::BitReader;
#[doc = "Field `HTFIF6` reader - Half transfer finish flag of channel 6"]
pub type Htfif6R = crate::BitReader;
#[doc = "Field `FTFIF6` reader - Full Transfer finish flag of channel 6"]
pub type Ftfif6R = crate::BitReader;
#[doc = "Field `FEEIF7` reader - FIFO error and exception of channel 7"]
pub type Feeif7R = crate::BitReader;
#[doc = "Field `SDEIF7` reader - Single data mode exception of channel 7"]
pub type Sdeif7R = crate::BitReader;
#[doc = "Field `TAEIF7` reader - Transfer access error flag of channel 7"]
pub type Taeif7R = crate::BitReader;
#[doc = "Field `HTFIF7` reader - Half transfer finish flag of channel 7"]
pub type Htfif7R = crate::BitReader;
#[doc = "Field `FTFIF7` reader - Full Transfer finish flag of channel 7"]
pub type Ftfif7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - FIFO error and exception of channel 4"]
    #[inline(always)]
    pub fn feeif4(&self) -> Feeif4R {
        Feeif4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Single data mode exception of channel 4"]
    #[inline(always)]
    pub fn sdeif4(&self) -> Sdeif4R {
        Sdeif4R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer access error flag of channel 4"]
    #[inline(always)]
    pub fn taeif4(&self) -> Taeif4R {
        Taeif4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Half transfer finish flag of channel 4"]
    #[inline(always)]
    pub fn htfif4(&self) -> Htfif4R {
        Htfif4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Full Transfer finish flag of channel 4"]
    #[inline(always)]
    pub fn ftfif4(&self) -> Ftfif4R {
        Ftfif4R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FIFO error and exception of channel 5"]
    #[inline(always)]
    pub fn feeif5(&self) -> Feeif5R {
        Feeif5R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Single data mode exception of channel 5"]
    #[inline(always)]
    pub fn sdeif5(&self) -> Sdeif5R {
        Sdeif5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transfer access error flag of channel 5"]
    #[inline(always)]
    pub fn taeif5(&self) -> Taeif5R {
        Taeif5R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Half transfer finish flag of channel 5"]
    #[inline(always)]
    pub fn htfif5(&self) -> Htfif5R {
        Htfif5R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Full Transfer finish flag of channel 5"]
    #[inline(always)]
    pub fn ftfif5(&self) -> Ftfif5R {
        Ftfif5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - FIFO error and exception of channel 6"]
    #[inline(always)]
    pub fn feeif6(&self) -> Feeif6R {
        Feeif6R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Single data mode exception of channel 6"]
    #[inline(always)]
    pub fn sdeif6(&self) -> Sdeif6R {
        Sdeif6R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transfer access error flag of channel 6"]
    #[inline(always)]
    pub fn taeif6(&self) -> Taeif6R {
        Taeif6R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Half transfer finish flag of channel 6"]
    #[inline(always)]
    pub fn htfif6(&self) -> Htfif6R {
        Htfif6R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Full Transfer finish flag of channel 6"]
    #[inline(always)]
    pub fn ftfif6(&self) -> Ftfif6R {
        Ftfif6R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - FIFO error and exception of channel 7"]
    #[inline(always)]
    pub fn feeif7(&self) -> Feeif7R {
        Feeif7R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Single data mode exception of channel 7"]
    #[inline(always)]
    pub fn sdeif7(&self) -> Sdeif7R {
        Sdeif7R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Transfer access error flag of channel 7"]
    #[inline(always)]
    pub fn taeif7(&self) -> Taeif7R {
        Taeif7R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Half transfer finish flag of channel 7"]
    #[inline(always)]
    pub fn htfif7(&self) -> Htfif7R {
        Htfif7R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Full Transfer finish flag of channel 7"]
    #[inline(always)]
    pub fn ftfif7(&self) -> Ftfif7R {
        Ftfif7R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "Interrupt flag register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intf1Spec;
impl crate::RegisterSpec for Intf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf1::R`](R) reader structure"]
impl crate::Readable for Intf1Spec {}
#[doc = "`reset()` method sets INTF1 to value 0"]
impl crate::Resettable for Intf1Spec {
    const RESET_VALUE: u32 = 0;
}
