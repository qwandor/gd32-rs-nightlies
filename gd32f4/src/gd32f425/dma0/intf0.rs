#[doc = "Register `INTF0` reader"]
pub type R = crate::R<Intf0Spec>;
#[doc = "Field `FEEIF0` reader - FIFO error and exception of channel 0"]
pub type Feeif0R = crate::BitReader;
#[doc = "Field `SDEIF0` reader - Single data mode exception of channel 0"]
pub type Sdeif0R = crate::BitReader;
#[doc = "Field `TAEIF0` reader - Transfer access error flag of channel 0"]
pub type Taeif0R = crate::BitReader;
#[doc = "Field `HTFIF0` reader - Half transfer finish flag of channel 0"]
pub type Htfif0R = crate::BitReader;
#[doc = "Field `FTFIF0` reader - Full Transfer finish flag of channel 0"]
pub type Ftfif0R = crate::BitReader;
#[doc = "Field `FEEIF1` reader - FIFO error and exception of channel 1"]
pub type Feeif1R = crate::BitReader;
#[doc = "Field `SDEIF1` reader - Single data mode exception of channel 1"]
pub type Sdeif1R = crate::BitReader;
#[doc = "Field `TAEIF1` reader - Transfer access error flag of channel 1"]
pub type Taeif1R = crate::BitReader;
#[doc = "Field `HTFIF1` reader - Half transfer finish flag of channel 1"]
pub type Htfif1R = crate::BitReader;
#[doc = "Field `FTFIF1` reader - Full Transfer finish flag of channel 1"]
pub type Ftfif1R = crate::BitReader;
#[doc = "Field `FEEIF2` reader - FIFO error and exception of channel 2"]
pub type Feeif2R = crate::BitReader;
#[doc = "Field `SDEIF2` reader - Single data mode exception of channel 2"]
pub type Sdeif2R = crate::BitReader;
#[doc = "Field `TAEIF2` reader - Transfer access error flag of channel 2"]
pub type Taeif2R = crate::BitReader;
#[doc = "Field `HTFIF2` reader - Half transfer finish flag of channel 2"]
pub type Htfif2R = crate::BitReader;
#[doc = "Field `FTFIF2` reader - Full Transfer finish flag of channel 2"]
pub type Ftfif2R = crate::BitReader;
#[doc = "Field `FEEIF3` reader - FIFO error and exception of channel 3"]
pub type Feeif3R = crate::BitReader;
#[doc = "Field `SDEIF3` reader - Single data mode exception of channel 3"]
pub type Sdeif3R = crate::BitReader;
#[doc = "Field `TAEIF3` reader - Transfer access error flag of channel 3"]
pub type Taeif3R = crate::BitReader;
#[doc = "Field `HTFIF3` reader - Half transfer finish flag of channel 3"]
pub type Htfif3R = crate::BitReader;
#[doc = "Field `FTFIF3` reader - Full Transfer finish flag of channel 3"]
pub type Ftfif3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - FIFO error and exception of channel 0"]
    #[inline(always)]
    pub fn feeif0(&self) -> Feeif0R {
        Feeif0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Single data mode exception of channel 0"]
    #[inline(always)]
    pub fn sdeif0(&self) -> Sdeif0R {
        Sdeif0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer access error flag of channel 0"]
    #[inline(always)]
    pub fn taeif0(&self) -> Taeif0R {
        Taeif0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Half transfer finish flag of channel 0"]
    #[inline(always)]
    pub fn htfif0(&self) -> Htfif0R {
        Htfif0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Full Transfer finish flag of channel 0"]
    #[inline(always)]
    pub fn ftfif0(&self) -> Ftfif0R {
        Ftfif0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FIFO error and exception of channel 1"]
    #[inline(always)]
    pub fn feeif1(&self) -> Feeif1R {
        Feeif1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Single data mode exception of channel 1"]
    #[inline(always)]
    pub fn sdeif1(&self) -> Sdeif1R {
        Sdeif1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transfer access error flag of channel 1"]
    #[inline(always)]
    pub fn taeif1(&self) -> Taeif1R {
        Taeif1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Half transfer finish flag of channel 1"]
    #[inline(always)]
    pub fn htfif1(&self) -> Htfif1R {
        Htfif1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Full Transfer finish flag of channel 1"]
    #[inline(always)]
    pub fn ftfif1(&self) -> Ftfif1R {
        Ftfif1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - FIFO error and exception of channel 2"]
    #[inline(always)]
    pub fn feeif2(&self) -> Feeif2R {
        Feeif2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Single data mode exception of channel 2"]
    #[inline(always)]
    pub fn sdeif2(&self) -> Sdeif2R {
        Sdeif2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transfer access error flag of channel 2"]
    #[inline(always)]
    pub fn taeif2(&self) -> Taeif2R {
        Taeif2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Half transfer finish flag of channel 2"]
    #[inline(always)]
    pub fn htfif2(&self) -> Htfif2R {
        Htfif2R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Full Transfer finish flag of channel 2"]
    #[inline(always)]
    pub fn ftfif2(&self) -> Ftfif2R {
        Ftfif2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - FIFO error and exception of channel 3"]
    #[inline(always)]
    pub fn feeif3(&self) -> Feeif3R {
        Feeif3R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Single data mode exception of channel 3"]
    #[inline(always)]
    pub fn sdeif3(&self) -> Sdeif3R {
        Sdeif3R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Transfer access error flag of channel 3"]
    #[inline(always)]
    pub fn taeif3(&self) -> Taeif3R {
        Taeif3R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Half transfer finish flag of channel 3"]
    #[inline(always)]
    pub fn htfif3(&self) -> Htfif3R {
        Htfif3R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Full Transfer finish flag of channel 3"]
    #[inline(always)]
    pub fn ftfif3(&self) -> Ftfif3R {
        Ftfif3R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "Interrupt flag register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intf0Spec;
impl crate::RegisterSpec for Intf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf0::R`](R) reader structure"]
impl crate::Readable for Intf0Spec {}
#[doc = "`reset()` method sets INTF0 to value 0"]
impl crate::Resettable for Intf0Spec {
    const RESET_VALUE: u32 = 0;
}
