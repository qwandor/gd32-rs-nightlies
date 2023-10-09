#[doc = "Register `FDATA` reader"]
pub type R = crate::R<FDATA_SPEC>;
#[doc = "Register `FDATA` writer"]
pub type W = crate::W<FDATA_SPEC>;
#[doc = "Field `FDATA` reader - General-purpose 8-bit data register bits"]
pub type FDATA_R = crate::FieldReader;
#[doc = "Field `FDATA` writer - General-purpose 8-bit data register bits"]
pub type FDATA_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - General-purpose 8-bit data register bits"]
    #[inline(always)]
    pub fn fdata(&self) -> FDATA_R {
        FDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - General-purpose 8-bit data register bits"]
    #[inline(always)]
    #[must_use]
    pub fn fdata(&mut self) -> FDATA_W<FDATA_SPEC, 0> {
        FDATA_W::new(self)
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
#[doc = "Free data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDATA_SPEC;
impl crate::RegisterSpec for FDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdata::R`](R) reader structure"]
impl crate::Readable for FDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdata::W`](W) writer structure"]
impl crate::Writable for FDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDATA to value 0"]
impl crate::Resettable for FDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}