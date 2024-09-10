#[doc = "Register `IV1L` reader"]
pub type R = crate::R<Iv1lSpec>;
#[doc = "Register `IV1L` writer"]
pub type W = crate::W<Iv1lSpec>;
#[doc = "Field `IV1L` reader - The initialization vector for DES,TDES,AES"]
pub type Iv1lR = crate::FieldReader<u32>;
#[doc = "Field `IV1L` writer - The initialization vector for DES,TDES,AES"]
pub type Iv1lW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The initialization vector for DES,TDES,AES"]
    #[inline(always)]
    pub fn iv1l(&self) -> Iv1lR {
        Iv1lR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The initialization vector for DES,TDES,AES"]
    #[inline(always)]
    #[must_use]
    pub fn iv1l(&mut self) -> Iv1lW<Iv1lSpec> {
        Iv1lW::new(self, 0)
    }
}
#[doc = "CAU initialization register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv1l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv1l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iv1lSpec;
impl crate::RegisterSpec for Iv1lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iv1l::R`](R) reader structure"]
impl crate::Readable for Iv1lSpec {}
#[doc = "`write(|w| ..)` method takes [`iv1l::W`](W) writer structure"]
impl crate::Writable for Iv1lSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IV1L to value 0"]
impl crate::Resettable for Iv1lSpec {
    const RESET_VALUE: u32 = 0;
}
