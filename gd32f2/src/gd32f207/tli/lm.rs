#[doc = "Register `LM` reader"]
pub type R = crate::R<LmSpec>;
#[doc = "Register `LM` writer"]
pub type W = crate::W<LmSpec>;
#[doc = "Field `LM` reader - Line Mark value"]
pub type LmR = crate::FieldReader<u16>;
#[doc = "Field `LM` writer - Line Mark value"]
pub type LmW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Line Mark value"]
    #[inline(always)]
    pub fn lm(&self) -> LmR {
        LmR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Line Mark value"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LmW<LmSpec> {
        LmW::new(self, 0)
    }
}
#[doc = "Line mark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LmSpec;
impl crate::RegisterSpec for LmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lm::R`](R) reader structure"]
impl crate::Readable for LmSpec {}
#[doc = "`write(|w| ..)` method takes [`lm::W`](W) writer structure"]
impl crate::Writable for LmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LM to value 0"]
impl crate::Resettable for LmSpec {
    const RESET_VALUE: u32 = 0;
}
