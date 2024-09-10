#[doc = "Register `BGC` reader"]
pub type R = crate::R<BgcSpec>;
#[doc = "Register `BGC` writer"]
pub type W = crate::W<BgcSpec>;
#[doc = "Field `BVB` reader - Background value blue"]
pub type BvbR = crate::FieldReader;
#[doc = "Field `BVB` writer - Background value blue"]
pub type BvbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BVG` reader - Background value green"]
pub type BvgR = crate::FieldReader;
#[doc = "Field `BVG` writer - Background value green"]
pub type BvgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BVR` reader - Background value red"]
pub type BvrR = crate::FieldReader;
#[doc = "Field `BVR` writer - Background value red"]
pub type BvrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Background value blue"]
    #[inline(always)]
    pub fn bvb(&self) -> BvbR {
        BvbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Background value green"]
    #[inline(always)]
    pub fn bvg(&self) -> BvgR {
        BvgR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Background value red"]
    #[inline(always)]
    pub fn bvr(&self) -> BvrR {
        BvrR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Background value blue"]
    #[inline(always)]
    #[must_use]
    pub fn bvb(&mut self) -> BvbW<BgcSpec> {
        BvbW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Background value green"]
    #[inline(always)]
    #[must_use]
    pub fn bvg(&mut self) -> BvgW<BgcSpec> {
        BvgW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Background value red"]
    #[inline(always)]
    #[must_use]
    pub fn bvr(&mut self) -> BvrW<BgcSpec> {
        BvrW::new(self, 16)
    }
}
#[doc = "Background color register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BgcSpec;
impl crate::RegisterSpec for BgcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bgc::R`](R) reader structure"]
impl crate::Readable for BgcSpec {}
#[doc = "`write(|w| ..)` method takes [`bgc::W`](W) writer structure"]
impl crate::Writable for BgcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BGC to value 0"]
impl crate::Resettable for BgcSpec {
    const RESET_VALUE: u32 = 0;
}
