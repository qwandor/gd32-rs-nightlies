#[doc = "Register `FCTL` reader"]
pub type R = crate::R<FctlSpec>;
#[doc = "Register `FCTL` writer"]
pub type W = crate::W<FctlSpec>;
#[doc = "Field `DF` reader - Digital noise filter"]
pub type DfR = crate::FieldReader;
#[doc = "Field `DF` writer - Digital noise filter"]
pub type DfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFD` reader - Analog noise filter disable"]
pub type AfdR = crate::BitReader;
#[doc = "Field `AFD` writer - Analog noise filter disable"]
pub type AfdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Digital noise filter"]
    #[inline(always)]
    pub fn df(&self) -> DfR {
        DfR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Analog noise filter disable"]
    #[inline(always)]
    pub fn afd(&self) -> AfdR {
        AfdR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Digital noise filter"]
    #[inline(always)]
    #[must_use]
    pub fn df(&mut self) -> DfW<FctlSpec> {
        DfW::new(self, 0)
    }
    #[doc = "Bit 4 - Analog noise filter disable"]
    #[inline(always)]
    #[must_use]
    pub fn afd(&mut self) -> AfdW<FctlSpec> {
        AfdW::new(self, 4)
    }
}
#[doc = "Filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FctlSpec;
impl crate::RegisterSpec for FctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fctl::R`](R) reader structure"]
impl crate::Readable for FctlSpec {}
#[doc = "`write(|w| ..)` method takes [`fctl::W`](W) writer structure"]
impl crate::Writable for FctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCTL to value 0"]
impl crate::Resettable for FctlSpec {
    const RESET_VALUE: u32 = 0;
}
