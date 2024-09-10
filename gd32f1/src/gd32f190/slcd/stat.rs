#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Field `ONF` reader - SLCD controller on flag"]
pub type OnfR = crate::BitReader;
#[doc = "Field `SOF` reader - Start of frame flag"]
pub type SofR = crate::BitReader;
#[doc = "Field `UPRF` reader - Update SLCD data request flag"]
pub type UprfR = crate::BitReader;
#[doc = "Field `UPRF` writer - Update SLCD data request flag"]
pub type UprfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDF` reader - Update SLCD data done flag"]
pub type UpdfR = crate::BitReader;
#[doc = "Field `VRDYF` reader - SLCD voltage ready flag"]
pub type VrdyfR = crate::BitReader;
#[doc = "Field `SYNF` reader - SLCD_CFG register synchronization flag"]
pub type SynfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SLCD controller on flag"]
    #[inline(always)]
    pub fn onf(&self) -> OnfR {
        OnfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start of frame flag"]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Update SLCD data request flag"]
    #[inline(always)]
    pub fn uprf(&self) -> UprfR {
        UprfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Update SLCD data done flag"]
    #[inline(always)]
    pub fn updf(&self) -> UpdfR {
        UpdfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SLCD voltage ready flag"]
    #[inline(always)]
    pub fn vrdyf(&self) -> VrdyfR {
        VrdyfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SLCD_CFG register synchronization flag"]
    #[inline(always)]
    pub fn synf(&self) -> SynfR {
        SynfR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Update SLCD data request flag"]
    #[inline(always)]
    #[must_use]
    pub fn uprf(&mut self) -> UprfW<StatSpec> {
        UprfW::new(self, 2)
    }
}
#[doc = "SLCD status flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets STAT to value 0x20"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0x20;
}
