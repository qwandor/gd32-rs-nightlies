#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `SFT` reader - Signal Free Time"]
pub type SftR = crate::FieldReader;
#[doc = "Field `SFT` writer - Signal Free Time"]
pub type SftW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RTOL` reader - Reception bit timing tolerance"]
pub type RtolR = crate::BitReader;
#[doc = "Field `RTOL` writer - Reception bit timing tolerance"]
pub type RtolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBRESTP` reader - Whether stop receive message when detected RBRE"]
pub type RbrestpR = crate::BitReader;
#[doc = "Field `RBRESTP` writer - Whether stop receive message when detected RBRE"]
pub type RbrestpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBREGEN` reader - Generate Error-bit when detected RBRE in singlecast"]
pub type RbregenR = crate::BitReader;
#[doc = "Field `RBREGEN` writer - Generate Error-bit when detected RBRE in singlecast"]
pub type RbregenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RLBPEGEN` reader - Generate Error-bit when detected RLBPE in singlecast"]
pub type RlbpegenR = crate::BitReader;
#[doc = "Field `RLBPEGEN` writer - Generate Error-bit when detected RLBPE in singlecast"]
pub type RlbpegenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCNG` reader - Do not generate Error-bit in broadcast message"]
pub type BcngR = crate::BitReader;
#[doc = "Field `BCNG` writer - Do not generate Error-bit in broadcast message"]
pub type BcngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFTOPT` reader - The SFT start option"]
pub type SftoptR = crate::BitReader;
#[doc = "Field `SFTOPT` writer - The SFT start option"]
pub type SftoptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OADR` reader - Own Address"]
pub type OadrR = crate::FieldReader<u16>;
#[doc = "Field `OADR` writer - Own Address"]
pub type OadrW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `LMEN` reader - Listen mode enable"]
pub type LmenR = crate::BitReader;
#[doc = "Field `LMEN` writer - Listen mode enable"]
pub type LmenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Signal Free Time"]
    #[inline(always)]
    pub fn sft(&self) -> SftR {
        SftR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Reception bit timing tolerance"]
    #[inline(always)]
    pub fn rtol(&self) -> RtolR {
        RtolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Whether stop receive message when detected RBRE"]
    #[inline(always)]
    pub fn rbrestp(&self) -> RbrestpR {
        RbrestpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Generate Error-bit when detected RBRE in singlecast"]
    #[inline(always)]
    pub fn rbregen(&self) -> RbregenR {
        RbregenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Generate Error-bit when detected RLBPE in singlecast"]
    #[inline(always)]
    pub fn rlbpegen(&self) -> RlbpegenR {
        RlbpegenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Do not generate Error-bit in broadcast message"]
    #[inline(always)]
    pub fn bcng(&self) -> BcngR {
        BcngR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The SFT start option"]
    #[inline(always)]
    pub fn sftopt(&self) -> SftoptR {
        SftoptR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:30 - Own Address"]
    #[inline(always)]
    pub fn oadr(&self) -> OadrR {
        OadrR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Listen mode enable"]
    #[inline(always)]
    pub fn lmen(&self) -> LmenR {
        LmenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Signal Free Time"]
    #[inline(always)]
    #[must_use]
    pub fn sft(&mut self) -> SftW<CfgSpec> {
        SftW::new(self, 0)
    }
    #[doc = "Bit 3 - Reception bit timing tolerance"]
    #[inline(always)]
    #[must_use]
    pub fn rtol(&mut self) -> RtolW<CfgSpec> {
        RtolW::new(self, 3)
    }
    #[doc = "Bit 4 - Whether stop receive message when detected RBRE"]
    #[inline(always)]
    #[must_use]
    pub fn rbrestp(&mut self) -> RbrestpW<CfgSpec> {
        RbrestpW::new(self, 4)
    }
    #[doc = "Bit 5 - Generate Error-bit when detected RBRE in singlecast"]
    #[inline(always)]
    #[must_use]
    pub fn rbregen(&mut self) -> RbregenW<CfgSpec> {
        RbregenW::new(self, 5)
    }
    #[doc = "Bit 6 - Generate Error-bit when detected RLBPE in singlecast"]
    #[inline(always)]
    #[must_use]
    pub fn rlbpegen(&mut self) -> RlbpegenW<CfgSpec> {
        RlbpegenW::new(self, 6)
    }
    #[doc = "Bit 7 - Do not generate Error-bit in broadcast message"]
    #[inline(always)]
    #[must_use]
    pub fn bcng(&mut self) -> BcngW<CfgSpec> {
        BcngW::new(self, 7)
    }
    #[doc = "Bit 8 - The SFT start option"]
    #[inline(always)]
    #[must_use]
    pub fn sftopt(&mut self) -> SftoptW<CfgSpec> {
        SftoptW::new(self, 8)
    }
    #[doc = "Bits 16:30 - Own Address"]
    #[inline(always)]
    #[must_use]
    pub fn oadr(&mut self) -> OadrW<CfgSpec> {
        OadrW::new(self, 16)
    }
    #[doc = "Bit 31 - Listen mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn lmen(&mut self) -> LmenW<CfgSpec> {
        LmenW::new(self, 31)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0;
}
