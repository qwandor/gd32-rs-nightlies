#[doc = "Register `SDTCFG0` reader"]
pub type R = crate::R<Sdtcfg0Spec>;
#[doc = "Register `SDTCFG0` writer"]
pub type W = crate::W<Sdtcfg0Spec>;
#[doc = "Field `LMRD` reader - Load Mode Register Delay"]
pub type LmrdR = crate::FieldReader;
#[doc = "Field `LMRD` writer - Load Mode Register Delay"]
pub type LmrdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `XSRD` reader - Exit Self-refresh delay"]
pub type XsrdR = crate::FieldReader;
#[doc = "Field `XSRD` writer - Exit Self-refresh delay"]
pub type XsrdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RASD` reader - Row address select delay"]
pub type RasdR = crate::FieldReader;
#[doc = "Field `RASD` writer - Row address select delay"]
pub type RasdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ARFD` reader - Auto refresh delay"]
pub type ArfdR = crate::FieldReader;
#[doc = "Field `ARFD` writer - Auto refresh delay"]
pub type ArfdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WRD` reader - Write recovery delay"]
pub type WrdR = crate::FieldReader;
#[doc = "Field `WRD` writer - Write recovery delay"]
pub type WrdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RPD` reader - Row precharge delay"]
pub type RpdR = crate::FieldReader;
#[doc = "Field `RPD` writer - Row precharge delay"]
pub type RpdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RCD` reader - Row to column delay"]
pub type RcdR = crate::FieldReader;
#[doc = "Field `RCD` writer - Row to column delay"]
pub type RcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Load Mode Register Delay"]
    #[inline(always)]
    pub fn lmrd(&self) -> LmrdR {
        LmrdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Exit Self-refresh delay"]
    #[inline(always)]
    pub fn xsrd(&self) -> XsrdR {
        XsrdR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Row address select delay"]
    #[inline(always)]
    pub fn rasd(&self) -> RasdR {
        RasdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Auto refresh delay"]
    #[inline(always)]
    pub fn arfd(&self) -> ArfdR {
        ArfdR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Write recovery delay"]
    #[inline(always)]
    pub fn wrd(&self) -> WrdR {
        WrdR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Row precharge delay"]
    #[inline(always)]
    pub fn rpd(&self) -> RpdR {
        RpdR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Row to column delay"]
    #[inline(always)]
    pub fn rcd(&self) -> RcdR {
        RcdR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Load Mode Register Delay"]
    #[inline(always)]
    #[must_use]
    pub fn lmrd(&mut self) -> LmrdW<Sdtcfg0Spec> {
        LmrdW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Exit Self-refresh delay"]
    #[inline(always)]
    #[must_use]
    pub fn xsrd(&mut self) -> XsrdW<Sdtcfg0Spec> {
        XsrdW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Row address select delay"]
    #[inline(always)]
    #[must_use]
    pub fn rasd(&mut self) -> RasdW<Sdtcfg0Spec> {
        RasdW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Auto refresh delay"]
    #[inline(always)]
    #[must_use]
    pub fn arfd(&mut self) -> ArfdW<Sdtcfg0Spec> {
        ArfdW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Write recovery delay"]
    #[inline(always)]
    #[must_use]
    pub fn wrd(&mut self) -> WrdW<Sdtcfg0Spec> {
        WrdW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Row precharge delay"]
    #[inline(always)]
    #[must_use]
    pub fn rpd(&mut self) -> RpdW<Sdtcfg0Spec> {
        RpdW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Row to column delay"]
    #[inline(always)]
    #[must_use]
    pub fn rcd(&mut self) -> RcdW<Sdtcfg0Spec> {
        RcdW::new(self, 24)
    }
}
#[doc = "SDRAM timing configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdtcfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdtcfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sdtcfg0Spec;
impl crate::RegisterSpec for Sdtcfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdtcfg0::R`](R) reader structure"]
impl crate::Readable for Sdtcfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`sdtcfg0::W`](W) writer structure"]
impl crate::Writable for Sdtcfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDTCFG0 to value 0x0fff_ffff"]
impl crate::Resettable for Sdtcfg0Spec {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
