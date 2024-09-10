#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `CSDT` reader - Current step data"]
pub type CsdtR = crate::FieldReader;
#[doc = "Field `CSDT` writer - Current step data"]
pub type CsdtW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SCMOD` reader - Sink current mode"]
pub type ScmodR = crate::BitReader;
#[doc = "Field `SCMOD` writer - Sink current mode"]
pub type ScmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPT` reader - Current precision trim"]
pub type CptR = crate::FieldReader;
#[doc = "Field `CPT` writer - Current precision trim"]
pub type CptW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SSEL` reader - Step selection"]
pub type SselR = crate::BitReader;
#[doc = "Field `SSEL` writer - Step selection"]
pub type SselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CREN` reader - Current reference enable"]
pub type CrenR = crate::BitReader;
#[doc = "Field `CREN` writer - Current reference enable"]
pub type CrenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Current step data"]
    #[inline(always)]
    pub fn csdt(&self) -> CsdtR {
        CsdtR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Sink current mode"]
    #[inline(always)]
    pub fn scmod(&self) -> ScmodR {
        ScmodR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Current precision trim"]
    #[inline(always)]
    pub fn cpt(&self) -> CptR {
        CptR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - Step selection"]
    #[inline(always)]
    pub fn ssel(&self) -> SselR {
        SselR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Current reference enable"]
    #[inline(always)]
    pub fn cren(&self) -> CrenR {
        CrenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Current step data"]
    #[inline(always)]
    #[must_use]
    pub fn csdt(&mut self) -> CsdtW<CtlSpec> {
        CsdtW::new(self, 0)
    }
    #[doc = "Bit 7 - Sink current mode"]
    #[inline(always)]
    #[must_use]
    pub fn scmod(&mut self) -> ScmodW<CtlSpec> {
        ScmodW::new(self, 7)
    }
    #[doc = "Bits 8:12 - Current precision trim"]
    #[inline(always)]
    #[must_use]
    pub fn cpt(&mut self) -> CptW<CtlSpec> {
        CptW::new(self, 8)
    }
    #[doc = "Bit 14 - Step selection"]
    #[inline(always)]
    #[must_use]
    pub fn ssel(&mut self) -> SselW<CtlSpec> {
        SselW::new(self, 14)
    }
    #[doc = "Bit 15 - Current reference enable"]
    #[inline(always)]
    #[must_use]
    pub fn cren(&mut self) -> CrenW<CtlSpec> {
        CrenW::new(self, 15)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0x0f00"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0x0f00;
}
