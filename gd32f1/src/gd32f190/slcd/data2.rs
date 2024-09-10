#[doc = "Register `DATA2` reader"]
pub type R = crate::R<Data2Spec>;
#[doc = "Register `DATA2` writer"]
pub type W = crate::W<Data2Spec>;
#[doc = "Field `SEG_DATA2` reader - Each bit corresponds to one segment to display"]
pub type SegData2R = crate::FieldReader<u32>;
#[doc = "Field `SEG_DATA2` writer - Each bit corresponds to one segment to display"]
pub type SegData2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Each bit corresponds to one segment to display"]
    #[inline(always)]
    pub fn seg_data2(&self) -> SegData2R {
        SegData2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit corresponds to one segment to display"]
    #[inline(always)]
    #[must_use]
    pub fn seg_data2(&mut self) -> SegData2W<Data2Spec> {
        SegData2W::new(self, 0)
    }
}
#[doc = "SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data2Spec;
impl crate::RegisterSpec for Data2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data2::R`](R) reader structure"]
impl crate::Readable for Data2Spec {}
#[doc = "`write(|w| ..)` method takes [`data2::W`](W) writer structure"]
impl crate::Writable for Data2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA2 to value 0"]
impl crate::Resettable for Data2Spec {
    const RESET_VALUE: u32 = 0;
}
