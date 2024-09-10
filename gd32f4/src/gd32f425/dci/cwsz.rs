#[doc = "Register `CWSZ` reader"]
pub type R = crate::R<CwszSpec>;
#[doc = "Register `CWSZ` writer"]
pub type W = crate::W<CwszSpec>;
#[doc = "Field `WHSZ` reader - Window Horizontal Size"]
pub type WhszR = crate::FieldReader<u16>;
#[doc = "Field `WHSZ` writer - Window Horizontal Size"]
pub type WhszW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `WVSZ` reader - Window Vertical Size"]
pub type WvszR = crate::FieldReader<u16>;
#[doc = "Field `WVSZ` writer - Window Vertical Size"]
pub type WvszW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Window Horizontal Size"]
    #[inline(always)]
    pub fn whsz(&self) -> WhszR {
        WhszR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Window Vertical Size"]
    #[inline(always)]
    pub fn wvsz(&self) -> WvszR {
        WvszR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Window Horizontal Size"]
    #[inline(always)]
    #[must_use]
    pub fn whsz(&mut self) -> WhszW<CwszSpec> {
        WhszW::new(self, 0)
    }
    #[doc = "Bits 16:29 - Window Vertical Size"]
    #[inline(always)]
    #[must_use]
    pub fn wvsz(&mut self) -> WvszW<CwszSpec> {
        WvszW::new(self, 16)
    }
}
#[doc = "Cropping window size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwsz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwsz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CwszSpec;
impl crate::RegisterSpec for CwszSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwsz::R`](R) reader structure"]
impl crate::Readable for CwszSpec {}
#[doc = "`write(|w| ..)` method takes [`cwsz::W`](W) writer structure"]
impl crate::Writable for CwszSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CWSZ to value 0"]
impl crate::Resettable for CwszSpec {
    const RESET_VALUE: u32 = 0;
}
