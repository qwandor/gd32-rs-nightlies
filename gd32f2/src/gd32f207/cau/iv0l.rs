#[doc = "Register `IV0L` reader"]
pub type R = crate::R<Iv0lSpec>;
#[doc = "Register `IV0L` writer"]
pub type W = crate::W<Iv0lSpec>;
#[doc = "Field `IV0L` reader - The initialization vector for DES,TDES,AES"]
pub type Iv0lR = crate::FieldReader<u32>;
#[doc = "Field `IV0L` writer - The initialization vector for DES,TDES,AES"]
pub type Iv0lW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The initialization vector for DES,TDES,AES"]
    #[inline(always)]
    pub fn iv0l(&self) -> Iv0lR {
        Iv0lR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The initialization vector for DES,TDES,AES"]
    #[inline(always)]
    #[must_use]
    pub fn iv0l(&mut self) -> Iv0lW<Iv0lSpec> {
        Iv0lW::new(self, 0)
    }
}
#[doc = "CAU initialization register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv0l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv0l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iv0lSpec;
impl crate::RegisterSpec for Iv0lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iv0l::R`](R) reader structure"]
impl crate::Readable for Iv0lSpec {}
#[doc = "`write(|w| ..)` method takes [`iv0l::W`](W) writer structure"]
impl crate::Writable for Iv0lSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IV0L to value 0"]
impl crate::Resettable for Iv0lSpec {
    const RESET_VALUE: u32 = 0;
}
