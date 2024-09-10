#[doc = "Register `DTS` reader"]
pub type R = crate::R<DtsSpec>;
#[doc = "Field `DAYU` reader - Date units in BCD format"]
pub type DayuR = crate::FieldReader;
#[doc = "Field `DAYT` reader - Date tens in BCD format"]
pub type DaytR = crate::FieldReader;
#[doc = "Field `MONU` reader - Month units in BCD format"]
pub type MonuR = crate::FieldReader;
#[doc = "Field `MONT` reader - Month tens in BCD format"]
pub type MontR = crate::BitReader;
#[doc = "Field `DOW` reader - Week day units"]
pub type DowR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Date units in BCD format"]
    #[inline(always)]
    pub fn dayu(&self) -> DayuR {
        DayuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Date tens in BCD format"]
    #[inline(always)]
    pub fn dayt(&self) -> DaytR {
        DaytR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Month units in BCD format"]
    #[inline(always)]
    pub fn monu(&self) -> MonuR {
        MonuR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Month tens in BCD format"]
    #[inline(always)]
    pub fn mont(&self) -> MontR {
        MontR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Week day units"]
    #[inline(always)]
    pub fn dow(&self) -> DowR {
        DowR::new(((self.bits >> 13) & 7) as u8)
    }
}
#[doc = "Date of time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtsSpec;
impl crate::RegisterSpec for DtsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dts::R`](R) reader structure"]
impl crate::Readable for DtsSpec {}
#[doc = "`reset()` method sets DTS to value 0"]
impl crate::Resettable for DtsSpec {
    const RESET_VALUE: u32 = 0;
}
