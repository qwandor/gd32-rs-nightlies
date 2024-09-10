#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Field `DRDY` reader - Random data ready status bit"]
pub type DrdyR = crate::BitReader;
#[doc = "Field `CECS` reader - Clock error current status"]
pub type CecsR = crate::BitReader;
#[doc = "Field `SECS` reader - Seed error current status"]
pub type SecsR = crate::BitReader;
#[doc = "Field `CEIF` reader - Clock error interrupt flag"]
pub type CeifR = crate::BitReader;
#[doc = "Field `CEIF` writer - Clock error interrupt flag"]
pub type CeifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEIF` reader - Seed error interrupt flag"]
pub type SeifR = crate::BitReader;
#[doc = "Field `SEIF` writer - Seed error interrupt flag"]
pub type SeifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Random data ready status bit"]
    #[inline(always)]
    pub fn drdy(&self) -> DrdyR {
        DrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock error current status"]
    #[inline(always)]
    pub fn cecs(&self) -> CecsR {
        CecsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Seed error current status"]
    #[inline(always)]
    pub fn secs(&self) -> SecsR {
        SecsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock error interrupt flag"]
    #[inline(always)]
    pub fn ceif(&self) -> CeifR {
        CeifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Seed error interrupt flag"]
    #[inline(always)]
    pub fn seif(&self) -> SeifR {
        SeifR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Clock error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ceif(&mut self) -> CeifW<StatSpec> {
        CeifW::new(self, 5)
    }
    #[doc = "Bit 6 - Seed error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn seif(&mut self) -> SeifW<StatSpec> {
        SeifW::new(self, 6)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0;
}
