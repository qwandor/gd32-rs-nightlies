#[doc = "Register `ADDEN` reader"]
pub type R = crate::R<AddenSpec>;
#[doc = "Register `ADDEN` writer"]
pub type W = crate::W<AddenSpec>;
#[doc = "I2C2 unit clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c2en {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<I2c2en> for bool {
    #[inline(always)]
    fn from(variant: I2c2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C2EN` reader - I2C2 unit clock enable"]
pub type I2c2enR = crate::BitReader<I2c2en>;
impl I2c2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c2en {
        match self.bits {
            false => I2c2en::Disabled,
            true => I2c2en::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2c2en::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2c2en::Enabled
    }
}
#[doc = "Field `I2C2EN` writer - I2C2 unit clock enable"]
pub type I2c2enW<'a, REG> = crate::BitWriter<'a, REG, I2c2en>;
impl<'a, REG> I2c2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2c2en::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2c2en::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - I2C2 unit clock enable"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2c2enR {
        I2c2enR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C2 unit clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2en(&mut self) -> I2c2enW<AddenSpec> {
        I2c2enW::new(self, 0)
    }
}
#[doc = "Additional enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adden::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adden::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddenSpec;
impl crate::RegisterSpec for AddenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adden::R`](R) reader structure"]
impl crate::Readable for AddenSpec {}
#[doc = "`write(|w| ..)` method takes [`adden::W`](W) writer structure"]
impl crate::Writable for AddenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDEN to value 0"]
impl crate::Resettable for AddenSpec {
    const RESET_VALUE: u32 = 0;
}
