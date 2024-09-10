#[doc = "Register `SIDH` reader"]
pub type R = crate::R<SidhSpec>;
#[doc = "Register `SIDH` writer"]
pub type W = crate::W<SidhSpec>;
#[doc = "Field `SIDH` reader - ID High Data saved for SPI Read ID Command"]
pub type SidhR = crate::FieldReader<u32>;
#[doc = "Field `SIDH` writer - ID High Data saved for SPI Read ID Command"]
pub type SidhW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ID High Data saved for SPI Read ID Command"]
    #[inline(always)]
    pub fn sidh(&self) -> SidhR {
        SidhR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ID High Data saved for SPI Read ID Command"]
    #[inline(always)]
    #[must_use]
    pub fn sidh(&mut self) -> SidhW<SidhSpec> {
        SidhW::new(self, 0)
    }
}
#[doc = "SPI ID high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sidh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sidh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SidhSpec;
impl crate::RegisterSpec for SidhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sidh::R`](R) reader structure"]
impl crate::Readable for SidhSpec {}
#[doc = "`write(|w| ..)` method takes [`sidh::W`](W) writer structure"]
impl crate::Writable for SidhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIDH to value 0"]
impl crate::Resettable for SidhSpec {
    const RESET_VALUE: u32 = 0;
}
