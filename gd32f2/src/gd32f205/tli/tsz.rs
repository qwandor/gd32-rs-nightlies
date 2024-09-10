#[doc = "Register `TSZ` reader"]
pub type R = crate::R<TszSpec>;
#[doc = "Register `TSZ` writer"]
pub type W = crate::W<TszSpec>;
#[doc = "Field `VTSZ` reader - Vertical total size of the display"]
pub type VtszR = crate::FieldReader<u16>;
#[doc = "Field `VTSZ` writer - Vertical total size of the display"]
pub type VtszW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HTSZ` reader - Horizontal total size of the display"]
pub type HtszR = crate::FieldReader<u16>;
#[doc = "Field `HTSZ` writer - Horizontal total size of the display"]
pub type HtszW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Vertical total size of the display"]
    #[inline(always)]
    pub fn vtsz(&self) -> VtszR {
        VtszR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Horizontal total size of the display"]
    #[inline(always)]
    pub fn htsz(&self) -> HtszR {
        HtszR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Vertical total size of the display"]
    #[inline(always)]
    #[must_use]
    pub fn vtsz(&mut self) -> VtszW<TszSpec> {
        VtszW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Horizontal total size of the display"]
    #[inline(always)]
    #[must_use]
    pub fn htsz(&mut self) -> HtszW<TszSpec> {
        HtszW::new(self, 16)
    }
}
#[doc = "Total size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TszSpec;
impl crate::RegisterSpec for TszSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsz::R`](R) reader structure"]
impl crate::Readable for TszSpec {}
#[doc = "`write(|w| ..)` method takes [`tsz::W`](W) writer structure"]
impl crate::Writable for TszSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSZ to value 0"]
impl crate::Resettable for TszSpec {
    const RESET_VALUE: u32 = 0;
}
