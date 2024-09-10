#[doc = "Register `ADDAPB1EN` reader"]
pub type R = crate::R<Addapb1enSpec>;
#[doc = "Register `ADDAPB1EN` writer"]
pub type W = crate::W<Addapb1enSpec>;
#[doc = "I2C2 clock enable\n\nValue on reset: 0"]
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
#[doc = "Field `I2C2EN` reader - I2C2 clock enable"]
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
#[doc = "Field `I2C2EN` writer - I2C2 clock enable"]
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
#[doc = "Field `UART6EN` reader - UART6 clock enable"]
pub use I2c2enR as Uart6enR;
#[doc = "Field `UART7EN` reader - UART7 clock enable"]
pub use I2c2enR as Uart7enR;
#[doc = "Field `UART6EN` writer - UART6 clock enable"]
pub use I2c2enW as Uart6enW;
#[doc = "Field `UART7EN` writer - UART7 clock enable"]
pub use I2c2enW as Uart7enW;
impl R {
    #[doc = "Bit 23 - I2C2 clock enable"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2c2enR {
        I2c2enR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - UART6 clock enable"]
    #[inline(always)]
    pub fn uart6en(&self) -> Uart6enR {
        Uart6enR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UART7 clock enable"]
    #[inline(always)]
    pub fn uart7en(&self) -> Uart7enR {
        Uart7enR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - I2C2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2en(&mut self) -> I2c2enW<Addapb1enSpec> {
        I2c2enW::new(self, 23)
    }
    #[doc = "Bit 30 - UART6 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart6en(&mut self) -> Uart6enW<Addapb1enSpec> {
        Uart6enW::new(self, 30)
    }
    #[doc = "Bit 31 - UART7 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart7en(&mut self) -> Uart7enW<Addapb1enSpec> {
        Uart7enW::new(self, 31)
    }
}
#[doc = "APB1 additional enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addapb1en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addapb1en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addapb1enSpec;
impl crate::RegisterSpec for Addapb1enSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addapb1en::R`](R) reader structure"]
impl crate::Readable for Addapb1enSpec {}
#[doc = "`write(|w| ..)` method takes [`addapb1en::W`](W) writer structure"]
impl crate::Writable for Addapb1enSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDAPB1EN to value 0"]
impl crate::Resettable for Addapb1enSpec {
    const RESET_VALUE: u32 = 0;
}
