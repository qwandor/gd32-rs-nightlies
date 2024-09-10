#[doc = "Register `DATA6` reader"]
pub type R = crate::R<Data6Spec>;
#[doc = "Register `DATA6` writer"]
pub type W = crate::W<Data6Spec>;
#[doc = "Field `SEG_DATA6` reader - Each bit corresponds to one segment to display"]
pub type SegData6R = crate::FieldReader<u32>;
#[doc = "Field `SEG_DATA6` writer - Each bit corresponds to one segment to display"]
pub type SegData6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Each bit corresponds to one segment to display"]
    #[inline(always)]
    pub fn seg_data6(&self) -> SegData6R {
        SegData6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit corresponds to one segment to display"]
    #[inline(always)]
    #[must_use]
    pub fn seg_data6(&mut self) -> SegData6W<Data6Spec> {
        SegData6W::new(self, 0)
    }
}
#[doc = "SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data6Spec;
impl crate::RegisterSpec for Data6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data6::R`](R) reader structure"]
impl crate::Readable for Data6Spec {}
#[doc = "`write(|w| ..)` method takes [`data6::W`](W) writer structure"]
impl crate::Writable for Data6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA6 to value 0"]
impl crate::Resettable for Data6Spec {
    const RESET_VALUE: u32 = 0;
}
