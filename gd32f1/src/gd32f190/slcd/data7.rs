#[doc = "Register `DATA7` reader"]
pub type R = crate::R<Data7Spec>;
#[doc = "Register `DATA7` writer"]
pub type W = crate::W<Data7Spec>;
#[doc = "Field `SEG_DATA7` reader - Each bit corresponds to one segment to display"]
pub type SegData7R = crate::FieldReader<u32>;
#[doc = "Field `SEG_DATA7` writer - Each bit corresponds to one segment to display"]
pub type SegData7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Each bit corresponds to one segment to display"]
    #[inline(always)]
    pub fn seg_data7(&self) -> SegData7R {
        SegData7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit corresponds to one segment to display"]
    #[inline(always)]
    #[must_use]
    pub fn seg_data7(&mut self) -> SegData7W<Data7Spec> {
        SegData7W::new(self, 0)
    }
}
#[doc = "SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data7Spec;
impl crate::RegisterSpec for Data7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data7::R`](R) reader structure"]
impl crate::Readable for Data7Spec {}
#[doc = "`write(|w| ..)` method takes [`data7::W`](W) writer structure"]
impl crate::Writable for Data7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA7 to value 0"]
impl crate::Resettable for Data7Spec {
    const RESET_VALUE: u32 = 0;
}
