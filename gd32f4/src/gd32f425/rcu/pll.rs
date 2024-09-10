#[doc = "Register `PLL` reader"]
pub type R = crate::R<PllSpec>;
#[doc = "Register `PLL` writer"]
pub type W = crate::W<PllSpec>;
#[doc = "Field `PLLPSC` reader - The PLL VCO source clock prescaler"]
pub type PllpscR = crate::FieldReader;
#[doc = "Field `PLLPSC` writer - The PLL VCO source clock prescaler"]
pub type PllpscW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PLLN` reader - The PLL VCO clock multi factor"]
pub type PllnR = crate::FieldReader<u16>;
#[doc = "Field `PLLN` writer - The PLL VCO clock multi factor"]
pub type PllnW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PLLP` reader - The PLLP output frequency division factor from PLL VCO clock"]
pub type PllpR = crate::FieldReader;
#[doc = "Field `PLLP` writer - The PLLP output frequency division factor from PLL VCO clock"]
pub type PllpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLLSEL` reader - PLL Clock Source Selection"]
pub type PllselR = crate::BitReader;
#[doc = "Field `PLLSEL` writer - PLL Clock Source Selection"]
pub type PllselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLQ` reader - The PLL Q output frequency division factor from PLL VCO clock"]
pub type PllqR = crate::FieldReader;
#[doc = "Field `PLLQ` writer - The PLL Q output frequency division factor from PLL VCO clock"]
pub type PllqW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:5 - The PLL VCO source clock prescaler"]
    #[inline(always)]
    pub fn pllpsc(&self) -> PllpscR {
        PllpscR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:14 - The PLL VCO clock multi factor"]
    #[inline(always)]
    pub fn plln(&self) -> PllnR {
        PllnR::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:17 - The PLLP output frequency division factor from PLL VCO clock"]
    #[inline(always)]
    pub fn pllp(&self) -> PllpR {
        PllpR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 22 - PLL Clock Source Selection"]
    #[inline(always)]
    pub fn pllsel(&self) -> PllselR {
        PllselR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:27 - The PLL Q output frequency division factor from PLL VCO clock"]
    #[inline(always)]
    pub fn pllq(&self) -> PllqR {
        PllqR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The PLL VCO source clock prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn pllpsc(&mut self) -> PllpscW<PllSpec> {
        PllpscW::new(self, 0)
    }
    #[doc = "Bits 6:14 - The PLL VCO clock multi factor"]
    #[inline(always)]
    #[must_use]
    pub fn plln(&mut self) -> PllnW<PllSpec> {
        PllnW::new(self, 6)
    }
    #[doc = "Bits 16:17 - The PLLP output frequency division factor from PLL VCO clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllp(&mut self) -> PllpW<PllSpec> {
        PllpW::new(self, 16)
    }
    #[doc = "Bit 22 - PLL Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn pllsel(&mut self) -> PllselW<PllSpec> {
        PllselW::new(self, 22)
    }
    #[doc = "Bits 24:27 - The PLL Q output frequency division factor from PLL VCO clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllq(&mut self) -> PllqW<PllSpec> {
        PllqW::new(self, 24)
    }
}
#[doc = "PLL register (RCU_PLL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllSpec;
impl crate::RegisterSpec for PllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll::R`](R) reader structure"]
impl crate::Readable for PllSpec {}
#[doc = "`write(|w| ..)` method takes [`pll::W`](W) writer structure"]
impl crate::Writable for PllSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL to value 0x2400_3010"]
impl crate::Resettable for PllSpec {
    const RESET_VALUE: u32 = 0x2400_3010;
}
