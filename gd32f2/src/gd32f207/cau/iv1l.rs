#[doc = "Register `IV1L` reader"]
pub type R = crate::R<IV1L_SPEC>;
#[doc = "Register `IV1L` writer"]
pub type W = crate::W<IV1L_SPEC>;
#[doc = "Field `IV1L` reader - The initialization vector for DES,TDES,AES"]
pub type IV1L_R = crate::FieldReader<u32>;
#[doc = "Field `IV1L` writer - The initialization vector for DES,TDES,AES"]
pub type IV1L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - The initialization vector for DES,TDES,AES"]
    #[inline(always)]
    pub fn iv1l(&self) -> IV1L_R {
        IV1L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The initialization vector for DES,TDES,AES"]
    #[inline(always)]
    #[must_use]
    pub fn iv1l(&mut self) -> IV1L_W<IV1L_SPEC, 0> {
        IV1L_W::new(self)
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
#[doc = "CAU initialization register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv1l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv1l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IV1L_SPEC;
impl crate::RegisterSpec for IV1L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iv1l::R`](R) reader structure"]
impl crate::Readable for IV1L_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iv1l::W`](W) writer structure"]
impl crate::Writable for IV1L_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IV1L to value 0"]
impl crate::Resettable for IV1L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}