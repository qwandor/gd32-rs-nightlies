#[doc = "Register `IPA_DPCTL` reader"]
pub type R = crate::R<IpaDpctlSpec>;
#[doc = "Register `IPA_DPCTL` writer"]
pub type W = crate::W<IpaDpctlSpec>;
#[doc = "Field `DPF` reader - Destination pixel format"]
pub type DpfR = crate::FieldReader;
#[doc = "Field `DPF` writer - Destination pixel format"]
pub type DpfW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Destination pixel format"]
    #[inline(always)]
    pub fn dpf(&self) -> DpfR {
        DpfR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Destination pixel format"]
    #[inline(always)]
    #[must_use]
    pub fn dpf(&mut self) -> DpfW<IpaDpctlSpec> {
        DpfW::new(self, 0)
    }
}
#[doc = "Destination pixel control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_dpctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_dpctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpaDpctlSpec;
impl crate::RegisterSpec for IpaDpctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipa_dpctl::R`](R) reader structure"]
impl crate::Readable for IpaDpctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ipa_dpctl::W`](W) writer structure"]
impl crate::Writable for IpaDpctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPA_DPCTL to value 0"]
impl crate::Resettable for IpaDpctlSpec {
    const RESET_VALUE: u32 = 0;
}
