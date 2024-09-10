#[doc = "Register `AHB3SPEN` reader"]
pub type R = crate::R<Ahb3spenSpec>;
#[doc = "Register `AHB3SPEN` writer"]
pub type W = crate::W<Ahb3spenSpec>;
#[doc = "Field `EXMCSPEN` reader - EXMC clock enable when sleep mode"]
pub type ExmcspenR = crate::BitReader;
#[doc = "Field `EXMCSPEN` writer - EXMC clock enable when sleep mode"]
pub type ExmcspenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EXMC clock enable when sleep mode"]
    #[inline(always)]
    pub fn exmcspen(&self) -> ExmcspenR {
        ExmcspenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXMC clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn exmcspen(&mut self) -> ExmcspenW<Ahb3spenSpec> {
        ExmcspenW::new(self, 0)
    }
}
#[doc = "AHB3 Sleep mode enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3spen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3spen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb3spenSpec;
impl crate::RegisterSpec for Ahb3spenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3spen::R`](R) reader structure"]
impl crate::Readable for Ahb3spenSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb3spen::W`](W) writer structure"]
impl crate::Writable for Ahb3spenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB3SPEN to value 0x01"]
impl crate::Resettable for Ahb3spenSpec {
    const RESET_VALUE: u32 = 0x01;
}
