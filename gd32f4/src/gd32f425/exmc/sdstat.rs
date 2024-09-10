#[doc = "Register `SDSTAT` reader"]
pub type R = crate::R<SdstatSpec>;
#[doc = "Register `SDSTAT` writer"]
pub type W = crate::W<SdstatSpec>;
#[doc = "Field `REIF` reader - Refresh error interrupt flag"]
pub type ReifR = crate::BitReader;
#[doc = "Field `REIF` writer - Refresh error interrupt flag"]
pub type ReifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STA0` reader - Device 0 status"]
pub type Sta0R = crate::FieldReader;
#[doc = "Field `STA0` writer - Device 0 status"]
pub type Sta0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STA1` reader - Device1 status"]
pub type Sta1R = crate::FieldReader;
#[doc = "Field `STA1` writer - Device1 status"]
pub type Sta1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NRDY` reader - Not Ready status"]
pub type NrdyR = crate::BitReader;
#[doc = "Field `NRDY` writer - Not Ready status"]
pub type NrdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Refresh error interrupt flag"]
    #[inline(always)]
    pub fn reif(&self) -> ReifR {
        ReifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Device 0 status"]
    #[inline(always)]
    pub fn sta0(&self) -> Sta0R {
        Sta0R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Device1 status"]
    #[inline(always)]
    pub fn sta1(&self) -> Sta1R {
        Sta1R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Not Ready status"]
    #[inline(always)]
    pub fn nrdy(&self) -> NrdyR {
        NrdyR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Refresh error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn reif(&mut self) -> ReifW<SdstatSpec> {
        ReifW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Device 0 status"]
    #[inline(always)]
    #[must_use]
    pub fn sta0(&mut self) -> Sta0W<SdstatSpec> {
        Sta0W::new(self, 1)
    }
    #[doc = "Bits 3:4 - Device1 status"]
    #[inline(always)]
    #[must_use]
    pub fn sta1(&mut self) -> Sta1W<SdstatSpec> {
        Sta1W::new(self, 3)
    }
    #[doc = "Bit 5 - Not Ready status"]
    #[inline(always)]
    #[must_use]
    pub fn nrdy(&mut self) -> NrdyW<SdstatSpec> {
        NrdyW::new(self, 5)
    }
}
#[doc = "SDRAM status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdstatSpec;
impl crate::RegisterSpec for SdstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdstat::R`](R) reader structure"]
impl crate::Readable for SdstatSpec {}
#[doc = "`write(|w| ..)` method takes [`sdstat::W`](W) writer structure"]
impl crate::Writable for SdstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDSTAT to value 0"]
impl crate::Resettable for SdstatSpec {
    const RESET_VALUE: u32 = 0;
}
