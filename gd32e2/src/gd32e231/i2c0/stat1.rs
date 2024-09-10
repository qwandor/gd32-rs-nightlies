#[doc = "Register `STAT1` reader"]
pub type R = crate::R<Stat1Spec>;
#[doc = "Master/slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Master {
    #[doc = "0: Slave mode"]
    Slave = 0,
    #[doc = "1: Master mode"]
    Master = 1,
}
impl From<Master> for bool {
    #[inline(always)]
    fn from(variant: Master) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASTER` reader - Master/slave"]
pub type MasterR = crate::BitReader<Master>;
impl MasterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Master {
        match self.bits {
            false => Master::Slave,
            true => Master::Master,
        }
    }
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == Master::Slave
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == Master::Master
    }
}
#[doc = "Bus busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2cbsy {
    #[doc = "0: No I2C communication"]
    NotBusy = 0,
    #[doc = "1: I2C communication active"]
    Busy = 1,
}
impl From<I2cbsy> for bool {
    #[inline(always)]
    fn from(variant: I2cbsy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2CBSY` reader - Bus busy"]
pub type I2cbsyR = crate::BitReader<I2cbsy>;
impl I2cbsyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2cbsy {
        match self.bits {
            false => I2cbsy::NotBusy,
            true => I2cbsy::Busy,
        }
    }
    #[doc = "No I2C communication"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == I2cbsy::NotBusy
    }
    #[doc = "I2C communication active"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == I2cbsy::Busy
    }
}
#[doc = "Transmitter/receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tr {
    #[doc = "0: Receiver"]
    Receiver = 0,
    #[doc = "1: Transmitter"]
    Transmitter = 1,
}
impl From<Tr> for bool {
    #[inline(always)]
    fn from(variant: Tr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TR` reader - Transmitter/receiver"]
pub type TrR = crate::BitReader<Tr>;
impl TrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tr {
        match self.bits {
            false => Tr::Receiver,
            true => Tr::Transmitter,
        }
    }
    #[doc = "Receiver"]
    #[inline(always)]
    pub fn is_receiver(&self) -> bool {
        *self == Tr::Receiver
    }
    #[doc = "Transmitter"]
    #[inline(always)]
    pub fn is_transmitter(&self) -> bool {
        *self == Tr::Transmitter
    }
}
#[doc = "General call address (Slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxgc {
    #[doc = "0: No general call address received"]
    NotReceived = 0,
    #[doc = "1: General call address received"]
    Received = 1,
}
impl From<Rxgc> for bool {
    #[inline(always)]
    fn from(variant: Rxgc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXGC` reader - General call address (Slave mode)"]
pub type RxgcR = crate::BitReader<Rxgc>;
impl RxgcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxgc {
        match self.bits {
            false => Rxgc::NotReceived,
            true => Rxgc::Received,
        }
    }
    #[doc = "No general call address received"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == Rxgc::NotReceived
    }
    #[doc = "General call address received"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == Rxgc::Received
    }
}
#[doc = "SMBus device default address (Slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Defsmb {
    #[doc = "0: Default address has not been received"]
    NotReceived = 0,
    #[doc = "1: Default address has been received"]
    Received = 1,
}
impl From<Defsmb> for bool {
    #[inline(always)]
    fn from(variant: Defsmb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEFSMB` reader - SMBus device default address (Slave mode)"]
pub type DefsmbR = crate::BitReader<Defsmb>;
impl DefsmbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Defsmb {
        match self.bits {
            false => Defsmb::NotReceived,
            true => Defsmb::Received,
        }
    }
    #[doc = "Default address has not been received"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == Defsmb::NotReceived
    }
    #[doc = "Default address has been received"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == Defsmb::Received
    }
}
#[doc = "SMBus host header (Slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hstsmb {
    #[doc = "0: No SMBus host header detected"]
    NoHeader = 0,
    #[doc = "1: SMBus host header detected"]
    Header = 1,
}
impl From<Hstsmb> for bool {
    #[inline(always)]
    fn from(variant: Hstsmb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSTSMB` reader - SMBus host header (Slave mode)"]
pub type HstsmbR = crate::BitReader<Hstsmb>;
impl HstsmbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hstsmb {
        match self.bits {
            false => Hstsmb::NoHeader,
            true => Hstsmb::Header,
        }
    }
    #[doc = "No SMBus host header detected"]
    #[inline(always)]
    pub fn is_no_header(&self) -> bool {
        *self == Hstsmb::NoHeader
    }
    #[doc = "SMBus host header detected"]
    #[inline(always)]
    pub fn is_header(&self) -> bool {
        *self == Hstsmb::Header
    }
}
#[doc = "Dual flag (Slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dumodf {
    #[doc = "0: The address matches SADDR0"]
    Saddr0 = 0,
    #[doc = "1: The address matches SADDR1"]
    Saddr1 = 1,
}
impl From<Dumodf> for bool {
    #[inline(always)]
    fn from(variant: Dumodf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DUMODF` reader - Dual flag (Slave mode)"]
pub type DumodfR = crate::BitReader<Dumodf>;
impl DumodfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dumodf {
        match self.bits {
            false => Dumodf::Saddr0,
            true => Dumodf::Saddr1,
        }
    }
    #[doc = "The address matches SADDR0"]
    #[inline(always)]
    pub fn is_saddr0(&self) -> bool {
        *self == Dumodf::Saddr0
    }
    #[doc = "The address matches SADDR1"]
    #[inline(always)]
    pub fn is_saddr1(&self) -> bool {
        *self == Dumodf::Saddr1
    }
}
#[doc = "Field `PECV` reader - Packet error checking register"]
pub type PecvR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Master/slave"]
    #[inline(always)]
    pub fn master(&self) -> MasterR {
        MasterR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bus busy"]
    #[inline(always)]
    pub fn i2cbsy(&self) -> I2cbsyR {
        I2cbsyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter/receiver"]
    #[inline(always)]
    pub fn tr(&self) -> TrR {
        TrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - General call address (Slave mode)"]
    #[inline(always)]
    pub fn rxgc(&self) -> RxgcR {
        RxgcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SMBus device default address (Slave mode)"]
    #[inline(always)]
    pub fn defsmb(&self) -> DefsmbR {
        DefsmbR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SMBus host header (Slave mode)"]
    #[inline(always)]
    pub fn hstsmb(&self) -> HstsmbR {
        HstsmbR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Dual flag (Slave mode)"]
    #[inline(always)]
    pub fn dumodf(&self) -> DumodfR {
        DumodfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Packet error checking register"]
    #[inline(always)]
    pub fn pecv(&self) -> PecvR {
        PecvR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Transfer status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stat1Spec;
impl crate::RegisterSpec for Stat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat1::R`](R) reader structure"]
impl crate::Readable for Stat1Spec {}
#[doc = "`reset()` method sets STAT1 to value 0"]
impl crate::Resettable for Stat1Spec {
    const RESET_VALUE: u32 = 0;
}
