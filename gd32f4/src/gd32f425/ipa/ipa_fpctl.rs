#[doc = "Register `IPA_FPCTL` reader"]
pub type R = crate::R<IpaFpctlSpec>;
#[doc = "Register `IPA_FPCTL` writer"]
pub type W = crate::W<IpaFpctlSpec>;
#[doc = "Field `FPF` reader - Foreground pixel format"]
pub type FpfR = crate::FieldReader;
#[doc = "Field `FPF` writer - Foreground pixel format"]
pub type FpfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FLPF` reader - Foreground LUT pixel format"]
pub type FlpfR = crate::BitReader;
#[doc = "Field `FLPF` writer - Foreground LUT pixel format"]
pub type FlpfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLLEN` reader - Foreground LUT loading enable"]
pub type FllenR = crate::BitReader;
#[doc = "Field `FLLEN` writer - Foreground LUT loading enable"]
pub type FllenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCNP` reader - Foreground LUT number of pixel"]
pub type FcnpR = crate::FieldReader;
#[doc = "Field `FCNP` writer - Foreground LUT number of pixel"]
pub type FcnpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FAVCA` reader - Foreground alpha value calculation algorithm"]
pub type FavcaR = crate::FieldReader;
#[doc = "Field `FAVCA` writer - Foreground alpha value calculation algorithm"]
pub type FavcaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FPDAV` reader - Foreground pre- defined alpha value"]
pub type FpdavR = crate::FieldReader;
#[doc = "Field `FPDAV` writer - Foreground pre- defined alpha value"]
pub type FpdavW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - Foreground pixel format"]
    #[inline(always)]
    pub fn fpf(&self) -> FpfR {
        FpfR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Foreground LUT pixel format"]
    #[inline(always)]
    pub fn flpf(&self) -> FlpfR {
        FlpfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Foreground LUT loading enable"]
    #[inline(always)]
    pub fn fllen(&self) -> FllenR {
        FllenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Foreground LUT number of pixel"]
    #[inline(always)]
    pub fn fcnp(&self) -> FcnpR {
        FcnpR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Foreground alpha value calculation algorithm"]
    #[inline(always)]
    pub fn favca(&self) -> FavcaR {
        FavcaR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:31 - Foreground pre- defined alpha value"]
    #[inline(always)]
    pub fn fpdav(&self) -> FpdavR {
        FpdavR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Foreground pixel format"]
    #[inline(always)]
    #[must_use]
    pub fn fpf(&mut self) -> FpfW<IpaFpctlSpec> {
        FpfW::new(self, 0)
    }
    #[doc = "Bit 4 - Foreground LUT pixel format"]
    #[inline(always)]
    #[must_use]
    pub fn flpf(&mut self) -> FlpfW<IpaFpctlSpec> {
        FlpfW::new(self, 4)
    }
    #[doc = "Bit 5 - Foreground LUT loading enable"]
    #[inline(always)]
    #[must_use]
    pub fn fllen(&mut self) -> FllenW<IpaFpctlSpec> {
        FllenW::new(self, 5)
    }
    #[doc = "Bits 8:15 - Foreground LUT number of pixel"]
    #[inline(always)]
    #[must_use]
    pub fn fcnp(&mut self) -> FcnpW<IpaFpctlSpec> {
        FcnpW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Foreground alpha value calculation algorithm"]
    #[inline(always)]
    #[must_use]
    pub fn favca(&mut self) -> FavcaW<IpaFpctlSpec> {
        FavcaW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Foreground pre- defined alpha value"]
    #[inline(always)]
    #[must_use]
    pub fn fpdav(&mut self) -> FpdavW<IpaFpctlSpec> {
        FpdavW::new(self, 24)
    }
}
#[doc = "Foreground pixel control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_fpctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_fpctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpaFpctlSpec;
impl crate::RegisterSpec for IpaFpctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipa_fpctl::R`](R) reader structure"]
impl crate::Readable for IpaFpctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ipa_fpctl::W`](W) writer structure"]
impl crate::Writable for IpaFpctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPA_FPCTL to value 0"]
impl crate::Resettable for IpaFpctlSpec {
    const RESET_VALUE: u32 = 0;
}
