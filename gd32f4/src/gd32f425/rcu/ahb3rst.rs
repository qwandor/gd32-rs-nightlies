#[doc = "Register `AHB3RST` reader"]
pub type R = crate::R<Ahb3rstSpec>;
#[doc = "Register `AHB3RST` writer"]
pub type W = crate::W<Ahb3rstSpec>;
#[doc = "Field `EXMCRST` reader - EXMC reset"]
pub type ExmcrstR = crate::BitReader;
#[doc = "Field `EXMCRST` writer - EXMC reset"]
pub type ExmcrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EXMC reset"]
    #[inline(always)]
    pub fn exmcrst(&self) -> ExmcrstR {
        ExmcrstR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXMC reset"]
    #[inline(always)]
    #[must_use]
    pub fn exmcrst(&mut self) -> ExmcrstW<Ahb3rstSpec> {
        ExmcrstW::new(self, 0)
    }
}
#[doc = "AHB3 reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb3rstSpec;
impl crate::RegisterSpec for Ahb3rstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3rst::R`](R) reader structure"]
impl crate::Readable for Ahb3rstSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb3rst::W`](W) writer structure"]
impl crate::Writable for Ahb3rstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB3RST to value 0"]
impl crate::Resettable for Ahb3rstSpec {
    const RESET_VALUE: u32 = 0;
}
