#[doc = "Register `DATA5` reader"]
pub type R = crate::R<DATA5_SPEC>;
#[doc = "Register `DATA5` writer"]
pub type W = crate::W<DATA5_SPEC>;
#[doc = "Field `SEG_DATA5` reader - Each bit corresponds to one segment to display"]
pub type SEG_DATA5_R = crate::FieldReader<u32>;
#[doc = "Field `SEG_DATA5` writer - Each bit corresponds to one segment to display"]
pub type SEG_DATA5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Each bit corresponds to one segment to display"]
    #[inline(always)]
    pub fn seg_data5(&self) -> SEG_DATA5_R {
        SEG_DATA5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit corresponds to one segment to display"]
    #[inline(always)]
    #[must_use]
    pub fn seg_data5(&mut self) -> SEG_DATA5_W<DATA5_SPEC, 0> {
        SEG_DATA5_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA5_SPEC;
impl crate::RegisterSpec for DATA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data5::R`](R) reader structure"]
impl crate::Readable for DATA5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data5::W`](W) writer structure"]
impl crate::Writable for DATA5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA5 to value 0"]
impl crate::Resettable for DATA5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}