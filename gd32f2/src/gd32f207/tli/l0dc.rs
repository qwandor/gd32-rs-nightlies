#[doc = "Register `L0DC` reader"]
pub type R = crate::R<L0dcSpec>;
#[doc = "Register `L0DC` writer"]
pub type W = crate::W<L0dcSpec>;
#[doc = "Field `DCB` reader - The default color blue"]
pub type DcbR = crate::FieldReader;
#[doc = "Field `DCB` writer - The default color blue"]
pub type DcbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DCG` reader - The default color green"]
pub type DcgR = crate::FieldReader;
#[doc = "Field `DCG` writer - The default color green"]
pub type DcgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DCR` reader - The default color red"]
pub type DcrR = crate::FieldReader;
#[doc = "Field `DCR` writer - The default color red"]
pub type DcrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DCA` reader - The default color ALPHA"]
pub type DcaR = crate::FieldReader;
#[doc = "Field `DCA` writer - The default color ALPHA"]
pub type DcaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The default color blue"]
    #[inline(always)]
    pub fn dcb(&self) -> DcbR {
        DcbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The default color green"]
    #[inline(always)]
    pub fn dcg(&self) -> DcgR {
        DcgR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - The default color red"]
    #[inline(always)]
    pub fn dcr(&self) -> DcrR {
        DcrR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - The default color ALPHA"]
    #[inline(always)]
    pub fn dca(&self) -> DcaR {
        DcaR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The default color blue"]
    #[inline(always)]
    #[must_use]
    pub fn dcb(&mut self) -> DcbW<L0dcSpec> {
        DcbW::new(self, 0)
    }
    #[doc = "Bits 8:15 - The default color green"]
    #[inline(always)]
    #[must_use]
    pub fn dcg(&mut self) -> DcgW<L0dcSpec> {
        DcgW::new(self, 8)
    }
    #[doc = "Bits 16:23 - The default color red"]
    #[inline(always)]
    #[must_use]
    pub fn dcr(&mut self) -> DcrW<L0dcSpec> {
        DcrW::new(self, 16)
    }
    #[doc = "Bits 24:31 - The default color ALPHA"]
    #[inline(always)]
    #[must_use]
    pub fn dca(&mut self) -> DcaW<L0dcSpec> {
        DcaW::new(self, 24)
    }
}
#[doc = "Layer 0 default color register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0dc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0dc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L0dcSpec;
impl crate::RegisterSpec for L0dcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l0dc::R`](R) reader structure"]
impl crate::Readable for L0dcSpec {}
#[doc = "`write(|w| ..)` method takes [`l0dc::W`](W) writer structure"]
impl crate::Writable for L0dcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L0DC to value 0"]
impl crate::Resettable for L0dcSpec {
    const RESET_VALUE: u32 = 0;
}
