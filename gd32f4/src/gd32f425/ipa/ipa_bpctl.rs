#[doc = "Register `IPA_BPCTL` reader"]
pub type R = crate::R<IpaBpctlSpec>;
#[doc = "Register `IPA_BPCTL` writer"]
pub type W = crate::W<IpaBpctlSpec>;
#[doc = "Field `BPF` reader - Background pixel format"]
pub type BpfR = crate::FieldReader;
#[doc = "Field `BPF` writer - Background pixel format"]
pub type BpfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BLPF` reader - Background LUT pixel format"]
pub type BlpfR = crate::BitReader;
#[doc = "Field `BLPF` writer - Background LUT pixel format"]
pub type BlpfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLLEN` reader - Background LUT loading enable"]
pub type BllenR = crate::BitReader;
#[doc = "Field `BLLEN` writer - Background LUT loading enable"]
pub type BllenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCNP` reader - Background LUT number of pixel"]
pub type BcnpR = crate::FieldReader;
#[doc = "Field `BCNP` writer - Background LUT number of pixel"]
pub type BcnpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BAVCA` reader - Background alpha value calculation algorithm"]
pub type BavcaR = crate::FieldReader;
#[doc = "Field `BAVCA` writer - Background alpha value calculation algorithm"]
pub type BavcaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BPDAV` reader - Background pre- defined alpha value"]
pub type BpdavR = crate::FieldReader;
#[doc = "Field `BPDAV` writer - Background pre- defined alpha value"]
pub type BpdavW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - Background pixel format"]
    #[inline(always)]
    pub fn bpf(&self) -> BpfR {
        BpfR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Background LUT pixel format"]
    #[inline(always)]
    pub fn blpf(&self) -> BlpfR {
        BlpfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Background LUT loading enable"]
    #[inline(always)]
    pub fn bllen(&self) -> BllenR {
        BllenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Background LUT number of pixel"]
    #[inline(always)]
    pub fn bcnp(&self) -> BcnpR {
        BcnpR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Background alpha value calculation algorithm"]
    #[inline(always)]
    pub fn bavca(&self) -> BavcaR {
        BavcaR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:31 - Background pre- defined alpha value"]
    #[inline(always)]
    pub fn bpdav(&self) -> BpdavR {
        BpdavR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Background pixel format"]
    #[inline(always)]
    #[must_use]
    pub fn bpf(&mut self) -> BpfW<IpaBpctlSpec> {
        BpfW::new(self, 0)
    }
    #[doc = "Bit 4 - Background LUT pixel format"]
    #[inline(always)]
    #[must_use]
    pub fn blpf(&mut self) -> BlpfW<IpaBpctlSpec> {
        BlpfW::new(self, 4)
    }
    #[doc = "Bit 5 - Background LUT loading enable"]
    #[inline(always)]
    #[must_use]
    pub fn bllen(&mut self) -> BllenW<IpaBpctlSpec> {
        BllenW::new(self, 5)
    }
    #[doc = "Bits 8:15 - Background LUT number of pixel"]
    #[inline(always)]
    #[must_use]
    pub fn bcnp(&mut self) -> BcnpW<IpaBpctlSpec> {
        BcnpW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Background alpha value calculation algorithm"]
    #[inline(always)]
    #[must_use]
    pub fn bavca(&mut self) -> BavcaW<IpaBpctlSpec> {
        BavcaW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Background pre- defined alpha value"]
    #[inline(always)]
    #[must_use]
    pub fn bpdav(&mut self) -> BpdavW<IpaBpctlSpec> {
        BpdavW::new(self, 24)
    }
}
#[doc = "Background pixel control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_bpctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_bpctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpaBpctlSpec;
impl crate::RegisterSpec for IpaBpctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipa_bpctl::R`](R) reader structure"]
impl crate::Readable for IpaBpctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ipa_bpctl::W`](W) writer structure"]
impl crate::Writable for IpaBpctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPA_BPCTL to value 0"]
impl crate::Resettable for IpaBpctlSpec {
    const RESET_VALUE: u32 = 0;
}
