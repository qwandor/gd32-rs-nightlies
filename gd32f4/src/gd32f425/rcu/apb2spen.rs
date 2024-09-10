#[doc = "Register `APB2SPEN` reader"]
pub type R = crate::R<Apb2spenSpec>;
#[doc = "Register `APB2SPEN` writer"]
pub type W = crate::W<Apb2spenSpec>;
#[doc = "Field `TIMER0SPEN` reader - TIMER0 clock enable when sleep mode"]
pub type Timer0spenR = crate::BitReader;
#[doc = "Field `TIMER0SPEN` writer - TIMER0 clock enable when sleep mode"]
pub type Timer0spenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER7SPEN` reader - TIMER7 clock enable when sleep mode"]
pub type Timer7spenR = crate::BitReader;
#[doc = "Field `TIMER7SPEN` writer - TIMER7 clock enable when sleep mode"]
pub type Timer7spenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART0SPEN` reader - USART0 clock enable when sleep mode"]
pub type Usart0spenR = crate::BitReader;
#[doc = "Field `USART0SPEN` writer - USART0 clock enable when sleep mode"]
pub type Usart0spenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART5SPEN` reader - USART5 clock enable when sleep mode"]
pub type Usart5spenR = crate::BitReader;
#[doc = "Field `USART5SPEN` writer - USART5 clock enable when sleep mode"]
pub type Usart5spenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC0SPEN` reader - ADC0 clock enable when sleep mode"]
pub type Adc0spenR = crate::BitReader;
#[doc = "Field `ADC0SPEN` writer - ADC0 clock enable when sleep mode"]
pub type Adc0spenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1SPEN` reader - ADC1 clock enable when sleep mode"]
pub type Adc1spenR = crate::BitReader;
#[doc = "Field `ADC1SPEN` writer - ADC1 clock enable when sleep mode"]
pub type Adc1spenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2SPEN` reader - ADC2 clock enable when sleep mode"]
pub type Adc2spenR = crate::BitReader;
#[doc = "Field `ADC2SPEN` writer - ADC2 clock enable when sleep mode"]
pub type Adc2spenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOSPEN` reader - SDIO clock enable when sleep mode"]
pub type SdiospenR = crate::BitReader;
#[doc = "Field `SDIOSPEN` writer - SDIO clock enable when sleep mode"]
pub type SdiospenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI0SPEN` reader - SPI0 clock enable when sleep mode"]
pub type Spi0spenR = crate::BitReader;
#[doc = "Field `SPI0SPEN` writer - SPI0 clock enable when sleep mode"]
pub type Spi0spenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3SPEN` reader - SPI3 clock enable when sleep mode"]
pub type Spi3spenR = crate::BitReader;
#[doc = "Field `SPI3SPEN` writer - SPI3 clock enable when sleep mode"]
pub type Spi3spenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFGSPEN` reader - SYSCFG clock enable when sleep mode"]
pub type SyscfgspenR = crate::BitReader;
#[doc = "Field `SYSCFGSPEN` writer - SYSCFG clock enable when sleep mode"]
pub type SyscfgspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER8SPEN` reader - TIMER8 clock enable when sleep mode"]
pub type Timer8spenR = crate::BitReader;
#[doc = "Field `TIMER8SPEN` writer - TIMER8 clock enable when sleep mode"]
pub type Timer8spenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER9SPEN` reader - TIMER9 clock enable when sleep mode"]
pub type Timer9spenR = crate::BitReader;
#[doc = "Field `TIMER9SPEN` writer - TIMER9 clock enable when sleep mode"]
pub type Timer9spenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER10SPEN` reader - TIMER10 clock enable when sleep mode"]
pub type Timer10spenR = crate::BitReader;
#[doc = "Field `TIMER10SPEN` writer - TIMER10 clock enable when sleep mode"]
pub type Timer10spenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI4SPEN` reader - SPI4 clock enable when sleep mode"]
pub type Spi4spenR = crate::BitReader;
#[doc = "Field `SPI4SPEN` writer - SPI4 clock enable when sleep mode"]
pub type Spi4spenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI5SPEN` reader - SPI5 clock enable when sleep mode"]
pub type Spi5spenR = crate::BitReader;
#[doc = "Field `SPI5SPEN` writer - SPI5 clock enable when sleep mode"]
pub type Spi5spenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TLISPEN` reader - TLI clock enable when sleep mode"]
pub type TlispenR = crate::BitReader;
#[doc = "Field `TLISPEN` writer - TLI clock enable when sleep mode"]
pub type TlispenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIMER0 clock enable when sleep mode"]
    #[inline(always)]
    pub fn timer0spen(&self) -> Timer0spenR {
        Timer0spenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIMER7 clock enable when sleep mode"]
    #[inline(always)]
    pub fn timer7spen(&self) -> Timer7spenR {
        Timer7spenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USART0 clock enable when sleep mode"]
    #[inline(always)]
    pub fn usart0spen(&self) -> Usart0spenR {
        Usart0spenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART5 clock enable when sleep mode"]
    #[inline(always)]
    pub fn usart5spen(&self) -> Usart5spenR {
        Usart5spenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC0 clock enable when sleep mode"]
    #[inline(always)]
    pub fn adc0spen(&self) -> Adc0spenR {
        Adc0spenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC1 clock enable when sleep mode"]
    #[inline(always)]
    pub fn adc1spen(&self) -> Adc1spenR {
        Adc1spenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC2 clock enable when sleep mode"]
    #[inline(always)]
    pub fn adc2spen(&self) -> Adc2spenR {
        Adc2spenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SDIO clock enable when sleep mode"]
    #[inline(always)]
    pub fn sdiospen(&self) -> SdiospenR {
        SdiospenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI0 clock enable when sleep mode"]
    #[inline(always)]
    pub fn spi0spen(&self) -> Spi0spenR {
        Spi0spenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI3 clock enable when sleep mode"]
    #[inline(always)]
    pub fn spi3spen(&self) -> Spi3spenR {
        Spi3spenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SYSCFG clock enable when sleep mode"]
    #[inline(always)]
    pub fn syscfgspen(&self) -> SyscfgspenR {
        SyscfgspenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIMER8 clock enable when sleep mode"]
    #[inline(always)]
    pub fn timer8spen(&self) -> Timer8spenR {
        Timer8spenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIMER9 clock enable when sleep mode"]
    #[inline(always)]
    pub fn timer9spen(&self) -> Timer9spenR {
        Timer9spenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIMER10 clock enable when sleep mode"]
    #[inline(always)]
    pub fn timer10spen(&self) -> Timer10spenR {
        Timer10spenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI4 clock enable when sleep mode"]
    #[inline(always)]
    pub fn spi4spen(&self) -> Spi4spenR {
        Spi4spenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SPI5 clock enable when sleep mode"]
    #[inline(always)]
    pub fn spi5spen(&self) -> Spi5spenR {
        Spi5spenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - TLI clock enable when sleep mode"]
    #[inline(always)]
    pub fn tlispen(&self) -> TlispenR {
        TlispenR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIMER0 clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn timer0spen(&mut self) -> Timer0spenW<Apb2spenSpec> {
        Timer0spenW::new(self, 0)
    }
    #[doc = "Bit 1 - TIMER7 clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn timer7spen(&mut self) -> Timer7spenW<Apb2spenSpec> {
        Timer7spenW::new(self, 1)
    }
    #[doc = "Bit 4 - USART0 clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart0spen(&mut self) -> Usart0spenW<Apb2spenSpec> {
        Usart0spenW::new(self, 4)
    }
    #[doc = "Bit 5 - USART5 clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart5spen(&mut self) -> Usart5spenW<Apb2spenSpec> {
        Usart5spenW::new(self, 5)
    }
    #[doc = "Bit 8 - ADC0 clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn adc0spen(&mut self) -> Adc0spenW<Apb2spenSpec> {
        Adc0spenW::new(self, 8)
    }
    #[doc = "Bit 9 - ADC1 clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn adc1spen(&mut self) -> Adc1spenW<Apb2spenSpec> {
        Adc1spenW::new(self, 9)
    }
    #[doc = "Bit 10 - ADC2 clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn adc2spen(&mut self) -> Adc2spenW<Apb2spenSpec> {
        Adc2spenW::new(self, 10)
    }
    #[doc = "Bit 11 - SDIO clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sdiospen(&mut self) -> SdiospenW<Apb2spenSpec> {
        SdiospenW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI0 clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi0spen(&mut self) -> Spi0spenW<Apb2spenSpec> {
        Spi0spenW::new(self, 12)
    }
    #[doc = "Bit 13 - SPI3 clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi3spen(&mut self) -> Spi3spenW<Apb2spenSpec> {
        Spi3spenW::new(self, 13)
    }
    #[doc = "Bit 14 - SYSCFG clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgspen(&mut self) -> SyscfgspenW<Apb2spenSpec> {
        SyscfgspenW::new(self, 14)
    }
    #[doc = "Bit 16 - TIMER8 clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn timer8spen(&mut self) -> Timer8spenW<Apb2spenSpec> {
        Timer8spenW::new(self, 16)
    }
    #[doc = "Bit 17 - TIMER9 clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn timer9spen(&mut self) -> Timer9spenW<Apb2spenSpec> {
        Timer9spenW::new(self, 17)
    }
    #[doc = "Bit 18 - TIMER10 clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn timer10spen(&mut self) -> Timer10spenW<Apb2spenSpec> {
        Timer10spenW::new(self, 18)
    }
    #[doc = "Bit 20 - SPI4 clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi4spen(&mut self) -> Spi4spenW<Apb2spenSpec> {
        Spi4spenW::new(self, 20)
    }
    #[doc = "Bit 21 - SPI5 clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi5spen(&mut self) -> Spi5spenW<Apb2spenSpec> {
        Spi5spenW::new(self, 21)
    }
    #[doc = "Bit 26 - TLI clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tlispen(&mut self) -> TlispenW<Apb2spenSpec> {
        TlispenW::new(self, 26)
    }
}
#[doc = "APB2 sleep mode enable register (RCU_APB2SPEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2spen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2spen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2spenSpec;
impl crate::RegisterSpec for Apb2spenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2spen::R`](R) reader structure"]
impl crate::Readable for Apb2spenSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2spen::W`](W) writer structure"]
impl crate::Writable for Apb2spenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2SPEN to value 0x0477_7f33"]
impl crate::Resettable for Apb2spenSpec {
    const RESET_VALUE: u32 = 0x0477_7f33;
}
