#[doc = "Register `DATE` reader"]
pub type R = crate::R<DateSpec>;
#[doc = "Register `DATE` writer"]
pub type W = crate::W<DateSpec>;
#[doc = "Field `DAYU` reader - Date units in BCD format"]
pub type DayuR = crate::FieldReader;
#[doc = "Field `DAYU` writer - Date units in BCD format"]
pub type DayuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DAYT` reader - Date tens in BCD format"]
pub type DaytR = crate::FieldReader;
#[doc = "Field `DAYT` writer - Date tens in BCD format"]
pub type DaytW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MONU` reader - Month units in BCD format"]
pub type MonuR = crate::FieldReader;
#[doc = "Field `MONU` writer - Month units in BCD format"]
pub type MonuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MONT` reader - Month tens in BCD format"]
pub type MontR = crate::BitReader;
#[doc = "Field `MONT` writer - Month tens in BCD format"]
pub type MontW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOW` reader - Week day units"]
pub type DowR = crate::FieldReader;
#[doc = "Field `DOW` writer - Week day units"]
pub type DowW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `YRU` reader - Year units in BCD format"]
pub type YruR = crate::FieldReader;
#[doc = "Field `YRU` writer - Year units in BCD format"]
pub type YruW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `YRT` reader - Year tens in BCD format"]
pub type YrtR = crate::FieldReader;
#[doc = "Field `YRT` writer - Year tens in BCD format"]
pub type YrtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
    #[doc = "Bits 16:19 - Year units in BCD format"]
    #[inline(always)]
    pub fn yru(&self) -> YruR {
        YruR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Year tens in BCD format"]
    #[inline(always)]
    pub fn yrt(&self) -> YrtR {
        YrtR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Date units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn dayu(&mut self) -> DayuW<DateSpec> {
        DayuW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Date tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn dayt(&mut self) -> DaytW<DateSpec> {
        DaytW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Month units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn monu(&mut self) -> MonuW<DateSpec> {
        MonuW::new(self, 8)
    }
    #[doc = "Bit 12 - Month tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn mont(&mut self) -> MontW<DateSpec> {
        MontW::new(self, 12)
    }
    #[doc = "Bits 13:15 - Week day units"]
    #[inline(always)]
    #[must_use]
    pub fn dow(&mut self) -> DowW<DateSpec> {
        DowW::new(self, 13)
    }
    #[doc = "Bits 16:19 - Year units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn yru(&mut self) -> YruW<DateSpec> {
        YruW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Year tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn yrt(&mut self) -> YrtW<DateSpec> {
        YrtW::new(self, 20)
    }
}
#[doc = "date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DateSpec;
impl crate::RegisterSpec for DateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DateSpec {}
#[doc = "`write(|w| ..)` method takes [`date::W`](W) writer structure"]
impl crate::Writable for DateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATE to value 0x2101"]
impl crate::Resettable for DateSpec {
    const RESET_VALUE: u32 = 0x2101;
}
