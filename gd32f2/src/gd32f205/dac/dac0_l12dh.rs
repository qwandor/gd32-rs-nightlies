#[doc = "Register `DAC0_L12DH` reader"]
pub type R = crate::R<Dac0L12dhSpec>;
#[doc = "Register `DAC0_L12DH` writer"]
pub type W = crate::W<Dac0L12dhSpec>;
#[doc = "Field `DAC0_DH` reader - DAC0 12-bit left-aligned data"]
pub type Dac0DhR = crate::FieldReader<u16>;
#[doc = "Field `DAC0_DH` writer - DAC0 12-bit left-aligned data"]
pub type Dac0DhW<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 4:15 - DAC0 12-bit left-aligned data"]
    #[inline(always)]
    pub fn dac0_dh(&self) -> Dac0DhR {
        Dac0DhR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15 - DAC0 12-bit left-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dac0_dh(&mut self) -> Dac0DhW<Dac0L12dhSpec> {
        Dac0DhW::new(self, 4)
    }
}
#[doc = "DAC0 12-bit left-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0_l12dh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0_l12dh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dac0L12dhSpec;
impl crate::RegisterSpec for Dac0L12dhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac0_l12dh::R`](R) reader structure"]
impl crate::Readable for Dac0L12dhSpec {}
#[doc = "`write(|w| ..)` method takes [`dac0_l12dh::W`](W) writer structure"]
impl crate::Writable for Dac0L12dhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC0_L12DH to value 0"]
impl crate::Resettable for Dac0L12dhSpec {
    const RESET_VALUE: u32 = 0;
}
