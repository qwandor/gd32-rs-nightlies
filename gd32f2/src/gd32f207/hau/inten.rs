#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `DIIE` reader - Data input interrupt enable"]
pub type DiieR = crate::BitReader;
#[doc = "Field `DIIE` writer - Data input interrupt enable"]
pub type DiieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCIE` reader - calculation completion interrupt enable"]
pub type CcieR = crate::BitReader;
#[doc = "Field `CCIE` writer - calculation completion interrupt enable"]
pub type CcieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data input interrupt enable"]
    #[inline(always)]
    pub fn diie(&self) -> DiieR {
        DiieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - calculation completion interrupt enable"]
    #[inline(always)]
    pub fn ccie(&self) -> CcieR {
        CcieR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data input interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn diie(&mut self) -> DiieW<IntenSpec> {
        DiieW::new(self, 0)
    }
    #[doc = "Bit 1 - calculation completion interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccie(&mut self) -> CcieW<IntenSpec> {
        CcieW::new(self, 1)
    }
}
#[doc = "HAU interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
