#[doc = "Register `DAC1_DO` reader"]
pub type R = crate::R<Dac1DoSpec>;
#[doc = "Field `DAC1_DO` reader - DAC1 data output"]
pub type Dac1DoR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - DAC1 data output"]
    #[inline(always)]
    pub fn dac1_do(&self) -> Dac1DoR {
        Dac1DoR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DAC1 data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1_do::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dac1DoSpec;
impl crate::RegisterSpec for Dac1DoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac1_do::R`](R) reader structure"]
impl crate::Readable for Dac1DoSpec {}
#[doc = "`reset()` method sets DAC1_DO to value 0"]
impl crate::Resettable for Dac1DoSpec {
    const RESET_VALUE: u32 = 0;
}
