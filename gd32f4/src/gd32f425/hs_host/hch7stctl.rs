#[doc = "Register `HCH7STCTL` reader"]
pub type R = crate::R<Hch7stctlSpec>;
#[doc = "Register `HCH7STCTL` writer"]
pub type W = crate::W<Hch7stctlSpec>;
#[doc = "Field `PADDR` reader - Port address"]
pub type PaddrR = crate::FieldReader;
#[doc = "Field `PADDR` writer - Port address"]
pub type PaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HADDR` reader - HUB address"]
pub type HaddrR = crate::FieldReader;
#[doc = "Field `HADDR` writer - HUB address"]
pub type HaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ISOPCE` reader - Isochronous OUT payload continuation encoding"]
pub type IsopceR = crate::FieldReader;
#[doc = "Field `ISOPCE` writer - Isochronous OUT payload continuation encoding"]
pub type IsopceW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSPLT` reader - Complete split enable"]
pub type CspltR = crate::BitReader;
#[doc = "Field `CSPLT` writer - Complete split enable"]
pub type CspltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLEN` reader - Enable high speed split transaction"]
pub type SplenR = crate::BitReader;
#[doc = "Field `SPLEN` writer - Enable high speed split transaction"]
pub type SplenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Port address"]
    #[inline(always)]
    pub fn paddr(&self) -> PaddrR {
        PaddrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - HUB address"]
    #[inline(always)]
    pub fn haddr(&self) -> HaddrR {
        HaddrR::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:15 - Isochronous OUT payload continuation encoding"]
    #[inline(always)]
    pub fn isopce(&self) -> IsopceR {
        IsopceR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Complete split enable"]
    #[inline(always)]
    pub fn csplt(&self) -> CspltR {
        CspltR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable high speed split transaction"]
    #[inline(always)]
    pub fn splen(&self) -> SplenR {
        SplenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Port address"]
    #[inline(always)]
    #[must_use]
    pub fn paddr(&mut self) -> PaddrW<Hch7stctlSpec> {
        PaddrW::new(self, 0)
    }
    #[doc = "Bits 7:13 - HUB address"]
    #[inline(always)]
    #[must_use]
    pub fn haddr(&mut self) -> HaddrW<Hch7stctlSpec> {
        HaddrW::new(self, 7)
    }
    #[doc = "Bits 14:15 - Isochronous OUT payload continuation encoding"]
    #[inline(always)]
    #[must_use]
    pub fn isopce(&mut self) -> IsopceW<Hch7stctlSpec> {
        IsopceW::new(self, 14)
    }
    #[doc = "Bit 16 - Complete split enable"]
    #[inline(always)]
    #[must_use]
    pub fn csplt(&mut self) -> CspltW<Hch7stctlSpec> {
        CspltW::new(self, 16)
    }
    #[doc = "Bit 31 - Enable high speed split transaction"]
    #[inline(always)]
    #[must_use]
    pub fn splen(&mut self) -> SplenW<Hch7stctlSpec> {
        SplenW::new(self, 31)
    }
}
#[doc = "host channel-7 split transaction control register (HCH7STCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch7stctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch7stctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hch7stctlSpec;
impl crate::RegisterSpec for Hch7stctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hch7stctl::R`](R) reader structure"]
impl crate::Readable for Hch7stctlSpec {}
#[doc = "`write(|w| ..)` method takes [`hch7stctl::W`](W) writer structure"]
impl crate::Writable for Hch7stctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCH7STCTL to value 0"]
impl crate::Resettable for Hch7stctlSpec {
    const RESET_VALUE: u32 = 0;
}
