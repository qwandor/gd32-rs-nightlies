#[doc = "Register `APB1RST` reader"]
pub type R = crate::R<Apb1rstSpec>;
#[doc = "Register `APB1RST` writer"]
pub type W = crate::W<Apb1rstSpec>;
#[doc = "TIMER2 timer reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer2rst {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<Timer2rst> for bool {
    #[inline(always)]
    fn from(variant: Timer2rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER2RST` reader - TIMER2 timer reset"]
pub type Timer2rstR = crate::BitReader<Timer2rst>;
impl Timer2rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Timer2rst> {
        match self.bits {
            true => Some(Timer2rst::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Timer2rst::Reset
    }
}
#[doc = "Field `TIMER2RST` writer - TIMER2 timer reset"]
pub type Timer2rstW<'a, REG> = crate::BitWriter<'a, REG, Timer2rst>;
impl<'a, REG> Timer2rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Timer2rst::Reset)
    }
}
#[doc = "Field `TIMER5RST` reader - TIMER5 timer reset"]
pub use Timer2rstR as Timer5rstR;
#[doc = "Field `TIMER13RST` reader - TIMER13 timer reset"]
pub use Timer2rstR as Timer13rstR;
#[doc = "Field `WWDGTRST` reader - Window watchdog timer reset"]
pub use Timer2rstR as WwdgtrstR;
#[doc = "Field `SPI1RST` reader - SPI1 reset"]
pub use Timer2rstR as Spi1rstR;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub use Timer2rstR as Usart1rstR;
#[doc = "Field `I2C0RST` reader - I2C0 reset"]
pub use Timer2rstR as I2c0rstR;
#[doc = "Field `I2C1RST` reader - I2C1 reset"]
pub use Timer2rstR as I2c1rstR;
#[doc = "Field `PMURST` reader - Power control reset"]
pub use Timer2rstR as PmurstR;
#[doc = "Field `TIMER5RST` writer - TIMER5 timer reset"]
pub use Timer2rstW as Timer5rstW;
#[doc = "Field `TIMER13RST` writer - TIMER13 timer reset"]
pub use Timer2rstW as Timer13rstW;
#[doc = "Field `WWDGTRST` writer - Window watchdog timer reset"]
pub use Timer2rstW as WwdgtrstW;
#[doc = "Field `SPI1RST` writer - SPI1 reset"]
pub use Timer2rstW as Spi1rstW;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub use Timer2rstW as Usart1rstW;
#[doc = "Field `I2C0RST` writer - I2C0 reset"]
pub use Timer2rstW as I2c0rstW;
#[doc = "Field `I2C1RST` writer - I2C1 reset"]
pub use Timer2rstW as I2c1rstW;
#[doc = "Field `PMURST` writer - Power control reset"]
pub use Timer2rstW as PmurstW;
impl R {
    #[doc = "Bit 1 - TIMER2 timer reset"]
    #[inline(always)]
    pub fn timer2rst(&self) -> Timer2rstR {
        Timer2rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TIMER5 timer reset"]
    #[inline(always)]
    pub fn timer5rst(&self) -> Timer5rstR {
        Timer5rstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - TIMER13 timer reset"]
    #[inline(always)]
    pub fn timer13rst(&self) -> Timer13rstR {
        Timer13rstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog timer reset"]
    #[inline(always)]
    pub fn wwdgtrst(&self) -> WwdgtrstR {
        WwdgtrstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> Spi1rstR {
        Spi1rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> Usart1rstR {
        Usart1rstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C0 reset"]
    #[inline(always)]
    pub fn i2c0rst(&self) -> I2c0rstR {
        I2c0rstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2c1rstR {
        I2c1rstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 28 - Power control reset"]
    #[inline(always)]
    pub fn pmurst(&self) -> PmurstR {
        PmurstR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TIMER2 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer2rst(&mut self) -> Timer2rstW<Apb1rstSpec> {
        Timer2rstW::new(self, 1)
    }
    #[doc = "Bit 4 - TIMER5 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer5rst(&mut self) -> Timer5rstW<Apb1rstSpec> {
        Timer5rstW::new(self, 4)
    }
    #[doc = "Bit 8 - TIMER13 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer13rst(&mut self) -> Timer13rstW<Apb1rstSpec> {
        Timer13rstW::new(self, 8)
    }
    #[doc = "Bit 11 - Window watchdog timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgtrst(&mut self) -> WwdgtrstW<Apb1rstSpec> {
        WwdgtrstW::new(self, 11)
    }
    #[doc = "Bit 14 - SPI1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> Spi1rstW<Apb1rstSpec> {
        Spi1rstW::new(self, 14)
    }
    #[doc = "Bit 17 - USART1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> Usart1rstW<Apb1rstSpec> {
        Usart1rstW::new(self, 17)
    }
    #[doc = "Bit 21 - I2C0 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0rst(&mut self) -> I2c0rstW<Apb1rstSpec> {
        I2c0rstW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2c1rstW<Apb1rstSpec> {
        I2c1rstW::new(self, 22)
    }
    #[doc = "Bit 28 - Power control reset"]
    #[inline(always)]
    #[must_use]
    pub fn pmurst(&mut self) -> PmurstW<Apb1rstSpec> {
        PmurstW::new(self, 28)
    }
}
#[doc = "APB1 reset register (RCU_APB1RST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1rstSpec;
impl crate::RegisterSpec for Apb1rstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1rst::R`](R) reader structure"]
impl crate::Readable for Apb1rstSpec {}
#[doc = "`write(|w| ..)` method takes [`apb1rst::W`](W) writer structure"]
impl crate::Writable for Apb1rstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1RST to value 0"]
impl crate::Resettable for Apb1rstSpec {
    const RESET_VALUE: u32 = 0;
}
