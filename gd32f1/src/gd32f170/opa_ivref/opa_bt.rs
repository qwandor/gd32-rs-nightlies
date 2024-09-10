#[doc = "Register `OPA_BT` reader"]
pub type R = crate::R<OpaBtSpec>;
#[doc = "Register `OPA_BT` writer"]
pub type W = crate::W<OpaBtSpec>;
#[doc = "Field `OA0_TRIM_LOW` reader - OPA0, normal mode 5-bit offset trim value for PMOS pairs"]
pub type Oa0TrimLowR = crate::FieldReader;
#[doc = "Field `OA0_TRIM_LOW` writer - OPA0, normal mode 5-bit offset trim value for PMOS pairs"]
pub type Oa0TrimLowW<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `OA0_TRIM_HIGH` reader - OPA0, normal mode 5-bit offset trim value for NMOS pairs"]
pub type Oa0TrimHighR = crate::FieldReader;
#[doc = "Field `OA0_TRIM_HIGH` writer - OPA0, normal mode 5-bit offset trim value for NMOS pairs"]
pub type Oa0TrimHighW<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `OA1_TRIM_LOW` reader - OPA1, normal mode 5-bit offset trim value for PMOS pairs"]
pub type Oa1TrimLowR = crate::FieldReader;
#[doc = "Field `OA1_TRIM_LOW` writer - OPA1, normal mode 5-bit offset trim value for PMOS pairs"]
pub type Oa1TrimLowW<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `OA1_TRIM_HIGH` reader - OPA1, normal mode 5-bit offset trim value for NMOS pairs"]
pub type Oa1TrimHighR = crate::FieldReader;
#[doc = "Field `OA1_TRIM_HIGH` writer - OPA1, normal mode 5-bit offset trim value for NMOS pairs"]
pub type Oa1TrimHighW<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `OA2_TRIM_LOW` reader - OPA2, normal mode 5-bit offset trim value for PMOS pairs"]
pub type Oa2TrimLowR = crate::FieldReader;
#[doc = "Field `OA2_TRIM_LOW` writer - OPA2, normal mode 5-bit offset trim value for PMOS pairs"]
pub type Oa2TrimLowW<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `OA2_TRIM_HIGH` reader - OPA2, normal mode 5-bit offset trim value for NMOS pairs"]
pub type Oa2TrimHighR = crate::FieldReader;
#[doc = "Field `OA2_TRIM_HIGH` writer - OPA2, normal mode 5-bit offset trim value for NMOS pairs"]
pub type Oa2TrimHighW<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "user programmed trimming value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtUser {
    #[doc = "0: Trim with factory-default values"]
    Factory = 0,
    #[doc = "1: Trim with user-programmed values"]
    User = 1,
}
impl From<OtUser> for bool {
    #[inline(always)]
    fn from(variant: OtUser) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT_USER` reader - user programmed trimming value"]
pub type OtUserR = crate::BitReader<OtUser>;
impl OtUserR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OtUser {
        match self.bits {
            false => OtUser::Factory,
            true => OtUser::User,
        }
    }
    #[doc = "Trim with factory-default values"]
    #[inline(always)]
    pub fn is_factory(&self) -> bool {
        *self == OtUser::Factory
    }
    #[doc = "Trim with user-programmed values"]
    #[inline(always)]
    pub fn is_user(&self) -> bool {
        *self == OtUser::User
    }
}
#[doc = "Field `OT_USER` writer - user programmed trimming value"]
pub type OtUserW<'a, REG> = crate::BitWriter<'a, REG, OtUser>;
impl<'a, REG> OtUserW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trim with factory-default values"]
    #[inline(always)]
    pub fn factory(self) -> &'a mut crate::W<REG> {
        self.variant(OtUser::Factory)
    }
    #[doc = "Trim with user-programmed values"]
    #[inline(always)]
    pub fn user(self) -> &'a mut crate::W<REG> {
        self.variant(OtUser::User)
    }
}
impl R {
    #[doc = "Bits 0:4 - OPA0, normal mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    pub fn oa0_trim_low(&self) -> Oa0TrimLowR {
        Oa0TrimLowR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - OPA0, normal mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    pub fn oa0_trim_high(&self) -> Oa0TrimHighR {
        Oa0TrimHighR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - OPA1, normal mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    pub fn oa1_trim_low(&self) -> Oa1TrimLowR {
        Oa1TrimLowR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - OPA1, normal mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    pub fn oa1_trim_high(&self) -> Oa1TrimHighR {
        Oa1TrimHighR::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - OPA2, normal mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    pub fn oa2_trim_low(&self) -> Oa2TrimLowR {
        Oa2TrimLowR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - OPA2, normal mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    pub fn oa2_trim_high(&self) -> Oa2TrimHighR {
        Oa2TrimHighR::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - user programmed trimming value"]
    #[inline(always)]
    pub fn ot_user(&self) -> OtUserR {
        OtUserR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - OPA0, normal mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    #[must_use]
    pub fn oa0_trim_low(&mut self) -> Oa0TrimLowW<OpaBtSpec> {
        Oa0TrimLowW::new(self, 0)
    }
    #[doc = "Bits 5:9 - OPA0, normal mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    #[must_use]
    pub fn oa0_trim_high(&mut self) -> Oa0TrimHighW<OpaBtSpec> {
        Oa0TrimHighW::new(self, 5)
    }
    #[doc = "Bits 10:14 - OPA1, normal mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    #[must_use]
    pub fn oa1_trim_low(&mut self) -> Oa1TrimLowW<OpaBtSpec> {
        Oa1TrimLowW::new(self, 10)
    }
    #[doc = "Bits 15:19 - OPA1, normal mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    #[must_use]
    pub fn oa1_trim_high(&mut self) -> Oa1TrimHighW<OpaBtSpec> {
        Oa1TrimHighW::new(self, 15)
    }
    #[doc = "Bits 20:24 - OPA2, normal mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    #[must_use]
    pub fn oa2_trim_low(&mut self) -> Oa2TrimLowW<OpaBtSpec> {
        Oa2TrimLowW::new(self, 20)
    }
    #[doc = "Bits 25:29 - OPA2, normal mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    #[must_use]
    pub fn oa2_trim_high(&mut self) -> Oa2TrimHighW<OpaBtSpec> {
        Oa2TrimHighW::new(self, 25)
    }
    #[doc = "Bit 31 - user programmed trimming value"]
    #[inline(always)]
    #[must_use]
    pub fn ot_user(&mut self) -> OtUserW<OpaBtSpec> {
        OtUserW::new(self, 31)
    }
}
#[doc = "OPA offset trimming for normal mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa_bt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa_bt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OpaBtSpec;
impl crate::RegisterSpec for OpaBtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opa_bt::R`](R) reader structure"]
impl crate::Readable for OpaBtSpec {}
#[doc = "`write(|w| ..)` method takes [`opa_bt::W`](W) writer structure"]
impl crate::Writable for OpaBtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPA_BT to value 0"]
impl crate::Resettable for OpaBtSpec {
    const RESET_VALUE: u32 = 0;
}
