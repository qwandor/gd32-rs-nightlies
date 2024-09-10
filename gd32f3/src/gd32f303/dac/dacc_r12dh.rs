#[doc = "Register `DACC_R12DH` reader"]
pub type R = crate::R<DaccR12dhSpec>;
#[doc = "Register `DACC_R12DH` writer"]
pub type W = crate::W<DaccR12dhSpec>;
#[doc = "Field `DAC0_DH` reader - DAC0 12-bit right-aligned data"]
pub type Dac0DhR = crate::FieldReader<u16>;
#[doc = "Field `DAC0_DH` writer - DAC0 12-bit right-aligned data"]
pub type Dac0DhW<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
#[doc = "Field `DAC1_DH` reader - DAC1 12-bit right-aligned data"]
pub type Dac1DhR = crate::FieldReader<u16>;
#[doc = "Field `DAC1_DH` writer - DAC1 12-bit right-aligned data"]
pub type Dac1DhW<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC0 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dac0_dh(&self) -> Dac0DhR {
        Dac0DhR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - DAC1 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dac1_dh(&self) -> Dac1DhR {
        Dac1DhR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC0 12-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dac0_dh(&mut self) -> Dac0DhW<DaccR12dhSpec> {
        Dac0DhW::new(self, 0)
    }
    #[doc = "Bits 16:27 - DAC1 12-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dac1_dh(&mut self) -> Dac1DhW<DaccR12dhSpec> {
        Dac1DhW::new(self, 16)
    }
}
#[doc = "DAC concurrent mode 12-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacc_r12dh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacc_r12dh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaccR12dhSpec;
impl crate::RegisterSpec for DaccR12dhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dacc_r12dh::R`](R) reader structure"]
impl crate::Readable for DaccR12dhSpec {}
#[doc = "`write(|w| ..)` method takes [`dacc_r12dh::W`](W) writer structure"]
impl crate::Writable for DaccR12dhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DACC_R12DH to value 0"]
impl crate::Resettable for DaccR12dhSpec {
    const RESET_VALUE: u32 = 0;
}
