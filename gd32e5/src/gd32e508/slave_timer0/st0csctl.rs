#[doc = "Register `ST0CSCTL` reader"]
pub type R = crate::R<St0csctlSpec>;
#[doc = "Register `ST0CSCTL` writer"]
pub type W = crate::W<St0csctlSpec>;
#[doc = "Field `CSPRD` reader - Carrier signal period"]
pub type CsprdR = crate::FieldReader;
#[doc = "Field `CSPRD` writer - Carrier signal period"]
pub type CsprdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CSDTY` reader - Carrier signal duty cycle"]
pub type CsdtyR = crate::FieldReader;
#[doc = "Field `CSDTY` writer - Carrier signal duty cycle"]
pub type CsdtyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CSFSTPW` reader - First carrier-signal pulse width"]
pub type CsfstpwR = crate::FieldReader;
#[doc = "Field `CSFSTPW` writer - First carrier-signal pulse width"]
pub type CsfstpwW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Carrier signal period"]
    #[inline(always)]
    pub fn csprd(&self) -> CsprdR {
        CsprdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Carrier signal duty cycle"]
    #[inline(always)]
    pub fn csdty(&self) -> CsdtyR {
        CsdtyR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:10 - First carrier-signal pulse width"]
    #[inline(always)]
    pub fn csfstpw(&self) -> CsfstpwR {
        CsfstpwR::new(((self.bits >> 7) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Carrier signal period"]
    #[inline(always)]
    #[must_use]
    pub fn csprd(&mut self) -> CsprdW<St0csctlSpec> {
        CsprdW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Carrier signal duty cycle"]
    #[inline(always)]
    #[must_use]
    pub fn csdty(&mut self) -> CsdtyW<St0csctlSpec> {
        CsdtyW::new(self, 4)
    }
    #[doc = "Bits 7:10 - First carrier-signal pulse width"]
    #[inline(always)]
    #[must_use]
    pub fn csfstpw(&mut self) -> CsfstpwW<St0csctlSpec> {
        CsfstpwW::new(self, 7)
    }
}
#[doc = "SHRTIMER Slave_TIMERx carrier-signal control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0csctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0csctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St0csctlSpec;
impl crate::RegisterSpec for St0csctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st0csctl::R`](R) reader structure"]
impl crate::Readable for St0csctlSpec {}
#[doc = "`write(|w| ..)` method takes [`st0csctl::W`](W) writer structure"]
impl crate::Writable for St0csctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ST0CSCTL to value 0"]
impl crate::Resettable for St0csctlSpec {
    const RESET_VALUE: u32 = 0;
}
