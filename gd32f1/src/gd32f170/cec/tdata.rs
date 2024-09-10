#[doc = "Register `TDATA` writer"]
pub type W = crate::W<TdataSpec>;
#[doc = "Field `TXDATA` writer - Tx Data register"]
pub type TxdataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Tx Data register"]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TxdataW<TdataSpec> {
        TxdataW::new(self, 0)
    }
}
#[doc = "Transmit data register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdataSpec;
impl crate::RegisterSpec for TdataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tdata::W`](W) writer structure"]
impl crate::Writable for TdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TDATA to value 0"]
impl crate::Resettable for TdataSpec {
    const RESET_VALUE: u32 = 0;
}
