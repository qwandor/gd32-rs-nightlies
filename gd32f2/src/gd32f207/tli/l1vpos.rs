#[doc = "Register `L1VPOS` reader"]
pub type R = crate::R<L1vposSpec>;
#[doc = "Register `L1VPOS` writer"]
pub type W = crate::W<L1vposSpec>;
#[doc = "Field `WTP` reader - Window top position"]
pub type WtpR = crate::FieldReader<u16>;
#[doc = "Field `WTP` writer - Window top position"]
pub type WtpW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WBP` reader - Window bottom position"]
pub type WbpR = crate::FieldReader<u16>;
#[doc = "Field `WBP` writer - Window bottom position"]
pub type WbpW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Window top position"]
    #[inline(always)]
    pub fn wtp(&self) -> WtpR {
        WtpR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Window bottom position"]
    #[inline(always)]
    pub fn wbp(&self) -> WbpR {
        WbpR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Window top position"]
    #[inline(always)]
    #[must_use]
    pub fn wtp(&mut self) -> WtpW<L1vposSpec> {
        WtpW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Window bottom position"]
    #[inline(always)]
    #[must_use]
    pub fn wbp(&mut self) -> WbpW<L1vposSpec> {
        WbpW::new(self, 16)
    }
}
#[doc = "Layer 1 vertical position parameters register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1vpos::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1vpos::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1vposSpec;
impl crate::RegisterSpec for L1vposSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1vpos::R`](R) reader structure"]
impl crate::Readable for L1vposSpec {}
#[doc = "`write(|w| ..)` method takes [`l1vpos::W`](W) writer structure"]
impl crate::Writable for L1vposSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1VPOS to value 0"]
impl crate::Resettable for L1vposSpec {
    const RESET_VALUE: u32 = 0;
}
