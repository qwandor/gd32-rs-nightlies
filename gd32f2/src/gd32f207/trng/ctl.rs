#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `TRNGEN` reader - TRNG enable bit"]
pub type TrngenR = crate::BitReader;
#[doc = "Field `TRNGEN` writer - TRNG enable bit"]
pub type TrngenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE` reader - Interrupt bit"]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - Interrupt bit"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - TRNG enable bit"]
    #[inline(always)]
    pub fn trngen(&self) -> TrngenR {
        TrngenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt bit"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - TRNG enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn trngen(&mut self) -> TrngenW<CtlSpec> {
        TrngenW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IeW<CtlSpec> {
        IeW::new(self, 3)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
