#[doc = "Register `INTC` reader"]
pub type R = crate::R<IntcSpec>;
#[doc = "Register `INTC` writer"]
pub type W = crate::W<IntcSpec>;
#[doc = "Field `CCTCF` reader - Clear charge-transfer complete flag"]
pub type CctcfR = crate::BitReader;
#[doc = "Field `CCTCF` writer - Clear charge-transfer complete flag"]
pub type CctcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMNERR` reader - Clear max cycle number error"]
pub type CmnerrR = crate::BitReader;
#[doc = "Field `CMNERR` writer - Clear max cycle number error"]
pub type CmnerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear charge-transfer complete flag"]
    #[inline(always)]
    pub fn cctcf(&self) -> CctcfR {
        CctcfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear max cycle number error"]
    #[inline(always)]
    pub fn cmnerr(&self) -> CmnerrR {
        CmnerrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear charge-transfer complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn cctcf(&mut self) -> CctcfW<IntcSpec> {
        CctcfW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear max cycle number error"]
    #[inline(always)]
    #[must_use]
    pub fn cmnerr(&mut self) -> CmnerrW<IntcSpec> {
        CmnerrW::new(self, 1)
    }
}
#[doc = "interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcSpec;
impl crate::RegisterSpec for IntcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intc::R`](R) reader structure"]
impl crate::Readable for IntcSpec {}
#[doc = "`write(|w| ..)` method takes [`intc::W`](W) writer structure"]
impl crate::Writable for IntcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTC to value 0"]
impl crate::Resettable for IntcSpec {
    const RESET_VALUE: u32 = 0;
}
