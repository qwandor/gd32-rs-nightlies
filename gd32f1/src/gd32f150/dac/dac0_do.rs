#[doc = "Register `DAC0_DO` reader"]
pub type R = crate::R<Dac0DoSpec>;
#[doc = "Field `DAC0_DO` reader - DAC0 output data"]
pub type Dac0DoR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - DAC0 output data"]
    #[inline(always)]
    pub fn dac0_do(&self) -> Dac0DoR {
        Dac0DoR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DAC0 data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0_do::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dac0DoSpec;
impl crate::RegisterSpec for Dac0DoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac0_do::R`](R) reader structure"]
impl crate::Readable for Dac0DoSpec {}
#[doc = "`reset()` method sets DAC0_DO to value 0"]
impl crate::Resettable for Dac0DoSpec {
    const RESET_VALUE: u32 = 0;
}
