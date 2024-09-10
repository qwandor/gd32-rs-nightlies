#[doc = "Register `CH2FCTL` reader"]
pub type R = crate::R<Ch2fctlSpec>;
#[doc = "Register `CH2FCTL` writer"]
pub type W = crate::W<Ch2fctlSpec>;
#[doc = "Field `FCCV` reader - FIFO counter critical value"]
pub type FccvR = crate::FieldReader;
#[doc = "Field `FCCV` writer - FIFO counter critical value"]
pub type FccvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MDMEN` reader - Multi-data mode enable"]
pub type MdmenR = crate::BitReader;
#[doc = "Field `MDMEN` writer - Multi-data mode enable"]
pub type MdmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCNT` reader - FIFO counter"]
pub type FcntR = crate::FieldReader;
#[doc = "Field `FCNT` writer - FIFO counter"]
pub type FcntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FEEIE` reader - Enable bit for FIFO error and exception interrupt"]
pub type FeeieR = crate::BitReader;
#[doc = "Field `FEEIE` writer - Enable bit for FIFO error and exception interrupt"]
pub type FeeieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - FIFO counter critical value"]
    #[inline(always)]
    pub fn fccv(&self) -> FccvR {
        FccvR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Multi-data mode enable"]
    #[inline(always)]
    pub fn mdmen(&self) -> MdmenR {
        MdmenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - FIFO counter"]
    #[inline(always)]
    pub fn fcnt(&self) -> FcntR {
        FcntR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - Enable bit for FIFO error and exception interrupt"]
    #[inline(always)]
    pub fn feeie(&self) -> FeeieR {
        FeeieR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FIFO counter critical value"]
    #[inline(always)]
    #[must_use]
    pub fn fccv(&mut self) -> FccvW<Ch2fctlSpec> {
        FccvW::new(self, 0)
    }
    #[doc = "Bit 2 - Multi-data mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn mdmen(&mut self) -> MdmenW<Ch2fctlSpec> {
        MdmenW::new(self, 2)
    }
    #[doc = "Bits 3:5 - FIFO counter"]
    #[inline(always)]
    #[must_use]
    pub fn fcnt(&mut self) -> FcntW<Ch2fctlSpec> {
        FcntW::new(self, 3)
    }
    #[doc = "Bit 7 - Enable bit for FIFO error and exception interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn feeie(&mut self) -> FeeieW<Ch2fctlSpec> {
        FeeieW::new(self, 7)
    }
}
#[doc = "Channel 2 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2fctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2fctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2fctlSpec;
impl crate::RegisterSpec for Ch2fctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2fctl::R`](R) reader structure"]
impl crate::Readable for Ch2fctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2fctl::W`](W) writer structure"]
impl crate::Writable for Ch2fctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH2FCTL to value 0"]
impl crate::Resettable for Ch2fctlSpec {
    const RESET_VALUE: u32 = 0;
}
