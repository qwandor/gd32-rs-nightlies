#[doc = "Register `APB1EN` reader"]
pub type R = crate::R<Apb1enSpec>;
#[doc = "Register `APB1EN` writer"]
pub type W = crate::W<Apb1enSpec>;
#[doc = "TIMER2 timer clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer2en {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<Timer2en> for bool {
    #[inline(always)]
    fn from(variant: Timer2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER2EN` reader - TIMER2 timer clock enable"]
pub type Timer2enR = crate::BitReader<Timer2en>;
impl Timer2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer2en {
        match self.bits {
            false => Timer2en::Disabled,
            true => Timer2en::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Timer2en::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Timer2en::Enabled
    }
}
#[doc = "Field `TIMER2EN` writer - TIMER2 timer clock enable"]
pub type Timer2enW<'a, REG> = crate::BitWriter<'a, REG, Timer2en>;
impl<'a, REG> Timer2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Timer2en::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Timer2en::Enabled)
    }
}
#[doc = "Field `TIMER5EN` reader - TIMER5 timer clock enable"]
pub use Timer2enR as Timer5enR;
#[doc = "Field `TIMER13EN` reader - TIMER13 timer clock enable"]
pub use Timer2enR as Timer13enR;
#[doc = "Field `WWDGTEN` reader - Window watchdog timer clock enable"]
pub use Timer2enR as WwdgtenR;
#[doc = "Field `SPI1EN` reader - SPI1 clock enable"]
pub use Timer2enR as Spi1enR;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub use Timer2enR as Usart1enR;
#[doc = "Field `I2C0EN` reader - I2C0 clock enable"]
pub use Timer2enR as I2c0enR;
#[doc = "Field `I2C1EN` reader - I2C1 clock enable"]
pub use Timer2enR as I2c1enR;
#[doc = "Field `PMUEN` reader - Power interface clock enable"]
pub use Timer2enR as PmuenR;
#[doc = "Field `TIMER5EN` writer - TIMER5 timer clock enable"]
pub use Timer2enW as Timer5enW;
#[doc = "Field `TIMER13EN` writer - TIMER13 timer clock enable"]
pub use Timer2enW as Timer13enW;
#[doc = "Field `WWDGTEN` writer - Window watchdog timer clock enable"]
pub use Timer2enW as WwdgtenW;
#[doc = "Field `SPI1EN` writer - SPI1 clock enable"]
pub use Timer2enW as Spi1enW;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub use Timer2enW as Usart1enW;
#[doc = "Field `I2C0EN` writer - I2C0 clock enable"]
pub use Timer2enW as I2c0enW;
#[doc = "Field `I2C1EN` writer - I2C1 clock enable"]
pub use Timer2enW as I2c1enW;
#[doc = "Field `PMUEN` writer - Power interface clock enable"]
pub use Timer2enW as PmuenW;
impl R {
    #[doc = "Bit 1 - TIMER2 timer clock enable"]
    #[inline(always)]
    pub fn timer2en(&self) -> Timer2enR {
        Timer2enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TIMER5 timer clock enable"]
    #[inline(always)]
    pub fn timer5en(&self) -> Timer5enR {
        Timer5enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - TIMER13 timer clock enable"]
    #[inline(always)]
    pub fn timer13en(&self) -> Timer13enR {
        Timer13enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog timer clock enable"]
    #[inline(always)]
    pub fn wwdgten(&self) -> WwdgtenR {
        WwdgtenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> Spi1enR {
        Spi1enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> Usart1enR {
        Usart1enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C0 clock enable"]
    #[inline(always)]
    pub fn i2c0en(&self) -> I2c0enR {
        I2c0enR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2c1enR {
        I2c1enR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pmuen(&self) -> PmuenR {
        PmuenR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TIMER2 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer2en(&mut self) -> Timer2enW<Apb1enSpec> {
        Timer2enW::new(self, 1)
    }
    #[doc = "Bit 4 - TIMER5 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer5en(&mut self) -> Timer5enW<Apb1enSpec> {
        Timer5enW::new(self, 4)
    }
    #[doc = "Bit 8 - TIMER13 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer13en(&mut self) -> Timer13enW<Apb1enSpec> {
        Timer13enW::new(self, 8)
    }
    #[doc = "Bit 11 - Window watchdog timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgten(&mut self) -> WwdgtenW<Apb1enSpec> {
        WwdgtenW::new(self, 11)
    }
    #[doc = "Bit 14 - SPI1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> Spi1enW<Apb1enSpec> {
        Spi1enW::new(self, 14)
    }
    #[doc = "Bit 17 - USART1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> Usart1enW<Apb1enSpec> {
        Usart1enW::new(self, 17)
    }
    #[doc = "Bit 21 - I2C0 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0en(&mut self) -> I2c0enW<Apb1enSpec> {
        I2c0enW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2c1enW<Apb1enSpec> {
        I2c1enW::new(self, 22)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pmuen(&mut self) -> PmuenW<Apb1enSpec> {
        PmuenW::new(self, 28)
    }
}
#[doc = "APB1 enable register (RCU_APB1EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1enSpec;
impl crate::RegisterSpec for Apb1enSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1en::R`](R) reader structure"]
impl crate::Readable for Apb1enSpec {}
#[doc = "`write(|w| ..)` method takes [`apb1en::W`](W) writer structure"]
impl crate::Writable for Apb1enSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1EN to value 0"]
impl crate::Resettable for Apb1enSpec {
    const RESET_VALUE: u32 = 0;
}
