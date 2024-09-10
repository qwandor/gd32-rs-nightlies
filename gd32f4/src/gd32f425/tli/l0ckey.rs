#[doc = "Register `L0CKEY` reader"]
pub type R = crate::R<L0ckeySpec>;
#[doc = "Register `L0CKEY` writer"]
pub type W = crate::W<L0ckeySpec>;
#[doc = "Field `CKEYB` reader - Color Key Blue"]
pub type CkeybR = crate::FieldReader;
#[doc = "Field `CKEYB` writer - Color Key Blue"]
pub type CkeybW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CKEYG` reader - Color Key Green"]
pub type CkeygR = crate::FieldReader;
#[doc = "Field `CKEYG` writer - Color Key Green"]
pub type CkeygW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CKEYR` reader - Color Key Red"]
pub type CkeyrR = crate::FieldReader;
#[doc = "Field `CKEYR` writer - Color Key Red"]
pub type CkeyrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Color Key Blue"]
    #[inline(always)]
    pub fn ckeyb(&self) -> CkeybR {
        CkeybR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Color Key Green"]
    #[inline(always)]
    pub fn ckeyg(&self) -> CkeygR {
        CkeygR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Color Key Red"]
    #[inline(always)]
    pub fn ckeyr(&self) -> CkeyrR {
        CkeyrR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Color Key Blue"]
    #[inline(always)]
    #[must_use]
    pub fn ckeyb(&mut self) -> CkeybW<L0ckeySpec> {
        CkeybW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Color Key Green"]
    #[inline(always)]
    #[must_use]
    pub fn ckeyg(&mut self) -> CkeygW<L0ckeySpec> {
        CkeygW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Color Key Red"]
    #[inline(always)]
    #[must_use]
    pub fn ckeyr(&mut self) -> CkeyrW<L0ckeySpec> {
        CkeyrW::new(self, 16)
    }
}
#[doc = "Layer 0 color key register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0ckey::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0ckey::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L0ckeySpec;
impl crate::RegisterSpec for L0ckeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l0ckey::R`](R) reader structure"]
impl crate::Readable for L0ckeySpec {}
#[doc = "`write(|w| ..)` method takes [`l0ckey::W`](W) writer structure"]
impl crate::Writable for L0ckeySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L0CKEY to value 0"]
impl crate::Resettable for L0ckeySpec {
    const RESET_VALUE: u32 = 0;
}
