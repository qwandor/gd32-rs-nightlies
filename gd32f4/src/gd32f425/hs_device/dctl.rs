#[doc = "Register `DCTL` reader"]
pub type R = crate::R<DctlSpec>;
#[doc = "Register `DCTL` writer"]
pub type W = crate::W<DctlSpec>;
#[doc = "Field `RWKUP` reader - Remote wakeup signaling"]
pub type RwkupR = crate::BitReader;
#[doc = "Field `RWKUP` writer - Remote wakeup signaling"]
pub type RwkupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD` reader - Soft disconnect"]
pub type SdR = crate::BitReader;
#[doc = "Field `SD` writer - Soft disconnect"]
pub type SdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GINS` reader - Global IN NAK status"]
pub type GinsR = crate::BitReader;
#[doc = "Field `GONS` reader - Global OUT NAK status"]
pub type GonsR = crate::BitReader;
#[doc = "Field `SGINAK` reader - Set global IN NAK"]
pub type SginakR = crate::BitReader;
#[doc = "Field `SGINAK` writer - Set global IN NAK"]
pub type SginakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGINAK` reader - Clear global IN NAK"]
pub type CginakR = crate::BitReader;
#[doc = "Field `CGINAK` writer - Clear global IN NAK"]
pub type CginakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SGONAK` reader - Set global OUT NAK"]
pub type SgonakR = crate::BitReader;
#[doc = "Field `SGONAK` writer - Set global OUT NAK"]
pub type SgonakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGONAK` reader - Clear global OUT NAK"]
pub type CgonakR = crate::BitReader;
#[doc = "Field `CGONAK` writer - Clear global OUT NAK"]
pub type CgonakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POIF` reader - Power-on initialization finished"]
pub type PoifR = crate::BitReader;
#[doc = "Field `POIF` writer - Power-on initialization finished"]
pub type PoifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Remote wakeup signaling"]
    #[inline(always)]
    pub fn rwkup(&self) -> RwkupR {
        RwkupR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    pub fn sd(&self) -> SdR {
        SdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global IN NAK status"]
    #[inline(always)]
    pub fn gins(&self) -> GinsR {
        GinsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global OUT NAK status"]
    #[inline(always)]
    pub fn gons(&self) -> GonsR {
        GonsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Set global IN NAK"]
    #[inline(always)]
    pub fn sginak(&self) -> SginakR {
        SginakR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Clear global IN NAK"]
    #[inline(always)]
    pub fn cginak(&self) -> CginakR {
        CginakR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set global OUT NAK"]
    #[inline(always)]
    pub fn sgonak(&self) -> SgonakR {
        SgonakR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clear global OUT NAK"]
    #[inline(always)]
    pub fn cgonak(&self) -> CgonakR {
        CgonakR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Power-on initialization finished"]
    #[inline(always)]
    pub fn poif(&self) -> PoifR {
        PoifR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remote wakeup signaling"]
    #[inline(always)]
    #[must_use]
    pub fn rwkup(&mut self) -> RwkupW<DctlSpec> {
        RwkupW::new(self, 0)
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    #[must_use]
    pub fn sd(&mut self) -> SdW<DctlSpec> {
        SdW::new(self, 1)
    }
    #[doc = "Bit 7 - Set global IN NAK"]
    #[inline(always)]
    #[must_use]
    pub fn sginak(&mut self) -> SginakW<DctlSpec> {
        SginakW::new(self, 7)
    }
    #[doc = "Bit 8 - Clear global IN NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cginak(&mut self) -> CginakW<DctlSpec> {
        CginakW::new(self, 8)
    }
    #[doc = "Bit 9 - Set global OUT NAK"]
    #[inline(always)]
    #[must_use]
    pub fn sgonak(&mut self) -> SgonakW<DctlSpec> {
        SgonakW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear global OUT NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cgonak(&mut self) -> CgonakW<DctlSpec> {
        CgonakW::new(self, 10)
    }
    #[doc = "Bit 11 - Power-on initialization finished"]
    #[inline(always)]
    #[must_use]
    pub fn poif(&mut self) -> PoifW<DctlSpec> {
        PoifW::new(self, 11)
    }
}
#[doc = "device control register (DCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DctlSpec;
impl crate::RegisterSpec for DctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dctl::R`](R) reader structure"]
impl crate::Readable for DctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dctl::W`](W) writer structure"]
impl crate::Writable for DctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCTL to value 0"]
impl crate::Resettable for DctlSpec {
    const RESET_VALUE: u32 = 0;
}
