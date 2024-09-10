#[doc = "Register `DATA0` reader"]
pub type R = crate::R<Data0Spec>;
#[doc = "Register `DATA0` writer"]
pub type W = crate::W<Data0Spec>;
#[doc = "Field `SEG_DATA0` reader - Each bit corresponds to one segment to display"]
pub type SegData0R = crate::FieldReader<u32>;
#[doc = "Field `SEG_DATA0` writer - Each bit corresponds to one segment to display"]
pub type SegData0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Each bit corresponds to one segment to display"]
    #[inline(always)]
    pub fn seg_data0(&self) -> SegData0R {
        SegData0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit corresponds to one segment to display"]
    #[inline(always)]
    #[must_use]
    pub fn seg_data0(&mut self) -> SegData0W<Data0Spec> {
        SegData0W::new(self, 0)
    }
}
#[doc = "SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data0Spec;
impl crate::RegisterSpec for Data0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0::R`](R) reader structure"]
impl crate::Readable for Data0Spec {}
#[doc = "`write(|w| ..)` method takes [`data0::W`](W) writer structure"]
impl crate::Writable for Data0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA0 to value 0"]
impl crate::Resettable for Data0Spec {
    const RESET_VALUE: u32 = 0;
}
