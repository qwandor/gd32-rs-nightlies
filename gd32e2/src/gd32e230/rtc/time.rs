#[doc = "Register `TIME` reader"]
pub type R = crate::R<TimeSpec>;
#[doc = "Register `TIME` writer"]
pub type W = crate::W<TimeSpec>;
#[doc = "Field `SCU` reader - Second units in BCD code"]
pub type ScuR = crate::FieldReader;
#[doc = "Field `SCU` writer - Second units in BCD code"]
pub type ScuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCT` reader - Second tens in BCD code"]
pub type SctR = crate::FieldReader;
#[doc = "Field `SCT` writer - Second tens in BCD code"]
pub type SctW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MNU` reader - Minute units in BCD code"]
pub type MnuR = crate::FieldReader;
#[doc = "Field `MNU` writer - Minute units in BCD code"]
pub type MnuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MNT` reader - Minute tens in BCD code"]
pub type MntR = crate::FieldReader;
#[doc = "Field `MNT` writer - Minute tens in BCD code"]
pub type MntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HRU` reader - Hour units in BCD format"]
pub type HruR = crate::FieldReader;
#[doc = "Field `HRU` writer - Hour units in BCD format"]
pub type HruW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HRT` reader - Hour tens in BCD code"]
pub type HrtR = crate::FieldReader;
#[doc = "Field `HRT` writer - Hour tens in BCD code"]
pub type HrtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PM` reader - AM/PM mark"]
pub type PmR = crate::BitReader;
#[doc = "Field `PM` writer - AM/PM mark"]
pub type PmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Second units in BCD code"]
    #[inline(always)]
    pub fn scu(&self) -> ScuR {
        ScuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Second tens in BCD code"]
    #[inline(always)]
    pub fn sct(&self) -> SctR {
        SctR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Minute units in BCD code"]
    #[inline(always)]
    pub fn mnu(&self) -> MnuR {
        MnuR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD code"]
    #[inline(always)]
    pub fn mnt(&self) -> MntR {
        MntR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    pub fn hru(&self) -> HruR {
        HruR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD code"]
    #[inline(always)]
    pub fn hrt(&self) -> HrtR {
        HrtR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - AM/PM mark"]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Second units in BCD code"]
    #[inline(always)]
    #[must_use]
    pub fn scu(&mut self) -> ScuW<TimeSpec> {
        ScuW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Second tens in BCD code"]
    #[inline(always)]
    #[must_use]
    pub fn sct(&mut self) -> SctW<TimeSpec> {
        SctW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Minute units in BCD code"]
    #[inline(always)]
    #[must_use]
    pub fn mnu(&mut self) -> MnuW<TimeSpec> {
        MnuW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD code"]
    #[inline(always)]
    #[must_use]
    pub fn mnt(&mut self) -> MntW<TimeSpec> {
        MntW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn hru(&mut self) -> HruW<TimeSpec> {
        HruW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD code"]
    #[inline(always)]
    #[must_use]
    pub fn hrt(&mut self) -> HrtW<TimeSpec> {
        HrtW::new(self, 20)
    }
    #[doc = "Bit 22 - AM/PM mark"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PmW<TimeSpec> {
        PmW::new(self, 22)
    }
}
#[doc = "time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimeSpec;
impl crate::RegisterSpec for TimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time::R`](R) reader structure"]
impl crate::Readable for TimeSpec {}
#[doc = "`write(|w| ..)` method takes [`time::W`](W) writer structure"]
impl crate::Writable for TimeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIME to value 0"]
impl crate::Resettable for TimeSpec {
    const RESET_VALUE: u32 = 0;
}
