#[doc = "Register `DEP1INT` reader"]
pub type R = crate::R<Dep1intSpec>;
#[doc = "Register `DEP1INT` writer"]
pub type W = crate::W<Dep1intSpec>;
#[doc = "Field `IEP1INT` reader - IN endpoint 1 interrupt bits"]
pub type Iep1intR = crate::BitReader;
#[doc = "Field `IEP1INT` writer - IN endpoint 1 interrupt bits"]
pub type Iep1intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEP1INT` reader - OUT endpoint 1 interrupt bits"]
pub type Oep1intR = crate::BitReader;
#[doc = "Field `OEP1INT` writer - OUT endpoint 1 interrupt bits"]
pub type Oep1intW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - IN endpoint 1 interrupt bits"]
    #[inline(always)]
    pub fn iep1int(&self) -> Iep1intR {
        Iep1intR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 17 - OUT endpoint 1 interrupt bits"]
    #[inline(always)]
    pub fn oep1int(&self) -> Oep1intR {
        Oep1intR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - IN endpoint 1 interrupt bits"]
    #[inline(always)]
    #[must_use]
    pub fn iep1int(&mut self) -> Iep1intW<Dep1intSpec> {
        Iep1intW::new(self, 1)
    }
    #[doc = "Bit 17 - OUT endpoint 1 interrupt bits"]
    #[inline(always)]
    #[must_use]
    pub fn oep1int(&mut self) -> Oep1intW<Dep1intSpec> {
        Oep1intW::new(self, 17)
    }
}
#[doc = "device endpoint 1 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dep1int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dep1int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dep1intSpec;
impl crate::RegisterSpec for Dep1intSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dep1int::R`](R) reader structure"]
impl crate::Readable for Dep1intSpec {}
#[doc = "`write(|w| ..)` method takes [`dep1int::W`](W) writer structure"]
impl crate::Writable for Dep1intSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEP1INT to value 0"]
impl crate::Resettable for Dep1intSpec {
    const RESET_VALUE: u32 = 0;
}
