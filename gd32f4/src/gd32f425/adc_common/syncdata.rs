#[doc = "Register `SYNCDATA` reader"]
pub type R = crate::R<SyncdataSpec>;
#[doc = "Field `SYNCDATA1` reader - Regular data1 in ADC sync mode"]
pub type Syncdata1R = crate::FieldReader<u16>;
#[doc = "Field `SYNCDATA2` reader - Regular data2 in ADC sync mode"]
pub type Syncdata2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Regular data1 in ADC sync mode"]
    #[inline(always)]
    pub fn syncdata1(&self) -> Syncdata1R {
        Syncdata1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Regular data2 in ADC sync mode"]
    #[inline(always)]
    pub fn syncdata2(&self) -> Syncdata2R {
        Syncdata2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Sync regular data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncdata::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncdataSpec;
impl crate::RegisterSpec for SyncdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncdata::R`](R) reader structure"]
impl crate::Readable for SyncdataSpec {}
#[doc = "`reset()` method sets SYNCDATA to value 0"]
impl crate::Resettable for SyncdataSpec {
    const RESET_VALUE: u32 = 0;
}
