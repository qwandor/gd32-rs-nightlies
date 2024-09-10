#[doc = "Register `APB2RST` reader"]
pub type R = crate::R<Apb2rstSpec>;
#[doc = "Register `APB2RST` writer"]
pub type W = crate::W<Apb2rstSpec>;
#[doc = "Field `TIMER0RST` reader - TIMER0 reset"]
pub type Timer0rstR = crate::BitReader;
#[doc = "Field `TIMER0RST` writer - TIMER0 reset"]
pub type Timer0rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER7RST` reader - TIMER7 reset"]
pub type Timer7rstR = crate::BitReader;
#[doc = "Field `TIMER7RST` writer - TIMER7 reset"]
pub type Timer7rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART0RST` reader - USART0 reset"]
pub type Usart0rstR = crate::BitReader;
#[doc = "Field `USART0RST` writer - USART0 reset"]
pub type Usart0rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART5RST` reader - USART5 reset"]
pub type Usart5rstR = crate::BitReader;
#[doc = "Field `USART5RST` writer - USART5 reset"]
pub type Usart5rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCRST` reader - ADC reset"]
pub type AdcrstR = crate::BitReader;
#[doc = "Field `ADCRST` writer - ADC reset"]
pub type AdcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIORST` reader - SDIO reset"]
pub type SdiorstR = crate::BitReader;
#[doc = "Field `SDIORST` writer - SDIO reset"]
pub type SdiorstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI0RST` reader - SPI0 Reset"]
pub type Spi0rstR = crate::BitReader;
#[doc = "Field `SPI0RST` writer - SPI0 Reset"]
pub type Spi0rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3RST` reader - SPI3 Reset"]
pub type Spi3rstR = crate::BitReader;
#[doc = "Field `SPI3RST` writer - SPI3 Reset"]
pub type Spi3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFGRST` reader - SYSCFG Reset"]
pub type SyscfgrstR = crate::BitReader;
#[doc = "Field `SYSCFGRST` writer - SYSCFG Reset"]
pub type SyscfgrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER8RST` reader - TIMER8 reset"]
pub type Timer8rstR = crate::BitReader;
#[doc = "Field `TIMER8RST` writer - TIMER8 reset"]
pub type Timer8rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER9RST` reader - TIMER9 reset"]
pub type Timer9rstR = crate::BitReader;
#[doc = "Field `TIMER9RST` writer - TIMER9 reset"]
pub type Timer9rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER10RST` reader - TIMER10 reset"]
pub type Timer10rstR = crate::BitReader;
#[doc = "Field `TIMER10RST` writer - TIMER10 reset"]
pub type Timer10rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI4RST` reader - SPI4 Reset"]
pub type Spi4rstR = crate::BitReader;
#[doc = "Field `SPI4RST` writer - SPI4 Reset"]
pub type Spi4rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI5RST` reader - SPI5 Reset"]
pub type Spi5rstR = crate::BitReader;
#[doc = "Field `SPI5RST` writer - SPI5 Reset"]
pub type Spi5rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TLIRST` reader - TLI Reset"]
pub type TlirstR = crate::BitReader;
#[doc = "Field `TLIRST` writer - TLI Reset"]
pub type TlirstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIMER0 reset"]
    #[inline(always)]
    pub fn timer0rst(&self) -> Timer0rstR {
        Timer0rstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIMER7 reset"]
    #[inline(always)]
    pub fn timer7rst(&self) -> Timer7rstR {
        Timer7rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USART0 reset"]
    #[inline(always)]
    pub fn usart0rst(&self) -> Usart0rstR {
        Usart0rstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART5 reset"]
    #[inline(always)]
    pub fn usart5rst(&self) -> Usart5rstR {
        Usart5rstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC reset"]
    #[inline(always)]
    pub fn adcrst(&self) -> AdcrstR {
        AdcrstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - SDIO reset"]
    #[inline(always)]
    pub fn sdiorst(&self) -> SdiorstR {
        SdiorstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI0 Reset"]
    #[inline(always)]
    pub fn spi0rst(&self) -> Spi0rstR {
        Spi0rstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI3 Reset"]
    #[inline(always)]
    pub fn spi3rst(&self) -> Spi3rstR {
        Spi3rstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SYSCFG Reset"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SyscfgrstR {
        SyscfgrstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIMER8 reset"]
    #[inline(always)]
    pub fn timer8rst(&self) -> Timer8rstR {
        Timer8rstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIMER9 reset"]
    #[inline(always)]
    pub fn timer9rst(&self) -> Timer9rstR {
        Timer9rstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIMER10 reset"]
    #[inline(always)]
    pub fn timer10rst(&self) -> Timer10rstR {
        Timer10rstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI4 Reset"]
    #[inline(always)]
    pub fn spi4rst(&self) -> Spi4rstR {
        Spi4rstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SPI5 Reset"]
    #[inline(always)]
    pub fn spi5rst(&self) -> Spi5rstR {
        Spi5rstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - TLI Reset"]
    #[inline(always)]
    pub fn tlirst(&self) -> TlirstR {
        TlirstR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIMER0 reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer0rst(&mut self) -> Timer0rstW<Apb2rstSpec> {
        Timer0rstW::new(self, 0)
    }
    #[doc = "Bit 1 - TIMER7 reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer7rst(&mut self) -> Timer7rstW<Apb2rstSpec> {
        Timer7rstW::new(self, 1)
    }
    #[doc = "Bit 4 - USART0 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart0rst(&mut self) -> Usart0rstW<Apb2rstSpec> {
        Usart0rstW::new(self, 4)
    }
    #[doc = "Bit 5 - USART5 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart5rst(&mut self) -> Usart5rstW<Apb2rstSpec> {
        Usart5rstW::new(self, 5)
    }
    #[doc = "Bit 8 - ADC reset"]
    #[inline(always)]
    #[must_use]
    pub fn adcrst(&mut self) -> AdcrstW<Apb2rstSpec> {
        AdcrstW::new(self, 8)
    }
    #[doc = "Bit 11 - SDIO reset"]
    #[inline(always)]
    #[must_use]
    pub fn sdiorst(&mut self) -> SdiorstW<Apb2rstSpec> {
        SdiorstW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI0 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi0rst(&mut self) -> Spi0rstW<Apb2rstSpec> {
        Spi0rstW::new(self, 12)
    }
    #[doc = "Bit 13 - SPI3 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi3rst(&mut self) -> Spi3rstW<Apb2rstSpec> {
        Spi3rstW::new(self, 13)
    }
    #[doc = "Bit 14 - SYSCFG Reset"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgrst(&mut self) -> SyscfgrstW<Apb2rstSpec> {
        SyscfgrstW::new(self, 14)
    }
    #[doc = "Bit 16 - TIMER8 reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer8rst(&mut self) -> Timer8rstW<Apb2rstSpec> {
        Timer8rstW::new(self, 16)
    }
    #[doc = "Bit 17 - TIMER9 reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer9rst(&mut self) -> Timer9rstW<Apb2rstSpec> {
        Timer9rstW::new(self, 17)
    }
    #[doc = "Bit 18 - TIMER10 reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer10rst(&mut self) -> Timer10rstW<Apb2rstSpec> {
        Timer10rstW::new(self, 18)
    }
    #[doc = "Bit 20 - SPI4 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi4rst(&mut self) -> Spi4rstW<Apb2rstSpec> {
        Spi4rstW::new(self, 20)
    }
    #[doc = "Bit 21 - SPI5 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi5rst(&mut self) -> Spi5rstW<Apb2rstSpec> {
        Spi5rstW::new(self, 21)
    }
    #[doc = "Bit 26 - TLI Reset"]
    #[inline(always)]
    #[must_use]
    pub fn tlirst(&mut self) -> TlirstW<Apb2rstSpec> {
        TlirstW::new(self, 26)
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
