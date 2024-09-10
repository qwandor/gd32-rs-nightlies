#[doc = "Register `AHB3EN` reader"]
pub type R = crate::R<Ahb3enSpec>;
#[doc = "Register `AHB3EN` writer"]
pub type W = crate::W<Ahb3enSpec>;
#[doc = "Field `EXMCEN` reader - EXMC clock enable"]
pub type ExmcenR = crate::BitReader;
#[doc = "Field `EXMCEN` writer - EXMC clock enable"]
pub type ExmcenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EXMC clock enable"]
    #[inline(always)]
    pub fn exmcen(&self) -> ExmcenR {
        ExmcenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXMC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn exmcen(&mut self) -> ExmcenW<Ahb3enSpec> {
        ExmcenW::new(self, 0)
    }
}
#[doc = "AHB3 clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb3enSpec;
impl crate::RegisterSpec for Ahb3enSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3en::R`](R) reader structure"]
impl crate::Readable for Ahb3enSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb3en::W`](W) writer structure"]
impl crate::Writable for Ahb3enSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB3EN to value 0"]
impl crate::Resettable for Ahb3enSpec {
    const RESET_VALUE: u32 = 0;
}
