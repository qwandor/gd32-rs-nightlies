#[doc = "Register `APB2RST` reader"]
pub type R = crate::R<Apb2rstSpec>;
#[doc = "Register `APB2RST` writer"]
pub type W = crate::W<Apb2rstSpec>;
#[doc = "System configuration and comparator reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfgcmprst {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<Cfgcmprst> for bool {
    #[inline(always)]
    fn from(variant: Cfgcmprst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGCMPRST` reader - System configuration and comparator reset"]
pub type CfgcmprstR = crate::BitReader<Cfgcmprst>;
impl CfgcmprstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cfgcmprst> {
        match self.bits {
            true => Some(Cfgcmprst::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Cfgcmprst::Reset
    }
}
#[doc = "Field `CFGCMPRST` writer - System configuration and comparator reset"]
pub type CfgcmprstW<'a, REG> = crate::BitWriter<'a, REG, Cfgcmprst>;
impl<'a, REG> CfgcmprstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Cfgcmprst::Reset)
    }
}
#[doc = "Field `ADCRST` reader - ADC reset"]
pub use CfgcmprstR as AdcrstR;
#[doc = "Field `TIMER0RST` reader - TIMER0 reset"]
pub use CfgcmprstR as Timer0rstR;
#[doc = "Field `SPI0RST` reader - SPI0 Reset"]
pub use CfgcmprstR as Spi0rstR;
#[doc = "Field `USART0RST` reader - USART0 Reset"]
pub use CfgcmprstR as Usart0rstR;
#[doc = "Field `TIMER14RST` reader - TIMER14 reset"]
pub use CfgcmprstR as Timer14rstR;
#[doc = "Field `TIMER15RST` reader - TIMER15 reset"]
pub use CfgcmprstR as Timer15rstR;
#[doc = "Field `TIMER16RST` reader - TIMER16 reset"]
pub use CfgcmprstR as Timer16rstR;
#[doc = "Field `ADCRST` writer - ADC reset"]
pub use CfgcmprstW as AdcrstW;
#[doc = "Field `TIMER0RST` writer - TIMER0 reset"]
pub use CfgcmprstW as Timer0rstW;
#[doc = "Field `SPI0RST` writer - SPI0 Reset"]
pub use CfgcmprstW as Spi0rstW;
#[doc = "Field `USART0RST` writer - USART0 Reset"]
pub use CfgcmprstW as Usart0rstW;
#[doc = "Field `TIMER14RST` writer - TIMER14 reset"]
pub use CfgcmprstW as Timer14rstW;
#[doc = "Field `TIMER15RST` writer - TIMER15 reset"]
pub use CfgcmprstW as Timer15rstW;
#[doc = "Field `TIMER16RST` writer - TIMER16 reset"]
pub use CfgcmprstW as Timer16rstW;
impl R {
    #[doc = "Bit 0 - System configuration and comparator reset"]
    #[inline(always)]
    pub fn cfgcmprst(&self) -> CfgcmprstR {
        CfgcmprstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9 - ADC reset"]
    #[inline(always)]
    pub fn adcrst(&self) -> AdcrstR {
        AdcrstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - TIMER0 reset"]
    #[inline(always)]
    pub fn timer0rst(&self) -> Timer0rstR {
        Timer0rstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI0 Reset"]
    #[inline(always)]
    pub fn spi0rst(&self) -> Spi0rstR {
        Spi0rstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART0 Reset"]
    #[inline(always)]
    pub fn usart0rst(&self) -> Usart0rstR {
        Usart0rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIMER14 reset"]
    #[inline(always)]
    pub fn timer14rst(&self) -> Timer14rstR {
        Timer14rstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIMER15 reset"]
    #[inline(always)]
    pub fn timer15rst(&self) -> Timer15rstR {
        Timer15rstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIMER16 reset"]
    #[inline(always)]
    pub fn timer16rst(&self) -> Timer16rstR {
        Timer16rstR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System configuration and comparator reset"]
    #[inline(always)]
    #[must_use]
    pub fn cfgcmprst(&mut self) -> CfgcmprstW<Apb2rstSpec> {
        CfgcmprstW::new(self, 0)
    }
    #[doc = "Bit 9 - ADC reset"]
    #[inline(always)]
    #[must_use]
    pub fn adcrst(&mut self) -> AdcrstW<Apb2rstSpec> {
        AdcrstW::new(self, 9)
    }
    #[doc = "Bit 11 - TIMER0 reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer0rst(&mut self) -> Timer0rstW<Apb2rstSpec> {
        Timer0rstW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI0 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi0rst(&mut self) -> Spi0rstW<Apb2rstSpec> {
        Spi0rstW::new(self, 12)
    }
    #[doc = "Bit 14 - USART0 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart0rst(&mut self) -> Usart0rstW<Apb2rstSpec> {
        Usart0rstW::new(self, 14)
    }
    #[doc = "Bit 16 - TIMER14 reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer14rst(&mut self) -> Timer14rstW<Apb2rstSpec> {
        Timer14rstW::new(self, 16)
    }
    #[doc = "Bit 17 - TIMER15 reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer15rst(&mut self) -> Timer15rstW<Apb2rstSpec> {
        Timer15rstW::new(self, 17)
    }
    #[doc = "Bit 18 - TIMER16 reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer16rst(&mut self) -> Timer16rstW<Apb2rstSpec> {
        Timer16rstW::new(self, 18)
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
