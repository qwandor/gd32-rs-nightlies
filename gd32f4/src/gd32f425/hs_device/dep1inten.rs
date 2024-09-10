#[doc = "Register `DEP1INTEN` reader"]
pub type R = crate::R<Dep1intenSpec>;
#[doc = "Register `DEP1INTEN` writer"]
pub type W = crate::W<Dep1intenSpec>;
#[doc = "Field `IEP1INTEN` reader - IN endpoint 1 interrupt enable bits"]
pub type Iep1intenR = crate::BitReader;
#[doc = "Field `IEP1INTEN` writer - IN endpoint 1 interrupt enable bits"]
pub type Iep1intenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEP1INTEN` reader - OUT endpoint 1 interrupt enable bits"]
pub type Oep1intenR = crate::BitReader;
#[doc = "Field `OEP1INTEN` writer - OUT endpoint 1 interrupt enable bits"]
pub type Oep1intenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - IN endpoint 1 interrupt enable bits"]
    #[inline(always)]
    pub fn iep1inten(&self) -> Iep1intenR {
        Iep1intenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 17 - OUT endpoint 1 interrupt enable bits"]
    #[inline(always)]
    pub fn oep1inten(&self) -> Oep1intenR {
        Oep1intenR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - IN endpoint 1 interrupt enable bits"]
    #[inline(always)]
    #[must_use]
    pub fn iep1inten(&mut self) -> Iep1intenW<Dep1intenSpec> {
        Iep1intenW::new(self, 1)
    }
    #[doc = "Bit 17 - OUT endpoint 1 interrupt enable bits"]
    #[inline(always)]
    #[must_use]
    pub fn oep1inten(&mut self) -> Oep1intenW<Dep1intenSpec> {
        Oep1intenW::new(self, 17)
    }
}
#[doc = "device endpoint 1 interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dep1inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dep1inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dep1intenSpec;
impl crate::RegisterSpec for Dep1intenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dep1inten::R`](R) reader structure"]
impl crate::Readable for Dep1intenSpec {}
#[doc = "`write(|w| ..)` method takes [`dep1inten::W`](W) writer structure"]
impl crate::Writable for Dep1intenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEP1INTEN to value 0"]
impl crate::Resettable for Dep1intenSpec {
    const RESET_VALUE: u32 = 0;
}
