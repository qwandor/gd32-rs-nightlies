#[doc = "Register `IV1H` reader"]
pub type R = crate::R<Iv1hSpec>;
#[doc = "Register `IV1H` writer"]
pub type W = crate::W<Iv1hSpec>;
#[doc = "Field `IV1H` reader - The initialization vector for DES,TDES,AES"]
pub type Iv1hR = crate::FieldReader<u32>;
#[doc = "Field `IV1H` writer - The initialization vector for DES,TDES,AES"]
pub type Iv1hW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The initialization vector for DES,TDES,AES"]
    #[inline(always)]
    pub fn iv1h(&self) -> Iv1hR {
        Iv1hR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The initialization vector for DES,TDES,AES"]
    #[inline(always)]
    #[must_use]
    pub fn iv1h(&mut self) -> Iv1hW<Iv1hSpec> {
        Iv1hW::new(self, 0)
    }
}
#[doc = "CAU initialization register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv1h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv1h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iv1hSpec;
impl crate::RegisterSpec for Iv1hSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iv1h::R`](R) reader structure"]
impl crate::Readable for Iv1hSpec {}
#[doc = "`write(|w| ..)` method takes [`iv1h::W`](W) writer structure"]
impl crate::Writable for Iv1hSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IV1H to value 0"]
impl crate::Resettable for Iv1hSpec {
    const RESET_VALUE: u32 = 0;
}
