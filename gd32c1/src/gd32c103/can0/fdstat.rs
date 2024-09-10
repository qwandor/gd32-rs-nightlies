#[doc = "Register `FDSTAT` reader"]
pub type R = crate::R<FdstatSpec>;
#[doc = "Register `FDSTAT` writer"]
pub type W = crate::W<FdstatSpec>;
#[doc = "Field `TDCV` reader - Transmitter delay compensation value"]
pub type TdcvR = crate::FieldReader;
#[doc = "Field `TDCV` writer - Transmitter delay compensation value"]
pub type TdcvW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PRE` reader - Protocol exception event"]
pub type PreR = crate::BitReader;
#[doc = "Field `PRE` writer - Protocol exception event"]
pub type PreW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Transmitter delay compensation value"]
    #[inline(always)]
    pub fn tdcv(&self) -> TdcvR {
        TdcvR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Protocol exception event"]
    #[inline(always)]
    pub fn pre(&self) -> PreR {
        PreR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transmitter delay compensation value"]
    #[inline(always)]
    #[must_use]
    pub fn tdcv(&mut self) -> TdcvW<FdstatSpec> {
        TdcvW::new(self, 0)
    }
    #[doc = "Bit 16 - Protocol exception event"]
    #[inline(always)]
    #[must_use]
    pub fn pre(&mut self) -> PreW<FdstatSpec> {
        PreW::new(self, 16)
    }
}
#[doc = "FD status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdstatSpec;
impl crate::RegisterSpec for FdstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdstat::R`](R) reader structure"]
impl crate::Readable for FdstatSpec {}
#[doc = "`write(|w| ..)` method takes [`fdstat::W`](W) writer structure"]
impl crate::Writable for FdstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDSTAT to value 0"]
impl crate::Resettable for FdstatSpec {
    const RESET_VALUE: u32 = 0;
}
