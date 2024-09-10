#[doc = "Register `TTS` reader"]
pub type R = crate::R<TtsSpec>;
#[doc = "Field `SCU` reader - Second units in BCD code"]
pub type ScuR = crate::FieldReader;
#[doc = "Field `SCT` reader - Second tens in BCD code"]
pub type SctR = crate::FieldReader;
#[doc = "Field `MNU` reader - Minute units in BCD code"]
pub type MnuR = crate::FieldReader;
#[doc = "Field `MNT` reader - Minute tens in BCD code"]
pub type MntR = crate::FieldReader;
#[doc = "Field `HRU` reader - Hour units in BCD code"]
pub type HruR = crate::FieldReader;
#[doc = "Field `HRT` reader - Hour tens in BCD code"]
pub type HrtR = crate::FieldReader;
#[doc = "Field `PM` reader - AM/PM mark"]
pub type PmR = crate::BitReader;
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
    #[doc = "Bits 16:19 - Hour units in BCD code"]
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
#[doc = "timestamp time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TtsSpec;
impl crate::RegisterSpec for TtsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tts::R`](R) reader structure"]
impl crate::Readable for TtsSpec {}
#[doc = "`reset()` method sets TTS to value 0"]
impl crate::Resettable for TtsSpec {
    const RESET_VALUE: u32 = 0;
}
