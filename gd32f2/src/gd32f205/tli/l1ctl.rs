#[doc = "Register `L1CTL` reader"]
pub type R = crate::R<L1ctlSpec>;
#[doc = "Register `L1CTL` writer"]
pub type W = crate::W<L1ctlSpec>;
#[doc = "Field `LEN` reader - Layer enable"]
pub type LenR = crate::BitReader;
#[doc = "Field `LEN` writer - Layer enable"]
pub type LenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKEYEN` reader - Color keying enable"]
pub type CkeyenR = crate::BitReader;
#[doc = "Field `CKEYEN` writer - Color keying enable"]
pub type CkeyenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LUTEN` reader - LUT enable"]
pub type LutenR = crate::BitReader;
#[doc = "Field `LUTEN` writer - LUT enable"]
pub type LutenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Layer enable"]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Color keying enable"]
    #[inline(always)]
    pub fn ckeyen(&self) -> CkeyenR {
        CkeyenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - LUT enable"]
    #[inline(always)]
    pub fn luten(&self) -> LutenR {
        LutenR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Layer enable"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LenW<L1ctlSpec> {
        LenW::new(self, 0)
    }
    #[doc = "Bit 1 - Color keying enable"]
    #[inline(always)]
    #[must_use]
    pub fn ckeyen(&mut self) -> CkeyenW<L1ctlSpec> {
        CkeyenW::new(self, 1)
    }
    #[doc = "Bit 4 - LUT enable"]
    #[inline(always)]
    #[must_use]
    pub fn luten(&mut self) -> LutenW<L1ctlSpec> {
        LutenW::new(self, 4)
    }
}
#[doc = "Layer 1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1ctlSpec;
impl crate::RegisterSpec for L1ctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1ctl::R`](R) reader structure"]
impl crate::Readable for L1ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`l1ctl::W`](W) writer structure"]
impl crate::Writable for L1ctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1CTL to value 0"]
impl crate::Resettable for L1ctlSpec {
    const RESET_VALUE: u32 = 0;
}
