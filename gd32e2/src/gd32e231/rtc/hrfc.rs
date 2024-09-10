#[doc = "Register `HRFC` reader"]
pub type R = crate::R<HrfcSpec>;
#[doc = "Register `HRFC` writer"]
pub type W = crate::W<HrfcSpec>;
#[doc = "Field `CMSK` reader - Calibration mask number"]
pub type CmskR = crate::FieldReader<u16>;
#[doc = "Field `CMSK` writer - Calibration mask number"]
pub type CmskW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `CWND16` reader - Frequency compensation window 16 second selected"]
pub type Cwnd16R = crate::BitReader;
#[doc = "Field `CWND16` writer - Frequency compensation window 16 second selected"]
pub type Cwnd16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWND8` reader - Frequency compensation window 8 second selected"]
pub type Cwnd8R = crate::BitReader;
#[doc = "Field `CWND8` writer - Frequency compensation window 8 second selected"]
pub type Cwnd8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREQI` reader - Increase RTC frequency by 488.5PPM"]
pub type FreqiR = crate::BitReader;
#[doc = "Field `FREQI` writer - Increase RTC frequency by 488.5PPM"]
pub type FreqiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Calibration mask number"]
    #[inline(always)]
    pub fn cmsk(&self) -> CmskR {
        CmskR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 13 - Frequency compensation window 16 second selected"]
    #[inline(always)]
    pub fn cwnd16(&self) -> Cwnd16R {
        Cwnd16R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Frequency compensation window 8 second selected"]
    #[inline(always)]
    pub fn cwnd8(&self) -> Cwnd8R {
        Cwnd8R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Increase RTC frequency by 488.5PPM"]
    #[inline(always)]
    pub fn freqi(&self) -> FreqiR {
        FreqiR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Calibration mask number"]
    #[inline(always)]
    #[must_use]
    pub fn cmsk(&mut self) -> CmskW<HrfcSpec> {
        CmskW::new(self, 0)
    }
    #[doc = "Bit 13 - Frequency compensation window 16 second selected"]
    #[inline(always)]
    #[must_use]
    pub fn cwnd16(&mut self) -> Cwnd16W<HrfcSpec> {
        Cwnd16W::new(self, 13)
    }
    #[doc = "Bit 14 - Frequency compensation window 8 second selected"]
    #[inline(always)]
    #[must_use]
    pub fn cwnd8(&mut self) -> Cwnd8W<HrfcSpec> {
        Cwnd8W::new(self, 14)
    }
    #[doc = "Bit 15 - Increase RTC frequency by 488.5PPM"]
    #[inline(always)]
    #[must_use]
    pub fn freqi(&mut self) -> FreqiW<HrfcSpec> {
        FreqiW::new(self, 15)
    }
}
#[doc = "High resolution frequency compensation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HrfcSpec;
impl crate::RegisterSpec for HrfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hrfc::R`](R) reader structure"]
impl crate::Readable for HrfcSpec {}
#[doc = "`write(|w| ..)` method takes [`hrfc::W`](W) writer structure"]
impl crate::Writable for HrfcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HRFC to value 0"]
impl crate::Resettable for HrfcSpec {
    const RESET_VALUE: u32 = 0;
}
