#[doc = "Register `CFG0` reader"]
pub type R = crate::R<Cfg0Spec>;
#[doc = "Register `CFG0` writer"]
pub type W = crate::W<Cfg0Spec>;
#[doc = "Boot mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BootMode {
    #[doc = "0: Boot from main flash"]
    Flash = 0,
    #[doc = "1: Boot from system memory"]
    SystemMemory = 1,
    #[doc = "3: Boot from embedded SRAM"]
    Sram = 3,
}
impl From<BootMode> for u8 {
    #[inline(always)]
    fn from(variant: BootMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BootMode {
    type Ux = u8;
}
#[doc = "Field `BOOT_MODE` reader - Boot mode"]
pub type BootModeR = crate::FieldReader<BootMode>;
impl BootModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BootMode> {
        match self.bits {
            0 => Some(BootMode::Flash),
            1 => Some(BootMode::SystemMemory),
            3 => Some(BootMode::Sram),
            _ => None,
        }
    }
    #[doc = "Boot from main flash"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == BootMode::Flash
    }
    #[doc = "Boot from system memory"]
    #[inline(always)]
    pub fn is_system_memory(&self) -> bool {
        *self == BootMode::SystemMemory
    }
    #[doc = "Boot from embedded SRAM"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == BootMode::Sram
    }
}
#[doc = "Field `PA11_PA12_RMP` reader - PA11 and PA12 remapping bit for small packages"]
pub type Pa11Pa12RmpR = crate::BitReader;
#[doc = "Field `PA11_PA12_RMP` writer - PA11 and PA12 remapping bit for small packages"]
pub type Pa11Pa12RmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "ADC DMA request remapping enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdcDmaRmp {
    #[doc = "0: ADC DMA requests are mapped to DMA channel 0"]
    Channel0 = 0,
    #[doc = "1: ADC DMA requests are remapped to DMA channel 1"]
    Channel1 = 1,
}
impl From<AdcDmaRmp> for bool {
    #[inline(always)]
    fn from(variant: AdcDmaRmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_DMA_RMP` reader - ADC DMA request remapping enable"]
pub type AdcDmaRmpR = crate::BitReader<AdcDmaRmp>;
impl AdcDmaRmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AdcDmaRmp {
        match self.bits {
            false => AdcDmaRmp::Channel0,
            true => AdcDmaRmp::Channel1,
        }
    }
    #[doc = "ADC DMA requests are mapped to DMA channel 0"]
    #[inline(always)]
    pub fn is_channel0(&self) -> bool {
        *self == AdcDmaRmp::Channel0
    }
    #[doc = "ADC DMA requests are remapped to DMA channel 1"]
    #[inline(always)]
    pub fn is_channel1(&self) -> bool {
        *self == AdcDmaRmp::Channel1
    }
}
#[doc = "Field `ADC_DMA_RMP` writer - ADC DMA request remapping enable"]
pub type AdcDmaRmpW<'a, REG> = crate::BitWriter<'a, REG, AdcDmaRmp>;
impl<'a, REG> AdcDmaRmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC DMA requests are mapped to DMA channel 0"]
    #[inline(always)]
    pub fn channel0(self) -> &'a mut crate::W<REG> {
        self.variant(AdcDmaRmp::Channel0)
    }
    #[doc = "ADC DMA requests are remapped to DMA channel 1"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut crate::W<REG> {
        self.variant(AdcDmaRmp::Channel1)
    }
}
#[doc = "USART0_TX DMA request remapping enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usart0TxDmaRmp {
    #[doc = "0: USART0 TX DMA requests are mapped to DMA channel 1"]
    Channel1 = 0,
    #[doc = "1: USART0 TX DMA requests are remapped to DMA channel 3"]
    Channel3 = 1,
}
impl From<Usart0TxDmaRmp> for bool {
    #[inline(always)]
    fn from(variant: Usart0TxDmaRmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART0_TX_DMA_RMP` reader - USART0_TX DMA request remapping enable"]
pub type Usart0TxDmaRmpR = crate::BitReader<Usart0TxDmaRmp>;
impl Usart0TxDmaRmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart0TxDmaRmp {
        match self.bits {
            false => Usart0TxDmaRmp::Channel1,
            true => Usart0TxDmaRmp::Channel3,
        }
    }
    #[doc = "USART0 TX DMA requests are mapped to DMA channel 1"]
    #[inline(always)]
    pub fn is_channel1(&self) -> bool {
        *self == Usart0TxDmaRmp::Channel1
    }
    #[doc = "USART0 TX DMA requests are remapped to DMA channel 3"]
    #[inline(always)]
    pub fn is_channel3(&self) -> bool {
        *self == Usart0TxDmaRmp::Channel3
    }
}
#[doc = "Field `USART0_TX_DMA_RMP` writer - USART0_TX DMA request remapping enable"]
pub type Usart0TxDmaRmpW<'a, REG> = crate::BitWriter<'a, REG, Usart0TxDmaRmp>;
impl<'a, REG> Usart0TxDmaRmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART0 TX DMA requests are mapped to DMA channel 1"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart0TxDmaRmp::Channel1)
    }
    #[doc = "USART0 TX DMA requests are remapped to DMA channel 3"]
    #[inline(always)]
    pub fn channel3(self) -> &'a mut crate::W<REG> {
        self.variant(Usart0TxDmaRmp::Channel3)
    }
}
#[doc = "USART0_RX DMA request remapping enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usart0RxDmaRmp {
    #[doc = "0: USART0 RX DMA requests are mapped to DMA channel 2"]
    Channel2 = 0,
    #[doc = "1: USART0 RX DMA requests are remapped to DMA channel 4"]
    Channel4 = 1,
}
impl From<Usart0RxDmaRmp> for bool {
    #[inline(always)]
    fn from(variant: Usart0RxDmaRmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART0_RX_DMA_RMP` reader - USART0_RX DMA request remapping enable"]
pub type Usart0RxDmaRmpR = crate::BitReader<Usart0RxDmaRmp>;
impl Usart0RxDmaRmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart0RxDmaRmp {
        match self.bits {
            false => Usart0RxDmaRmp::Channel2,
            true => Usart0RxDmaRmp::Channel4,
        }
    }
    #[doc = "USART0 RX DMA requests are mapped to DMA channel 2"]
    #[inline(always)]
    pub fn is_channel2(&self) -> bool {
        *self == Usart0RxDmaRmp::Channel2
    }
    #[doc = "USART0 RX DMA requests are remapped to DMA channel 4"]
    #[inline(always)]
    pub fn is_channel4(&self) -> bool {
        *self == Usart0RxDmaRmp::Channel4
    }
}
#[doc = "Field `USART0_RX_DMA_RMP` writer - USART0_RX DMA request remapping enable"]
pub type Usart0RxDmaRmpW<'a, REG> = crate::BitWriter<'a, REG, Usart0RxDmaRmp>;
impl<'a, REG> Usart0RxDmaRmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART0 RX DMA requests are mapped to DMA channel 2"]
    #[inline(always)]
    pub fn channel2(self) -> &'a mut crate::W<REG> {
        self.variant(Usart0RxDmaRmp::Channel2)
    }
    #[doc = "USART0 RX DMA requests are remapped to DMA channel 4"]
    #[inline(always)]
    pub fn channel4(self) -> &'a mut crate::W<REG> {
        self.variant(Usart0RxDmaRmp::Channel4)
    }
}
#[doc = "Timer 15 DMA request remapping enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer15DmaRmp {
    #[doc = "0: Timer 15 DMA requests are mapped to DMA channel 2"]
    Channel2 = 0,
    #[doc = "1: Timer 15 DMA requests are remapped to DMA channel 3"]
    Channel3 = 1,
}
impl From<Timer15DmaRmp> for bool {
    #[inline(always)]
    fn from(variant: Timer15DmaRmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER15_DMA_RMP` reader - Timer 15 DMA request remapping enable"]
pub type Timer15DmaRmpR = crate::BitReader<Timer15DmaRmp>;
impl Timer15DmaRmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer15DmaRmp {
        match self.bits {
            false => Timer15DmaRmp::Channel2,
            true => Timer15DmaRmp::Channel3,
        }
    }
    #[doc = "Timer 15 DMA requests are mapped to DMA channel 2"]
    #[inline(always)]
    pub fn is_channel2(&self) -> bool {
        *self == Timer15DmaRmp::Channel2
    }
    #[doc = "Timer 15 DMA requests are remapped to DMA channel 3"]
    #[inline(always)]
    pub fn is_channel3(&self) -> bool {
        *self == Timer15DmaRmp::Channel3
    }
}
#[doc = "Field `TIMER15_DMA_RMP` writer - Timer 15 DMA request remapping enable"]
pub type Timer15DmaRmpW<'a, REG> = crate::BitWriter<'a, REG, Timer15DmaRmp>;
impl<'a, REG> Timer15DmaRmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer 15 DMA requests are mapped to DMA channel 2"]
    #[inline(always)]
    pub fn channel2(self) -> &'a mut crate::W<REG> {
        self.variant(Timer15DmaRmp::Channel2)
    }
    #[doc = "Timer 15 DMA requests are remapped to DMA channel 3"]
    #[inline(always)]
    pub fn channel3(self) -> &'a mut crate::W<REG> {
        self.variant(Timer15DmaRmp::Channel3)
    }
}
#[doc = "Timer 16 DMA request remapping enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer16DmaRmp {
    #[doc = "0: Timer 16 DMA requests are mapped to DMA channel 0"]
    Channel0 = 0,
    #[doc = "1: Timer 16 DMA requests are remapped to DMA channel 1"]
    Channel1 = 1,
}
impl From<Timer16DmaRmp> for bool {
    #[inline(always)]
    fn from(variant: Timer16DmaRmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER16_DMA_RMP` reader - Timer 16 DMA request remapping enable"]
pub type Timer16DmaRmpR = crate::BitReader<Timer16DmaRmp>;
impl Timer16DmaRmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer16DmaRmp {
        match self.bits {
            false => Timer16DmaRmp::Channel0,
            true => Timer16DmaRmp::Channel1,
        }
    }
    #[doc = "Timer 16 DMA requests are mapped to DMA channel 0"]
    #[inline(always)]
    pub fn is_channel0(&self) -> bool {
        *self == Timer16DmaRmp::Channel0
    }
    #[doc = "Timer 16 DMA requests are remapped to DMA channel 1"]
    #[inline(always)]
    pub fn is_channel1(&self) -> bool {
        *self == Timer16DmaRmp::Channel1
    }
}
#[doc = "Field `TIMER16_DMA_RMP` writer - Timer 16 DMA request remapping enable"]
pub type Timer16DmaRmpW<'a, REG> = crate::BitWriter<'a, REG, Timer16DmaRmp>;
impl<'a, REG> Timer16DmaRmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer 16 DMA requests are mapped to DMA channel 0"]
    #[inline(always)]
    pub fn channel0(self) -> &'a mut crate::W<REG> {
        self.variant(Timer16DmaRmp::Channel0)
    }
    #[doc = "Timer 16 DMA requests are remapped to DMA channel 1"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut crate::W<REG> {
        self.variant(Timer16DmaRmp::Channel1)
    }
}
#[doc = "PB9 pin high current capability enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pb9Hcce {
    #[doc = "0: High current capability is disabled"]
    LowCurrent = 0,
    #[doc = "1: High current capability is enabled, and speed control is bypassed"]
    HighCurrent = 1,
}
impl From<Pb9Hcce> for bool {
    #[inline(always)]
    fn from(variant: Pb9Hcce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PB9_HCCE` reader - PB9 pin high current capability enable"]
pub type Pb9HcceR = crate::BitReader<Pb9Hcce>;
impl Pb9HcceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pb9Hcce {
        match self.bits {
            false => Pb9Hcce::LowCurrent,
            true => Pb9Hcce::HighCurrent,
        }
    }
    #[doc = "High current capability is disabled"]
    #[inline(always)]
    pub fn is_low_current(&self) -> bool {
        *self == Pb9Hcce::LowCurrent
    }
    #[doc = "High current capability is enabled, and speed control is bypassed"]
    #[inline(always)]
    pub fn is_high_current(&self) -> bool {
        *self == Pb9Hcce::HighCurrent
    }
}
#[doc = "Field `PB9_HCCE` writer - PB9 pin high current capability enable"]
pub type Pb9HcceW<'a, REG> = crate::BitWriter<'a, REG, Pb9Hcce>;
impl<'a, REG> Pb9HcceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High current capability is disabled"]
    #[inline(always)]
    pub fn low_current(self) -> &'a mut crate::W<REG> {
        self.variant(Pb9Hcce::LowCurrent)
    }
    #[doc = "High current capability is enabled, and speed control is bypassed"]
    #[inline(always)]
    pub fn high_current(self) -> &'a mut crate::W<REG> {
        self.variant(Pb9Hcce::HighCurrent)
    }
}
impl R {
    #[doc = "Bits 0:1 - Boot mode"]
    #[inline(always)]
    pub fn boot_mode(&self) -> BootModeR {
        BootModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - PA11 and PA12 remapping bit for small packages"]
    #[inline(always)]
    pub fn pa11_pa12_rmp(&self) -> Pa11Pa12RmpR {
        Pa11Pa12RmpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC DMA request remapping enable"]
    #[inline(always)]
    pub fn adc_dma_rmp(&self) -> AdcDmaRmpR {
        AdcDmaRmpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USART0_TX DMA request remapping enable"]
    #[inline(always)]
    pub fn usart0_tx_dma_rmp(&self) -> Usart0TxDmaRmpR {
        Usart0TxDmaRmpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USART0_RX DMA request remapping enable"]
    #[inline(always)]
    pub fn usart0_rx_dma_rmp(&self) -> Usart0RxDmaRmpR {
        Usart0RxDmaRmpR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer 15 DMA request remapping enable"]
    #[inline(always)]
    pub fn timer15_dma_rmp(&self) -> Timer15DmaRmpR {
        Timer15DmaRmpR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timer 16 DMA request remapping enable"]
    #[inline(always)]
    pub fn timer16_dma_rmp(&self) -> Timer16DmaRmpR {
        Timer16DmaRmpR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 19 - PB9 pin high current capability enable"]
    #[inline(always)]
    pub fn pb9_hcce(&self) -> Pb9HcceR {
        Pb9HcceR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - PA11 and PA12 remapping bit for small packages"]
    #[inline(always)]
    #[must_use]
    pub fn pa11_pa12_rmp(&mut self) -> Pa11Pa12RmpW<Cfg0Spec> {
        Pa11Pa12RmpW::new(self, 4)
    }
    #[doc = "Bit 8 - ADC DMA request remapping enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dma_rmp(&mut self) -> AdcDmaRmpW<Cfg0Spec> {
        AdcDmaRmpW::new(self, 8)
    }
    #[doc = "Bit 9 - USART0_TX DMA request remapping enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart0_tx_dma_rmp(&mut self) -> Usart0TxDmaRmpW<Cfg0Spec> {
        Usart0TxDmaRmpW::new(self, 9)
    }
    #[doc = "Bit 10 - USART0_RX DMA request remapping enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart0_rx_dma_rmp(&mut self) -> Usart0RxDmaRmpW<Cfg0Spec> {
        Usart0RxDmaRmpW::new(self, 10)
    }
    #[doc = "Bit 11 - Timer 15 DMA request remapping enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer15_dma_rmp(&mut self) -> Timer15DmaRmpW<Cfg0Spec> {
        Timer15DmaRmpW::new(self, 11)
    }
    #[doc = "Bit 12 - Timer 16 DMA request remapping enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer16_dma_rmp(&mut self) -> Timer16DmaRmpW<Cfg0Spec> {
        Timer16DmaRmpW::new(self, 12)
    }
    #[doc = "Bit 19 - PB9 pin high current capability enable"]
    #[inline(always)]
    #[must_use]
    pub fn pb9_hcce(&mut self) -> Pb9HcceW<Cfg0Spec> {
        Pb9HcceW::new(self, 19)
    }
}
#[doc = "System configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Spec;
impl crate::RegisterSpec for Cfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0::R`](R) reader structure"]
impl crate::Readable for Cfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0::W`](W) writer structure"]
impl crate::Writable for Cfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0 to value 0"]
impl crate::Resettable for Cfg0Spec {
    const RESET_VALUE: u32 = 0;
}
