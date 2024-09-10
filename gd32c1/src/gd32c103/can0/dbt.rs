#[doc = "Register `DBT` reader"]
pub type R = crate::R<DbtSpec>;
#[doc = "Register `DBT` writer"]
pub type W = crate::W<DbtSpec>;
#[doc = "Field `DBAUDPSC` reader - Baud rate prescaler"]
pub type DbaudpscR = crate::FieldReader<u16>;
#[doc = "Field `DBAUDPSC` writer - Baud rate prescaler"]
pub type DbaudpscW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DBS1` reader - Bit segment 1"]
pub type Dbs1R = crate::FieldReader;
#[doc = "Field `DBS1` writer - Bit segment 1"]
pub type Dbs1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DBS2` reader - Bit segment 2"]
pub type Dbs2R = crate::FieldReader;
#[doc = "Field `DBS2` writer - Bit segment 2"]
pub type Dbs2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DSJW` reader - Resynchronization jump width"]
pub type DsjwR = crate::FieldReader;
#[doc = "Field `DSJW` writer - Resynchronization jump width"]
pub type DsjwW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:9 - Baud rate prescaler"]
    #[inline(always)]
    pub fn dbaudpsc(&self) -> DbaudpscR {
        DbaudpscR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:19 - Bit segment 1"]
    #[inline(always)]
    pub fn dbs1(&self) -> Dbs1R {
        Dbs1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Bit segment 2"]
    #[inline(always)]
    pub fn dbs2(&self) -> Dbs2R {
        Dbs2R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Resynchronization jump width"]
    #[inline(always)]
    pub fn dsjw(&self) -> DsjwR {
        DsjwR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Baud rate prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn dbaudpsc(&mut self) -> DbaudpscW<DbtSpec> {
        DbaudpscW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Bit segment 1"]
    #[inline(always)]
    #[must_use]
    pub fn dbs1(&mut self) -> Dbs1W<DbtSpec> {
        Dbs1W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Bit segment 2"]
    #[inline(always)]
    #[must_use]
    pub fn dbs2(&mut self) -> Dbs2W<DbtSpec> {
        Dbs2W::new(self, 20)
    }
    #[doc = "Bits 24:26 - Resynchronization jump width"]
    #[inline(always)]
    #[must_use]
    pub fn dsjw(&mut self) -> DsjwW<DbtSpec> {
        DsjwW::new(self, 24)
    }
}
#[doc = "Date Bit timing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbtSpec;
impl crate::RegisterSpec for DbtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbt::R`](R) reader structure"]
impl crate::Readable for DbtSpec {}
#[doc = "`write(|w| ..)` method takes [`dbt::W`](W) writer structure"]
impl crate::Writable for DbtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBT to value 0x0123_0000"]
impl crate::Resettable for DbtSpec {
    const RESET_VALUE: u32 = 0x0123_0000;
}
