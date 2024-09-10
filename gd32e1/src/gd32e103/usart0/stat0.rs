#[doc = "Register `STAT0` reader"]
pub type R = crate::R<Stat0Spec>;
#[doc = "Register `STAT0` writer"]
pub type W = crate::W<Stat0Spec>;
#[doc = "Field `PERR` reader - Parity error flag"]
pub type PerrR = crate::BitReader;
#[doc = "Field `FERR` reader - Frame error flag"]
pub type FerrR = crate::BitReader;
#[doc = "Field `NERR` reader - Noise error flag"]
pub type NerrR = crate::BitReader;
#[doc = "Field `ORERR` reader - Overrun error"]
pub type OrerrR = crate::BitReader;
#[doc = "Field `IDLEF` reader - IDLE frame detected flag"]
pub type IdlefR = crate::BitReader;
#[doc = "Field `RBNE` reader - Read data buffer not empty"]
pub type RbneR = crate::BitReader;
#[doc = "Field `RBNE` writer - Read data buffer not empty"]
pub type RbneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC` reader - Transmission complete"]
pub type TcR = crate::BitReader;
#[doc = "Field `TC` writer - Transmission complete"]
pub type TcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBE` reader - Transmit data buffer empty"]
pub type TbeR = crate::BitReader;
#[doc = "Field `LBDF` reader - LIN break detection flag"]
pub type LbdfR = crate::BitReader;
#[doc = "Field `LBDF` writer - LIN break detection flag"]
pub type LbdfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSF` reader - CTS change flag"]
pub type CtsfR = crate::BitReader;
#[doc = "Field `CTSF` writer - CTS change flag"]
pub type CtsfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Parity error flag"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame error flag"]
    #[inline(always)]
    pub fn ferr(&self) -> FerrR {
        FerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Noise error flag"]
    #[inline(always)]
    pub fn nerr(&self) -> NerrR {
        NerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error"]
    #[inline(always)]
    pub fn orerr(&self) -> OrerrR {
        OrerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE frame detected flag"]
    #[inline(always)]
    pub fn idlef(&self) -> IdlefR {
        IdlefR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read data buffer not empty"]
    #[inline(always)]
    pub fn rbne(&self) -> RbneR {
        RbneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit data buffer empty"]
    #[inline(always)]
    pub fn tbe(&self) -> TbeR {
        TbeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LIN break detection flag"]
    #[inline(always)]
    pub fn lbdf(&self) -> LbdfR {
        LbdfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS change flag"]
    #[inline(always)]
    pub fn ctsf(&self) -> CtsfR {
        CtsfR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Read data buffer not empty"]
    #[inline(always)]
    #[must_use]
    pub fn rbne(&mut self) -> RbneW<Stat0Spec> {
        RbneW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmission complete"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TcW<Stat0Spec> {
        TcW::new(self, 6)
    }
    #[doc = "Bit 8 - LIN break detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn lbdf(&mut self) -> LbdfW<Stat0Spec> {
        LbdfW::new(self, 8)
    }
    #[doc = "Bit 9 - CTS change flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctsf(&mut self) -> CtsfW<Stat0Spec> {
        CtsfW::new(self, 9)
    }
}
#[doc = "Status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stat0Spec;
impl crate::RegisterSpec for Stat0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat0::R`](R) reader structure"]
impl crate::Readable for Stat0Spec {}
#[doc = "`write(|w| ..)` method takes [`stat0::W`](W) writer structure"]
impl crate::Writable for Stat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT0 to value 0xc0"]
impl crate::Resettable for Stat0Spec {
    const RESET_VALUE: u32 = 0xc0;
}
