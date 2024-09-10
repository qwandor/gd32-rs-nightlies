#[doc = "Register `APB2EN` reader"]
pub type R = crate::R<Apb2enSpec>;
#[doc = "Register `APB2EN` writer"]
pub type W = crate::W<Apb2enSpec>;
#[doc = "System configuration and comparator clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfgcmpen {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<Cfgcmpen> for bool {
    #[inline(always)]
    fn from(variant: Cfgcmpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGCMPEN` reader - System configuration and comparator clock enable"]
pub type CfgcmpenR = crate::BitReader<Cfgcmpen>;
impl CfgcmpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfgcmpen {
        match self.bits {
            false => Cfgcmpen::Disabled,
            true => Cfgcmpen::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cfgcmpen::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cfgcmpen::Enabled
    }
}
#[doc = "Field `CFGCMPEN` writer - System configuration and comparator clock enable"]
pub type CfgcmpenW<'a, REG> = crate::BitWriter<'a, REG, Cfgcmpen>;
impl<'a, REG> CfgcmpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cfgcmpen::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cfgcmpen::Enabled)
    }
}
#[doc = "Field `ADCEN` reader - ADC interface clock enable"]
pub use CfgcmpenR as AdcenR;
#[doc = "Field `TIMER0EN` reader - TIMER0 timer clock enable"]
pub use CfgcmpenR as Timer0enR;
#[doc = "Field `SPI0EN` reader - SPI0 clock enable"]
pub use CfgcmpenR as Spi0enR;
#[doc = "Field `USART0EN` reader - USART0 clock enable"]
pub use CfgcmpenR as Usart0enR;
#[doc = "Field `TIMER14EN` reader - TIMER14 timer clock enable"]
pub use CfgcmpenR as Timer14enR;
#[doc = "Field `TIMER15EN` reader - TIMER15 timer clock enable"]
pub use CfgcmpenR as Timer15enR;
#[doc = "Field `TIMER16EN` reader - TIMER16 timer clock enable"]
pub use CfgcmpenR as Timer16enR;
#[doc = "Field `ADCEN` writer - ADC interface clock enable"]
pub use CfgcmpenW as AdcenW;
#[doc = "Field `TIMER0EN` writer - TIMER0 timer clock enable"]
pub use CfgcmpenW as Timer0enW;
#[doc = "Field `SPI0EN` writer - SPI0 clock enable"]
pub use CfgcmpenW as Spi0enW;
#[doc = "Field `USART0EN` writer - USART0 clock enable"]
pub use CfgcmpenW as Usart0enW;
#[doc = "Field `TIMER14EN` writer - TIMER14 timer clock enable"]
pub use CfgcmpenW as Timer14enW;
#[doc = "Field `TIMER15EN` writer - TIMER15 timer clock enable"]
pub use CfgcmpenW as Timer15enW;
#[doc = "Field `TIMER16EN` writer - TIMER16 timer clock enable"]
pub use CfgcmpenW as Timer16enW;
impl R {
    #[doc = "Bit 0 - System configuration and comparator clock enable"]
    #[inline(always)]
    pub fn cfgcmpen(&self) -> CfgcmpenR {
        CfgcmpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9 - ADC interface clock enable"]
    #[inline(always)]
    pub fn adcen(&self) -> AdcenR {
        AdcenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - TIMER0 timer clock enable"]
    #[inline(always)]
    pub fn timer0en(&self) -> Timer0enR {
        Timer0enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI0 clock enable"]
    #[inline(always)]
    pub fn spi0en(&self) -> Spi0enR {
        Spi0enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART0 clock enable"]
    #[inline(always)]
    pub fn usart0en(&self) -> Usart0enR {
        Usart0enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIMER14 timer clock enable"]
    #[inline(always)]
    pub fn timer14en(&self) -> Timer14enR {
        Timer14enR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIMER15 timer clock enable"]
    #[inline(always)]
    pub fn timer15en(&self) -> Timer15enR {
        Timer15enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIMER16 timer clock enable"]
    #[inline(always)]
    pub fn timer16en(&self) -> Timer16enR {
        Timer16enR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System configuration and comparator clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfgcmpen(&mut self) -> CfgcmpenW<Apb2enSpec> {
        CfgcmpenW::new(self, 0)
    }
    #[doc = "Bit 9 - ADC interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> AdcenW<Apb2enSpec> {
        AdcenW::new(self, 9)
    }
    #[doc = "Bit 11 - TIMER0 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer0en(&mut self) -> Timer0enW<Apb2enSpec> {
        Timer0enW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI0 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi0en(&mut self) -> Spi0enW<Apb2enSpec> {
        Spi0enW::new(self, 12)
    }
    #[doc = "Bit 14 - USART0 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart0en(&mut self) -> Usart0enW<Apb2enSpec> {
        Usart0enW::new(self, 14)
    }
    #[doc = "Bit 16 - TIMER14 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer14en(&mut self) -> Timer14enW<Apb2enSpec> {
        Timer14enW::new(self, 16)
    }
    #[doc = "Bit 17 - TIMER15 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer15en(&mut self) -> Timer15enW<Apb2enSpec> {
        Timer15enW::new(self, 17)
    }
    #[doc = "Bit 18 - TIMER16 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer16en(&mut self) -> Timer16enW<Apb2enSpec> {
        Timer16enW::new(self, 18)
    }
}
#[doc = "APB2 enable register (RCU_APB2EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2enSpec;
impl crate::RegisterSpec for Apb2enSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2en::R`](R) reader structure"]
impl crate::Readable for Apb2enSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2en::W`](W) writer structure"]
impl crate::Writable for Apb2enSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2EN to value 0"]
impl crate::Resettable for Apb2enSpec {
    const RESET_VALUE: u32 = 0;
}
