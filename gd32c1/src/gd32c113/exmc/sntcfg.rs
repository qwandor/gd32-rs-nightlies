#[doc = "Register `SNTCFG` reader"]
pub type R = crate::R<SntcfgSpec>;
#[doc = "Register `SNTCFG` writer"]
pub type W = crate::W<SntcfgSpec>;
#[doc = "Field `ASET` reader - address setup time"]
pub type AsetR = crate::FieldReader;
#[doc = "Field `ASET` writer - address setup time"]
pub type AsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AHLD` reader - address hold time"]
pub type AhldR = crate::FieldReader;
#[doc = "Field `AHLD` writer - address hold time"]
pub type AhldW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DSET` reader - data setup time"]
pub type DsetR = crate::FieldReader;
#[doc = "Field `DSET` writer - data setup time"]
pub type DsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BUSLAT` reader - bus latency"]
pub type BuslatR = crate::FieldReader;
#[doc = "Field `BUSLAT` writer - bus latency"]
pub type BuslatW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CKDIV` reader - synchronous clock divide ratio"]
pub type CkdivR = crate::FieldReader;
#[doc = "Field `CKDIV` writer - synchronous clock divide ratio"]
pub type CkdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DLAT` reader - data latency for NOR Flash"]
pub type DlatR = crate::FieldReader;
#[doc = "Field `DLAT` writer - data latency for NOR Flash"]
pub type DlatW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ASYNCMOD` reader - asynchronous access mode"]
pub type AsyncmodR = crate::FieldReader;
#[doc = "Field `ASYNCMOD` writer - asynchronous access mode"]
pub type AsyncmodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - address setup time"]
    #[inline(always)]
    pub fn aset(&self) -> AsetR {
        AsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - address hold time"]
    #[inline(always)]
    pub fn ahld(&self) -> AhldR {
        AhldR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - data setup time"]
    #[inline(always)]
    pub fn dset(&self) -> DsetR {
        DsetR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - bus latency"]
    #[inline(always)]
    pub fn buslat(&self) -> BuslatR {
        BuslatR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - synchronous clock divide ratio"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CkdivR {
        CkdivR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - data latency for NOR Flash"]
    #[inline(always)]
    pub fn dlat(&self) -> DlatR {
        DlatR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - asynchronous access mode"]
    #[inline(always)]
    pub fn asyncmod(&self) -> AsyncmodR {
        AsyncmodR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - address setup time"]
    #[inline(always)]
    #[must_use]
    pub fn aset(&mut self) -> AsetW<SntcfgSpec> {
        AsetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - address hold time"]
    #[inline(always)]
    #[must_use]
    pub fn ahld(&mut self) -> AhldW<SntcfgSpec> {
        AhldW::new(self, 4)
    }
    #[doc = "Bits 8:15 - data setup time"]
    #[inline(always)]
    #[must_use]
    pub fn dset(&mut self) -> DsetW<SntcfgSpec> {
        DsetW::new(self, 8)
    }
    #[doc = "Bits 16:19 - bus latency"]
    #[inline(always)]
    #[must_use]
    pub fn buslat(&mut self) -> BuslatW<SntcfgSpec> {
        BuslatW::new(self, 16)
    }
    #[doc = "Bits 20:23 - synchronous clock divide ratio"]
    #[inline(always)]
    #[must_use]
    pub fn ckdiv(&mut self) -> CkdivW<SntcfgSpec> {
        CkdivW::new(self, 20)
    }
    #[doc = "Bits 24:27 - data latency for NOR Flash"]
    #[inline(always)]
    #[must_use]
    pub fn dlat(&mut self) -> DlatW<SntcfgSpec> {
        DlatW::new(self, 24)
    }
    #[doc = "Bits 28:29 - asynchronous access mode"]
    #[inline(always)]
    #[must_use]
    pub fn asyncmod(&mut self) -> AsyncmodW<SntcfgSpec> {
        AsyncmodW::new(self, 28)
    }
}
#[doc = "SRAM/NOR Flash timing configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sntcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sntcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SntcfgSpec;
impl crate::RegisterSpec for SntcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sntcfg::R`](R) reader structure"]
impl crate::Readable for SntcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sntcfg::W`](W) writer structure"]
impl crate::Writable for SntcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SNTCFG to value 0x0fff_ffff"]
impl crate::Resettable for SntcfgSpec {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
