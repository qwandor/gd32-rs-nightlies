#[doc = "Register `FCTL` reader"]
pub type R = crate::R<FctlSpec>;
#[doc = "Register `FCTL` writer"]
pub type W = crate::W<FctlSpec>;
#[doc = "Filter lock disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fld {
    #[doc = "0: Filter lock disabled"]
    Disabled = 0,
    #[doc = "1: Filter lock enabled"]
    Enabled = 1,
}
impl From<Fld> for bool {
    #[inline(always)]
    fn from(variant: Fld) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLD` reader - Filter lock disable"]
pub type FldR = crate::BitReader<Fld>;
impl FldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fld {
        match self.bits {
            false => Fld::Disabled,
            true => Fld::Enabled,
        }
    }
    #[doc = "Filter lock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fld::Disabled
    }
    #[doc = "Filter lock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fld::Enabled
    }
}
#[doc = "Field `FLD` writer - Filter lock disable"]
pub type FldW<'a, REG> = crate::BitWriter<'a, REG, Fld>;
impl<'a, REG> FldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filter lock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fld::Disabled)
    }
    #[doc = "Filter lock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fld::Enabled)
    }
}
#[doc = "Field `HBC1F` reader - Header bank of CAN1 filter"]
pub type Hbc1fR = crate::FieldReader;
#[doc = "Field `HBC1F` writer - Header bank of CAN1 filter"]
pub type Hbc1fW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Filter lock disable"]
    #[inline(always)]
    pub fn fld(&self) -> FldR {
        FldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:13 - Header bank of CAN1 filter"]
    #[inline(always)]
    pub fn hbc1f(&self) -> Hbc1fR {
        Hbc1fR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Filter lock disable"]
    #[inline(always)]
    #[must_use]
    pub fn fld(&mut self) -> FldW<FctlSpec> {
        FldW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Header bank of CAN1 filter"]
    #[inline(always)]
    #[must_use]
    pub fn hbc1f(&mut self) -> Hbc1fW<FctlSpec> {
        Hbc1fW::new(self, 8)
    }
}
#[doc = "Filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FctlSpec;
impl crate::RegisterSpec for FctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fctl::R`](R) reader structure"]
impl crate::Readable for FctlSpec {}
#[doc = "`write(|w| ..)` method takes [`fctl::W`](W) writer structure"]
impl crate::Writable for FctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCTL to value 0x2a1c_0e01"]
impl crate::Resettable for FctlSpec {
    const RESET_VALUE: u32 = 0x2a1c_0e01;
}
