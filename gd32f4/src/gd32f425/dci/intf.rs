#[doc = "Register `INTF` reader"]
pub type R = crate::R<IntfSpec>;
#[doc = "Field `EFIF` reader - End of Frame Interrupt Flag"]
pub type EfifR = crate::BitReader;
#[doc = "Field `OVRIF` reader - FIFO Overrun Interrupt Flag"]
pub type OvrifR = crate::BitReader;
#[doc = "Field `ESEIF` reader - Embedded Synchronous Error Interrupt Flag"]
pub type EseifR = crate::BitReader;
#[doc = "Field `VSIF` reader - Vsync Interrupt Flag"]
pub type VsifR = crate::BitReader;
#[doc = "Field `ELIF` reader - End of Line Interrupt Flag"]
pub type ElifR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - End of Frame Interrupt Flag"]
    #[inline(always)]
    pub fn efif(&self) -> EfifR {
        EfifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Overrun Interrupt Flag"]
    #[inline(always)]
    pub fn ovrif(&self) -> OvrifR {
        OvrifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Embedded Synchronous Error Interrupt Flag"]
    #[inline(always)]
    pub fn eseif(&self) -> EseifR {
        EseifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Vsync Interrupt Flag"]
    #[inline(always)]
    pub fn vsif(&self) -> VsifR {
        VsifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Line Interrupt Flag"]
    #[inline(always)]
    pub fn elif(&self) -> ElifR {
        ElifR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfSpec;
impl crate::RegisterSpec for IntfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf::R`](R) reader structure"]
impl crate::Readable for IntfSpec {}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for IntfSpec {
    const RESET_VALUE: u32 = 0;
}
