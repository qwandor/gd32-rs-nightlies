#[doc = "Register `APB1RST` reader"]
pub type R = crate::R<Apb1rstSpec>;
#[doc = "Register `APB1RST` writer"]
pub type W = crate::W<Apb1rstSpec>;
#[doc = "TIMER1 timer reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer1rst {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<Timer1rst> for bool {
    #[inline(always)]
    fn from(variant: Timer1rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER1RST` reader - TIMER1 timer reset"]
pub type Timer1rstR = crate::BitReader<Timer1rst>;
impl Timer1rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Timer1rst> {
        match self.bits {
            true => Some(Timer1rst::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Timer1rst::Reset
    }
}
#[doc = "Field `TIMER1RST` writer - TIMER1 timer reset"]
pub type Timer1rstW<'a, REG> = crate::BitWriter<'a, REG, Timer1rst>;
impl<'a, REG> Timer1rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Timer1rst::Reset)
    }
}
#[doc = "Field `TIMER2RST` reader - TIMER2 timer reset"]
pub use Timer1rstR as Timer2rstR;
#[doc = "Field `TIMER5RST` reader - TIMER5 timer reset"]
pub use Timer1rstR as Timer5rstR;
#[doc = "Field `TIMER13RST` reader - TIMER13 timer reset"]
pub use Timer1rstR as Timer13rstR;
#[doc = "Field `SLCDRST` reader - SLCD reset"]
pub use Timer1rstR as SlcdrstR;
#[doc = "Field `WWDGTRST` reader - Window watchdog timer reset"]
pub use Timer1rstR as WwdgtrstR;
#[doc = "Field `SPI1RST` reader - SPI1 reset"]
pub use Timer1rstR as Spi1rstR;
#[doc = "Field `SPI2RST` reader - SPI2 reset"]
pub use Timer1rstR as Spi2rstR;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub use Timer1rstR as Usart1rstR;
#[doc = "Field `I2C0RST` reader - I2C0 reset"]
pub use Timer1rstR as I2c0rstR;
#[doc = "Field `I2C1RST` reader - I2C1 reset"]
pub use Timer1rstR as I2c1rstR;
#[doc = "Field `CAN0RST` reader - CAN0 reset"]
pub use Timer1rstR as Can0rstR;
#[doc = "Field `CAN1RST` reader - CAN1 reset"]
pub use Timer1rstR as Can1rstR;
#[doc = "Field `PMURST` reader - Power control reset"]
pub use Timer1rstR as PmurstR;
#[doc = "Field `DACRST` reader - DAC reset"]
pub use Timer1rstR as DacrstR;
#[doc = "Field `CECRST` reader - HDMI CEC reset"]
pub use Timer1rstR as CecrstR;
#[doc = "Field `OPAIVREFRST` reader - OPA and IVREF reset"]
pub use Timer1rstR as OpaivrefrstR;
#[doc = "Field `TIMER2RST` writer - TIMER2 timer reset"]
pub use Timer1rstW as Timer2rstW;
#[doc = "Field `TIMER5RST` writer - TIMER5 timer reset"]
pub use Timer1rstW as Timer5rstW;
#[doc = "Field `TIMER13RST` writer - TIMER13 timer reset"]
pub use Timer1rstW as Timer13rstW;
#[doc = "Field `SLCDRST` writer - SLCD reset"]
pub use Timer1rstW as SlcdrstW;
#[doc = "Field `WWDGTRST` writer - Window watchdog timer reset"]
pub use Timer1rstW as WwdgtrstW;
#[doc = "Field `SPI1RST` writer - SPI1 reset"]
pub use Timer1rstW as Spi1rstW;
#[doc = "Field `SPI2RST` writer - SPI2 reset"]
pub use Timer1rstW as Spi2rstW;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub use Timer1rstW as Usart1rstW;
#[doc = "Field `I2C0RST` writer - I2C0 reset"]
pub use Timer1rstW as I2c0rstW;
#[doc = "Field `I2C1RST` writer - I2C1 reset"]
pub use Timer1rstW as I2c1rstW;
#[doc = "Field `CAN0RST` writer - CAN0 reset"]
pub use Timer1rstW as Can0rstW;
#[doc = "Field `CAN1RST` writer - CAN1 reset"]
pub use Timer1rstW as Can1rstW;
#[doc = "Field `PMURST` writer - Power control reset"]
pub use Timer1rstW as PmurstW;
#[doc = "Field `DACRST` writer - DAC reset"]
pub use Timer1rstW as DacrstW;
#[doc = "Field `CECRST` writer - HDMI CEC reset"]
pub use Timer1rstW as CecrstW;
#[doc = "Field `OPAIVREFRST` writer - OPA and IVREF reset"]
pub use Timer1rstW as OpaivrefrstW;
impl R {
    #[doc = "Bit 0 - TIMER1 timer reset"]
    #[inline(always)]
    pub fn timer1rst(&self) -> Timer1rstR {
        Timer1rstR::new((self.bits & 1) != 0)
    }
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
    #[doc = "Bit 9 - SLCD reset"]
    #[inline(always)]
    pub fn slcdrst(&self) -> SlcdrstR {
        SlcdrstR::new(((self.bits >> 9) & 1) != 0)
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
    #[doc = "Bit 15 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2rst(&self) -> Spi2rstR {
        Spi2rstR::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bit 25 - CAN0 reset"]
    #[inline(always)]
    pub fn can0rst(&self) -> Can0rstR {
        Can0rstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CAN1 reset"]
    #[inline(always)]
    pub fn can1rst(&self) -> Can1rstR {
        Can1rstR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Power control reset"]
    #[inline(always)]
    pub fn pmurst(&self) -> PmurstR {
        PmurstR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC reset"]
    #[inline(always)]
    pub fn dacrst(&self) -> DacrstR {
        DacrstR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - HDMI CEC reset"]
    #[inline(always)]
    pub fn cecrst(&self) -> CecrstR {
        CecrstR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - OPA and IVREF reset"]
    #[inline(always)]
    pub fn opaivrefrst(&self) -> OpaivrefrstR {
        OpaivrefrstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIMER1 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer1rst(&mut self) -> Timer1rstW<Apb1rstSpec> {
        Timer1rstW::new(self, 0)
    }
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
    #[doc = "Bit 9 - SLCD reset"]
    #[inline(always)]
    #[must_use]
    pub fn slcdrst(&mut self) -> SlcdrstW<Apb1rstSpec> {
        SlcdrstW::new(self, 9)
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
    #[doc = "Bit 15 - SPI2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi2rst(&mut self) -> Spi2rstW<Apb1rstSpec> {
        Spi2rstW::new(self, 15)
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
    #[doc = "Bit 25 - CAN0 reset"]
    #[inline(always)]
    #[must_use]
    pub fn can0rst(&mut self) -> Can0rstW<Apb1rstSpec> {
        Can0rstW::new(self, 25)
    }
    #[doc = "Bit 26 - CAN1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn can1rst(&mut self) -> Can1rstW<Apb1rstSpec> {
        Can1rstW::new(self, 26)
    }
    #[doc = "Bit 28 - Power control reset"]
    #[inline(always)]
    #[must_use]
    pub fn pmurst(&mut self) -> PmurstW<Apb1rstSpec> {
        PmurstW::new(self, 28)
    }
    #[doc = "Bit 29 - DAC reset"]
    #[inline(always)]
    #[must_use]
    pub fn dacrst(&mut self) -> DacrstW<Apb1rstSpec> {
        DacrstW::new(self, 29)
    }
    #[doc = "Bit 30 - HDMI CEC reset"]
    #[inline(always)]
    #[must_use]
    pub fn cecrst(&mut self) -> CecrstW<Apb1rstSpec> {
        CecrstW::new(self, 30)
    }
    #[doc = "Bit 31 - OPA and IVREF reset"]
    #[inline(always)]
    #[must_use]
    pub fn opaivrefrst(&mut self) -> OpaivrefrstW<Apb1rstSpec> {
        OpaivrefrstW::new(self, 31)
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
