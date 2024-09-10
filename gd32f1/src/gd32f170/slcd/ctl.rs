#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `SLCDON` reader - SLCD controller start"]
pub type SlcdonR = crate::BitReader;
#[doc = "Field `SLCDON` writer - SLCD controller start"]
pub type SlcdonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSRC` reader - SLCD voltage source"]
pub type VsrcR = crate::BitReader;
#[doc = "Field `VSRC` writer - SLCD voltage source"]
pub type VsrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY` reader - Duty select"]
pub type DutyR = crate::FieldReader;
#[doc = "Field `DUTY` writer - Duty select"]
pub type DutyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIAS` reader - Bias select"]
pub type BiasR = crate::FieldReader;
#[doc = "Field `BIAS` writer - Bias select"]
pub type BiasW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMS` reader - Common/segment padselect"]
pub type ComsR = crate::BitReader;
#[doc = "Field `COMS` writer - Common/segment padselect"]
pub type ComsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SLCD controller start"]
    #[inline(always)]
    pub fn slcdon(&self) -> SlcdonR {
        SlcdonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SLCD voltage source"]
    #[inline(always)]
    pub fn vsrc(&self) -> VsrcR {
        VsrcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Duty select"]
    #[inline(always)]
    pub fn duty(&self) -> DutyR {
        DutyR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:6 - Bias select"]
    #[inline(always)]
    pub fn bias(&self) -> BiasR {
        BiasR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Common/segment padselect"]
    #[inline(always)]
    pub fn coms(&self) -> ComsR {
        ComsR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SLCD controller start"]
    #[inline(always)]
    #[must_use]
    pub fn slcdon(&mut self) -> SlcdonW<CtlSpec> {
        SlcdonW::new(self, 0)
    }
    #[doc = "Bit 1 - SLCD voltage source"]
    #[inline(always)]
    #[must_use]
    pub fn vsrc(&mut self) -> VsrcW<CtlSpec> {
        VsrcW::new(self, 1)
    }
    #[doc = "Bits 2:4 - Duty select"]
    #[inline(always)]
    #[must_use]
    pub fn duty(&mut self) -> DutyW<CtlSpec> {
        DutyW::new(self, 2)
    }
    #[doc = "Bits 5:6 - Bias select"]
    #[inline(always)]
    #[must_use]
    pub fn bias(&mut self) -> BiasW<CtlSpec> {
        BiasW::new(self, 5)
    }
    #[doc = "Bit 7 - Common/segment padselect"]
    #[inline(always)]
    #[must_use]
    pub fn coms(&mut self) -> ComsW<CtlSpec> {
        ComsW::new(self, 7)
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
