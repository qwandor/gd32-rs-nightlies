#[doc = "Register `TSZ` reader"]
pub type R = crate::R<TSZ_SPEC>;
#[doc = "Register `TSZ` writer"]
pub type W = crate::W<TSZ_SPEC>;
#[doc = "Field `VTSZ` reader - Vertical total size of the display"]
pub type VTSZ_R = crate::FieldReader<u16>;
#[doc = "Field `VTSZ` writer - Vertical total size of the display"]
pub type VTSZ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `HTSZ` reader - Horizontal total size of the display"]
pub type HTSZ_R = crate::FieldReader<u16>;
#[doc = "Field `HTSZ` writer - Horizontal total size of the display"]
pub type HTSZ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Vertical total size of the display"]
    #[inline(always)]
    pub fn vtsz(&self) -> VTSZ_R {
        VTSZ_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Horizontal total size of the display"]
    #[inline(always)]
    pub fn htsz(&self) -> HTSZ_R {
        HTSZ_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Vertical total size of the display"]
    #[inline(always)]
    #[must_use]
    pub fn vtsz(&mut self) -> VTSZ_W<TSZ_SPEC, 0> {
        VTSZ_W::new(self)
    }
    #[doc = "Bits 16:27 - Horizontal total size of the display"]
    #[inline(always)]
    #[must_use]
    pub fn htsz(&mut self) -> HTSZ_W<TSZ_SPEC, 16> {
        HTSZ_W::new(self)
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
#[doc = "Total size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSZ_SPEC;
impl crate::RegisterSpec for TSZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsz::R`](R) reader structure"]
impl crate::Readable for TSZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsz::W`](W) writer structure"]
impl crate::Writable for TSZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSZ to value 0"]
impl crate::Resettable for TSZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}