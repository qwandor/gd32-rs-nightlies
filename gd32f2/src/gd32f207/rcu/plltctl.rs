#[doc = "Register `PLLTCTL` reader"]
pub type R = crate::R<PlltctlSpec>;
#[doc = "Register `PLLTCTL` writer"]
pub type W = crate::W<PlltctlSpec>;
#[doc = "Field `PLLTEN` reader - PLLT enable"]
pub type PlltenR = crate::BitReader;
#[doc = "Field `PLLTEN` writer - PLLT enable"]
pub type PlltenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLTSTB` reader - PLLTclock stabilization flag"]
pub type PlltstbR = crate::BitReader;
impl R {
    #[doc = "Bit 28 - PLLT enable"]
    #[inline(always)]
    pub fn pllten(&self) -> PlltenR {
        PlltenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PLLTclock stabilization flag"]
    #[inline(always)]
    pub fn plltstb(&self) -> PlltstbR {
        PlltstbR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - PLLT enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllten(&mut self) -> PlltenW<PlltctlSpec> {
        PlltenW::new(self, 28)
    }
}
#[doc = "PLLT control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plltctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plltctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlltctlSpec;
impl crate::RegisterSpec for PlltctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plltctl::R`](R) reader structure"]
impl crate::Readable for PlltctlSpec {}
#[doc = "`write(|w| ..)` method takes [`plltctl::W`](W) writer structure"]
impl crate::Writable for PlltctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLTCTL to value 0"]
impl crate::Resettable for PlltctlSpec {
    const RESET_VALUE: u32 = 0;
}
