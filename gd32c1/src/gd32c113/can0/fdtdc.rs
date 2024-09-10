#[doc = "Register `FDTDC` reader"]
pub type R = crate::R<FdtdcSpec>;
#[doc = "Register `FDTDC` writer"]
pub type W = crate::W<FdtdcSpec>;
#[doc = "Field `TDCF` reader - Transmitter delay compensation filter"]
pub type TdcfR = crate::FieldReader;
#[doc = "Field `TDCF` writer - Transmitter delay compensation filter"]
pub type TdcfW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TDCO` reader - Transmitter delay compensation offset"]
pub type TdcoR = crate::FieldReader;
#[doc = "Field `TDCO` writer - Transmitter delay compensation offset"]
pub type TdcoW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Transmitter delay compensation filter"]
    #[inline(always)]
    pub fn tdcf(&self) -> TdcfR {
        TdcfR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Transmitter delay compensation offset"]
    #[inline(always)]
    pub fn tdco(&self) -> TdcoR {
        TdcoR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transmitter delay compensation filter"]
    #[inline(always)]
    #[must_use]
    pub fn tdcf(&mut self) -> TdcfW<FdtdcSpec> {
        TdcfW::new(self, 0)
    }
    #[doc = "Bits 8:14 - Transmitter delay compensation offset"]
    #[inline(always)]
    #[must_use]
    pub fn tdco(&mut self) -> TdcoW<FdtdcSpec> {
        TdcoW::new(self, 8)
    }
}
#[doc = "FD transmitter delay compensation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdtdc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdtdc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdtdcSpec;
impl crate::RegisterSpec for FdtdcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdtdc::R`](R) reader structure"]
impl crate::Readable for FdtdcSpec {}
#[doc = "`write(|w| ..)` method takes [`fdtdc::W`](W) writer structure"]
impl crate::Writable for FdtdcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDTDC to value 0"]
impl crate::Resettable for FdtdcSpec {
    const RESET_VALUE: u32 = 0;
}
