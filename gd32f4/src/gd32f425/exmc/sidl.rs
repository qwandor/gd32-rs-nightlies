#[doc = "Register `SIDL` reader"]
pub type R = crate::R<SidlSpec>;
#[doc = "Register `SIDL` writer"]
pub type W = crate::W<SidlSpec>;
#[doc = "Field `SIDL` reader - ID Low Data saved for SPI Read ID Command"]
pub type SidlR = crate::FieldReader<u32>;
#[doc = "Field `SIDL` writer - ID Low Data saved for SPI Read ID Command"]
pub type SidlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ID Low Data saved for SPI Read ID Command"]
    #[inline(always)]
    pub fn sidl(&self) -> SidlR {
        SidlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ID Low Data saved for SPI Read ID Command"]
    #[inline(always)]
    #[must_use]
    pub fn sidl(&mut self) -> SidlW<SidlSpec> {
        SidlW::new(self, 0)
    }
}
#[doc = "SPI ID low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sidl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sidl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SidlSpec;
impl crate::RegisterSpec for SidlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sidl::R`](R) reader structure"]
impl crate::Readable for SidlSpec {}
#[doc = "`write(|w| ..)` method takes [`sidl::W`](W) writer structure"]
impl crate::Writable for SidlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIDL to value 0"]
impl crate::Resettable for SidlSpec {
    const RESET_VALUE: u32 = 0;
}
