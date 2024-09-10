#[doc = "Register `DATA5` reader"]
pub type R = crate::R<Data5Spec>;
#[doc = "Register `DATA5` writer"]
pub type W = crate::W<Data5Spec>;
#[doc = "Field `SEG_DATA5` reader - Each bit corresponds to one segment to display"]
pub type SegData5R = crate::FieldReader<u32>;
#[doc = "Field `SEG_DATA5` writer - Each bit corresponds to one segment to display"]
pub type SegData5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Each bit corresponds to one segment to display"]
    #[inline(always)]
    pub fn seg_data5(&self) -> SegData5R {
        SegData5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit corresponds to one segment to display"]
    #[inline(always)]
    #[must_use]
    pub fn seg_data5(&mut self) -> SegData5W<Data5Spec> {
        SegData5W::new(self, 0)
    }
}
#[doc = "SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data5Spec;
impl crate::RegisterSpec for Data5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data5::R`](R) reader structure"]
impl crate::Readable for Data5Spec {}
#[doc = "`write(|w| ..)` method takes [`data5::W`](W) writer structure"]
impl crate::Writable for Data5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA5 to value 0"]
impl crate::Resettable for Data5Spec {
    const RESET_VALUE: u32 = 0;
}
