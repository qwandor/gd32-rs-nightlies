#[doc = "Register `DAC0_R8DH` reader"]
pub type R = crate::R<Dac0R8dhSpec>;
#[doc = "Register `DAC0_R8DH` writer"]
pub type W = crate::W<Dac0R8dhSpec>;
#[doc = "Field `DAC0_DH` reader - DAC0 8-bit right-aligned data"]
pub type Dac0DhR = crate::FieldReader;
#[doc = "Field `DAC0_DH` writer - DAC0 8-bit right-aligned data"]
pub type Dac0DhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DAC0 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dac0_dh(&self) -> Dac0DhR {
        Dac0DhR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC0 8-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dac0_dh(&mut self) -> Dac0DhW<Dac0R8dhSpec> {
        Dac0DhW::new(self, 0)
    }
}
#[doc = "DAC0 8-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0_r8dh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0_r8dh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dac0R8dhSpec;
impl crate::RegisterSpec for Dac0R8dhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac0_r8dh::R`](R) reader structure"]
impl crate::Readable for Dac0R8dhSpec {}
#[doc = "`write(|w| ..)` method takes [`dac0_r8dh::W`](W) writer structure"]
impl crate::Writable for Dac0R8dhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC0_R8DH to value 0"]
impl crate::Resettable for Dac0R8dhSpec {
    const RESET_VALUE: u32 = 0;
}
