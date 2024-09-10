#[doc = "Register `PLLTCFG` reader"]
pub type R = crate::R<PlltcfgSpec>;
#[doc = "Register `PLLTCFG` writer"]
pub type W = crate::W<PlltcfgSpec>;
#[doc = "Field `PLLTPSC` reader - PLLT prescaler selection"]
pub type PlltpscR = crate::FieldReader;
#[doc = "Field `PLLTPSC` writer - PLLT prescaler selection"]
pub type PlltpscW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PLLTMF` reader - PLLT multiply factor for VCO"]
pub type PlltmfR = crate::FieldReader<u16>;
#[doc = "Field `PLLTMF` writer - PLLT multiply factor for VCO"]
pub type PlltmfW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `TLIPSC` reader - TLI prescaler selection"]
pub type TlipscR = crate::FieldReader;
#[doc = "Field `TLIPSC` writer - TLI prescaler selection"]
pub type TlipscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLLTRPSC` reader - PLLTR prescaler selection"]
pub type PlltrpscR = crate::FieldReader;
#[doc = "Field `PLLTRPSC` writer - PLLTR prescaler selection"]
pub type PlltrpscW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLLTSEL` reader - PLLT clock source select"]
pub type PlltselR = crate::BitReader;
#[doc = "Field `PLLTSEL` writer - PLLT clock source select"]
pub type PlltselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - PLLT prescaler selection"]
    #[inline(always)]
    pub fn plltpsc(&self) -> PlltpscR {
        PlltpscR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:14 - PLLT multiply factor for VCO"]
    #[inline(always)]
    pub fn plltmf(&self) -> PlltmfR {
        PlltmfR::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:17 - TLI prescaler selection"]
    #[inline(always)]
    pub fn tlipsc(&self) -> TlipscR {
        TlipscR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 28:30 - PLLTR prescaler selection"]
    #[inline(always)]
    pub fn plltrpsc(&self) -> PlltrpscR {
        PlltrpscR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - PLLT clock source select"]
    #[inline(always)]
    pub fn plltsel(&self) -> PlltselR {
        PlltselR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - PLLT prescaler selection"]
    #[inline(always)]
    #[must_use]
    pub fn plltpsc(&mut self) -> PlltpscW<PlltcfgSpec> {
        PlltpscW::new(self, 0)
    }
    #[doc = "Bits 6:14 - PLLT multiply factor for VCO"]
    #[inline(always)]
    #[must_use]
    pub fn plltmf(&mut self) -> PlltmfW<PlltcfgSpec> {
        PlltmfW::new(self, 6)
    }
    #[doc = "Bits 16:17 - TLI prescaler selection"]
    #[inline(always)]
    #[must_use]
    pub fn tlipsc(&mut self) -> TlipscW<PlltcfgSpec> {
        TlipscW::new(self, 16)
    }
    #[doc = "Bits 28:30 - PLLTR prescaler selection"]
    #[inline(always)]
    #[must_use]
    pub fn plltrpsc(&mut self) -> PlltrpscW<PlltcfgSpec> {
        PlltrpscW::new(self, 28)
    }
    #[doc = "Bit 31 - PLLT clock source select"]
    #[inline(always)]
    #[must_use]
    pub fn plltsel(&mut self) -> PlltselW<PlltcfgSpec> {
        PlltselW::new(self, 31)
    }
}
#[doc = "PLLT configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plltcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plltcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlltcfgSpec;
impl crate::RegisterSpec for PlltcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plltcfg::R`](R) reader structure"]
impl crate::Readable for PlltcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`plltcfg::W`](W) writer structure"]
impl crate::Writable for PlltcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLTCFG to value 0x2000_3010"]
impl crate::Resettable for PlltcfgSpec {
    const RESET_VALUE: u32 = 0x2000_3010;
}
