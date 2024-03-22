#[doc = "Register `ST1CAR` reader"]
pub type R = crate::R<St1carSpec>;
#[doc = "Register `ST1CAR` writer"]
pub type W = crate::W<St1carSpec>;
#[doc = "Field `CARL` reader - Counter auto reload value"]
pub type CarlR = crate::FieldReader<u16>;
#[doc = "Field `CARL` writer - Counter auto reload value"]
pub type CarlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter auto reload value"]
    #[inline(always)]
    pub fn carl(&self) -> CarlR {
        CarlR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter auto reload value"]
    #[inline(always)]
    #[must_use]
    pub fn carl(&mut self) -> CarlW<St1carSpec> {
        CarlW::new(self, 0)
    }
}
#[doc = "SHRTIMER Slave_TIMER1 counter auto reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1car::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1car::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St1carSpec;
impl crate::RegisterSpec for St1carSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st1car::R`](R) reader structure"]
impl crate::Readable for St1carSpec {}
#[doc = "`write(|w| ..)` method takes [`st1car::W`](W) writer structure"]
impl crate::Writable for St1carSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ST1CAR to value 0"]
impl crate::Resettable for St1carSpec {
    const RESET_VALUE: u32 = 0;
}
