#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Field `RBNE` reader - Receive Buffer Not Empty"]
pub type RbneR = crate::BitReader;
#[doc = "Field `TBE` reader - Transmit Buffer Empty"]
pub type TbeR = crate::BitReader;
#[doc = "Field `I2SCH` reader - I2S channel side"]
pub type I2schR = crate::BitReader;
#[doc = "Field `TXURERR` reader - Transmission underrun error bit"]
pub type TxurerrR = crate::BitReader;
#[doc = "Field `CRCERR` reader - SPI CRC Error Bit"]
pub type CrcerrR = crate::BitReader;
#[doc = "Field `CRCERR` writer - SPI CRC Error Bit"]
pub type CrcerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONFERR` reader - SPI Configuration error"]
pub type ConferrR = crate::BitReader;
#[doc = "Field `RXORERR` reader - Reception Overrun Error Bit"]
pub type RxorerrR = crate::BitReader;
#[doc = "Field `TRANS` reader - Transmitting On-going Bit"]
pub type TransR = crate::BitReader;
#[doc = "Field `FERR` reader - Format Error"]
pub type FerrR = crate::BitReader;
#[doc = "Field `FERR` writer - Format Error"]
pub type FerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive Buffer Not Empty"]
    #[inline(always)]
    pub fn rbne(&self) -> RbneR {
        RbneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Buffer Empty"]
    #[inline(always)]
    pub fn tbe(&self) -> TbeR {
        TbeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2S channel side"]
    #[inline(always)]
    pub fn i2sch(&self) -> I2schR {
        I2schR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission underrun error bit"]
    #[inline(always)]
    pub fn txurerr(&self) -> TxurerrR {
        TxurerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI CRC Error Bit"]
    #[inline(always)]
    pub fn crcerr(&self) -> CrcerrR {
        CrcerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI Configuration error"]
    #[inline(always)]
    pub fn conferr(&self) -> ConferrR {
        ConferrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reception Overrun Error Bit"]
    #[inline(always)]
    pub fn rxorerr(&self) -> RxorerrR {
        RxorerrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmitting On-going Bit"]
    #[inline(always)]
    pub fn trans(&self) -> TransR {
        TransR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Format Error"]
    #[inline(always)]
    pub fn ferr(&self) -> FerrR {
        FerrR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - SPI CRC Error Bit"]
    #[inline(always)]
    #[must_use]
    pub fn crcerr(&mut self) -> CrcerrW<StatSpec> {
        CrcerrW::new(self, 4)
    }
    #[doc = "Bit 8 - Format Error"]
    #[inline(always)]
    #[must_use]
    pub fn ferr(&mut self) -> FerrW<StatSpec> {
        FerrW::new(self, 8)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0x02"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0x02;
}
