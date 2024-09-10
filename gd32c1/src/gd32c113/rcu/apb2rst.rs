#[doc = "Register `APB2RST` reader"]
pub type R = crate::R<Apb2rstSpec>;
#[doc = "Register `APB2RST` writer"]
pub type W = crate::W<Apb2rstSpec>;
#[doc = "Alternate function I/O reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Afrst {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<Afrst> for bool {
    #[inline(always)]
    fn from(variant: Afrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AFRST` reader - Alternate function I/O reset"]
pub type AfrstR = crate::BitReader<Afrst>;
impl AfrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Afrst> {
        match self.bits {
            true => Some(Afrst::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Afrst::Reset
    }
}
#[doc = "Field `AFRST` writer - Alternate function I/O reset"]
pub type AfrstW<'a, REG> = crate::BitWriter<'a, REG, Afrst>;
impl<'a, REG> AfrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Afrst::Reset)
    }
}
#[doc = "Field `PARST` reader - GPIO port A reset"]
pub use AfrstR as ParstR;
#[doc = "Field `PBRST` reader - GPIO port B reset"]
pub use AfrstR as PbrstR;
#[doc = "Field `PCRST` reader - GPIO port C reset"]
pub use AfrstR as PcrstR;
#[doc = "Field `PDRST` reader - GPIO port D reset"]
pub use AfrstR as PdrstR;
#[doc = "Field `PERST` reader - GPIO port E reset"]
pub use AfrstR as PerstR;
#[doc = "Field `ADC0RST` reader - ADC0 reset"]
pub use AfrstR as Adc0rstR;
#[doc = "Field `ADC1RST` reader - ADC1 reset"]
pub use AfrstR as Adc1rstR;
#[doc = "Field `TIMER0RST` reader - Timer 0 reset"]
pub use AfrstR as Timer0rstR;
#[doc = "Field `SPI0RST` reader - SPI0 reset"]
pub use AfrstR as Spi0rstR;
#[doc = "Field `TIMER7RST` reader - Timer 7 reset"]
pub use AfrstR as Timer7rstR;
#[doc = "Field `USART0RST` reader - USART0 Reset"]
pub use AfrstR as Usart0rstR;
#[doc = "Field `TIMER8RST` reader - Timer 8 reset"]
pub use AfrstR as Timer8rstR;
#[doc = "Field `TIMER9RST` reader - Timer 9 reset"]
pub use AfrstR as Timer9rstR;
#[doc = "Field `TIMER10RST` reader - Timer 10 reset"]
pub use AfrstR as Timer10rstR;
#[doc = "Field `PARST` writer - GPIO port A reset"]
pub use AfrstW as ParstW;
#[doc = "Field `PBRST` writer - GPIO port B reset"]
pub use AfrstW as PbrstW;
#[doc = "Field `PCRST` writer - GPIO port C reset"]
pub use AfrstW as PcrstW;
#[doc = "Field `PDRST` writer - GPIO port D reset"]
pub use AfrstW as PdrstW;
#[doc = "Field `PERST` writer - GPIO port E reset"]
pub use AfrstW as PerstW;
#[doc = "Field `ADC0RST` writer - ADC0 reset"]
pub use AfrstW as Adc0rstW;
#[doc = "Field `ADC1RST` writer - ADC1 reset"]
pub use AfrstW as Adc1rstW;
#[doc = "Field `TIMER0RST` writer - Timer 0 reset"]
pub use AfrstW as Timer0rstW;
#[doc = "Field `SPI0RST` writer - SPI0 reset"]
pub use AfrstW as Spi0rstW;
#[doc = "Field `TIMER7RST` writer - Timer 7 reset"]
pub use AfrstW as Timer7rstW;
#[doc = "Field `USART0RST` writer - USART0 Reset"]
pub use AfrstW as Usart0rstW;
#[doc = "Field `TIMER8RST` writer - Timer 8 reset"]
pub use AfrstW as Timer8rstW;
#[doc = "Field `TIMER9RST` writer - Timer 9 reset"]
pub use AfrstW as Timer9rstW;
#[doc = "Field `TIMER10RST` writer - Timer 10 reset"]
pub use AfrstW as Timer10rstW;
impl R {
    #[doc = "Bit 0 - Alternate function I/O reset"]
    #[inline(always)]
    pub fn afrst(&self) -> AfrstR {
        AfrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO port A reset"]
    #[inline(always)]
    pub fn parst(&self) -> ParstR {
        ParstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO port B reset"]
    #[inline(always)]
    pub fn pbrst(&self) -> PbrstR {
        PbrstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO port C reset"]
    #[inline(always)]
    pub fn pcrst(&self) -> PcrstR {
        PcrstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO port D reset"]
    #[inline(always)]
    pub fn pdrst(&self) -> PdrstR {
        PdrstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO port E reset"]
    #[inline(always)]
    pub fn perst(&self) -> PerstR {
        PerstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC0 reset"]
    #[inline(always)]
    pub fn adc0rst(&self) -> Adc0rstR {
        Adc0rstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC1 reset"]
    #[inline(always)]
    pub fn adc1rst(&self) -> Adc1rstR {
        Adc1rstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer 0 reset"]
    #[inline(always)]
    pub fn timer0rst(&self) -> Timer0rstR {
        Timer0rstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI0 reset"]
    #[inline(always)]
    pub fn spi0rst(&self) -> Spi0rstR {
        Spi0rstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timer 7 reset"]
    #[inline(always)]
    pub fn timer7rst(&self) -> Timer7rstR {
        Timer7rstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART0 Reset"]
    #[inline(always)]
    pub fn usart0rst(&self) -> Usart0rstR {
        Usart0rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer 8 reset"]
    #[inline(always)]
    pub fn timer8rst(&self) -> Timer8rstR {
        Timer8rstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer 9 reset"]
    #[inline(always)]
    pub fn timer9rst(&self) -> Timer9rstR {
        Timer9rstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer 10 reset"]
    #[inline(always)]
    pub fn timer10rst(&self) -> Timer10rstR {
        Timer10rstR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alternate function I/O reset"]
    #[inline(always)]
    #[must_use]
    pub fn afrst(&mut self) -> AfrstW<Apb2rstSpec> {
        AfrstW::new(self, 0)
    }
    #[doc = "Bit 2 - GPIO port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn parst(&mut self) -> ParstW<Apb2rstSpec> {
        ParstW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst(&mut self) -> PbrstW<Apb2rstSpec> {
        PbrstW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst(&mut self) -> PcrstW<Apb2rstSpec> {
        PcrstW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO port D reset"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst(&mut self) -> PdrstW<Apb2rstSpec> {
        PdrstW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO port E reset"]
    #[inline(always)]
    #[must_use]
    pub fn perst(&mut self) -> PerstW<Apb2rstSpec> {
        PerstW::new(self, 6)
    }
    #[doc = "Bit 9 - ADC0 reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc0rst(&mut self) -> Adc0rstW<Apb2rstSpec> {
        Adc0rstW::new(self, 9)
    }
    #[doc = "Bit 10 - ADC1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc1rst(&mut self) -> Adc1rstW<Apb2rstSpec> {
        Adc1rstW::new(self, 10)
    }
    #[doc = "Bit 11 - Timer 0 reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer0rst(&mut self) -> Timer0rstW<Apb2rstSpec> {
        Timer0rstW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI0 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi0rst(&mut self) -> Spi0rstW<Apb2rstSpec> {
        Spi0rstW::new(self, 12)
    }
    #[doc = "Bit 13 - Timer 7 reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer7rst(&mut self) -> Timer7rstW<Apb2rstSpec> {
        Timer7rstW::new(self, 13)
    }
    #[doc = "Bit 14 - USART0 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart0rst(&mut self) -> Usart0rstW<Apb2rstSpec> {
        Usart0rstW::new(self, 14)
    }
    #[doc = "Bit 19 - Timer 8 reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer8rst(&mut self) -> Timer8rstW<Apb2rstSpec> {
        Timer8rstW::new(self, 19)
    }
    #[doc = "Bit 20 - Timer 9 reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer9rst(&mut self) -> Timer9rstW<Apb2rstSpec> {
        Timer9rstW::new(self, 20)
    }
    #[doc = "Bit 21 - Timer 10 reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer10rst(&mut self) -> Timer10rstW<Apb2rstSpec> {
        Timer10rstW::new(self, 21)
    }
}
#[doc = "APB2 reset register (RCU_APB2RST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2rstSpec;
impl crate::RegisterSpec for Apb2rstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rst::R`](R) reader structure"]
impl crate::Readable for Apb2rstSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2rst::W`](W) writer structure"]
impl crate::Writable for Apb2rstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2RST to value 0"]
impl crate::Resettable for Apb2rstSpec {
    const RESET_VALUE: u32 = 0;
}
