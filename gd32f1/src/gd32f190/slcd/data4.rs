#[doc = "Register `DATA4` reader"]
pub type R = crate::R<Data4Spec>;
#[doc = "Register `DATA4` writer"]
pub type W = crate::W<Data4Spec>;
#[doc = "Field `SEG_DATA4` reader - Each bit corresponds to one segment to display"]
pub type SegData4R = crate::FieldReader<u32>;
#[doc = "Field `SEG_DATA4` writer - Each bit corresponds to one segment to display"]
pub type SegData4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Each bit corresponds to one segment to display"]
    #[inline(always)]
    pub fn seg_data4(&self) -> SegData4R {
        SegData4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit corresponds to one segment to display"]
    #[inline(always)]
    #[must_use]
    pub fn seg_data4(&mut self) -> SegData4W<Data4Spec> {
        SegData4W::new(self, 0)
    }
}
#[doc = "SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data4Spec;
impl crate::RegisterSpec for Data4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data4::R`](R) reader structure"]
impl crate::Readable for Data4Spec {}
#[doc = "`write(|w| ..)` method takes [`data4::W`](W) writer structure"]
impl crate::Writable for Data4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA4 to value 0"]
impl crate::Resettable for Data4Spec {
    const RESET_VALUE: u32 = 0;
}
