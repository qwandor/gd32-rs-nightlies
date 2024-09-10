#[doc = "Register `CFG0` reader"]
pub type R = crate::R<Cfg0Spec>;
#[doc = "Register `CFG0` writer"]
pub type W = crate::W<Cfg0Spec>;
#[doc = "Field `BOOT_MODE` reader - Boot mode"]
pub type BootModeR = crate::FieldReader;
#[doc = "Field `BOOT_MODE` writer - Boot mode"]
pub type BootModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FMC_SWP` reader - FMC memory mapping swap"]
pub type FmcSwpR = crate::BitReader;
#[doc = "Field `FMC_SWP` writer - FMC memory mapping swap"]
pub type FmcSwpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXMC_SWP` reader - EXMC memory mapping swap"]
pub type ExmcSwpR = crate::FieldReader;
#[doc = "Field `EXMC_SWP` writer - EXMC memory mapping swap"]
pub type ExmcSwpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - Boot mode"]
    #[inline(always)]
    pub fn boot_mode(&self) -> BootModeR {
        BootModeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - FMC memory mapping swap"]
    #[inline(always)]
    pub fn fmc_swp(&self) -> FmcSwpR {
        FmcSwpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - EXMC memory mapping swap"]
    #[inline(always)]
    pub fn exmc_swp(&self) -> ExmcSwpR {
        ExmcSwpR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Boot mode"]
    #[inline(always)]
    #[must_use]
    pub fn boot_mode(&mut self) -> BootModeW<Cfg0Spec> {
        BootModeW::new(self, 0)
    }
    #[doc = "Bit 8 - FMC memory mapping swap"]
    #[inline(always)]
    #[must_use]
    pub fn fmc_swp(&mut self) -> FmcSwpW<Cfg0Spec> {
        FmcSwpW::new(self, 8)
    }
    #[doc = "Bits 10:11 - EXMC memory mapping swap"]
    #[inline(always)]
    #[must_use]
    pub fn exmc_swp(&mut self) -> ExmcSwpW<Cfg0Spec> {
        ExmcSwpW::new(self, 10)
    }
}
#[doc = "Configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Spec;
impl crate::RegisterSpec for Cfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0::R`](R) reader structure"]
impl crate::Readable for Cfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0::W`](W) writer structure"]
impl crate::Writable for Cfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0 to value 0"]
impl crate::Resettable for Cfg0Spec {
    const RESET_VALUE: u32 = 0;
}
