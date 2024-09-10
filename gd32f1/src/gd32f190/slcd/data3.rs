#[doc = "Register `DATA3` reader"]
pub type R = crate::R<Data3Spec>;
#[doc = "Register `DATA3` writer"]
pub type W = crate::W<Data3Spec>;
#[doc = "Field `SEG_DATA3` reader - Each bit corresponds to one segment to display"]
pub type SegData3R = crate::FieldReader<u32>;
#[doc = "Field `SEG_DATA3` writer - Each bit corresponds to one segment to display"]
pub type SegData3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Each bit corresponds to one segment to display"]
    #[inline(always)]
    pub fn seg_data3(&self) -> SegData3R {
        SegData3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit corresponds to one segment to display"]
    #[inline(always)]
    #[must_use]
    pub fn seg_data3(&mut self) -> SegData3W<Data3Spec> {
        SegData3W::new(self, 0)
    }
}
#[doc = "SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data3Spec;
impl crate::RegisterSpec for Data3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data3::R`](R) reader structure"]
impl crate::Readable for Data3Spec {}
#[doc = "`write(|w| ..)` method takes [`data3::W`](W) writer structure"]
impl crate::Writable for Data3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA3 to value 0"]
impl crate::Resettable for Data3Spec {
    const RESET_VALUE: u32 = 0;
}
