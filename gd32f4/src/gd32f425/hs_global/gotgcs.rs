#[doc = "Register `GOTGCS` reader"]
pub type R = crate::R<GotgcsSpec>;
#[doc = "Register `GOTGCS` writer"]
pub type W = crate::W<GotgcsSpec>;
#[doc = "Field `SRPS` reader - Session request success"]
pub type SrpsR = crate::BitReader;
#[doc = "Field `SRPREQ` reader - SRP request"]
pub type SrpreqR = crate::BitReader;
#[doc = "Field `SRPREQ` writer - SRP request"]
pub type SrpreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNPS` reader - Host negotiation success"]
pub type HnpsR = crate::BitReader;
#[doc = "Field `HNPREQ` reader - HNP request"]
pub type HnpreqR = crate::BitReader;
#[doc = "Field `HNPREQ` writer - HNP request"]
pub type HnpreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HHNPEN` reader - Host HNP enable"]
pub type HhnpenR = crate::BitReader;
#[doc = "Field `HHNPEN` writer - Host HNP enable"]
pub type HhnpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DHNPEN` reader - Device HNP enabled"]
pub type DhnpenR = crate::BitReader;
#[doc = "Field `DHNPEN` writer - Device HNP enabled"]
pub type DhnpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIDPS` reader - ID pin status"]
pub type CidpsR = crate::BitReader;
#[doc = "Field `DI` reader - Debounce interval of a detected connection"]
pub type DiR = crate::BitReader;
#[doc = "Field `ASV` reader - A-session valid"]
pub type AsvR = crate::BitReader;
#[doc = "Field `BSV` reader - B-session valid"]
pub type BsvR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Session request success"]
    #[inline(always)]
    pub fn srps(&self) -> SrpsR {
        SrpsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRP request"]
    #[inline(always)]
    pub fn srpreq(&self) -> SrpreqR {
        SrpreqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Host negotiation success"]
    #[inline(always)]
    pub fn hnps(&self) -> HnpsR {
        HnpsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    pub fn hnpreq(&self) -> HnpreqR {
        HnpreqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Host HNP enable"]
    #[inline(always)]
    pub fn hhnpen(&self) -> HhnpenR {
        HhnpenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    pub fn dhnpen(&self) -> DhnpenR {
        DhnpenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - ID pin status"]
    #[inline(always)]
    pub fn cidps(&self) -> CidpsR {
        CidpsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Debounce interval of a detected connection"]
    #[inline(always)]
    pub fn di(&self) -> DiR {
        DiR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A-session valid"]
    #[inline(always)]
    pub fn asv(&self) -> AsvR {
        AsvR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - B-session valid"]
    #[inline(always)]
    pub fn bsv(&self) -> BsvR {
        BsvR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SRP request"]
    #[inline(always)]
    #[must_use]
    pub fn srpreq(&mut self) -> SrpreqW<GotgcsSpec> {
        SrpreqW::new(self, 1)
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    #[must_use]
    pub fn hnpreq(&mut self) -> HnpreqW<GotgcsSpec> {
        HnpreqW::new(self, 9)
    }
    #[doc = "Bit 10 - Host HNP enable"]
    #[inline(always)]
    #[must_use]
    pub fn hhnpen(&mut self) -> HhnpenW<GotgcsSpec> {
        HhnpenW::new(self, 10)
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dhnpen(&mut self) -> DhnpenW<GotgcsSpec> {
        DhnpenW::new(self, 11)
    }
}
#[doc = "control and status register (GOTGCS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgcs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgcs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GotgcsSpec;
impl crate::RegisterSpec for GotgcsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gotgcs::R`](R) reader structure"]
impl crate::Readable for GotgcsSpec {}
#[doc = "`write(|w| ..)` method takes [`gotgcs::W`](W) writer structure"]
impl crate::Writable for GotgcsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GOTGCS to value 0x0800"]
impl crate::Resettable for GotgcsSpec {
    const RESET_VALUE: u32 = 0x0800;
}
