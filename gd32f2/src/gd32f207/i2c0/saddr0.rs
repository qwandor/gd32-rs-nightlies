#[doc = "Register `SADDR0` reader"]
pub type R = crate::R<Saddr0Spec>;
#[doc = "Register `SADDR0` writer"]
pub type W = crate::W<Saddr0Spec>;
#[doc = "Field `ADDRESS` reader - Highest two bits of a 10-bit address"]
pub type AddressR = crate::FieldReader<u16>;
#[doc = "Field `ADDRESS` writer - Highest two bits of a 10-bit address"]
pub type AddressW<'a, REG> = crate::FieldWriterSafe<'a, REG, 10, u16>;
#[doc = "Address mode for the I2C slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addformat {
    #[doc = "0: 7-bit slave address (note that you'll need to shift the address by 1b)"]
    Add7 = 0,
    #[doc = "1: 10-bit slave address"]
    Add10 = 1,
}
impl From<Addformat> for bool {
    #[inline(always)]
    fn from(variant: Addformat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDFORMAT` reader - Address mode for the I2C slave"]
pub type AddformatR = crate::BitReader<Addformat>;
impl AddformatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addformat {
        match self.bits {
            false => Addformat::Add7,
            true => Addformat::Add10,
        }
    }
    #[doc = "7-bit slave address (note that you'll need to shift the address by 1b)"]
    #[inline(always)]
    pub fn is_add7(&self) -> bool {
        *self == Addformat::Add7
    }
    #[doc = "10-bit slave address"]
    #[inline(always)]
    pub fn is_add10(&self) -> bool {
        *self == Addformat::Add10
    }
}
#[doc = "Field `ADDFORMAT` writer - Address mode for the I2C slave"]
pub type AddformatW<'a, REG> = crate::BitWriter<'a, REG, Addformat>;
impl<'a, REG> AddformatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "7-bit slave address (note that you'll need to shift the address by 1b)"]
    #[inline(always)]
    pub fn add7(self) -> &'a mut crate::W<REG> {
        self.variant(Addformat::Add7)
    }
    #[doc = "10-bit slave address"]
    #[inline(always)]
    pub fn add10(self) -> &'a mut crate::W<REG> {
        self.variant(Addformat::Add10)
    }
}
impl R {
    #[doc = "Bits 0:9 - Highest two bits of a 10-bit address"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - Address mode for the I2C slave"]
    #[inline(always)]
    pub fn addformat(&self) -> AddformatR {
        AddformatR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Highest two bits of a 10-bit address"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> AddressW<Saddr0Spec> {
        AddressW::new(self, 0)
    }
    #[doc = "Bit 15 - Address mode for the I2C slave"]
    #[inline(always)]
    #[must_use]
    pub fn addformat(&mut self) -> AddformatW<Saddr0Spec> {
        AddformatW::new(self, 15)
    }
}
#[doc = "Slave address register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saddr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saddr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Saddr0Spec;
impl crate::RegisterSpec for Saddr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saddr0::R`](R) reader structure"]
impl crate::Readable for Saddr0Spec {}
#[doc = "`write(|w| ..)` method takes [`saddr0::W`](W) writer structure"]
impl crate::Writable for Saddr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SADDR0 to value 0"]
impl crate::Resettable for Saddr0Spec {
    const RESET_VALUE: u32 = 0;
}
