#[doc = "Register `L0HPOS` reader"]
pub type R = crate::R<L0hposSpec>;
#[doc = "Register `L0HPOS` writer"]
pub type W = crate::W<L0hposSpec>;
#[doc = "Field `WLP` reader - Window left position"]
pub type WlpR = crate::FieldReader<u16>;
#[doc = "Field `WLP` writer - Window left position"]
pub type WlpW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WRP` reader - Window right position"]
pub type WrpR = crate::FieldReader<u16>;
#[doc = "Field `WRP` writer - Window right position"]
pub type WrpW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Window left position"]
    #[inline(always)]
    pub fn wlp(&self) -> WlpR {
        WlpR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Window right position"]
    #[inline(always)]
    pub fn wrp(&self) -> WrpR {
        WrpR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Window left position"]
    #[inline(always)]
    #[must_use]
    pub fn wlp(&mut self) -> WlpW<L0hposSpec> {
        WlpW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Window right position"]
    #[inline(always)]
    #[must_use]
    pub fn wrp(&mut self) -> WrpW<L0hposSpec> {
        WrpW::new(self, 16)
    }
}
#[doc = "Layer 0 horizontal position parameters register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0hpos::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0hpos::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L0hposSpec;
impl crate::RegisterSpec for L0hposSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l0hpos::R`](R) reader structure"]
impl crate::Readable for L0hposSpec {}
#[doc = "`write(|w| ..)` method takes [`l0hpos::W`](W) writer structure"]
impl crate::Writable for L0hposSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L0HPOS to value 0"]
impl crate::Resettable for L0hposSpec {
    const RESET_VALUE: u32 = 0;
}
