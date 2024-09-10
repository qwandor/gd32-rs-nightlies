#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `HDEN` reader - High drive enable"]
pub type HdenR = crate::BitReader;
#[doc = "Field `HDEN` writer - High drive enable"]
pub type HdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFIE` reader - Start of frame interrupt enable"]
pub type SofieR = crate::BitReader;
#[doc = "Field `SOFIE` writer - Start of frame interrupt enable"]
pub type SofieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDIE` reader - SLCD update done interrupt enable"]
pub type UpdieR = crate::BitReader;
#[doc = "Field `UPDIE` writer - SLCD update done interrupt enable"]
pub type UpdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PULSE` reader - Pulse on duration"]
pub type PulseR = crate::FieldReader;
#[doc = "Field `PULSE` writer - Pulse on duration"]
pub type PulseW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DTD` reader - Dead time duration"]
pub type DtdR = crate::FieldReader;
#[doc = "Field `DTD` writer - Dead time duration"]
pub type DtdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CONR` reader - Contrast ratio"]
pub type ConrR = crate::FieldReader;
#[doc = "Field `CONR` writer - Contrast ratio"]
pub type ConrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BLKDIV` reader - Blink frequency divider"]
pub type BlkdivR = crate::FieldReader;
#[doc = "Field `BLKDIV` writer - Blink frequency divider"]
pub type BlkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BLKMOD` reader - Blink mode"]
pub type BlkmodR = crate::FieldReader;
#[doc = "Field `BLKMOD` writer - Blink mode"]
pub type BlkmodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DIV` reader - SLCD clock divider"]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - SLCD clock divider"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PSC` reader - SLCD clock prescaler"]
pub type PscR = crate::FieldReader;
#[doc = "Field `PSC` writer - SLCD clock prescaler"]
pub type PscW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - High drive enable"]
    #[inline(always)]
    pub fn hden(&self) -> HdenR {
        HdenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start of frame interrupt enable"]
    #[inline(always)]
    pub fn sofie(&self) -> SofieR {
        SofieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SLCD update done interrupt enable"]
    #[inline(always)]
    pub fn updie(&self) -> UpdieR {
        UpdieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Pulse on duration"]
    #[inline(always)]
    pub fn pulse(&self) -> PulseR {
        PulseR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - Dead time duration"]
    #[inline(always)]
    pub fn dtd(&self) -> DtdR {
        DtdR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:12 - Contrast ratio"]
    #[inline(always)]
    pub fn conr(&self) -> ConrR {
        ConrR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Blink frequency divider"]
    #[inline(always)]
    pub fn blkdiv(&self) -> BlkdivR {
        BlkdivR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Blink mode"]
    #[inline(always)]
    pub fn blkmod(&self) -> BlkmodR {
        BlkmodR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:21 - SLCD clock divider"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25 - SLCD clock prescaler"]
    #[inline(always)]
    pub fn psc(&self) -> PscR {
        PscR::new(((self.bits >> 22) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - High drive enable"]
    #[inline(always)]
    #[must_use]
    pub fn hden(&mut self) -> HdenW<CfgSpec> {
        HdenW::new(self, 0)
    }
    #[doc = "Bit 1 - Start of frame interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofie(&mut self) -> SofieW<CfgSpec> {
        SofieW::new(self, 1)
    }
    #[doc = "Bit 3 - SLCD update done interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn updie(&mut self) -> UpdieW<CfgSpec> {
        UpdieW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Pulse on duration"]
    #[inline(always)]
    #[must_use]
    pub fn pulse(&mut self) -> PulseW<CfgSpec> {
        PulseW::new(self, 4)
    }
    #[doc = "Bits 7:9 - Dead time duration"]
    #[inline(always)]
    #[must_use]
    pub fn dtd(&mut self) -> DtdW<CfgSpec> {
        DtdW::new(self, 7)
    }
    #[doc = "Bits 10:12 - Contrast ratio"]
    #[inline(always)]
    #[must_use]
    pub fn conr(&mut self) -> ConrW<CfgSpec> {
        ConrW::new(self, 10)
    }
    #[doc = "Bits 13:15 - Blink frequency divider"]
    #[inline(always)]
    #[must_use]
    pub fn blkdiv(&mut self) -> BlkdivW<CfgSpec> {
        BlkdivW::new(self, 13)
    }
    #[doc = "Bits 16:17 - Blink mode"]
    #[inline(always)]
    #[must_use]
    pub fn blkmod(&mut self) -> BlkmodW<CfgSpec> {
        BlkmodW::new(self, 16)
    }
    #[doc = "Bits 18:21 - SLCD clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<CfgSpec> {
        DivW::new(self, 18)
    }
    #[doc = "Bits 22:25 - SLCD clock prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PscW<CfgSpec> {
        PscW::new(self, 22)
    }
}
#[doc = "SLCD configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
