#[doc = "Register `CAR` reader"]
pub type R = crate::R<CarSpec>;
#[doc = "Register `CAR` writer"]
pub type W = crate::W<CarSpec>;
#[doc = "Field `CAR` reader - Counter auto reload register"]
pub type CarR = crate::FieldReader<u16>;
#[doc = "Field `CAR` writer - Counter auto reload register"]
pub type CarW<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter auto reload register"]
    #[inline(always)]
    pub fn car(&self) -> CarR {
        CarR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter auto reload register"]
    #[inline(always)]
    #[must_use]
    pub fn car(&mut self) -> CarW<CarSpec> {
        CarW::new(self, 0)
    }
}
#[doc = "Counter auto reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`car::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`car::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CarSpec;
impl crate::RegisterSpec for CarSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`car::R`](R) reader structure"]
impl crate::Readable for CarSpec {}
#[doc = "`write(|w| ..)` method takes [`car::W`](W) writer structure"]
impl crate::Writable for CarSpec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CAR to value 0"]
impl crate::Resettable for CarSpec {
    const RESET_VALUE: u16 = 0;
}
