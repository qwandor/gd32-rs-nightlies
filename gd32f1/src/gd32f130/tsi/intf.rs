#[doc = "Register `INTF` reader"]
pub type R = crate::R<IntfSpec>;
#[doc = "Register `INTF` writer"]
pub type W = crate::W<IntfSpec>;
#[doc = "Field `CTCF` reader - Charge-Transfer complete flag"]
pub type CtcfR = crate::BitReader;
#[doc = "Field `CTCF` writer - Charge-Transfer complete flag"]
pub type CtcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MNERR` reader - Max count error flag"]
pub type MnerrR = crate::BitReader;
#[doc = "Field `MNERR` writer - Max count error flag"]
pub type MnerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Charge-Transfer complete flag"]
    #[inline(always)]
    pub fn ctcf(&self) -> CtcfR {
        CtcfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Max count error flag"]
    #[inline(always)]
    pub fn mnerr(&self) -> MnerrR {
        MnerrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Charge-Transfer complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctcf(&mut self) -> CtcfW<IntfSpec> {
        CtcfW::new(self, 0)
    }
    #[doc = "Bit 1 - Max count error flag"]
    #[inline(always)]
    #[must_use]
    pub fn mnerr(&mut self) -> MnerrW<IntfSpec> {
        MnerrW::new(self, 1)
    }
}
#[doc = "interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfSpec;
impl crate::RegisterSpec for IntfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf::R`](R) reader structure"]
impl crate::Readable for IntfSpec {}
#[doc = "`write(|w| ..)` method takes [`intf::W`](W) writer structure"]
impl crate::Writable for IntfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for IntfSpec {
    const RESET_VALUE: u32 = 0;
}
