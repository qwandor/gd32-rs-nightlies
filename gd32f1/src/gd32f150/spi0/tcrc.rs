#[doc = "Register `TCRC` reader"]
pub type R = crate::R<TcrcSpec>;
#[doc = "Field `TCRC` reader - CRC value of the transmitted bytes"]
pub type TcrcR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - CRC value of the transmitted bytes"]
    #[inline(always)]
    pub fn tcrc(&self) -> TcrcR {
        TcrcR::new(self.bits)
    }
}
#[doc = "Transmit CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcrc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcrcSpec;
impl crate::RegisterSpec for TcrcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tcrc::R`](R) reader structure"]
impl crate::Readable for TcrcSpec {}
#[doc = "`reset()` method sets TCRC to value 0"]
impl crate::Resettable for TcrcSpec {
    const RESET_VALUE: u16 = 0;
}
