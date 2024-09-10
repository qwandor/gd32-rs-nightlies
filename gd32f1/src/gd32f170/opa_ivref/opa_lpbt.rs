#[doc = "Register `OPA_LPBT` reader"]
pub type R = crate::R<OpaLpbtSpec>;
#[doc = "Register `OPA_LPBT` writer"]
pub type W = crate::W<OpaLpbtSpec>;
#[doc = "Field `OA0_TRIM_LP_LOW` reader - OPA0, low power mode 5-bit offset trim value for PMOS pairs"]
pub type Oa0TrimLpLowR = crate::FieldReader;
#[doc = "Field `OA0_TRIM_LP_LOW` writer - OPA0, low power mode 5-bit offset trim value for PMOS pairs"]
pub type Oa0TrimLpLowW<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `OA0_TRIM_LP_HIGH` reader - OPA0, low power mode 5-bit offset trim value for NMOS pairs"]
pub type Oa0TrimLpHighR = crate::FieldReader;
#[doc = "Field `OA0_TRIM_LP_HIGH` writer - OPA0, low power mode 5-bit offset trim value for NMOS pairs"]
pub type Oa0TrimLpHighW<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `OA1_TRIM_LP_LOW` reader - OPA1, low power mode 5-bit offset trim value for PMOS pairs"]
pub type Oa1TrimLpLowR = crate::FieldReader;
#[doc = "Field `OA1_TRIM_LP_LOW` writer - OPA1, low power mode 5-bit offset trim value for PMOS pairs"]
pub type Oa1TrimLpLowW<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `OA1_TRIM_LP_HIGH` reader - OPA1, low power mode 5-bit offset trim value for NMOS pairs"]
pub type Oa1TrimLpHighR = crate::FieldReader;
#[doc = "Field `OA1_TRIM_LP_HIGH` writer - OPA1, low power mode 5-bit offset trim value for NMOS pairs"]
pub type Oa1TrimLpHighW<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `OA2_TRIM_LP_LOW` reader - OPA2, low power mode 5-bit offset trim value for PMOS pairs"]
pub type Oa2TrimLpLowR = crate::FieldReader;
#[doc = "Field `OA2_TRIM_LP_LOW` writer - OPA2, low power mode 5-bit offset trim value for PMOS pairs"]
pub type Oa2TrimLpLowW<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `OA2_TRIM_LP_HIGH` reader - OPA2, low power mode 5-bit offset trim value for NMOS pairs"]
pub type Oa2TrimLpHighR = crate::FieldReader;
#[doc = "Field `OA2_TRIM_LP_HIGH` writer - OPA2, low power mode 5-bit offset trim value for NMOS pairs"]
pub type Oa2TrimLpHighW<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - OPA0, low power mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    pub fn oa0_trim_lp_low(&self) -> Oa0TrimLpLowR {
        Oa0TrimLpLowR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - OPA0, low power mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    pub fn oa0_trim_lp_high(&self) -> Oa0TrimLpHighR {
        Oa0TrimLpHighR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - OPA1, low power mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    pub fn oa1_trim_lp_low(&self) -> Oa1TrimLpLowR {
        Oa1TrimLpLowR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - OPA1, low power mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    pub fn oa1_trim_lp_high(&self) -> Oa1TrimLpHighR {
        Oa1TrimLpHighR::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - OPA2, low power mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    pub fn oa2_trim_lp_low(&self) -> Oa2TrimLpLowR {
        Oa2TrimLpLowR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - OPA2, low power mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    pub fn oa2_trim_lp_high(&self) -> Oa2TrimLpHighR {
        Oa2TrimLpHighR::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - OPA0, low power mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    #[must_use]
    pub fn oa0_trim_lp_low(&mut self) -> Oa0TrimLpLowW<OpaLpbtSpec> {
        Oa0TrimLpLowW::new(self, 0)
    }
    #[doc = "Bits 5:9 - OPA0, low power mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    #[must_use]
    pub fn oa0_trim_lp_high(&mut self) -> Oa0TrimLpHighW<OpaLpbtSpec> {
        Oa0TrimLpHighW::new(self, 5)
    }
    #[doc = "Bits 10:14 - OPA1, low power mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    #[must_use]
    pub fn oa1_trim_lp_low(&mut self) -> Oa1TrimLpLowW<OpaLpbtSpec> {
        Oa1TrimLpLowW::new(self, 10)
    }
    #[doc = "Bits 15:19 - OPA1, low power mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    #[must_use]
    pub fn oa1_trim_lp_high(&mut self) -> Oa1TrimLpHighW<OpaLpbtSpec> {
        Oa1TrimLpHighW::new(self, 15)
    }
    #[doc = "Bits 20:24 - OPA2, low power mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    #[must_use]
    pub fn oa2_trim_lp_low(&mut self) -> Oa2TrimLpLowW<OpaLpbtSpec> {
        Oa2TrimLpLowW::new(self, 20)
    }
    #[doc = "Bits 25:29 - OPA2, low power mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    #[must_use]
    pub fn oa2_trim_lp_high(&mut self) -> Oa2TrimLpHighW<OpaLpbtSpec> {
        Oa2TrimLpHighW::new(self, 25)
    }
}
#[doc = "OPA offset trimming for low power mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa_lpbt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa_lpbt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OpaLpbtSpec;
impl crate::RegisterSpec for OpaLpbtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opa_lpbt::R`](R) reader structure"]
impl crate::Readable for OpaLpbtSpec {}
#[doc = "`write(|w| ..)` method takes [`opa_lpbt::W`](W) writer structure"]
impl crate::Writable for OpaLpbtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPA_LPBT to value 0"]
impl crate::Resettable for OpaLpbtSpec {
    const RESET_VALUE: u32 = 0;
}
