#[doc = "Register `FDCTL` reader"]
pub type R = crate::R<FdctlSpec>;
#[doc = "Register `FDCTL` writer"]
pub type W = crate::W<FdctlSpec>;
#[doc = "Field `FDEN` reader - FD operation enable"]
pub type FdenR = crate::BitReader;
#[doc = "Field `FDEN` writer - FD operation enable"]
pub type FdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRED` reader - Protocol exception event detection disable"]
pub type PredR = crate::BitReader;
#[doc = "Field `PRED` writer - Protocol exception event detection disable"]
pub type PredW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NISO` reader - ISO/Bosch"]
pub type NisoR = crate::BitReader;
#[doc = "Field `NISO` writer - ISO/Bosch"]
pub type NisoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDCEN` reader - Transmitter delay compensation enable"]
pub type TdcenR = crate::BitReader;
#[doc = "Field `TDCEN` writer - Transmitter delay compensation enable"]
pub type TdcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDCMOD` reader - Transmitter delay compensation mode"]
pub type TdcmodR = crate::BitReader;
#[doc = "Field `TDCMOD` writer - Transmitter delay compensation mode"]
pub type TdcmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESIMOD` reader - Error state indicator mode"]
pub type EsimodR = crate::BitReader;
#[doc = "Field `ESIMOD` writer - Error state indicator mode"]
pub type EsimodW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FD operation enable"]
    #[inline(always)]
    pub fn fden(&self) -> FdenR {
        FdenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Protocol exception event detection disable"]
    #[inline(always)]
    pub fn pred(&self) -> PredR {
        PredR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ISO/Bosch"]
    #[inline(always)]
    pub fn niso(&self) -> NisoR {
        NisoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmitter delay compensation enable"]
    #[inline(always)]
    pub fn tdcen(&self) -> TdcenR {
        TdcenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmitter delay compensation mode"]
    #[inline(always)]
    pub fn tdcmod(&self) -> TdcmodR {
        TdcmodR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Error state indicator mode"]
    #[inline(always)]
    pub fn esimod(&self) -> EsimodR {
        EsimodR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FD operation enable"]
    #[inline(always)]
    #[must_use]
    pub fn fden(&mut self) -> FdenW<FdctlSpec> {
        FdenW::new(self, 0)
    }
    #[doc = "Bit 2 - Protocol exception event detection disable"]
    #[inline(always)]
    #[must_use]
    pub fn pred(&mut self) -> PredW<FdctlSpec> {
        PredW::new(self, 2)
    }
    #[doc = "Bit 3 - ISO/Bosch"]
    #[inline(always)]
    #[must_use]
    pub fn niso(&mut self) -> NisoW<FdctlSpec> {
        NisoW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmitter delay compensation enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdcen(&mut self) -> TdcenW<FdctlSpec> {
        TdcenW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmitter delay compensation mode"]
    #[inline(always)]
    #[must_use]
    pub fn tdcmod(&mut self) -> TdcmodW<FdctlSpec> {
        TdcmodW::new(self, 5)
    }
    #[doc = "Bit 6 - Error state indicator mode"]
    #[inline(always)]
    #[must_use]
    pub fn esimod(&mut self) -> EsimodW<FdctlSpec> {
        EsimodW::new(self, 6)
    }
}
#[doc = "FD control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdctlSpec;
impl crate::RegisterSpec for FdctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdctl::R`](R) reader structure"]
impl crate::Readable for FdctlSpec {}
#[doc = "`write(|w| ..)` method takes [`fdctl::W`](W) writer structure"]
impl crate::Writable for FdctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCTL to value 0"]
impl crate::Resettable for FdctlSpec {
    const RESET_VALUE: u32 = 0;
}
