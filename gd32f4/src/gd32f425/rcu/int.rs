#[doc = "Register `INT` reader"]
pub type R = crate::R<IntSpec>;
#[doc = "Register `INT` writer"]
pub type W = crate::W<IntSpec>;
#[doc = "Field `IRC32KSTBIF` reader - IRC32K stabilization interrupt flag"]
pub type Irc32kstbifR = crate::BitReader;
#[doc = "Field `LXTALSTBIF` reader - LXTAL stabilization interrupt flag"]
pub type LxtalstbifR = crate::BitReader;
#[doc = "Field `IRC16MSTBIF` reader - IRC16M stabilization interrupt flag"]
pub type Irc16mstbifR = crate::BitReader;
#[doc = "Field `HXTALSTBIF` reader - HXTAL stabilization interrupt flag"]
pub type HxtalstbifR = crate::BitReader;
#[doc = "Field `PLLSTBIF` reader - PLL stabilization interrupt flag"]
pub type PllstbifR = crate::BitReader;
#[doc = "Field `PLLI2SSTBIF` reader - PLLI2S stabilization interrupt flag"]
pub type Plli2sstbifR = crate::BitReader;
#[doc = "Field `PLLSAISTBIF` reader - PLLSAI stabilization interrupt flag"]
pub type PllsaistbifR = crate::BitReader;
#[doc = "Field `CKMIF` reader - HXTAL Clock Stuck Interrupt Flag"]
pub type CkmifR = crate::BitReader;
#[doc = "Field `IRC32KSTBIE` reader - IRC32K Stabilization interrupt enable"]
pub type Irc32kstbieR = crate::BitReader;
#[doc = "Field `IRC32KSTBIE` writer - IRC32K Stabilization interrupt enable"]
pub type Irc32kstbieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LXTALSTBIE` reader - LXTAL Stabilization Interrupt Enable"]
pub type LxtalstbieR = crate::BitReader;
#[doc = "Field `LXTALSTBIE` writer - LXTAL Stabilization Interrupt Enable"]
pub type LxtalstbieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRC16MSTBIE` reader - IRC16M Stabilization Interrupt Enable"]
pub type Irc16mstbieR = crate::BitReader;
#[doc = "Field `IRC16MSTBIE` writer - IRC16M Stabilization Interrupt Enable"]
pub type Irc16mstbieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HXTALSTBIE` reader - HXTAL Stabilization Interrupt Enable"]
pub type HxtalstbieR = crate::BitReader;
#[doc = "Field `HXTALSTBIE` writer - HXTAL Stabilization Interrupt Enable"]
pub type HxtalstbieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSTBIE` reader - PLL Stabilization Interrupt Enable"]
pub type PllstbieR = crate::BitReader;
#[doc = "Field `PLLSTBIE` writer - PLL Stabilization Interrupt Enable"]
pub type PllstbieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLI2SSTBIE` reader - PLLI2S Stabilization Interrupt Enable"]
pub type Plli2sstbieR = crate::BitReader;
#[doc = "Field `PLLI2SSTBIE` writer - PLLI2S Stabilization Interrupt Enable"]
pub type Plli2sstbieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAISTBIE` reader - PLLSAI Stabilization Interrupt Enable"]
pub type PllsaistbieR = crate::BitReader;
#[doc = "Field `PLLSAISTBIE` writer - PLLSAI Stabilization Interrupt Enable"]
pub type PllsaistbieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRC32KSTBIC` writer - IRC32K Stabilization Interrupt Clear"]
pub type Irc32kstbicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LXTALSTBIC` writer - LXTAL Stabilization Interrupt Clear"]
pub type LxtalstbicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRC16MSTBIC` writer - IRC16M Stabilization Interrupt Clear"]
pub type Irc16mstbicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HXTALSTBIC` writer - HXTAL Stabilization Interrupt Clear"]
pub type HxtalstbicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSTBIC` writer - PLL stabilization Interrupt Clear"]
pub type PllstbicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLI2SSTBIC` writer - PLLI2S stabilization Interrupt Clear"]
pub type Plli2sstbicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAISTBIC` writer - PLLSAI stabilization Interrupt Clear"]
pub type PllsaistbicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKMIC` writer - HXTAL Clock Stuck Interrupt Clear"]
pub type CkmicW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IRC32K stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc32kstbif(&self) -> Irc32kstbifR {
        Irc32kstbifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LXTAL stabilization interrupt flag"]
    #[inline(always)]
    pub fn lxtalstbif(&self) -> LxtalstbifR {
        LxtalstbifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IRC16M stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc16mstbif(&self) -> Irc16mstbifR {
        Irc16mstbifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HXTAL stabilization interrupt flag"]
    #[inline(always)]
    pub fn hxtalstbif(&self) -> HxtalstbifR {
        HxtalstbifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL stabilization interrupt flag"]
    #[inline(always)]
    pub fn pllstbif(&self) -> PllstbifR {
        PllstbifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLLI2S stabilization interrupt flag"]
    #[inline(always)]
    pub fn plli2sstbif(&self) -> Plli2sstbifR {
        Plli2sstbifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLLSAI stabilization interrupt flag"]
    #[inline(always)]
    pub fn pllsaistbif(&self) -> PllsaistbifR {
        PllsaistbifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HXTAL Clock Stuck Interrupt Flag"]
    #[inline(always)]
    pub fn ckmif(&self) -> CkmifR {
        CkmifR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IRC32K Stabilization interrupt enable"]
    #[inline(always)]
    pub fn irc32kstbie(&self) -> Irc32kstbieR {
        Irc32kstbieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn lxtalstbie(&self) -> LxtalstbieR {
        LxtalstbieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IRC16M Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn irc16mstbie(&self) -> Irc16mstbieR {
        Irc16mstbieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn hxtalstbie(&self) -> HxtalstbieR {
        HxtalstbieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PLL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pllstbie(&self) -> PllstbieR {
        PllstbieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PLLI2S Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn plli2sstbie(&self) -> Plli2sstbieR {
        Plli2sstbieR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PLLSAI Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pllsaistbie(&self) -> PllsaistbieR {
        PllsaistbieR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - IRC32K Stabilization interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn irc32kstbie(&mut self) -> Irc32kstbieW<IntSpec> {
        Irc32kstbieW::new(self, 8)
    }
    #[doc = "Bit 9 - LXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lxtalstbie(&mut self) -> LxtalstbieW<IntSpec> {
        LxtalstbieW::new(self, 9)
    }
    #[doc = "Bit 10 - IRC16M Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn irc16mstbie(&mut self) -> Irc16mstbieW<IntSpec> {
        Irc16mstbieW::new(self, 10)
    }
    #[doc = "Bit 11 - HXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hxtalstbie(&mut self) -> HxtalstbieW<IntSpec> {
        HxtalstbieW::new(self, 11)
    }
    #[doc = "Bit 12 - PLL Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllstbie(&mut self) -> PllstbieW<IntSpec> {
        PllstbieW::new(self, 12)
    }
    #[doc = "Bit 13 - PLLI2S Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn plli2sstbie(&mut self) -> Plli2sstbieW<IntSpec> {
        Plli2sstbieW::new(self, 13)
    }
    #[doc = "Bit 14 - PLLSAI Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllsaistbie(&mut self) -> PllsaistbieW<IntSpec> {
        PllsaistbieW::new(self, 14)
    }
    #[doc = "Bit 16 - IRC32K Stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn irc32kstbic(&mut self) -> Irc32kstbicW<IntSpec> {
        Irc32kstbicW::new(self, 16)
    }
    #[doc = "Bit 17 - LXTAL Stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lxtalstbic(&mut self) -> LxtalstbicW<IntSpec> {
        LxtalstbicW::new(self, 17)
    }
    #[doc = "Bit 18 - IRC16M Stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn irc16mstbic(&mut self) -> Irc16mstbicW<IntSpec> {
        Irc16mstbicW::new(self, 18)
    }
    #[doc = "Bit 19 - HXTAL Stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hxtalstbic(&mut self) -> HxtalstbicW<IntSpec> {
        HxtalstbicW::new(self, 19)
    }
    #[doc = "Bit 20 - PLL stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pllstbic(&mut self) -> PllstbicW<IntSpec> {
        PllstbicW::new(self, 20)
    }
    #[doc = "Bit 21 - PLLI2S stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn plli2sstbic(&mut self) -> Plli2sstbicW<IntSpec> {
        Plli2sstbicW::new(self, 21)
    }
    #[doc = "Bit 22 - PLLSAI stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pllsaistbic(&mut self) -> PllsaistbicW<IntSpec> {
        PllsaistbicW::new(self, 22)
    }
    #[doc = "Bit 23 - HXTAL Clock Stuck Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ckmic(&mut self) -> CkmicW<IntSpec> {
        CkmicW::new(self, 23)
    }
}
#[doc = "Clock interrupt register (RCU_INT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntSpec;
impl crate::RegisterSpec for IntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int::R`](R) reader structure"]
impl crate::Readable for IntSpec {}
#[doc = "`write(|w| ..)` method takes [`int::W`](W) writer structure"]
impl crate::Writable for IntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT to value 0"]
impl crate::Resettable for IntSpec {
    const RESET_VALUE: u32 = 0;
}
