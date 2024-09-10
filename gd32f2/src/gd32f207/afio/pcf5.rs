#[doc = "Register `PCF5` reader"]
pub type R = crate::R<Pcf5Spec>;
#[doc = "Register `PCF5` writer"]
pub type W = crate::W<Pcf5Spec>;
#[doc = "Field `I2C2_REMAP0` reader - I2C2 remapping 0"]
pub type I2c2Remap0R = crate::BitReader;
#[doc = "Field `I2C2_REMAP0` writer - I2C2 remapping 0"]
pub type I2c2Remap0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_REMAP1` reader - I2C2 remapping 1"]
pub type I2c2Remap1R = crate::BitReader;
#[doc = "Field `I2C2_REMAP1` writer - I2C2 remapping 1"]
pub type I2c2Remap1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_CH0_REMAP` reader - TIMER1_CH0 remapping"]
pub type Timer1Ch0RemapR = crate::BitReader;
#[doc = "Field `TIMER1_CH0_REMAP` writer - TIMER1_CH0 remapping"]
pub type Timer1Ch0RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER4_REMAP` reader - TIMER4 remapping"]
pub type Timer4RemapR = crate::BitReader;
#[doc = "Field `TIMER4_REMAP` writer - TIMER4 remapping"]
pub type Timer4RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER7_CHON_REMAP` reader - TIMER7_CHON remapping"]
pub type Timer7ChonRemapR = crate::FieldReader;
#[doc = "Field `TIMER7_CHON_REMAP` writer - TIMER7_CHON remapping"]
pub type Timer7ChonRemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMER7_CH_REMAP` reader - TIMER7_CH remapping"]
pub type Timer7ChRemapR = crate::BitReader;
#[doc = "Field `TIMER7_CH_REMAP` writer - TIMER7_CH remapping"]
pub type Timer7ChRemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_REMAP` reader - I2C1 remapping"]
pub type I2c1RemapR = crate::FieldReader;
#[doc = "Field `I2C1_REMAP` writer - I2C1 remapping"]
pub type I2c1RemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI1_NSCK_REMAP` reader - SPI1_NSCK remapping"]
pub type Spi1NsckRemapR = crate::FieldReader;
#[doc = "Field `SPI1_NSCK_REMAP` writer - SPI1_NSCK remapping"]
pub type Spi1NsckRemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI1_IO_REMAP` reader - SPI1_IO remapping"]
pub type Spi1IoRemapR = crate::FieldReader;
#[doc = "Field `SPI1_IO_REMAP` writer - SPI1_IO remapping"]
pub type Spi1IoRemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART3_REMAP` reader - UART3 remapping"]
pub type Uart3RemapR = crate::BitReader;
#[doc = "Field `UART3_REMAP` writer - UART3 remapping"]
pub type Uart3RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER11_REMAP` reader - TIMER11 remapping"]
pub type Timer11RemapR = crate::BitReader;
#[doc = "Field `TIMER11_REMAP` writer - TIMER11 remapping"]
pub type Timer11RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN0_ADD_REMAP` reader - CAN0_ADD remapping"]
pub type Can0AddRemapR = crate::BitReader;
#[doc = "Field `CAN0_ADD_REMAP` writer - CAN0_ADD remapping"]
pub type Can0AddRemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENET_TXD3_REMAP` reader - ENET _TXD3 remapping"]
pub type EnetTxd3RemapR = crate::BitReader;
#[doc = "Field `ENET_TXD3_REMAP` writer - ENET _TXD3 remapping"]
pub type EnetTxd3RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPS_HI_REMAP` reader - PPS_HI remapping"]
pub type PpsHiRemapR = crate::BitReader;
#[doc = "Field `PPS_HI_REMAP` writer - PPS_HI remapping"]
pub type PpsHiRemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENET_TXD01_REMAP` reader - ENET_TXD01 remapping"]
pub type EnetTxd01RemapR = crate::BitReader;
#[doc = "Field `ENET_TXD01_REMAP` writer - ENET_TXD01 remapping"]
pub type EnetTxd01RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENET_CRSCOL_REMAP` reader - ENET_CRSCOL remapping"]
pub type EnetCrscolRemapR = crate::BitReader;
#[doc = "Field `ENET_CRSCOL_REMAP` writer - ENET_CRSCOL remapping"]
pub type EnetCrscolRemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENET_RX_HI_REMAP` reader - ENET _RX_HI remapping"]
pub type EnetRxHiRemapR = crate::BitReader;
#[doc = "Field `ENET_RX_HI_REMAP` writer - ENET _RX_HI remapping"]
pub type EnetRxHiRemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART6_REMAP` reader - UART6 remapping"]
pub type Uart6RemapR = crate::BitReader;
#[doc = "Field `UART6_REMAP` writer - UART6 remapping"]
pub type Uart6RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART5_CK_REMAP` reader - USART5_CK remapping"]
pub type Usart5CkRemapR = crate::BitReader;
#[doc = "Field `USART5_CK_REMAP` writer - USART5_CK remapping"]
pub type Usart5CkRemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART5_RTS_REMAP` reader - USART5_RTS remapping"]
pub type Usart5RtsRemapR = crate::BitReader;
#[doc = "Field `USART5_RTS_REMAP` writer - USART5_RTS remapping"]
pub type Usart5RtsRemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART5_CTS_REMAP` reader - USART5_CTS remapping"]
pub type Usart5CtsRemapR = crate::BitReader;
#[doc = "Field `USART5_CTS_REMAP` writer - USART5_CTS remapping"]
pub type Usart5CtsRemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART5_TX_REMAP` reader - USART5_TX remapping"]
pub type Usart5TxRemapR = crate::BitReader;
#[doc = "Field `USART5_TX_REMAP` writer - USART5_TX remapping"]
pub type Usart5TxRemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART5_RX_REMAP` reader - USART5_RX remapping"]
pub type Usart5RxRemapR = crate::BitReader;
#[doc = "Field `USART5_RX_REMAP` writer - USART5_RX remapping"]
pub type Usart5RxRemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXMC_SDNWE_REMAP` reader - EXMC_SDNWE remapping"]
pub type ExmcSdnweRemapR = crate::BitReader;
#[doc = "Field `EXMC_SDNWE_REMAP` writer - EXMC_SDNWE remapping"]
pub type ExmcSdnweRemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXMC_SDCKE0_REMAP` reader - EXMC_SDCKE0 remapping"]
pub type ExmcSdcke0RemapR = crate::BitReader;
#[doc = "Field `EXMC_SDCKE0_REMAP` writer - EXMC_SDCKE0 remapping"]
pub type ExmcSdcke0RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXMC_SDCKE1_REMAP` reader - EXMC_SDCKE1 remapping"]
pub type ExmcSdcke1RemapR = crate::BitReader;
#[doc = "Field `EXMC_SDCKE1_REMAP` writer - EXMC_SDCKE1 remapping"]
pub type ExmcSdcke1RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXMC_SDNE0_REMAP` reader - EXMC_SDNE0 remapping"]
pub type ExmcSdne0RemapR = crate::BitReader;
#[doc = "Field `EXMC_SDNE0_REMAP` writer - EXMC_SDNE0 remapping"]
pub type ExmcSdne0RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXMC_SDNE1_REMAP` reader - EXMC_SDNE1 remapping"]
pub type ExmcSdne1RemapR = crate::BitReader;
#[doc = "Field `EXMC_SDNE1_REMAP` writer - EXMC_SDNE1 remapping"]
pub type ExmcSdne1RemapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C2 remapping 0"]
    #[inline(always)]
    pub fn i2c2_remap0(&self) -> I2c2Remap0R {
        I2c2Remap0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C2 remapping 1"]
    #[inline(always)]
    pub fn i2c2_remap1(&self) -> I2c2Remap1R {
        I2c2Remap1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIMER1_CH0 remapping"]
    #[inline(always)]
    pub fn timer1_ch0_remap(&self) -> Timer1Ch0RemapR {
        Timer1Ch0RemapR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIMER4 remapping"]
    #[inline(always)]
    pub fn timer4_remap(&self) -> Timer4RemapR {
        Timer4RemapR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - TIMER7_CHON remapping"]
    #[inline(always)]
    pub fn timer7_chon_remap(&self) -> Timer7ChonRemapR {
        Timer7ChonRemapR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - TIMER7_CH remapping"]
    #[inline(always)]
    pub fn timer7_ch_remap(&self) -> Timer7ChRemapR {
        Timer7ChRemapR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - I2C1 remapping"]
    #[inline(always)]
    pub fn i2c1_remap(&self) -> I2c1RemapR {
        I2c1RemapR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - SPI1_NSCK remapping"]
    #[inline(always)]
    pub fn spi1_nsck_remap(&self) -> Spi1NsckRemapR {
        Spi1NsckRemapR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - SPI1_IO remapping"]
    #[inline(always)]
    pub fn spi1_io_remap(&self) -> Spi1IoRemapR {
        Spi1IoRemapR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - UART3 remapping"]
    #[inline(always)]
    pub fn uart3_remap(&self) -> Uart3RemapR {
        Uart3RemapR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TIMER11 remapping"]
    #[inline(always)]
    pub fn timer11_remap(&self) -> Timer11RemapR {
        Timer11RemapR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CAN0_ADD remapping"]
    #[inline(always)]
    pub fn can0_add_remap(&self) -> Can0AddRemapR {
        Can0AddRemapR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ENET _TXD3 remapping"]
    #[inline(always)]
    pub fn enet_txd3_remap(&self) -> EnetTxd3RemapR {
        EnetTxd3RemapR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PPS_HI remapping"]
    #[inline(always)]
    pub fn pps_hi_remap(&self) -> PpsHiRemapR {
        PpsHiRemapR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ENET_TXD01 remapping"]
    #[inline(always)]
    pub fn enet_txd01_remap(&self) -> EnetTxd01RemapR {
        EnetTxd01RemapR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ENET_CRSCOL remapping"]
    #[inline(always)]
    pub fn enet_crscol_remap(&self) -> EnetCrscolRemapR {
        EnetCrscolRemapR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ENET _RX_HI remapping"]
    #[inline(always)]
    pub fn enet_rx_hi_remap(&self) -> EnetRxHiRemapR {
        EnetRxHiRemapR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - UART6 remapping"]
    #[inline(always)]
    pub fn uart6_remap(&self) -> Uart6RemapR {
        Uart6RemapR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - USART5_CK remapping"]
    #[inline(always)]
    pub fn usart5_ck_remap(&self) -> Usart5CkRemapR {
        Usart5CkRemapR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USART5_RTS remapping"]
    #[inline(always)]
    pub fn usart5_rts_remap(&self) -> Usart5RtsRemapR {
        Usart5RtsRemapR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - USART5_CTS remapping"]
    #[inline(always)]
    pub fn usart5_cts_remap(&self) -> Usart5CtsRemapR {
        Usart5CtsRemapR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - USART5_TX remapping"]
    #[inline(always)]
    pub fn usart5_tx_remap(&self) -> Usart5TxRemapR {
        Usart5TxRemapR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - USART5_RX remapping"]
    #[inline(always)]
    pub fn usart5_rx_remap(&self) -> Usart5RxRemapR {
        Usart5RxRemapR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - EXMC_SDNWE remapping"]
    #[inline(always)]
    pub fn exmc_sdnwe_remap(&self) -> ExmcSdnweRemapR {
        ExmcSdnweRemapR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - EXMC_SDCKE0 remapping"]
    #[inline(always)]
    pub fn exmc_sdcke0_remap(&self) -> ExmcSdcke0RemapR {
        ExmcSdcke0RemapR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - EXMC_SDCKE1 remapping"]
    #[inline(always)]
    pub fn exmc_sdcke1_remap(&self) -> ExmcSdcke1RemapR {
        ExmcSdcke1RemapR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - EXMC_SDNE0 remapping"]
    #[inline(always)]
    pub fn exmc_sdne0_remap(&self) -> ExmcSdne0RemapR {
        ExmcSdne0RemapR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - EXMC_SDNE1 remapping"]
    #[inline(always)]
    pub fn exmc_sdne1_remap(&self) -> ExmcSdne1RemapR {
        ExmcSdne1RemapR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C2 remapping 0"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_remap0(&mut self) -> I2c2Remap0W<Pcf5Spec> {
        I2c2Remap0W::new(self, 0)
    }
    #[doc = "Bit 1 - I2C2 remapping 1"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_remap1(&mut self) -> I2c2Remap1W<Pcf5Spec> {
        I2c2Remap1W::new(self, 1)
    }
    #[doc = "Bit 2 - TIMER1_CH0 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_ch0_remap(&mut self) -> Timer1Ch0RemapW<Pcf5Spec> {
        Timer1Ch0RemapW::new(self, 2)
    }
    #[doc = "Bit 3 - TIMER4 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn timer4_remap(&mut self) -> Timer4RemapW<Pcf5Spec> {
        Timer4RemapW::new(self, 3)
    }
    #[doc = "Bits 4:5 - TIMER7_CHON remapping"]
    #[inline(always)]
    #[must_use]
    pub fn timer7_chon_remap(&mut self) -> Timer7ChonRemapW<Pcf5Spec> {
        Timer7ChonRemapW::new(self, 4)
    }
    #[doc = "Bit 6 - TIMER7_CH remapping"]
    #[inline(always)]
    #[must_use]
    pub fn timer7_ch_remap(&mut self) -> Timer7ChRemapW<Pcf5Spec> {
        Timer7ChRemapW::new(self, 6)
    }
    #[doc = "Bits 7:8 - I2C1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_remap(&mut self) -> I2c1RemapW<Pcf5Spec> {
        I2c1RemapW::new(self, 7)
    }
    #[doc = "Bits 9:10 - SPI1_NSCK remapping"]
    #[inline(always)]
    #[must_use]
    pub fn spi1_nsck_remap(&mut self) -> Spi1NsckRemapW<Pcf5Spec> {
        Spi1NsckRemapW::new(self, 9)
    }
    #[doc = "Bits 11:12 - SPI1_IO remapping"]
    #[inline(always)]
    #[must_use]
    pub fn spi1_io_remap(&mut self) -> Spi1IoRemapW<Pcf5Spec> {
        Spi1IoRemapW::new(self, 11)
    }
    #[doc = "Bit 13 - UART3 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn uart3_remap(&mut self) -> Uart3RemapW<Pcf5Spec> {
        Uart3RemapW::new(self, 13)
    }
    #[doc = "Bit 14 - TIMER11 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn timer11_remap(&mut self) -> Timer11RemapW<Pcf5Spec> {
        Timer11RemapW::new(self, 14)
    }
    #[doc = "Bit 15 - CAN0_ADD remapping"]
    #[inline(always)]
    #[must_use]
    pub fn can0_add_remap(&mut self) -> Can0AddRemapW<Pcf5Spec> {
        Can0AddRemapW::new(self, 15)
    }
    #[doc = "Bit 16 - ENET _TXD3 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn enet_txd3_remap(&mut self) -> EnetTxd3RemapW<Pcf5Spec> {
        EnetTxd3RemapW::new(self, 16)
    }
    #[doc = "Bit 17 - PPS_HI remapping"]
    #[inline(always)]
    #[must_use]
    pub fn pps_hi_remap(&mut self) -> PpsHiRemapW<Pcf5Spec> {
        PpsHiRemapW::new(self, 17)
    }
    #[doc = "Bit 18 - ENET_TXD01 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn enet_txd01_remap(&mut self) -> EnetTxd01RemapW<Pcf5Spec> {
        EnetTxd01RemapW::new(self, 18)
    }
    #[doc = "Bit 19 - ENET_CRSCOL remapping"]
    #[inline(always)]
    #[must_use]
    pub fn enet_crscol_remap(&mut self) -> EnetCrscolRemapW<Pcf5Spec> {
        EnetCrscolRemapW::new(self, 19)
    }
    #[doc = "Bit 20 - ENET _RX_HI remapping"]
    #[inline(always)]
    #[must_use]
    pub fn enet_rx_hi_remap(&mut self) -> EnetRxHiRemapW<Pcf5Spec> {
        EnetRxHiRemapW::new(self, 20)
    }
    #[doc = "Bit 21 - UART6 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn uart6_remap(&mut self) -> Uart6RemapW<Pcf5Spec> {
        Uart6RemapW::new(self, 21)
    }
    #[doc = "Bit 22 - USART5_CK remapping"]
    #[inline(always)]
    #[must_use]
    pub fn usart5_ck_remap(&mut self) -> Usart5CkRemapW<Pcf5Spec> {
        Usart5CkRemapW::new(self, 22)
    }
    #[doc = "Bit 23 - USART5_RTS remapping"]
    #[inline(always)]
    #[must_use]
    pub fn usart5_rts_remap(&mut self) -> Usart5RtsRemapW<Pcf5Spec> {
        Usart5RtsRemapW::new(self, 23)
    }
    #[doc = "Bit 24 - USART5_CTS remapping"]
    #[inline(always)]
    #[must_use]
    pub fn usart5_cts_remap(&mut self) -> Usart5CtsRemapW<Pcf5Spec> {
        Usart5CtsRemapW::new(self, 24)
    }
    #[doc = "Bit 25 - USART5_TX remapping"]
    #[inline(always)]
    #[must_use]
    pub fn usart5_tx_remap(&mut self) -> Usart5TxRemapW<Pcf5Spec> {
        Usart5TxRemapW::new(self, 25)
    }
    #[doc = "Bit 26 - USART5_RX remapping"]
    #[inline(always)]
    #[must_use]
    pub fn usart5_rx_remap(&mut self) -> Usart5RxRemapW<Pcf5Spec> {
        Usart5RxRemapW::new(self, 26)
    }
    #[doc = "Bit 27 - EXMC_SDNWE remapping"]
    #[inline(always)]
    #[must_use]
    pub fn exmc_sdnwe_remap(&mut self) -> ExmcSdnweRemapW<Pcf5Spec> {
        ExmcSdnweRemapW::new(self, 27)
    }
    #[doc = "Bit 28 - EXMC_SDCKE0 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn exmc_sdcke0_remap(&mut self) -> ExmcSdcke0RemapW<Pcf5Spec> {
        ExmcSdcke0RemapW::new(self, 28)
    }
    #[doc = "Bit 29 - EXMC_SDCKE1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn exmc_sdcke1_remap(&mut self) -> ExmcSdcke1RemapW<Pcf5Spec> {
        ExmcSdcke1RemapW::new(self, 29)
    }
    #[doc = "Bit 30 - EXMC_SDNE0 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn exmc_sdne0_remap(&mut self) -> ExmcSdne0RemapW<Pcf5Spec> {
        ExmcSdne0RemapW::new(self, 30)
    }
    #[doc = "Bit 31 - EXMC_SDNE1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn exmc_sdne1_remap(&mut self) -> ExmcSdne1RemapW<Pcf5Spec> {
        ExmcSdne1RemapW::new(self, 31)
    }
}
#[doc = "AFIO port configuration register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcf5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcf5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcf5Spec;
impl crate::RegisterSpec for Pcf5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcf5::R`](R) reader structure"]
impl crate::Readable for Pcf5Spec {}
#[doc = "`write(|w| ..)` method takes [`pcf5::W`](W) writer structure"]
impl crate::Writable for Pcf5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCF5 to value 0"]
impl crate::Resettable for Pcf5Spec {
    const RESET_VALUE: u32 = 0;
}
