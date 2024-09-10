#[doc = "Register `CWSPOS` reader"]
pub type R = crate::R<CwsposSpec>;
#[doc = "Register `CWSPOS` writer"]
pub type W = crate::W<CwsposSpec>;
#[doc = "Field `WHSP` reader - Window Horizontal Start Position"]
pub type WhspR = crate::FieldReader<u16>;
#[doc = "Field `WHSP` writer - Window Horizontal Start Position"]
pub type WhspW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `WVSP` reader - Window Vertical Start Position"]
pub type WvspR = crate::FieldReader<u16>;
#[doc = "Field `WVSP` writer - Window Vertical Start Position"]
pub type WvspW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:13 - Window Horizontal Start Position"]
    #[inline(always)]
    pub fn whsp(&self) -> WhspR {
        WhspR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:28 - Window Vertical Start Position"]
    #[inline(always)]
    pub fn wvsp(&self) -> WvspR {
        WvspR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Window Horizontal Start Position"]
    #[inline(always)]
    #[must_use]
    pub fn whsp(&mut self) -> WhspW<CwsposSpec> {
        WhspW::new(self, 0)
    }
    #[doc = "Bits 16:28 - Window Vertical Start Position"]
    #[inline(always)]
    #[must_use]
    pub fn wvsp(&mut self) -> WvspW<CwsposSpec> {
        WvspW::new(self, 16)
    }
}
#[doc = "Cropping window start position register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwspos::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwspos::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CwsposSpec;
impl crate::RegisterSpec for CwsposSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwspos::R`](R) reader structure"]
impl crate::Readable for CwsposSpec {}
#[doc = "`write(|w| ..)` method takes [`cwspos::W`](W) writer structure"]
impl crate::Writable for CwsposSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CWSPOS to value 0"]
impl crate::Resettable for CwsposSpec {
    const RESET_VALUE: u32 = 0;
}
