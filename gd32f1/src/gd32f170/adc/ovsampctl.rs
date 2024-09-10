#[doc = "Register `OVSAMPCTL` reader"]
pub type R = crate::R<OvsampctlSpec>;
#[doc = "Register `OVSAMPCTL` writer"]
pub type W = crate::W<OvsampctlSpec>;
#[doc = "Oversampler Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovsen {
    #[doc = "0: Oversampling disabled"]
    Disabled = 0,
    #[doc = "1: Oversampling enabled"]
    Enabled = 1,
}
impl From<Ovsen> for bool {
    #[inline(always)]
    fn from(variant: Ovsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVSEN` reader - Oversampler Enable"]
pub type OvsenR = crate::BitReader<Ovsen>;
impl OvsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovsen {
        match self.bits {
            false => Ovsen::Disabled,
            true => Ovsen::Enabled,
        }
    }
    #[doc = "Oversampling disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ovsen::Disabled
    }
    #[doc = "Oversampling enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ovsen::Enabled
    }
}
#[doc = "Field `OVSEN` writer - Oversampler Enable"]
pub type OvsenW<'a, REG> = crate::BitWriter<'a, REG, Ovsen>;
impl<'a, REG> OvsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Oversampling disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsen::Disabled)
    }
    #[doc = "Oversampling enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsen::Enabled)
    }
}
#[doc = "Oversampling ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ovsr {
    #[doc = "0: 2x"]
    Times2 = 0,
    #[doc = "1: 4x"]
    Times4 = 1,
    #[doc = "2: 8x"]
    Times8 = 2,
    #[doc = "3: 16x"]
    Times16 = 3,
    #[doc = "4: 32x"]
    Times32 = 4,
    #[doc = "5: 64x"]
    Times64 = 5,
    #[doc = "6: 128x"]
    Times128 = 6,
    #[doc = "7: 256x"]
    Times256 = 7,
}
impl From<Ovsr> for u8 {
    #[inline(always)]
    fn from(variant: Ovsr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ovsr {
    type Ux = u8;
}
#[doc = "Field `OVSR` reader - Oversampling ratio"]
pub type OvsrR = crate::FieldReader<Ovsr>;
impl OvsrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovsr {
        match self.bits {
            0 => Ovsr::Times2,
            1 => Ovsr::Times4,
            2 => Ovsr::Times8,
            3 => Ovsr::Times16,
            4 => Ovsr::Times32,
            5 => Ovsr::Times64,
            6 => Ovsr::Times128,
            7 => Ovsr::Times256,
            _ => unreachable!(),
        }
    }
    #[doc = "2x"]
    #[inline(always)]
    pub fn is_times2(&self) -> bool {
        *self == Ovsr::Times2
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn is_times4(&self) -> bool {
        *self == Ovsr::Times4
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn is_times8(&self) -> bool {
        *self == Ovsr::Times8
    }
    #[doc = "16x"]
    #[inline(always)]
    pub fn is_times16(&self) -> bool {
        *self == Ovsr::Times16
    }
    #[doc = "32x"]
    #[inline(always)]
    pub fn is_times32(&self) -> bool {
        *self == Ovsr::Times32
    }
    #[doc = "64x"]
    #[inline(always)]
    pub fn is_times64(&self) -> bool {
        *self == Ovsr::Times64
    }
    #[doc = "128x"]
    #[inline(always)]
    pub fn is_times128(&self) -> bool {
        *self == Ovsr::Times128
    }
    #[doc = "256x"]
    #[inline(always)]
    pub fn is_times256(&self) -> bool {
        *self == Ovsr::Times256
    }
}
#[doc = "Field `OVSR` writer - Oversampling ratio"]
pub type OvsrW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Ovsr>;
impl<'a, REG> OvsrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2x"]
    #[inline(always)]
    pub fn times2(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsr::Times2)
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn times4(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsr::Times4)
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn times8(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsr::Times8)
    }
    #[doc = "16x"]
    #[inline(always)]
    pub fn times16(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsr::Times16)
    }
    #[doc = "32x"]
    #[inline(always)]
    pub fn times32(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsr::Times32)
    }
    #[doc = "64x"]
    #[inline(always)]
    pub fn times64(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsr::Times64)
    }
    #[doc = "128x"]
    #[inline(always)]
    pub fn times128(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsr::Times128)
    }
    #[doc = "256x"]
    #[inline(always)]
    pub fn times256(self) -> &'a mut crate::W<REG> {
        self.variant(Ovsr::Times256)
    }
}
#[doc = "Field `OVSS` reader - Oversampling shift"]
pub type OvssR = crate::FieldReader;
#[doc = "Field `OVSS` writer - Oversampling shift"]
pub type OvssW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Triggered Oversampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tovs {
    #[doc = "0: All oversampled conversions are done consecutively"]
    Consecutive = 0,
    #[doc = "1: Each oversampled conversion needs a trigger"]
    Individual = 1,
}
impl From<Tovs> for bool {
    #[inline(always)]
    fn from(variant: Tovs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOVS` reader - Triggered Oversampling"]
pub type TovsR = crate::BitReader<Tovs>;
impl TovsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tovs {
        match self.bits {
            false => Tovs::Consecutive,
            true => Tovs::Individual,
        }
    }
    #[doc = "All oversampled conversions are done consecutively"]
    #[inline(always)]
    pub fn is_consecutive(&self) -> bool {
        *self == Tovs::Consecutive
    }
    #[doc = "Each oversampled conversion needs a trigger"]
    #[inline(always)]
    pub fn is_individual(&self) -> bool {
        *self == Tovs::Individual
    }
}
#[doc = "Field `TOVS` writer - Triggered Oversampling"]
pub type TovsW<'a, REG> = crate::BitWriter<'a, REG, Tovs>;
impl<'a, REG> TovsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All oversampled conversions are done consecutively"]
    #[inline(always)]
    pub fn consecutive(self) -> &'a mut crate::W<REG> {
        self.variant(Tovs::Consecutive)
    }
    #[doc = "Each oversampled conversion needs a trigger"]
    #[inline(always)]
    pub fn individual(self) -> &'a mut crate::W<REG> {
        self.variant(Tovs::Individual)
    }
}
impl R {
    #[doc = "Bit 0 - Oversampler Enable"]
    #[inline(always)]
    pub fn ovsen(&self) -> OvsenR {
        OvsenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:4 - Oversampling ratio"]
    #[inline(always)]
    pub fn ovsr(&self) -> OvsrR {
        OvsrR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:8 - Oversampling shift"]
    #[inline(always)]
    pub fn ovss(&self) -> OvssR {
        OvssR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Triggered Oversampling"]
    #[inline(always)]
    pub fn tovs(&self) -> TovsR {
        TovsR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Oversampler Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovsen(&mut self) -> OvsenW<OvsampctlSpec> {
        OvsenW::new(self, 0)
    }
    #[doc = "Bits 2:4 - Oversampling ratio"]
    #[inline(always)]
    #[must_use]
    pub fn ovsr(&mut self) -> OvsrW<OvsampctlSpec> {
        OvsrW::new(self, 2)
    }
    #[doc = "Bits 5:8 - Oversampling shift"]
    #[inline(always)]
    #[must_use]
    pub fn ovss(&mut self) -> OvssW<OvsampctlSpec> {
        OvssW::new(self, 5)
    }
    #[doc = "Bit 9 - Triggered Oversampling"]
    #[inline(always)]
    #[must_use]
    pub fn tovs(&mut self) -> TovsW<OvsampctlSpec> {
        TovsW::new(self, 9)
    }
}
#[doc = "oversample control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ovsampctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ovsampctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OvsampctlSpec;
impl crate::RegisterSpec for OvsampctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ovsampctl::R`](R) reader structure"]
impl crate::Readable for OvsampctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ovsampctl::W`](W) writer structure"]
impl crate::Writable for OvsampctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OVSAMPCTL to value 0"]
impl crate::Resettable for OvsampctlSpec {
    const RESET_VALUE: u32 = 0;
}
