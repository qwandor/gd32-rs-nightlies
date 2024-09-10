#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Field `END` reader - End of operation flag bit"]
pub type EndR = crate::BitReader;
#[doc = "Field `END` writer - End of operation flag bit"]
pub type EndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPERR` reader - Flash operation error flag bit"]
pub type OperrR = crate::BitReader;
#[doc = "Field `OPERR` writer - Flash operation error flag bit"]
pub type OperrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPERR` reader - Erase/Program protection error flag bit"]
pub type WperrR = crate::BitReader;
#[doc = "Field `WPERR` writer - Erase/Program protection error flag bit"]
pub type WperrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGMERR` reader - Program size not match error flag bit"]
pub type PgmerrR = crate::BitReader;
#[doc = "Field `PGMERR` writer - Program size not match error flag bit"]
pub type PgmerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGSERR` reader - Program sequence error flag bit"]
pub type PgserrR = crate::BitReader;
#[doc = "Field `PGSERR` writer - Program sequence error flag bit"]
pub type PgserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDDERR` reader - Read D-bus protection error flag bit"]
pub type RdderrR = crate::BitReader;
#[doc = "Field `RDDERR` writer - Read D-bus protection error flag bit"]
pub type RdderrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - The flash is busy bit"]
pub type BusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - End of operation flag bit"]
    #[inline(always)]
    pub fn end(&self) -> EndR {
        EndR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash operation error flag bit"]
    #[inline(always)]
    pub fn operr(&self) -> OperrR {
        OperrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Erase/Program protection error flag bit"]
    #[inline(always)]
    pub fn wperr(&self) -> WperrR {
        WperrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Program size not match error flag bit"]
    #[inline(always)]
    pub fn pgmerr(&self) -> PgmerrR {
        PgmerrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Program sequence error flag bit"]
    #[inline(always)]
    pub fn pgserr(&self) -> PgserrR {
        PgserrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Read D-bus protection error flag bit"]
    #[inline(always)]
    pub fn rdderr(&self) -> RdderrR {
        RdderrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - The flash is busy bit"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of operation flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn end(&mut self) -> EndW<StatSpec> {
        EndW::new(self, 0)
    }
    #[doc = "Bit 1 - Flash operation error flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn operr(&mut self) -> OperrW<StatSpec> {
        OperrW::new(self, 1)
    }
    #[doc = "Bit 4 - Erase/Program protection error flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn wperr(&mut self) -> WperrW<StatSpec> {
        WperrW::new(self, 4)
    }
    #[doc = "Bit 6 - Program size not match error flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn pgmerr(&mut self) -> PgmerrW<StatSpec> {
        PgmerrW::new(self, 6)
    }
    #[doc = "Bit 7 - Program sequence error flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn pgserr(&mut self) -> PgserrW<StatSpec> {
        PgserrW::new(self, 7)
    }
    #[doc = "Bit 8 - Read D-bus protection error flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn rdderr(&mut self) -> RdderrW<StatSpec> {
        RdderrW::new(self, 8)
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
