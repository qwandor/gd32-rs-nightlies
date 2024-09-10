#[doc = "Register `L0BLEND` reader"]
pub type R = crate::R<L0blendSpec>;
#[doc = "Register `L0BLEND` writer"]
pub type W = crate::W<L0blendSpec>;
#[doc = "Field `ACF2` reader - Alpha Calculation Factor 2 of Blending Method"]
pub type Acf2R = crate::FieldReader;
#[doc = "Field `ACF2` writer - Alpha Calculation Factor 2 of Blending Method"]
pub type Acf2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ACF1` reader - Alpha Calculation Factor 1 of Blending Method"]
pub type Acf1R = crate::FieldReader;
#[doc = "Field `ACF1` writer - Alpha Calculation Factor 1 of Blending Method"]
pub type Acf1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Alpha Calculation Factor 2 of Blending Method"]
    #[inline(always)]
    pub fn acf2(&self) -> Acf2R {
        Acf2R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Alpha Calculation Factor 1 of Blending Method"]
    #[inline(always)]
    pub fn acf1(&self) -> Acf1R {
        Acf1R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Alpha Calculation Factor 2 of Blending Method"]
    #[inline(always)]
    #[must_use]
    pub fn acf2(&mut self) -> Acf2W<L0blendSpec> {
        Acf2W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Alpha Calculation Factor 1 of Blending Method"]
    #[inline(always)]
    #[must_use]
    pub fn acf1(&mut self) -> Acf1W<L0blendSpec> {
        Acf1W::new(self, 8)
    }
}
#[doc = "Layer 0 blending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0blend::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0blend::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L0blendSpec;
impl crate::RegisterSpec for L0blendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l0blend::R`](R) reader structure"]
impl crate::Readable for L0blendSpec {}
#[doc = "`write(|w| ..)` method takes [`l0blend::W`](W) writer structure"]
impl crate::Writable for L0blendSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L0BLEND to value 0x0607"]
impl crate::Resettable for L0blendSpec {
    const RESET_VALUE: u32 = 0x0607;
}
