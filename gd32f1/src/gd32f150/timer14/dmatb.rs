#[doc = "Register `DMATB` reader"]
pub type R = crate::R<DmatbSpec>;
#[doc = "Register `DMATB` writer"]
pub type W = crate::W<DmatbSpec>;
#[doc = "Field `DMATB` reader - DMA transfer"]
pub type DmatbR = crate::FieldReader<u16>;
#[doc = "Field `DMATB` writer - DMA transfer"]
pub type DmatbW<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DMA transfer"]
    #[inline(always)]
    pub fn dmatb(&self) -> DmatbR {
        DmatbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - DMA transfer"]
    #[inline(always)]
    #[must_use]
    pub fn dmatb(&mut self) -> DmatbW<DmatbSpec> {
        DmatbW::new(self, 0)
    }
}
#[doc = "DMA transfer buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmatbSpec;
impl crate::RegisterSpec for DmatbSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dmatb::R`](R) reader structure"]
impl crate::Readable for DmatbSpec {}
#[doc = "`write(|w| ..)` method takes [`dmatb::W`](W) writer structure"]
impl crate::Writable for DmatbSpec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DMATB to value 0"]
impl crate::Resettable for DmatbSpec {
    const RESET_VALUE: u16 = 0;
}
