#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Field `WDE` reader - Analog watchdog event flag"]
pub type WdeR = crate::BitReader;
#[doc = "Field `WDE` writer - Analog watchdog event flag"]
pub type WdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC` reader - End of group conversion flag"]
pub type EocR = crate::BitReader;
#[doc = "Field `EOC` writer - End of group conversion flag"]
pub type EocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOIC` reader - End of inserted group conversion flag"]
pub type EoicR = crate::BitReader;
#[doc = "Field `EOIC` writer - End of inserted group conversion flag"]
pub type EoicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STIC` reader - Start flag of inserted channel group"]
pub type SticR = crate::BitReader;
#[doc = "Field `STIC` writer - Start flag of inserted channel group"]
pub type SticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRC` reader - Start flag of regular channel group"]
pub type StrcR = crate::BitReader;
#[doc = "Field `STRC` writer - Start flag of regular channel group"]
pub type StrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROVF` reader - Regular data register overflow"]
pub type RovfR = crate::BitReader;
#[doc = "Field `ROVF` writer - Regular data register overflow"]
pub type RovfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Analog watchdog event flag"]
    #[inline(always)]
    pub fn wde(&self) -> WdeR {
        WdeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of group conversion flag"]
    #[inline(always)]
    pub fn eoc(&self) -> EocR {
        EocR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of inserted group conversion flag"]
    #[inline(always)]
    pub fn eoic(&self) -> EoicR {
        EoicR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start flag of inserted channel group"]
    #[inline(always)]
    pub fn stic(&self) -> SticR {
        SticR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Start flag of regular channel group"]
    #[inline(always)]
    pub fn strc(&self) -> StrcR {
        StrcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Regular data register overflow"]
    #[inline(always)]
    pub fn rovf(&self) -> RovfR {
        RovfR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog watchdog event flag"]
    #[inline(always)]
    #[must_use]
    pub fn wde(&mut self) -> WdeW<StatSpec> {
        WdeW::new(self, 0)
    }
    #[doc = "Bit 1 - End of group conversion flag"]
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EocW<StatSpec> {
        EocW::new(self, 1)
    }
    #[doc = "Bit 2 - End of inserted group conversion flag"]
    #[inline(always)]
    #[must_use]
    pub fn eoic(&mut self) -> EoicW<StatSpec> {
        EoicW::new(self, 2)
    }
    #[doc = "Bit 3 - Start flag of inserted channel group"]
    #[inline(always)]
    #[must_use]
    pub fn stic(&mut self) -> SticW<StatSpec> {
        SticW::new(self, 3)
    }
    #[doc = "Bit 4 - Start flag of regular channel group"]
    #[inline(always)]
    #[must_use]
    pub fn strc(&mut self) -> StrcW<StatSpec> {
        StrcW::new(self, 4)
    }
    #[doc = "Bit 5 - Regular data register overflow"]
    #[inline(always)]
    #[must_use]
    pub fn rovf(&mut self) -> RovfW<StatSpec> {
        RovfW::new(self, 5)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
